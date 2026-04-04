# Changelog

## Unreleased

* Add support for CBOR sequences using `seq=True` argument
* Update cbor-edn dependency (no changes in behavior)
* Extend test coverage

## 1.1.3

* Add type stubs using pyo3-stub-gen
* Drop support for Python 3.9
* Extend documentation on cbor2 interaction and stability
* Update build dependencies (Maturin, PyO3)
* Minor refacteoring

## 1.1.2

* Package metadata is exported through maturin, populating PyPI presentation

## 1.1.1

* Editorial changes to documentation.
* Dependencies updated.
* Simplified code based on cbor-edn 0.0.8 enhancements.

## 1.1.0

* The backend is switched from cbor-diag to cbor-edn.

  This enables processing of diagnostic data in the latest draft
  version, and application oriented literals (eg. ip'2001:db8::/64').
  This change also simplifies the output, because encoding indicators
  are now only emitted where necessary for round-tripping.

* The conversion methods have arguments `from999` and `to999`, enabling
  applications to do their own processing of application-oriented
  literals.

* Maturin and PyO3 are updated.


## 1.0.3

* PyO3 updated to 0.22, therefore supporting Python 3.13
* CI fixes


## 1.0.2

* Maturin updated to version 1.5
* Updates to Cargo.lock

  This restores building when Cargo.lock is observed on the latest
  nightly versions due to proc-macro2's opportunistic use of nightly
  features.


## 1.0.1

* Documentation updates
* Updates to Cargo.lock

  While this would usually not be a relevant change in a library crate,
  this being a Python package to which ther Cargo.lock is input makes it
  relevant, especially as the updated cargo-diag crate pulls in a newer
  version of nom that does not use accidental features of rustc.
