#!/usr/bin/env python3
"""F1 diagnosis check: rank mod r of the producer's dumped first core (uncommitted
scratch file core131.json from the debug build), the number of distinct rows,
and whether the target column ct is determined (e_ct in the row space).
Usage: check_f1_core.py core131.json"""
import json, sys
d = json.load(open(sys.argv[1]))
r, cc, ct = d["r"], d["cc"], d["ct"]
rows = d["rows"]
dense = []
for sp, rhs in rows:
    v = [0] * cc
    for c, x in sp:
        v[c] = (v[c] + x) % r
    dense.append(v)
def rank_and_basis(M):
    M = [row[:] for row in M]
    piv = []
    rk = 0
    for c in range(cc):
        pr = next((i for i in range(rk, len(M)) if M[i][c] % r), None)
        if pr is None:
            continue
        M[rk], M[pr] = M[pr], M[rk]
        inv = pow(M[rk][c], r - 2, r)
        M[rk] = [(x * inv) % r for x in M[rk]]
        for i in range(len(M)):
            if i != rk and M[i][c]:
                f = M[i][c]
                M[i] = [(a - f * b) % r for a, b in zip(M[i], M[rk])]
        piv.append(c)
        rk += 1
    return rk, M[:rk], piv
rk, B, piv = rank_and_basis(dense)
# e_ct in row space <=> rank(rows + e_ct) == rank(rows)
e = [0] * cc; e[ct] = 1
rk2, _, _ = rank_and_basis(dense + [e])
distinct = len({tuple(v) for v in dense})
empty = sum(1 for v in dense if not any(v))
print(json.dumps({"r": r, "rows": len(rows), "cols": cc, "ct": ct, "rank": rk, "distinct_rows": distinct,
                  "zero_rows": empty, "target_column_determined": rk2 == rk, "t_in_file": d.get("t")}))
