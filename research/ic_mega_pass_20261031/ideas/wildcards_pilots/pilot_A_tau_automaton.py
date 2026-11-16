#!/usr/bin/env python3
"""Pilot A (MP-wildcards-4): is the tau-adic low-weight base recognisable from the
bits of x in a normal basis?  (automaton / Myhill-Nerode complexity census)

K_0: y^2+xy=x^3+1 over F_{2^n}, P of prime order r.  tau = Frobenius (x->x^2).
S_w = { sum_{i in I} s_i tau^i P : |I| <= w, s_i = +-1 }  (Frobenius-stable,
negation-stable, tau-NAF weight <= w).  L_w = { x(R) : R in S_w } written in a
normal basis {alpha^{2^i}} so that tau acts as a cyclic bit shift.
For each prefix length j we record
  states_j = number of distinct residual sets  (nodes at level j of the minimal
             fixed-order DFA / ROBDD of the finite language),
  rank_j   = F_2 rank of the 2^j x 2^{n-j} incidence matrix (linear complexity).
Controls: (i) random set of the same size; (ii) random SHIFT-INVARIANT set
(union of random Frobenius orbits) of matched size, i.e. same symmetry.
"""
import json, sys, random, time, itertools
import numpy as np
import cypari2
pari = cypari2.Pari()
pari.allocatemem(2**29)

def ff_to_int(x, n=None):
    sx = str(x)
    if sx in ('0', '1'):
        return int(sx)
    pol = pari('(%s).pol' % sx)
    return int(pari.fromdigits(pari.Vec(pari.lift(pol)), 2))

def f2_rank_rows(rows_bits):
    piv = {}; r = 0
    for v in rows_bits:
        while v:
            h = v.bit_length() - 1
            if h in piv: v ^= piv[h]
            else: piv[h] = v; r += 1; break
    return r

def f2_solve_basis(cols, n):
    """cols: list of n ints (basis vectors as bitmasks). Return function mapping
    a vector to coordinates in that basis (as int bitmask), via inverse matrix."""
    # Build augmented rows: row i = (bit i of each basis vec) | identity
    M = []
    for i in range(n):
        row = 0
        for k, c in enumerate(cols):
            if (c >> i) & 1: row |= 1 << k
        M.append((row, 1 << i))
    # Gaussian elimination to get inverse
    piv = {}
    for row, e in M:
        while row:
            h = row.bit_length() - 1
            if h in piv:
                prow, pe = piv[h]; row ^= prow; e ^= pe
            else:
                piv[h] = (row, e); break
        else:
            raise ValueError("basis not independent")
    # back-substitute to reduced form: for each pivot h, eliminate h from others
    keys = sorted(piv.keys())
    for h in keys:
        prow, pe = piv[h]
        for h2 in keys:
            if h2 != h and (piv[h2][0] >> h) & 1:
                r2, e2 = piv[h2]; piv[h2] = (r2 ^ prow, e2 ^ pe)
    # now piv[h] = (1<<h, e_h): coordinate h of v = parity(v & e_h)
    E = [piv[h][1] for h in range(n)]
    def coords(v):
        out = 0
        for h in range(n):
            if bin(v & E[h]).count('1') & 1: out |= 1 << h
        return out
    return coords

def main():
    n = int(sys.argv[1]) if len(sys.argv) > 1 else 23
    wmax = int(sys.argv[2]) if len(sys.argv) > 2 else 3
    rng = random.Random(3)
    t0 = time.time()
    g = pari(f"ffgen(Mod(1,2)*ffinit(2,{n}),'g)")
    A1 = {11: 1, 17: 1}.get(n, 0); cof = 2 if A1 else 4
    E = pari.ellinit([1, g**0 if A1 else 0, 0, 0, g**0])
    card = int(pari.ellcard(E)); r = card // cof
    assert pari.isprime(r)
    while True:
        P = pari.ellmul(E, pari.random(E), cof)
        if P != 0 and int(pari.ellorder(E, P)) == r: break
    # normal element alpha: conjugates linearly independent
    while True:
        alpha = pari.random(g)
        conj = [alpha]
        for i in range(1, n): conj.append(conj[-1] ** 2)
        cols = [ff_to_int(c) for c in conj]
        if f2_rank_rows(list(cols)) == n: break
    coords = f2_solve_basis(cols, n)
    # sanity: tau acts as cyclic shift of coordinates
    x = pari.random(g)
    cx, cx2 = coords(ff_to_int(x)), coords(ff_to_int(x ** 2))
    rot = ((cx << 1) | (cx >> (n - 1))) & ((1 << n) - 1)
    assert rot == cx2, "normal basis shift check failed"
    # tau^i P
    tP = [P]
    for i in range(1, n):
        Q = tP[-1]
        tP.append(pari.Vec([Q[0] ** 2, Q[1] ** 2]))
    tPneg = [pari.ellneg(E, Q) for Q in tP]
    results = {"n": n, "r": r, "levels": {}}
    S = {}  # x-int -> weight
    S_pts = {0: None}
    cur = [(pari(0) * 0, ())]  # placeholder
    # build S_w incrementally by signed subsets
    pts_by_w = {0: {("O",): None}}
    seen_x = {}
    frontier = [(None, -1)]  # (point, last index)
    layer = [(None, -1)]
    for w in range(1, wmax + 1):
        new_layer = []
        for (Q, last) in layer:
            for i in range(last + 1, n):
                for T in (tP[i], tPneg[i]):
                    R = T if Q is None else pari.elladd(E, Q, T)
                    new_layer.append((R, i))
                    if R != 0:
                        xi = coords(ff_to_int(R[0]))
                        if xi not in seen_x: seen_x[xi] = w
        layer = new_layer
        L = sorted(seen_x.keys())
        size = len(L)
        # controls
        rand_set = set(rng.sample(range(1 << n), size))
        orb = set()
        while len(orb) < size:
            v = rng.getrandbits(n)
            for s in range(n):
                orb.add(((v << s) | (v >> (n - s))) & ((1 << n) - 1))
        orb = set(list(orb)[:size]) if len(orb) > size else orb
        prof = {}
        for name, Lset in [("tau_base", set(L)), ("random", rand_set), ("random_orbits", orb)]:
            levels = []
            for j in range(1, n):
                rows = {}
                for v in Lset:
                    pre = v >> (n - j); suf = v & ((1 << (n - j)) - 1)
                    rows[pre] = rows.get(pre, 0) | (1 << suf)
                states = len(set(rows.values()))
                rk = f2_rank_rows(list(rows.values()))
                levels.append(dict(j=j, states=states, rank=rk, nonzero_rows=len(rows)))
            prof[name] = levels
        results["levels"][w] = dict(size=size, density=size / (1 << n), profiles=prof)
        summ = {k: (max(l["states"] for l in v), max(l["rank"] for l in v)) for k, v in prof.items()}
        print(json.dumps(dict(w=w, size=size, max_states_and_rank=summ, secs=round(time.time() - t0, 1))), flush=True)
    json.dump(results, open(sys.argv[0].replace('.py', f'_n{n}_results.json'), 'w'))

if __name__ == "__main__":
    main()
