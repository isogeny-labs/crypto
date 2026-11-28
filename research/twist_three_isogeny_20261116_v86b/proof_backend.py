import ctypes,hashlib
from pathlib import Path
PROOF_DOCUMENTATION="https://flintlib.org/doc/fmpz.html#c.fmpz_is_prime"
def _library_path(supplied):return Path(supplied).resolve()

def prove_prime(integer, flint_library=None):
    """Use the documented deterministic proof entry point; retain provenance."""
    if isinstance(integer, bool) or not isinstance(integer, int) or integer < 2:
        raise ValueError('Prime candidate must be an integer at least two')
    if ctypes.sizeof(ctypes.c_long) != 8:
        raise RuntimeError('This version of the FLINT binding requires a 64-bit long ABI')
    path = _library_path(flint_library)
    library = ctypes.CDLL(str(path))
    pointer = ctypes.POINTER(ctypes.c_long)
    library.fmpz_init.argtypes = [pointer]
    library.fmpz_init.restype = None
    library.fmpz_set_str.argtypes = [pointer, ctypes.c_char_p, ctypes.c_int]
    library.fmpz_set_str.restype = ctypes.c_int
    library.fmpz_is_prime.argtypes = [pointer]
    library.fmpz_is_prime.restype = ctypes.c_int
    library.fmpz_clear.argtypes = [pointer]
    library.fmpz_clear.restype = None
    value = ctypes.c_long()
    library.fmpz_init(ctypes.byref(value))
    try:
        if library.fmpz_set_str(ctypes.byref(value), str(integer).encode('ascii'), 10):
            raise ValueError('FLINT rejected the prime candidate encoding')
        outcome = library.fmpz_is_prime(ctypes.byref(value))
    finally:
        library.fmpz_clear(ctypes.byref(value))
    if outcome != 1:
        raise ValueError('Exact FLINT primality proof rejected the candidate')
    return {'method': 'FLINT fmpz_is_prime deterministic proof', 'integer_hex': hex(integer), 'proven_prime': True, 'library_path': str(path), 'library_sha256': hashlib.sha256(path.read_bytes()).hexdigest(), 'documentation': PROOF_DOCUMENTATION, 'portable_pocklington_certificate_emitted': False}
