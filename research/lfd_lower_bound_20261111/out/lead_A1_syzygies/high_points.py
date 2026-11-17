#!/usr/bin/env python3
"""Post-hoc (not preregistered): count C in V with dim A_C = k, i.e. Phi*_C has a 2-dim kernel
on V, and compare with the points C = X1 + X2 with X1, X2 in V, X1 X2 = z (roots of A^2+CA+z).
Usage: python3 high_points.py N SEED   (run from research/lfd_lower_bound_20261111)"""
import sys, json
sys.path.insert(0, '.')
from lfd_small import make_instance, rank_f2
n, seed = int(sys.argv[1]), int(sys.argv[2])
inst = make_instance(n, seed); K = inst['K']; V = inst['V']; z = inst['z']; k = len(V)
span = [0] * (1 << k)
for m in range(1, 1 << k):
    low = m & -m; span[m] = span[m ^ low] ^ V[low.bit_length() - 1]
Vset = set(span)
pairs = set()
for X1 in span[1:]:
    X2 = K.mul(z, K.inv(X1))
    if X2 in Vset: pairs.add(X1 ^ X2)
def phis(C, A):
    A2 = K.sq(A); return K.sq(A2) ^ K.mul(K.sq(C) ^ z, A2) ^ K.mul(K.mul(z, C), A)
high = [C for C in span if C and rank_f2([phis(C, v) for v in V]) == k - 2]
print(json.dumps({'n': n, 'seed': seed, 'C_from_X1X2_eq_z': len(pairs), 'C_with_rank_drop': len(high),
                  'same_set': sorted(pairs) == sorted(high), 'sqrt_z_in_V': K.pow(z, 1 << (n - 1)) in Vset}))
