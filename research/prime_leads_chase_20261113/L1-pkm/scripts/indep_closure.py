#!/usr/bin/env python3
"""Independent dense check of the PC closure degree (different representation: dict monomials,
full RREF, iterate-to-fixpoint). For tiny cells only. Usage: indep_closure.py SYS.json DMAX"""
import json, sys, numpy as np
d = json.load(open(sys.argv[1])); p, m, t = d['p'], d['m'], d['t']; k = d.get('k', max(m-2, 0)); DM = int(sys.argv[2])
mask = (1 << t) - 1
def deg(mo): return sum(bin(e).count('1') for e in mo[:m]) + sum(mo[m:])
def mulv(mo, v):
    mo = list(mo)
    if v < m*t: i, j = divmod(v, t); mo[i] = (mo[i] + (1 << j)) & mask
    else: mo[m + v - m*t] += 1
    return tuple(mo)
gens = [{tuple(e): c % p for e, c in P} for P in d['polys']]
for D in range(1, DM+1):
    # monomials of deg <= D
    from itertools import product
    monos = [mo for mo in product(*([range(1 << t)]*m + [range(D+1)]*k)) if deg(mo) <= D]
    monos.sort(key=lambda mo: (-deg(mo), mo)); col = {mo: i for i, mo in enumerate(monos)}
    rows = [g for g in gens if max(map(deg, g)) <= D]
    if not rows: continue
    def mat(rs):
        M = np.zeros((len(rs), len(monos)), dtype=np.int64)
        for r, g in enumerate(rs):
            for mo, c in g.items(): M[r, col[mo]] = c
        return M
    def rref(M):
        M = M % p; r = 0; piv = []
        for c in range(M.shape[1]):
            nz = np.nonzero(M[r:, c])[0]
            if len(nz) == 0: continue
            i = r + nz[0]; M[[r, i]] = M[[i, r]]
            M[r] = M[r] * pow(int(M[r, c]), p-2, p) % p
            others = np.nonzero(M[:, c])[0]
            for o in others:
                if o != r: M[o] = (M[o] - M[o, c] * M[r]) % p
            piv.append(c); r += 1
            if r == M.shape[0]: break
        return M[:r], piv
    B, piv = rref(mat(rows)); rank = -1
    while True:
        new = []
        for row in B:
            nzc = np.nonzero(row)[0]
            if max(deg(monos[c]) for c in nzc) <= D-1:
                for v in range(m*t+k):
                    new.append({mulv(monos[c], v): int(row[c]) for c in nzc})
        if not new: break
        B, piv = rref(np.vstack([B, mat(new)]))
        if len(piv) == rank: break
        rank = len(piv)
    one = col[tuple([0]*(m+k))] in piv
    print(json.dumps({'D': D, 'rank': len(piv), 'cols': len(monos), 'one_in_W': one})); sys.stdout.flush()
    if one: break
