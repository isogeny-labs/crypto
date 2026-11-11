#!/usr/bin/env python3
"""Exact check of Proposition S (NOTE.md sec. 5): the selector proof system.

For an unsatisfiable m=2 instance, let e_min be the least e for which there is a map
lambda: V -> K whose F2-coordinates are Boolean polynomials of degree <= e in the
coordinates c of C = X1 + X2, with, for every C in V,
   Tr(lambda(C) * Phi*_C(v_i)) = 0  (i = 1..k),   Phi*_C(A) = A^4 + (C^2+z)A^2 + zCA,
   Tr(lambda(C) * h_C) = 1,                       h_C = z^2 C^2 + a6.
Proposition S: this is equivalent to an identity 1 = sum_i g_i(c) f_i over the n
coordinate equations f_i with deg g_i <= e.  The script solves the linear system for
e = 0,1,... and prints e_min, next to dSEL from lfd_small (reduced generator basis).
Usage: python3 selector_check.py --n 11 --unsat 3
"""
import argparse, itertools
from lfd_small import make_instance, descend, reduced_basis, macaulay, least_D, popcount

def e_min(inst, maxe):
    K = inst['K']; n = inst['n']; V = inst['V']; k = len(V); z = inst['z']; a6 = inst['a6']
    span = [0] * (1 << k)
    for m in range(1, 1 << k):
        low = m & -m; span[m] = span[m ^ low] ^ V[low.bit_length() - 1]
    z2 = K.sq(z)
    trbasis = [K.tr(1 << j) for j in range(n)]  # not used directly
    def tr_row(w):  # bit j = Tr(e_j * w), e_j = x^j
        r = 0
        for j in range(n):
            if K.tr(K.mul(1 << j, w)): r |= 1 << j
        return r
    rows_at = []  # per C: list of (n-bit row over coordinates, rhs)
    for ci in range(1 << k):
        C = span[ci]
        a = K.sq(C) ^ z; b = K.mul(z, C)
        cons = []
        for v in V:
            w = K.sq(K.sq(v)) ^ K.mul(a, K.sq(v)) ^ K.mul(b, v)
            cons.append((tr_row(w), 0))
        h = K.mul(z2, K.sq(C)) ^ a6
        cons.append((tr_row(h), 1))
        rows_at.append(cons)
    for e in range(0, maxe + 1):
        mons = [sum(1 << i for i in c) for d in range(e + 1) for c in itertools.combinations(range(k), d)]
        M = len(mons)
        # unknown index: j*M + t ; lambda_j(C) = sum_t x[j,t] * mu_t(C)
        piv = {}; incons = False
        for ci in range(1 << k):
            mu = [1 if (m & ci) == m else 0 for m in mons]
            for (row, rhs) in rows_at[ci]:
                v = 0
                for j in range(n):
                    if row >> j & 1:
                        for t in range(M):
                            if mu[t]: v ^= 1 << (j * M + t)
                v = (v << 1) | rhs  # bit 0 = rhs
                while v >> 1:
                    p = v.bit_length() - 1
                    if p in piv: v ^= piv[p]
                    else: piv[p] = v; break
                else:
                    if v & 1: incons = True; break
            if incons: break
        if not incons: return e
    return None

def count_solutions(inst):
    # number of (A,C) in V x V with S(A,C)=0, via: for fixed C, S(.,C) = Phi*_C(A) + h_C is
    # F2-affine in A, so count = sum_C #{A in V: Phi*_C(A) = h_C} (exact linear algebra).
    K = inst['K']; V = inst['V']; k = len(V); z = inst['z']; a6 = inst['a6']
    span = [0] * (1 << k)
    for m in range(1, 1 << k):
        low = m & -m; span[m] = span[m ^ low] ^ V[low.bit_length() - 1]
    z2 = K.sq(z); total = 0
    for C in span:
        a = K.sq(C) ^ z; b = K.mul(z, C)
        imgs = [K.sq(K.sq(v)) ^ K.mul(a, K.sq(v)) ^ K.mul(b, v) for v in V]
        h = K.mul(z2, K.sq(C)) ^ a6
        piv = {}; rank = 0
        for w in imgs:
            while w:
                p = w.bit_length() - 1
                if p in piv: w ^= piv[p]
                else: piv[p] = w; rank += 1; break
        w = h
        while w:
            p = w.bit_length() - 1
            if p in piv: w ^= piv[p]
            else: break
        if w == 0: total += 1 << (k - rank)
    return total

def main():
    ap = argparse.ArgumentParser(); ap.add_argument('--n', type=int, default=9)
    ap.add_argument('--k', type=int, default=None)
    ap.add_argument('--unsat', type=int, default=3); ap.add_argument('--maxe', type=int, default=6)
    ap.add_argument('--no-dsel', action='store_true', help='skip the ANF/Macaulay dSEL cross-check (needs the 2^(2k) grid)')
    a = ap.parse_args(); found = 0
    for seed in itertools.count(1):
        if found >= a.unsat: break
        inst = make_instance(a.n, seed, a.k)
        if a.no_dsel:
            # unsatisfiability by direct count: S(A,C)=0 for some A,C in V?
            nz = count_solutions(inst)
            N = 2 * inst['k']
        else:
            polys, N, nz = descend(inst, 'ab')
        if nz: continue
        found += 1
        em = e_min(inst, a.maxe)
        dsel = None
        if not a.no_dsel:
            polc, _, _ = descend(inst, 'ac'); gc = reduced_basis(polc, N)
            dsel = least_D(lambda D: macaulay(gc, N, D, list(range(inst['k'], N)))[0], 1, a.maxe + 2)
        print(dict(n=a.n, k=inst['k'], seed=seed, e_min=em, e_min_plus_2=None if em is None else em + 2, dSEL=dsel), flush=True)

if __name__ == '__main__':
    main()
