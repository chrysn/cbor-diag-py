// As per pyo3-stub-gen documentation

use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = _cbor_diag::stub_info()?;
    stub.generate()?;
    Ok(())
}
