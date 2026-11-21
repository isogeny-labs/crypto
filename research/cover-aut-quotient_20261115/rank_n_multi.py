#!/usr/bin/env python3
"""Arm-N rank analysis on p=251 seeds 1,2,3 (first 420 recorded relations each, DUMP_N_ROWS dumps):
plain N, N with the B free iota rows, and the orbit-quotient system (columns 2i,2i+1 merged, x(i,-) = -x(i,+)),
which is what arm A solves. Reports the first prefix (grid step 10) at which d is determined. [pcc:unverified]"""
import json, os, sys
D = os.path.dirname(os.path.abspath(__file__))
def analyse(seed):
    d = json.load(open(f"{D}/results/naive_p251_s{seed}_first420_rows.json"))
    l, B, rows = d["l"], d["small"], d["rows"]; n = 2 * B + 1
    def build(k, mode):
        M = []; nn = n if mode != "quot" else B + 1
        for r in rows[:k]:
            v = [0] * (nn + 1); v[nn] = r["rhs"] % l
            for c, x in r["cols"]:
                if mode == "quot":
                    if c == 2 * B: v[B] = (v[B] + x) % l
                    elif c % 2 == 0: v[c // 2] = (v[c // 2] + x) % l
                    else: v[c // 2] = (v[c // 2] - x) % l
                else: v[c] = x
            M.append(v)
        if mode == "iota":
            for i in range(B):
                v = [0] * (nn + 1); v[2 * i] = 1; v[2 * i + 1] = 1; M.append(v)
        return M, nn
    def solve(M, nn):
        M = [r[:] for r in M]; rk = 0; dp = False
        for c in range(nn):
            p = next((i for i in range(rk, len(M)) if M[i][c]), None)
            if p is None: continue
            M[rk], M[p] = M[p], M[rk]
            inv = pow(M[rk][c], l - 2, l); M[rk] = [x * inv % l for x in M[rk]]
            for i in range(len(M)):
                if i != rk and M[i][c]:
                    f = M[i][c]; M[i] = [(x - f * y) % l for x, y in zip(M[i], M[rk])]
            if c == nn - 1: dp = M[rk][nn] == d["d"] % l
            rk += 1
        return rk, dp
    first = {"plain": None, "iota": None, "quot": None}; rk420 = {}
    for k in range(100, 421, 10):
        for mode in first:
            M, nn = build(k, mode); rk, dp = solve(M, nn)
            if k == 420: rk420[mode] = [rk, nn]
            if dp and first[mode] is None: first[mode] = k
    return {"seed": seed, "B": B, "unknowns_N": n, "first_k_d_determined": first, "rank_and_columns_at_420": rk420}
res = [analyse(s) for s in (1, 2, 3)]
json.dump(res, open(D + "/results/narm_rank_multi.json", "w"), indent=1)
for r in res: print(r)
