from cbor_diag import *

def test_ellipsis():
    diagnostic = '[1, 2, ..., 99]'
    try:
        encoded = diag2cbor(diagnostic)
    except ValueError:
        pass
    else:
        raise RuntimeError("Expected error, received encoded {encoded.hex()}")

def test_inline_ellipsis():
    diagnostic = "h'00 11 .. ff'"
    try:
        encoded = diag2cbor(diagnostic)
    except ValueError:
        pass
    else:
        raise RuntimeError("Expected error, received encoded {encoded.hex()}")
