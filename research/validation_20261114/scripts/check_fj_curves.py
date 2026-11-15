#!/usr/bin/env python3
"""Curve-data consistency for research/full_jacobian_ic_20261113 (Validator V, 2026-11-14).

From the val_fj_dump lines: (1) the bielliptic coefficients F = c0 + c1 X + c2 X^2 + c3 X^3
equal the declared selection rule candidate_coeffs(p, candidate) (formula restated
here from the protocol's code reference, negation_remeasure.rs:822); (2) E1 and E2 are
the monic models of Y^2 = F(X) and Y^2 = X^3 F(1/X); (3) the quintic model H64_f on
which the solver ran has the same number of F_p-points as C: y^2 = F(x^2) (sextic,
2 or 0 points at infinity), so the Jacobian the logs were certified in is that of
the cover of E1 (with #J = #E1 #E2 checked in check_fj_certs.py).
Usage: check_fj_curves.py DUMP.jsonl [DUMP2.jsonl ...] OUT
"""
import json, sys

M = (1 << 64) - 1
paths, out = sys.argv[1:-1], sys.argv[-1]
res, fails = [], []


def cand(p, s):
    return [1 + ((s * 2654435761 & M) >> 3) % (p - 1), ((s * 40503 & M) >> 5) % p,
            ((s * 2246822519 & M) >> 7) % p, 1 + ((s * 3266489917 & M) >> 9) % (p - 1)]


def chi(v, p):
    v %= p
    return 0 if v == 0 else (1 if pow(v, (p - 1) // 2, p) == 1 else -1)


for path in paths:
    for line in open(path):
        if not line.strip(): continue
        D = json.loads(line)
        p, c = D["p"], D["F_c"]
        c0, c1, c2, c3 = c
        ok_c = c == cand(p, D["candidate"])
        e1 = [c2 % p, c1 * c3 % p, c0 * c3 * c3 % p]
        e2 = [c1 % p, c2 * c0 % p, c3 * c0 * c0 % p]
        ok_e = D["E1"] == e1 and D["E2"] == e2
        # sextic count y^2 = F(x^2): leading coeff c3 (degree 6); 2 points at infinity iff c3 is a square
        n_sext = sum(1 + chi(c0 + c1 * x * x + c2 * x ** 4 + c3 * x ** 6, p) for x in range(p)) + (1 + chi(c3, p))
        f = D["H64_f"]
        n_quint = 1 + sum(1 + chi(sum(cf * pow(t, i, p) for i, cf in enumerate(f)), p) for t in range(p))
        ok_n = n_sext == n_quint
        r = {"p": p, "candidate": D["candidate"], "F_c_matches_rule": ok_c, "E1_E2_models": ok_e,
             "N1_sextic": n_sext, "N1_quintic": n_quint, "same_N1": ok_n}
        res.append(r)
        if not (ok_c and ok_e and ok_n): fails.append(r)
        print(r)
json.dump({"rows": res, "failures": fails}, open(out, "w"), indent=1)
print("failures", len(fails))
