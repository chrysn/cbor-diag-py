use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::gen_stub_pyfunction};

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
///
/// * With ``seq=True``, `CBOR sequences`_
///   are tolerated:
///
/// >>> diag2cbor("1, 2, 3", seq=True)
/// '\x01\x02\x03'
///
/// .. _`CBOR sequences`: https://datatracker.ietf.org/doc/html/rfc8742
#[gen_stub_pyfunction]
#[pyfunction(signature = (diagnostic, *, to999=false, seq=false))]
fn diag2cbor(py: Python<'_>, diagnostic: &str, to999: bool, seq: bool) -> PyResult<Py<PyBytes>> {
    let mut data = cbor_edn::Sequence::parse(diagnostic)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{}", e)))?;

    check_sequence_expectation(&data, seq)?;

    let mut transform = cbor_edn::Transformation::new().for_cbor_serialization();

    if to999 {
        transform = transform.to_999();
    }

    transform.apply_to(&mut data);

    let bytes = data
        .to_cbor()
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{}", e)))?;
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
/// * With `seq=True`, `CBOR sequences`_ are tolerated:
///
/// >>> print(cbor2diag('\x01\x02\x03', seq=True))
/// 1,
/// 2,
/// 3
/// <BLANKLINE>
///
/// .. _`CBOR sequences`: https://datatracker.ietf.org/doc/html/rfc8742
///
/// * With ``from999=True``, CBOR tag 999 will be rendered as application oriented literal. Unlike
///   other tags, this does not happen by default, as that tag is not intended to be used that way
///   by default.
///
/// >>> cbor2diag(bytes.fromhex("d9 03e7 82 63 666f6f 63 626172"), from999=True)
/// "foo'bar'"
///
/// * With ``incomplete=True``, CBOR that terminates mid-data is accepted, and expressed with
///   ellipses. For example, this is useful to show a transfer in progress:
///
/// >>> full = cbor2.dumps({"hello": "world"})
/// >>> partial = full[:10]
/// >>> print(cbor2diag(partial, incomplete=True))
/// {"hello": "wo" + ...}
#[gen_stub_pyfunction]
#[pyfunction(signature = (encoded, *, pretty=true, from999=false, seq=false, incomplete=false))]
fn cbor2diag(
    _py: Python<'_>,
    // Staying generic for compatibility (we do still accept a [int]), but declare just bytes.
    #[gen_stub(override_type(type_repr = "bytes"))] encoded: &[u8],
    pretty: bool,
    from999: bool,
    seq: bool,
    incomplete: bool,
) -> PyResult<String> {
    let parsed = if incomplete {
        cbor_edn::Sequence::from_cbor_possibly_incomplete(encoded)
    } else {
        cbor_edn::Sequence::from_cbor(encoded)
    };
    let mut parsed =
        parsed.map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{}", e)))?;

    // Treating everything as a sequence and just later checking if it's just one item has the
    // downside that we can't rely on _possibly_incomplete to use our prior knowledge; feeding that
    // knowledge back in by trimming off the trailing ellipsis.
    if incomplete
        && parsed
            .items()
            .nth(1)
            .is_some_and(|i| i.serialize() == "...")
    {
        parsed = cbor_edn::Sequence::new(parsed.items().take(1).cloned());
    }

    check_sequence_expectation(&parsed, seq)?;

    let mut transform = cbor_edn::Transformation::new();

    if pretty {
        transform = transform.pretty();
    } else {
        // Well that's a choice, we could also leave the defaults
        transform = transform.minify();
    }
    if from999 {
        transform = transform.from_999();
    }

    transform.apply_to(&mut parsed);

    Ok(parsed.serialize())
}

/// "Raises" a ValueError if the sequence is really a sequence (and not really just a single item),
/// unless `seq=true` (i.e., the user *asked* for sequence handling).
fn check_sequence_expectation(data: &cbor_edn::Sequence, seq: bool) -> PyResult<()> {
    if seq {
        return Ok(());
    }

    let count = data.items().count();
    if count == 1 {
        Ok(())
    } else {
        Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Expected single item, found sequence of {}",
            count
        )))
    }
}

/// This provides conversion functions between CBOR's diagnostic notation (EDN) and its binary
/// representation.
///
/// CBOR is a binary data format defined in RFC8949_, often used IoT and modern security
/// applications. Its diagnostic notation is a human readable form of it and looks similar to JSON
/// (of which it is a superset thereof), and is defined in `the edn-literals draft`_.
///
/// For producing binary representations of CBOR, and for processing them, the cbor2_ package's
/// `loads()`_ and `dumps()`_  functions is recommended. This is not done automatically because
/// those come with a variety of arguments that influence the representation in Python,
/// orthogonally to the diagnostic notation.
///
/// .. _RFC8949: https://www.rfc-editor.org/rfc/rfc8949
/// .. _`the edn-literals draft`: https://www.ietf.org/archive/id/draft-ietf-cbor-edn-literals-09.html
/// .. _cbor2: https://pypi.org/project/cbor2/
/// .. Styling those is just too much effort in ReST
/// .. _`loads()`: https://cbor2.readthedocs.io/en/latest/api.html#cbor2.loads
/// .. _`dumps()`: https://cbor2.readthedocs.io/en/latest/api.html#cbor2.dumps
///
/// Stability
/// ---------
///
/// This package aims to be semver stable, in that its API only changes in breaking ways when the
/// major version is increased.
///
/// However, this does so far only extend to the usage, and not to the outcome. Until there is a
/// published RFC, there *will* be changes to which values are accepted as EDN, and which EDN is
/// produced for some CBOR items. If this is not acceptable for your use case, please depend on a
/// specific minor version of this package. The behavior will not change over patch versions.
///
/// Until this note is removed, new minor versions may also changes to which (especially exotic)
/// values are accepted outside of changes to the specification.
///
/// Functions
/// ---------
#[pymodule]
fn _cbor_diag(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(diag2cbor, m)?)?;
    m.add_function(wrap_pyfunction!(cbor2diag, m)?)?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
