use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Given a string in CBOR diagnostic notation, produce its CBOR binary encoding.
///
/// >>> from cbor_diag import *
/// >>> diag = '{1: "hello"}'
/// >>> encoded = diag2cbor(diag)
/// >>> encoded.hex()
/// 'a1016568656c6c6f'
/// >>> import cbor2                # doctest: +SKIP
/// >>> cbor2.loads(encoded)        # doctest: +SKIP
/// {1: 'hello'}
#[pyfunction]
fn diag2cbor(py: Python<'_>, diagnostic: &str) -> PyResult<PyObject> {
    let bytes = cbor_diag_rs::parse_diag(diagnostic)
        .map_err(|e| match e {
            cbor_diag_rs::Error::Todo(s) => pyo3::exceptions::PyValueError::new_err(s),
        })?
        .to_bytes();
    Ok(PyBytes::new(py, &bytes).into())
}

/// Given a byte string containing encoded CBOR, produce some diagnostic notation.
///
/// >>> from cbor_diag import *
/// >>> encoded = bytes.fromhex('a1016568656c6c6f')
/// >>> cbor2diag(encoded)
/// '{1: "hello"}'
///
/// Key word arguments influence additional details:
///
/// * With ``pretty=False``, no space is left after colons, commas etc.
#[pyfunction(signature = (encoded, *, pretty=true))]
fn cbor2diag(_py: Python<'_>, encoded: &[u8], pretty: bool) -> PyResult<String> {
    let parsed = cbor_diag_rs::parse_bytes(encoded)
        .map_err(|e| match e {
            cbor_diag_rs::Error::Todo(s) => pyo3::exceptions::PyValueError::new_err(s),
        })?;
    if pretty {
        Ok(parsed.to_diag_pretty())
    } else {
        Ok(parsed.to_diag())
    }
}

/// cbor-diag
///
/// This module provides conversion functions between CBOR's diagnostic notation and its binary
/// representation.
///
/// See RFC8949_ for the definition of CBOR and its diagnostic notation.
///
/// For producing binary representations of CBOR, and for processing them, the cbor2_ package is
/// recommended.
///
/// .. _RFC8949: https://www.rfc-editor.org/rfc/rfc8949
/// .. _cbor2: https://pypi.org/project/cbor2/
#[pymodule]
fn _cbor_diag(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(diag2cbor, m)?)?;
    m.add_function(wrap_pyfunction!(cbor2diag, m)?)?;
    Ok(())
}
