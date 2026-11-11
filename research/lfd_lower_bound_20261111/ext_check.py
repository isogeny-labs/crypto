#!/usr/bin/env python3
"""Sanity check of Lemma B (extension variables collapse the last fall degree to 2).

For an unsatisfiable m=2 instance F with d_F(F)=D, build Fhat over N + sum_{j=2..D} C(N,j)
variables: linearised generators fhat (each monomial m of degree>=2 replaced by y_m) and the
instance-independent axioms x_i*Y(m) + Y(m u {i}) for deg m <= D-1.  Lemma B says
1 in W_2(Fhat).  Usage: python3 ext_check.py --n 7 --unsat 3
"""
import argparse, itertools
from lfd_small import make_instance, descend, reduced_basis, closure, popcount, least_D

def build_ext(gens, N, D):
    mons = [m for d in range(2, D + 1) for m in
            (sum(1 << i for i in c) for c in itertools.combinations(range(N), d))]
    yidx = {m: N + j for j, m in enumerate(mons)}
    def Y(m):  # monomial (as mask over new variables) representing the old monomial m
        d = popcount(m)
        if d == 0: return 0
        if d == 1: return m
        return 1 << yidx[m]
    Np = N + len(mons)
    out = []
    for g in gens:  # linearisation
        acc = {}
        for m in g:
            t = Y(m); acc[t] = acc.get(t, 0) ^ 1
        out.append([t for t, c in acc.items() if c])
    nax = 0
    for d in range(0, D):
        for c in itertools.combinations(range(N), d):
            m = sum(1 << i for i in c)
            for i in range(N):
                a = (1 << i) | Y(m); b = Y(m | (1 << i))
                poly = [a] if a == b else [a, b]
                if a == b: continue  # x_i*x_i = x_i: trivial axiom
                out.append(poly); nax += 1
    return out, Np, nax

def main():
    ap = argparse.ArgumentParser(); ap.add_argument('--n', type=int, default=7)
    ap.add_argument('--unsat', type=int, default=3); a = ap.parse_args()
    found = 0
    for seed in itertools.count(1):
        if found >= a.unsat: break
        inst = make_instance(a.n, seed)
        polys, N, nz = descend(inst, 'ab')
        if nz: continue
        found += 1
        gens = reduced_basis(polys, N)
        dF = least_D(lambda D: closure(gens, N, D)[0], 1, 6)
        ext, Np, nax = build_ext(gens, N, dF)
        one2, dim2, _ = closure(ext, Np, 2)
        one1, _, _ = closure(ext, Np, 1)
        print(dict(n=a.n, seed=seed, N=N, dF=dF, ext_vars=Np, axioms=nax,
                   one_in_W1_ext=one1, one_in_W2_ext=one2, dimW2_ext=dim2), flush=True)

if __name__ == '__main__':
    main()
