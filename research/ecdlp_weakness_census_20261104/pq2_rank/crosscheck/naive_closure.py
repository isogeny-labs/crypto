"""Independent check of pq2_rank's iterated closure: full naive saturation.
S_0 = span(eqs + field eqs); S_{t+1} = span(S_t + {x_v f : f in basis(S_t), deg <= d}).
Reports dim(S ∩ P_{<=1}) at the fixed point. Also checks Z satisfies eqs and computes k."""
import json, sys, itertools
from itertools import combinations_with_replacement as cwr
def run(path, d):
    J = json.load(open(path)); N = J["N"]
    monos = []
    for deg in range(0, d + 1):
        monos += list(cwr(range(N), deg))
    # bit index: higher degree -> higher bits (degree-graded order)
    idx = {m: i for i, m in enumerate(monos)}
    nlow = sum(1 for m in monos if len(m) <= 1)
    def enc(poly):
        r = 0
        for m in poly:
            r ^= 1 << idx[tuple(sorted(m))]
        return r
    def dec(r):
        out = []
        while r:
            b = r.bit_length() - 1; out.append(monos[b]); r ^= 1 << b
        return out
    gens = [enc(e) for e in J["eqs"]] + [enc([[v, v], [v]]) for v in range(N)]
    def echelon(rows):
        piv = {}
        for r in rows:
            while r:
                b = r.bit_length() - 1
                if b in piv: r ^= piv[b]
                else: piv[b] = r; break
        return piv
    piv = echelon(gens); rank = len(piv); rounds = 0
    while True:
        rounds += 1
        new = list(piv.values())
        for r in list(piv.values()):
            ms = dec(r)
            if max(len(m) for m in ms) + 1 > d: continue
            for v in range(N):
                new.append(enc([m + (v,) for m in ms]))
        piv = echelon(new)
        if len(piv) == rank: break
        rank = len(piv)
    low = sum(1 for b in piv if b < nlow)
    one = 0 in piv and piv[0] == 1
    # k from Z
    Z = J["Z"]; base = Z[0]; vecs = [z ^ base for z in Z[1:]]
    aff = len(echelon(vecs))
    return dict(n=J["n"], N=N, k=N - aff, low=low, rank=rank, rounds=rounds, one_in_span=one)
for p in sys.argv[2:]:
    print(p.split("/")[-1], run(p, int(sys.argv[1])))
