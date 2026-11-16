#!/usr/bin/env python3
"""Barrier check for the flat-restriction method (NOTE.md sec. 5.4).

For C in V let Rcheck_C = Phi*_C(V) + F2*h_C  (a subspace of K of dim <= k+1).
A degree-e selector restricted to an (e+1)-dimensional affine flat U of V sums to 0 over U,
so U alone obstructs degree e only if the intersection of Rcheck_C over C in U is nonzero.
This prints the distribution of dim(cap_{C in U} Rcheck_C) over random 1-flats and 2-flats.
Usage: python3 flat_check.py --n 15 --unsat 2 --samples 2000
"""
import argparse, itertools, random
from lfd_small import make_instance
from selector_check import count_solutions

def basis_of(vecs):
    piv = {}
    for w in vecs:
        while w:
            p = w.bit_length() - 1
            if p in piv: w ^= piv[p]
            else: piv[p] = w; break
    return list(piv.values())

def intersect(B1, B2, n):
    # intersection of two F2-subspaces of F2^n given by bases; via kernel of [B1; B2]
    rows = [(b, 1 << i) for i, b in enumerate(B1)] + [(b, 1 << (len(B1) + j)) for j, b in enumerate(B2)]
    piv = {}; out = []
    for v, tag in rows:
        while v:
            p = v.bit_length() - 1
            if p in piv: pv, pt = piv[p]; v ^= pv; tag ^= pt
            else: piv[p] = (v, tag); break
        if v == 0:  # dependency: sum of B1-part equals sum of B2-part
            w = 0
            for i, b in enumerate(B1):
                if tag >> i & 1: w ^= b
            out.append(w)
    return basis_of(out)

def main():
    ap = argparse.ArgumentParser(); ap.add_argument('--n', type=int, default=11)
    ap.add_argument('--unsat', type=int, default=2); ap.add_argument('--samples', type=int, default=2000)
    a = ap.parse_args(); found = 0; rng = random.Random(12345)
    for seed in itertools.count(1):
        if found >= a.unsat: break
        inst = make_instance(a.n, seed)
        if count_solutions(inst): continue
        found += 1
        K = inst['K']; V = inst['V']; k = len(V); z = inst['z']; a6 = inst['a6']; z2 = K.sq(z)
        span = [0] * (1 << k)
        for m in range(1, 1 << k):
            low = m & -m; span[m] = span[m ^ low] ^ V[low.bit_length() - 1]
        def Rc(C):
            aa = K.sq(C) ^ z; b = K.mul(z, C)
            return basis_of([K.sq(K.sq(v)) ^ K.mul(aa, K.sq(v)) ^ K.mul(b, v) for v in V] + [K.mul(z2, K.sq(C)) ^ a6])
        R = [Rc(C) for C in span]
        h1 = {}; h2 = {}
        for _ in range(a.samples):
            c0 = rng.randrange(1 << k); d1 = rng.randrange(1, 1 << k)
            I = intersect(R[c0], R[c0 ^ d1], a.n); h1[len(I)] = h1.get(len(I), 0) + 1
            d2 = rng.randrange(1, 1 << k)
            if d2 == d1: continue
            I2 = intersect(intersect(I, R[c0 ^ d2], a.n), R[c0 ^ d1 ^ d2], a.n)
            h2[len(I2)] = h2.get(len(I2), 0) + 1
        print(dict(n=a.n, k=k, seed=seed, dimRcheck=sorted({len(r) for r in R}),
                   dim_cap_1flat=dict(sorted(h1.items())), dim_cap_2flat=dict(sorted(h2.items()))), flush=True)

if __name__ == '__main__':
    main()
