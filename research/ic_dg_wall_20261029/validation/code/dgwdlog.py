#!/usr/bin/env python3
"""[d]G = Q on every record of the six sessions, with own short-Weierstrass arithmetic.
a, b are solved from G and Q of each target (two points on y^2 = x^3 + a x + b) and must
agree across all targets of a curve and reproduce the ICV1 j-invariant and group order
(r * G = O).  Stdlib only.  Usage: python3 -I dgwdlog.py <evidence_dir> <out.json>"""
import json
import os
import sys

EV, OUTP = sys.argv[1:3]


def add(P, Q, a, p):
    if P is None:
        return Q
    if Q is None:
        return P
    if P[0] == Q[0] and (P[1] + Q[1]) % p == 0:
        return None
    if P == Q:
        l = (3 * P[0] * P[0] + a) * pow(2 * P[1], -1, p) % p
    else:
        l = (Q[1] - P[1]) * pow(Q[0] - P[0], -1, p) % p
    x = (l * l - P[0] - Q[0]) % p
    return (x, (l * (P[0] - x) - P[1]) % p)


def mul(k, P, a, p):
    R = None
    while k:
        if k & 1:
            R = add(R, P, a, p)
        P = add(P, P, a, p)
        k >>= 1
    return R


out = {}
for b in (24, 28, 32):
    for s in ("wall", "counted"):
        path = os.path.join(EV, "P%d" % b, s, "eks-results/session/records.jsonl")
        curves = {}
        n = ok = 0
        bad = []
        for line in open(path):
            r = json.loads(line)
            c = r["workload"]["curve"]
            f = c["icv1"].split(":")
            p, j, order = int(f[1].split("-")[1]), int(f[4]), int(f[3])
            G = tuple(int(x, 16) for x in c["generator"])
            Q = tuple(int(x, 16) for x in r["workload"]["target"])
            cv = curves.setdefault(c["slug"], {"p": p, "j": j, "order": order, "G": G, "ab": set(), "targets": set()})
            # solve a, b from G and Q
            if Q[0] != G[0]:
                lhs1 = (G[1] ** 2 - G[0] ** 3) % p
                lhs2 = (Q[1] ** 2 - Q[0] ** 3) % p
                a = (lhs1 - lhs2) * pow(G[0] - Q[0], -1, p) % p
                bb = (lhs1 - a * G[0]) % p
                cv["ab"].add((a, bb))
            cv["targets"].add(Q)
            n += 1
            cv.setdefault("records", []).append((int(r["outcome"]["recovered"]), Q, r["seq"]))
        res = {}
        for slug, cv in curves.items():
            p = cv["p"]
            ab_unique = len(cv["ab"]) == 1
            a, bb = next(iter(cv["ab"]))
            num = 1728 * 4 * pow(a, 3, p) % p
            den = (4 * pow(a, 3, p) + 27 * bb * bb) % p
            jj = num * pow(den, -1, p) % p
            on = all((Q[1] ** 2 - Q[0] ** 3 - a * Q[0] - bb) % p == 0 for Q in cv["targets"] | {cv["G"]})
            rG_inf = mul(cv["order"], cv["G"], a, p) is None
            okc = 0
            for d, Q, seq in cv["records"]:
                if mul(d, cv["G"], a, p) == Q:
                    okc += 1
                else:
                    bad.append(seq)
            ok += okc
            res[slug] = {"a_b_unique_across_targets": ab_unique, "j_matches_icv1": jj == cv["j"],
                         "points_on_curve": on, "rG_is_identity": rG_inf, "records": len(cv["records"]),
                         "dG_eq_Q": okc, "distinct_targets": len(cv["targets"])}
        out["P%d/%s" % (b, s)] = {"records": n, "dG_eq_Q": ok, "failures": bad, "curves": res}
        print("P%d %s: %d/%d [d]G=Q" % (b, s, ok, n), res, flush=True)
json.dump(out, open(OUTP, "w"), indent=1)
