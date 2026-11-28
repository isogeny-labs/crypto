"""Numerical theta machinery for the affine-twist study (double precision thetas,
period matrix from the frozen g3_transfer_gates theta_arm.py, imported read-only)."""
import cmath
import importlib.util
import itertools
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_ARM = os.path.join(HERE, "..", "g3_transfer_gates_20261120", "theta_arm.py")
_spec = importlib.util.spec_from_file_location("theta_arm", _ARM)
theta_arm = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(theta_arm)

BITS3 = list(itertools.product((0, 1), repeat=3))
CHARS = [(a, b) for a in BITS3 for b in BITS3]
EVEN = [(a, b) for (a, b) in CHARS if sum(x * y for x, y in zip(a, b)) % 2 == 0]


def thetas(T, R=None):
    """All 64 theta[a/2; b/2](0, T), a,b bit-vectors. T: 3x3 complex nested list."""
    im = [[T[i][j].imag for j in range(3)] for i in range(3)]
    lam = min(sum(v[i] * im[i][j] * v[j] for i in range(3) for j in range(3))
              for v in theta_arm._unit_vectors())
    if R is None:
        R = int(math.ceil(math.sqrt(16.0 / max(lam, 1e-3)))) + 2
    out = {}
    for a in BITS3:
        sums = {b: 0j for b in BITS3}
        for n in itertools.product(range(-R, R + 1), repeat=3):
            v = [n[i] + a[i] / 2 for i in range(3)]
            Q = sum(v[i] * T[i][j] * v[j] for i in range(3) for j in range(3))
            E = cmath.exp(1j * math.pi * Q)
            for b in BITS3:
                ph = cmath.exp(1j * math.pi * sum(v[k] * b[k] for k in range(3)))
                sums[b] += E * ph
        for b in BITS3:
            out[(a, b)] = sums[b]
    return out


def reduce_char(A, G):
    """theta[A/2; G/2] = sign * theta[A'/2; G'/2], A', G' bits. Returns (A', G', sign)."""
    A2 = tuple(x % 2 for x in A)
    G2 = tuple(x % 2 for x in G)
    m = [(g - g2) // 2 for g, g2 in zip(G, G2)]
    s = sum(a * mm for a, mm in zip(A2, m)) % 2
    return A2, G2, (-1 if s else 1)
