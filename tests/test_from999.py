import cbor2
from cbor_diag import *

def test_regular():
    item = cbor2.CBORTag(999, ["foo", "bar"])
    encoded = cbor2.dumps(item)
    edn = cbor2diag(encoded, from999=True)
    assert edn == "foo'bar'"

def test_not_by_default():
    item = cbor2.CBORTag(999, ["foo", "bar"])
    encoded = cbor2.dumps(item)
    edn = cbor2diag(encoded)
    assert edn == '''999(["foo", "bar"])'''

def test_failing():
    bad = [
        cbor2.CBORTag(999, 1),
        cbor2.CBORTag(999, []),
        cbor2.CBORTag(999, ["foo"]),
        cbor2.CBORTag(999, ["foo", "bar", "baz"]),
        cbor2.CBORTag(999, [123, "bar"]),
        cbor2.CBORTag(999, ["foo", 123]),
    ]
    for item in bad:
        encoded = cbor2.dumps(item)
        edn = cbor2diag(encoded, from999=True)
        # It emits some comment, we don't check for which precisely.
        assert edn.startswith(cbor2diag(encoded) + '/ '), f"Item {item!r} was expected to leave the broken tag 999 and show an error inline, but produced {edn!r}"
