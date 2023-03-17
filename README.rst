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

.. _`cbor-diag crate`: https://crates.io/crates/cbor-diag
.. _cbor2: https://pypi.org/project/cbor2/
