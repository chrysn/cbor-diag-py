=======================================
cbor-diag: Diagnostic notation for CBOR
=======================================

This Python module is a minimal wapper around the `cbor-diag crate`_.
Unlike the crate,
which offers lots of functionality for manipulating an AST,
this module only exposes two very simple functions:

* ``diag2cbor``, which parses diagnostic notation and produces a corresponding CBOR binary string, and
* ``cbor2diag``, which does the inverse.

It is recommended to use the cbor2_ package to actually process the CBOR data.

Maintenance
===========

This package is considered feature-complete at release,
and maintainened reactively --
when changes to the ecosystem threaten to make it unusable.

New features are only expected to be added
if they are already present in the underlying `cbor-diag crate`_,
and will likely manifest as extra arguments to ``cbor2diag``.

This package is built using maturin_ and pyo3_
and largely follows their template.
The built module is renamed from ``cbor_diag`` to ``_cbor_diag``
(and consequently wrapped manually)
to avoid it being part of the package's public API.

.. _`cbor-diag crate`: https://crates.io/crates/cbor-diag
.. _cbor2: https://pypi.org/project/cbor2/
.. _maturin: https://www.maturin.rs/
.. _pyo3: https://pyo3.rs/
