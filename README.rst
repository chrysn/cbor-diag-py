=============================================
cbor-diag: Diagnostic notation (EDN) for CBOR
=============================================

This Python module is a minimal wapper around the `cbor-edn crate`_
(formally around the `cbor-diag crate`_).
Unlike those crates,
which offers lots of functionality for manipulating CBOR and its diagnostic notation,
this module only exposes two very simple functions:

* ``diag2cbor``, which parses diagnostic notation and produces a corresponding CBOR binary string, and
* ``cbor2diag``, which does the inverse.

It is recommended to use the cbor2_ package to actually process the CBOR data.

Documentation is available `on readthedocs`_.

Installation
============

This project can be installed `from PyPI`_;
binary wheels are available there for the common platforms.

Maintenance
===========

This package is considered feature-complete at release,
and maintainened reactively --
when changes to the ecosystem threaten to make it unusable.

New features are only expected to be added
if they are already present in the underlying `cbor-edn crate`_,
and will likely manifest as extra arguments to ``cbor2diag``.

This package is built using maturin_ and pyo3_
and largely follows their template.
The built module is renamed from ``cbor_diag`` to ``_cbor_diag``
(and consequently wrapped manually)
to avoid it being part of the package's public API.
(The need for the workaround is tracked `at maturin`_ and through there `in the typing module`_).

The package is currently hosted on GitHub at https://github.com/chrysn/cbor-diag-py
because maturin can `not yet`_ build pipelines for GitLab or codeberg.

License
=======

This package was written by Christian Amsüss <chrysn@fsfe.org>,
and is published under the terms of MIT_ or Apache-2.0_ license,
at the user's choice.

Special thanks to Nemo157 for providing the `cbor-diag crate`_,
the authors of the `peg crate`_ (which does cbor-edn's heavy lifting),
and Carsten Bormann for providing a PEG parser ready ABNF in `the edn-literals draft`_.

.. _`cbor-edn crate`: https://crates.io/crates/cbor-edn
.. _`cbor-diag crate`: https://crates.io/crates/cbor-diag
.. _cbor2: https://pypi.org/project/cbor2/
.. _`on readthedocs`: https://cbor-diag.readthedocs.io/
.. _`from PyPI`: https://pypi.org/project/cbor-diag/
.. _maturin: https://www.maturin.rs/
.. _pyo3: https://pyo3.rs/
.. _`at maturin`: https://github.com/PyO3/maturin/issues/1399
.. _`in the typing module`: https://github.com/python/typing/issues/1333
.. _`not yet`: https://github.com/PyO3/maturin/issues/1507
.. _MIT: https://spdx.org/licenses/MIT.html
.. _Apache-2.0: https://spdx.org/licenses/Apache-2.0.html
.. _`peg crate`: https://crates.io/crates/peg
.. _`the edn-literals draft`: https://www.ietf.org/archive/id/draft-ietf-cbor-edn-literals-16.html
