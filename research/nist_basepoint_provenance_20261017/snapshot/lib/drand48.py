"""POSIX drand48 / srand48, bit-exact (X_{n+1} = a X_n + c mod 2^48)."""
A, C, M = 0x5DEECE66D, 0xB, 1 << 48
MASK = M - 1

def srand48_state(seedval: int) -> int:
    # srand48: high 32 bits of X = low 32 bits of seedval, low 16 bits = 0x330E
    return ((seedval & 0xFFFFFFFF) << 16) | 0x330E

class Drand48:
    def __init__(self, seedval=None, state=None):
        self.x = srand48_state(seedval) if state is None else state & MASK
    def step(self) -> int:
        self.x = (A * self.x + C) & MASK
        return self.x
    def drand48(self) -> float:
        return self.step() / M
    def byte(self) -> int:
        # (int)(drand48()*256) == top 8 bits of the new state (exact: 48-bit / 2^48 * 256)
        return self.step() >> 40
    def bytes(self, n: int) -> bytes:
        return bytes(self.byte() for _ in range(n))

def seed_blocks(seedval: int, nblocks: int, blen: int = 20):
    g = Drand48(seedval)
    for i in range(1, nblocks + 1):
        yield i, g.bytes(blen)
