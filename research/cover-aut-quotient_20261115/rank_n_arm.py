#!/usr/bin/env python3
"""Rank analysis of arm N (unquotiented signed unknowns) on the first 420 recorded relations of
p=251 seed=1 (results/naive_p251_s1_first420_rows.json): without and with the free iota identification rows."""
import json, os
D = os.path.dirname(os.path.abspath(__file__))
d = json.load(open(D + "/results/naive_p251_s1_first420_rows.json"))
l, B, rows = d["l"], d["small"], d["rows"]
n = 2 * B + 1
def mat(k, ident):
    M = []
    for r in rows[:k]:
        v = [0] * (n + 1)
        for c, x in r["cols"]: v[c] = x
        v[n] = r["rhs"]; M.append(v)
    if ident:
        for i in range(B):
            v = [0] * (n + 1); v[2 * i] = 1; v[2 * i + 1] = 1; M.append(v)
    return M
def solve(M):
    M = [r[:] for r in M]; rk = 0; dp = False
    for c in range(n):
        p = next((i for i in range(rk, len(M)) if M[i][c]), None)
        if p is None: continue
        M[rk], M[p] = M[p], M[rk]
        inv = pow(M[rk][c], l - 2, l); M[rk] = [x * inv % l for x in M[rk]]
        for i in range(len(M)):
            if i != rk and M[i][c]:
                f = M[i][c]; M[i] = [(x - f * y) % l for x, y in zip(M[i], M[rk])]
        if c == n - 1: dp = M[rk][n] == d["d"] % l
        rk += 1
    return rk, dp
out = {"p": 251, "B": B, "unknowns_N": n, "prefix": []}
first = {"plain": None, "with_iota_rows": None}
for k in list(range(100, 421, 10)):
    r0, d0 = solve(mat(k, False)); r1, d1 = solve(mat(k, True))
    out["prefix"].append({"relations": k, "rank_plain": r0, "d_determined_plain": d0, "rank_with_iota_rows": r1, "d_correct_with_iota_rows": d1})
    if d0 and first["plain"] is None: first["plain"] = k
    if d1 and first["with_iota_rows"] is None: first["with_iota_rows"] = k
out["first_relations_d_determined"] = first
out["note"] = "plain = arm N as run; with_iota_rows adds x_(i,+) + x_(i,-) = 0 (free, B rows). Arm A (quotient) solved this instance with 154 relations."
json.dump(out, open(D + "/results/narm_rank_analysis.json", "w"), indent=1)
print(first, out["prefix"][-1])
