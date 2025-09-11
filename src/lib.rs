use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Given a string in CBOR diagnostic notation, produce its CBOR binary encoding.
///
/// >>> from cbor_diag import *
/// >>> diag = '{1: "hello"}'
/// >>> encoded = diag2cbor(diag)
/// >>> encoded.hex()
/// 'a1016568656c6c6f'
/// >>> import cbor2
/// >>> cbor2.loads(encoded)
/// {1: 'hello'}
///
/// Key word arguments influence additional details:
///
/// * With ``to999=True``, unknown application-oriented literals are kept in tag 999 for the
///   application to process further:
///
/// >>> cbor2.loads(diag2cbor("[1, spam'eggs']", to999=True))
/// [1, CBORTag(999, ['spam', 'eggs'])]
#[pyfunction(signature = (diagnostic, *, to999=false))]
fn diag2cbor(py: Python<'_>, diagnostic: &str, to999: bool) -> PyResult<Py<PyBytes>> {
    let mut data = cbor_edn::StandaloneItem::parse(diagnostic)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{}", e)))?;

    data.visit_application_literals(&mut cbor_edn::application::all_aol_to_item);
    if to999 {
        data.visit_application_literals(&mut cbor_edn::application::any_aol_to_tag999);
    }

    let bytes = data
        .to_cbor()
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{}", e)))?
        ;
    Ok(PyBytes::new(py, &bytes).into())
}

/// Given a byte string containing encoded CBOR, produce some diagnostic notation.
///
/// >>> from cbor_diag import *
/// >>> encoded = bytes.fromhex('a1016568656c6c6f')
/// >>> cbor2diag(encoded)
/// '{1: "hello"}'
///
/// By default, this recognizes several CBOR tags into application-oriented literals:
///
/// >>> encoded = bytes.fromhex("c105")
/// >>> cbor2diag(encoded)
/// "DT'1970-01-01T00:00:05+00:00'"
///
/// Key word arguments influence additional details:
///
/// * With ``pretty=False``, no space is left after colons, commas etc., and no
///   application-oriented literals are created:
///
/// >>> cbor2diag(encoded, pretty=False)
/// '1(5)'
/// >>> cbor2diag(cbor2.dumps([1, 2]), pretty=False)
/// '[1,2]'
///
/// * With ``from999=True``, CBOR tag 999 will be rendered as application oriented literal. Unlike
///   other tags, this does not happen by default, as that tag is not intended to be used that way
///   by default.
///
/// >>> cbor2diag(bytes.fromhex("d9 03e7 82 63 666f6f 63 626172"), from999=True)
/// "foo'bar'"
#[pyfunction(signature = (encoded, *, pretty=true, from999=false))]
fn cbor2diag(_py: Python<'_>, encoded: &[u8], pretty: bool, from999: bool) -> PyResult<String> {
    let mut parsed = cbor_edn::StandaloneItem::from_cbor(encoded)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{}", e)))?;
    if pretty {
        parsed.visit_tag(&mut cbor_edn::application::all_tag_prettify);
    }
    if from999 {
        parsed.visit_tag(&mut |tag, item: &mut cbor_edn::Item| {
            if tag != 999 {
                return Ok(());
            }
            let tagged = item.get_tagged().expect("Visitor promises this is true");
            let Ok(mut items) = tagged.item().get_array_items() else {
                return Err("should be array".into());
            };
            let Some(ident) = items.next() else {
                return Err("should contain 2 items".into());
            };
            let Some(value) = items.next() else {
                return Err("should contain 2 items".into());
            };
            let None = items.next() else {
                return Err("should contain 2 items".into());
            };
            drop(items);
            let ident = ident.get_string()
                .map_err(|_| "ident should be string")?;
            let value = value.get_string()
                .map_err(|_| "value should be string")?;
            let new_item = cbor_edn::Item::new_application_literal(&ident, &value)
                // I don't see how value could ever trigger anything here
                .map_err(|_| "ident string is unsuitable for application-oriented literal")?;
            *item = new_item;
            Ok(())
        });
    }
    if pretty {
        parsed.set_delimiters(cbor_edn::DelimiterPolicy::indented());
    } else {
        parsed.set_delimiters(cbor_edn::DelimiterPolicy::DiscardAll);
    }
    Ok(parsed.serialize())
}

/// This provides conversion functions between CBOR's diagnostic notation (EDN) and its binary
/// representation.
///
/// CBOR is a binary data format defined in RFC8949_, often used IoT and modern security
/// applications. Its diagnostic notation is a human readable form of it and looks similar to JSON
/// (of which it is a superset thereof), and is defined in `the edn-literals draft`_.
///
/// For producing binary representations of CBOR, and for processing them, the cbor2_ package is
/// recommended.
///
/// .. _RFC8949: https://www.rfc-editor.org/rfc/rfc8949
/// .. _`the edn-literals draft`: https://www.ietf.org/archive/id/draft-ietf-cbor-edn-literals-09.html
/// .. _cbor2: https://pypi.org/project/cbor2/
#[pymodule]
fn _cbor_diag(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(diag2cbor, m)?)?;
    m.add_function(wrap_pyfunction!(cbor2diag, m)?)?;
    Ok(())
}
