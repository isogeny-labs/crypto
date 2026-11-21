#!/usr/bin/env python3
"""Independent check of a feasibility certificate written by emin.c (lambda_e<E>.bin).

Uses lfd_small's field arithmetic (not emin.c's) to check, for every C in V,
  Tr(lambda(C) * Phi*_C(v_i)) = 0 for i = 1..k  and  Tr(lambda(C) * h_C) = 1,
where lambda(C) is read as an element of GF(2^n) in the polynomial basis (bit j = x^j),
and that every coordinate of lambda has algebraic degree <= E (Moebius transform).
Usage: python3 -I verify_lambda.py N SEED E CERT.bin   (run from this directory)
"""
import sys, struct
sys.path.insert(0, '.')
from lfd_small import make_instance

n, seed, e, path = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
inst = make_instance(n, seed); K = inst['K']; V = inst['V']; k = len(V); z = inst['z']; a6 = inst['a6']
P = 1 << k
lam = list(struct.unpack('<%dQ' % P, open(path, 'rb').read()))
span = [0] * P
for m in range(1, P):
    low = m & -m; span[m] = span[m ^ low] ^ V[low.bit_length() - 1]
z2 = K.sq(z); bad = 0
for ci in range(P):
    C = span[ci]; L = lam[ci]
    a = K.sq(C) ^ z; b = K.mul(z, C)
    for v in V:
        w = K.sq(K.sq(v)) ^ K.mul(a, K.sq(v)) ^ K.mul(b, v)
        if K.tr(K.mul(L, w)) != 0: bad += 1
    if K.tr(K.mul(L, K.mul(z2, K.sq(C)) ^ a6)) != 1: bad += 1
# degree: ANF of each coordinate
anf = lam[:]
for i in range(k):
    bit = 1 << i
    for c in range(P):
        if c & bit: anf[c] ^= anf[c ^ bit]
deg = max((bin(c).count('1') for c in range(P) if anf[c]), default=-1)
print(dict(n=n, seed=seed, e=e, violated=bad, anf_degree=deg, ok=(bad == 0 and deg <= e)))
