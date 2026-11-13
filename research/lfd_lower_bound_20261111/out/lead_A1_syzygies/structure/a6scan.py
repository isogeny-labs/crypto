#!/usr/bin/env python3
"""Addendum A: fixed (V, z), fresh a6'.  For each a6': satisfiability (emin count, independent C
code), d_I(V,z), dims of I_d, R_d, feasibility per d, whether R_{d_I} = RM(d_I+1).
Usage: python3 a6scan.py N SEED DRAWS  -> JSON lines."""
import json, os, random, subprocess, sys
import numpy as np
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from syzstruct import Sys, nullspace, rank, Mbin, Elim  # noqa: E402

EMIN = os.path.abspath(os.path.join(HERE, '..', '..', '..', 'emin'))

def analyse(S, dmax):
    k, P = S.k, S.P
    S.zeta = S.K.inv(S.K.sq(S.z)); S.t0 = S.K.tr(S.K.mul(S.a6, S.zeta))
    # h rows depend on a6: rebuild the last trace row
    for c in range(P):
        hc = S.hC(S.span[c]); S.W[c][k] = hc; S.R[c][k] = S.trrow(hc)
    S.Rnp = np.array(S.R, dtype=np.uint64)
    rec = {'dim_I': [], 'dim_R': [], 'feasible': [], 'R_full': []}
    Rcache = []
    dI = None
    one = S.bits_to_int(np.ones(P, dtype=np.uint8))
    for d in range(dmax + 1):
        S.layout(d)
        Ib = nullspace(S.rows('I'), S.ncols)
        Rints = [S.bits_to_int(S.tr_vals(S.evaluate(v))) for v in Ib]
        Rcache.append(Rints)
        E = Elim()
        for x in Rints: E.add(x)
        rec['dim_I'].append(len(Ib)); rec['dim_R'].append(E.rank)
        rec['feasible'].append(E.reduce(one) == 0)
        rec['R_full'].append(E.rank == Mbin(k, d + 1))
        if dI is None and len(Ib) != Mbin(k, d):
            dI = d
    rec['d_I'] = dI
    # Z = {C : h_C in Phi*_C(V)} (trace rows are an isomorphic image of W_C)
    Z = [c for c in range(P) if rank(S.R[c][:k]) == rank(S.R[c])]
    rec['Z_size'] = len(Z)
    # per d: does R_d vanish on Z, and dim of the vanishing ideal of Z in RM(d+1)
    rec['R_vanishes_on_Z'] = []
    rec['dim_ideal_Z_in_RM'] = []
    for d in range(dmax + 1):
        mons = [m for m in range(P) if bin(m).count('1') <= d + 1]
        ev = [sum(1 << zi for zi, c in enumerate(Z) if m & c == m) for m in mons]
        rec['dim_ideal_Z_in_RM'].append(len(mons) - rank(ev))
        rec['R_vanishes_on_Z'].append(all(not (Rcache[d][i] >> c & 1) for i in range(len(Rcache[d])) for c in Z))
    rec['e_min'] = next((d for d in range(dmax + 1) if rec['feasible'][d]), None)
    return rec

def main():
    n, seed, draws = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
    S0 = Sys(n, seed)
    inst = S0.inst; K = S0.K; z = S0.z; a2 = inst['a2']
    rng = random.Random(1000 * n + seed)
    a6s = [('orig', S0.a6)]
    while len(a6s) < draws + 1:
        a6 = rng.randrange(1, 1 << n)
        if K.tr(z ^ a2 ^ K.mul(a6, K.inv(K.sq(z)))) == 0:
            a6s.append(('draw', a6))
    for tag, a6 in a6s:
        line = ' '.join(map(str, [n, S0.k, inst['K'].poly, a6, z] + list(S0.V)))
        nz = int(subprocess.run([EMIN, 'count'], input=line, capture_output=True, text=True).stdout.split()[0])
        S = Sys(n, seed, a6=a6)
        rec = analyse(S, 4 if n <= 17 else 4)
        rec.update(n=n, seed=seed, tag=tag, a6=a6, solutions=nz, satisfiable=nz > 0)
        print(json.dumps(rec), flush=True)

if __name__ == '__main__':
    main()
