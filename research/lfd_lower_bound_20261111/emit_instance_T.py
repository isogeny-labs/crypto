#!/usr/bin/env python3
"""Print one T-type instance for emin.c (LEAD-A-selector-counting-law):
V a random k-dimensional subspace of ker Tr, z the abscissa of a curve point with
Tr(a6/z^2) = 0 (so ell_0 == 0 on V).  Same field and output format as emit_instance.py:
   n k poly a6 z v_1 ... v_k
Usage: python3 emit_instance_T.py N SEED K
"""
import random, sys
from lfd_small import GF, rank_f2

n, seed, k = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
rng = random.Random(1_000_003 * seed + 7)
K = GF(n)
a2 = rng.randrange(2)
while True:
    a6 = rng.randrange(1, 1 << n)
    z = rng.randrange(1, 1 << n)
    t = K.mul(a6, K.inv(K.sq(z)))
    if K.tr(t) == 0 and K.tr(z ^ a2 ^ t) == 0:   # Tr(a6/z^2) = 0 and z is a curve abscissa
        break
while True:
    V = []
    while len(V) < k:
        v = rng.randrange(1, 1 << n)
        if K.tr(v) == 0:
            V.append(v)
    if rank_f2(V) == k:
        break
print(n, k, K.poly, a6, z, *V)
