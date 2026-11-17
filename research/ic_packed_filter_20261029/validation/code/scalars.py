"""Own scalar check (J5 companion, TASK-20261029-654f73).

For every record of a session, warm-ups included, check with the validator's own
short-Weierstrass arithmetic that [recovered] G = Q, where G is the curve's
generator and Q the record's target. The curve is bound independently of the
record's claim: (A, B) are fitted from G and the targets of that curve; the
fit must place every point on the curve, j(A, B) must equal the j-invariant in
the ICV1 string, p + 1 - t must equal r, and [r] G must be the identity.
Counted sessions are read through vcommon.load_counted (no wall field).

usage: scalars.py OUT.json
"""
import json
import sys

import vcommon as v


def ec_add(P, Q, A, p):
    if P is None:
        return Q
    if Q is None:
        return P
    x1, y1 = P
    x2, y2 = Q
    if x1 == x2:
        if (y1 + y2) % p == 0:
            return None
        lam = (3 * x1 * x1 + A) * pow(2 * y1, -1, p) % p
    else:
        lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)


def ec_mul(k, P, A, p):
    R = None
    while k:
        if k & 1:
            R = ec_add(R, P, A, p)
        P = ec_add(P, P, A, p)
        k >>= 1
    return R


def bind_curve(curve, points):
    f = curve["icv1"].split(":")
    p = int(f[1].split("-")[1]); t = int(f[2]); r = int(f[3]); j = int(f[4])
    G = tuple(int(x, 16) for x in curve["generator"])
    lhs = lambda P: (P[1] * P[1] - P[0] ** 3) % p
    Q = next(q for q in points if q[0] != G[0])
    A = (lhs(G) - lhs(Q)) * pow(G[0] - Q[0], -1, p) % p
    B = (lhs(G) - A * G[0]) % p
    on = all((lhs(P) - A * P[0] - B) % p == 0 for P in [G] + list(points))
    jj = 1728 * 4 * pow(A, 3, p) * pow(4 * pow(A, 3, p) + 27 * B * B, -1, p) % p
    checks = {
        "p": p, "A": A, "B": B, "trace": t, "r": r,
        "all_points_on_curve": on,
        "j_matches_icv1": jj == j,
        "hasse_r_eq_p_plus_1_minus_t": p + 1 - t == r and r == curve["r"],
        "r_G_is_identity": ec_mul(r, G, A, p) is None,
        "p_le_2pow32_minus_1": p <= 4294967295,
    }
    return checks, G


def check_session(records):
    by_curve = {}
    for rec in records:
        c = rec["workload"]["curve"]
        by_curve.setdefault(c["slug"], (c, set()))[1].add(tuple(int(x, 16) for x in rec["workload"]["target"]))
    curves = {}
    for slug, (c, pts) in by_curve.items():
        chk, G = bind_curve(c, pts)
        curves[slug] = (chk, G)
    out = {"records": 0, "warmups": 0, "scalar_ok": 0, "status_verified": 0,
           "matches_target_true": 0, "failures": [],
           "curves": {s: c[0] for s, c in curves.items()}}
    for rec in records:
        out["records"] += 1
        out["warmups"] += bool(rec["warmup"])
        chk, G = curves[rec["workload"]["curve"]["slug"]]
        Q = tuple(int(x, 16) for x in rec["workload"]["target"])
        o = rec["outcome"]
        ok = False
        if o.get("recovered") is not None:
            k = int(o["recovered"])
            ok = ec_mul(k, G, chk["A"], chk["p"]) == Q
        out["scalar_ok"] += ok
        out["status_verified"] += o["status"] == "verified"
        out["matches_target_true"] += o.get("matches_target") is True
        if not (ok and o["status"] == "verified" and o.get("matches_target") is True):
            out["failures"].append({"record_id": rec["record_id"], "seq": rec["seq"],
                                    "arm": rec["arm"], "recovered": o.get("recovered"),
                                    "status": o["status"]})
    out["all_ok"] = (not out["failures"]) and all(
        all(vv for kk, vv in c.items() if isinstance(vv, bool)) for c in out["curves"].values())
    return out


def main():
    res = {}
    for b in (24, 32, 28):
        res[f"P{b}-counted"] = check_session(v.load_counted(b))
        if b == 28:
            # descriptive: wall fields removed, counts only
            res["P28-wall(descriptive)"] = check_session(v.load_wall_counts_only(28))
        else:
            res[f"P{b}-wall"] = check_session(v.load_counted(b, "wall"))
    v.dump(res, sys.argv[1])
    for k, r in res.items():
        print(k, r["records"], "warmups", r["warmups"], "scalar_ok", r["scalar_ok"],
              "failures", len(r["failures"]), "all_ok", r["all_ok"])


if __name__ == "__main__":
    main()
