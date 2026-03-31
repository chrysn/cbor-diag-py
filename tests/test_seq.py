import cbor2
from cbor_diag import *

def test_empty_c2d():
    try:
        cbor2diag(b'')
    except ValueError:
        pass
    else:
        raise RuntimeError('Empty CBOR item should have raised an error.')

def test_empty_d2c():
    try:
        diag2cbor('/ just a comment /')
    except ValueError:
        pass
    else:
        raise RuntimeError('Empty CBOR item should have raised an error.')

def test_multiple_c2d():
    try:
        cbor2diag(b'\0\0')
    except ValueError:
        pass
    else:
        raise RuntimeError('Non-singular CBOR sequence should have raised an error.')

def test_multiple_d2c():
    try:
        diag2cbor('0, 0')
    except ValueError:
        pass
    else:
        raise RuntimeError('Non-singular CBOR sequence should have raised an error.')
