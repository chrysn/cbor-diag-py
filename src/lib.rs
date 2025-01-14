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
fn diag2cbor(py: Python<'_>, diagnostic: &str, to999: bool) -> PyResult<PyObject> {
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
/// * With ``from999=True`, CBOR tag 999 will be rendered as application oriented literal. Unlike
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
            // This is the most absurd effect of <https://codeberg.org/chrysn/cbor-edn/issues/16>:
            // We have to serialize the item, turn it into a sequence, and use that
            let tagged_serialized = tagged.serialize();
            let mut tagged_as_seq = cbor_edn::Sequence::parse(&tagged_serialized).unwrap();
            let tagged_again = tagged_as_seq.get_items_mut().next().unwrap();
            let Ok(mut items) = tagged_again.get_array_items() else {
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
            // This is crude for lack of <https://codeberg.org/chrysn/cbor-edn/issues/16>, but
            // shows how to work around missing parts
            let ident = ident.serialize();
            let value = value.serialize();
            if !ident.starts_with('"') || !value.starts_with('"') {
                return Err("should be strings".into());
            }
            let constructed = format!("{}'{}'", ident.trim_matches('"'), value.trim_matches('"'));
            drop(items);
            let mut constructed = cbor_edn::Sequence::parse(&constructed)
                .map_err(|_| "Might contain escapes that can't be processed yet")?;
            let constructed = constructed.get_items_mut().next().unwrap();
            // Can't use constructed yet because of
            // <https://codeberg.org/chrysn/cbor-edn/issues/17>
            let (ident, value) = constructed.get_application_literal().expect("Was parsed successfully from something roughly app literal shaped");
            *item = cbor_edn::Item::new_application_literal(&ident, &value).expect("Was just produced by parsing");
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

/// cbor-diag
///
/// This module provides conversion functions between CBOR's diagnostic notation (EDN) and its
/// binary representation.
///
/// See RFC8949_ for the definition of CBOR, and `the edn-literals draft`_ its diagnostic notation.
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
