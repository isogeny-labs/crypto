#!/usr/bin/env python3
"""P1 stage 1 instance selection (PROTOCOL.md) -> prints one line per run: p a b t D kmin kmax tag"""
import sys, hashlib, os, numpy as np
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from unit_test import resultant_unit
def leg(x, p): return pow(x % p, (p - 1) // 2, p)
for p in (13313, 15361, 19457):
    for a in (1, 2):
        t = next(t for t in range(2, p) if leg(t, p) == p - 1 and (t**3 + a * t) % p and leg(t**3 + a * t, p) == 1)
        assert resultant_unit(p, a, 0, t, 9)['unit']
        for D, k0 in ((3, 2), (4, 4)): print(p, a, 0, t, D, k0, 9, f'unit-a{a}')
    rng = np.random.default_rng(int(hashlib.sha256(f'p1-{p}'.encode()).hexdigest()[:8], 16))
    while True:
        a, b, t = (int(x) for x in rng.integers(1, p, size=3))
        if (4 * a**3 + 27 * b**2) % p == 0: continue
        if not resultant_unit(p, a, b, t, 9)['unit']: break
    for D, k0 in ((3, 2), (4, 4)): print(p, a, b, t, D, k0, 9, 'proper')
