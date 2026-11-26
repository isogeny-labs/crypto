#!/usr/bin/env python3
"""Validator: re-verify every discrete-log certificate of the study from Git alone.

Adapted for research/quartic_diem_nofb_20261126 from the parent validation's
verify_certificates.py (quartic_diem_la_20261123/validation/, at cc7e5c5488a): STUDY
changed, and C10 (both arms of each (curve, solver seed) recover the same logarithm)
added.  picarith.py is copied byte-identical (sha256 b6b60f41...3b16); it shares no
code with the solver.  Both arms (Q_diem_2lp_sfb_sge, Q_diem_2lp_sfb_sge_nofb) are
checked row by row.

Usage:
  python3 -I verify_certificates.py REPO SHA OUT.json [--primes 131101,...] [--count-points]

Reads raw/run.jsonl and raw/run131101.jsonl with
`git show SHA:<path>` (never the working tree), groups the run rows by curve
and checks, with the validator's own arithmetic (picarith.py, no solver code):

  C1  F is smooth (Macaulay degree-7 test on the partials), P0 is on C;
  C2  every row of a curve carries the same instance (f, p0, r, order, h, t, G, T);
  C3  r is prime, h = 1, order = r, and (sqrt p - 1)^6 <= r <= (sqrt p + 1)^6 < 2r
      (exact integer comparison), so with ord(G) = r it follows that #J = r;
  C4  G and T are RREF nets whose base scheme has length 3 and lies on C;
  C5  G != 0 and [r]G = 0 (independent group law), so ord(G) = r;
  C6  [t_secret]G = T, and every run row (IC and rho) has recovered = t_secret,
      certified = ok = true (a row with another recovered value is checked as
      [recovered]G = T);
  C7  negative controls: [t+1]G != T, and a tampered T (one coefficient + 1)
      is rejected (either not a valid class record or not equal to [t]G);
  C8  [r]X = 0 for two classes X of random rational points (group-law sanity
      and consistency with #J = r);
  C9  (--count-points) #C(F_p) recounted by the validator equals n_points;
  C10 for every solver seed of the curve both arms are present, once each, and
      their recovered values are equal.
"""
import json
import math
import random
import subprocess
import sys
import time

import numpy as np

sys.path.insert(0, __import__("os").path.dirname(__import__("os").path.abspath(__file__)))
import picarith as pa  # noqa: E402

FILES = ["raw/run.jsonl", "raw/run131101.jsonl"]
STUDY = "research/quartic_diem_nofb_20261126"
ARMS = ("Q_diem_2lp_sfb_sge", "Q_diem_2lp_sfb_sge_nofb")
INST_KEYS = ["f", "p0", "order", "r", "h", "t_secret", "g_class", "target_class"]


def git_rows(repo, sha, path):
    out = subprocess.run(["git", "-C", repo, "show", f"{sha}:{STUDY}/{path}"],
                         check=True, capture_output=True).stdout
    return [json.loads(l) for l in out.decode().splitlines() if l.strip()]


def is_prime(n):
    if n < 2:
        return False
    small = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41]
    for q in small:
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in small:  # deterministic for n < 3.3e24
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def hw_bounds_ok(p, r):
    """(s-1)^6 <= r <= (s+1)^6 < 2r with s = sqrt(p), exactly: (s+-1)^6 = A +- s*B."""
    A = p ** 3 + 15 * p ** 2 + 15 * p + 1
    Bc = 6 * p ** 2 + 20 * p + 6

    def le_upper(x):   # x <= A + s*Bc
        d = x - A
        return d <= 0 or d * d <= p * Bc * Bc

    def ge_lower(x):   # x >= A - s*Bc
        d = A - x
        return d <= 0 or d * d <= p * Bc * Bc

    upper_lt_2r = not le_upper(2 * r)  # 2r > (s+1)^6
    return {"lower_ok": ge_lower(r), "upper_ok": le_upper(r), "hw_upper_below_2r": upper_lt_2r}


def count_points(p, f):
    """#C(F_p) by the validator: distinct roots of F(1,y,z) per y (vectorised
    z^p mod g_y, then a gcd per y), plus the points with x = 0."""
    m4, _ = pa.mons(4)
    ys = np.arange(p, dtype=np.int64)
    co = [np.zeros(p, dtype=np.int64) for _ in range(5)]
    for coef, (a, b, c) in zip(f, m4):
        if coef:
            yb = np.ones(p, dtype=np.int64)
            for _ in range(b):
                yb = yb * ys % p
            co[c] = (co[c] + int(coef) * yb) % p
    lead = int(co[4][0])
    assert all(int(v) == lead for v in co[4][:5]), "z^4 coefficient depends on y?"
    total = 0
    if lead == 0:
        raise NotImplementedError("f_004 = 0: not needed for these curves")
    inv = pow(lead, p - 2, p)
    g = [c * inv % p for c in co[:4]]  # monic: z^4 + g3 z^3 + ... + g0

    def mulmod(a, b):
        prod = [np.zeros(p, dtype=np.int64) for _ in range(7)]
        for i in range(4):
            for j in range(4):
                prod[i + j] = (prod[i + j] + a[i] * b[j]) % p
        for d in range(6, 3, -1):
            t = prod[d]
            for k in range(4):
                prod[d - 4 + k] = (prod[d - 4 + k] - t * g[k]) % p
        return prod[:4]

    one = [np.ones(p, dtype=np.int64)] + [np.zeros(p, dtype=np.int64) for _ in range(3)]
    zz = [np.zeros(p, dtype=np.int64), np.ones(p, dtype=np.int64)] + [np.zeros(p, dtype=np.int64)] * 2
    res, base, e = one, zz, p
    while e:
        if e & 1:
            res = mulmod(res, base)
        e >>= 1
        if e:
            base = mulmod(base, base)
    h = [res[0], (res[1] - 1) % p, res[2], res[3]]  # z^p - z mod g

    def pgcd_deg(a, b):
        def trim(v):
            while v and v[-1] == 0:
                v.pop()
            return v
        a, b = trim(list(a)), trim(list(b))
        while b:
            ib = pow(b[-1], p - 2, p)
            while len(a) >= len(b):
                c = a[-1] * ib % p
                sh = len(a) - len(b)
                for k in range(len(b)):
                    a[sh + k] = (a[sh + k] - c * b[k]) % p
                trim(a)
                if not a:
                    break
            a, b = b, a
        return len(a) - 1

    G = np.stack(g + [np.ones(p, dtype=np.int64)], axis=1).tolist()
    H = np.stack(h, axis=1).tolist()
    for y in range(p):
        total += pgcd_deg(G[y], H[y])
    # x = 0: [0:1:z] with F(0,1,z) = 0, and [0:0:1] iff f_004 = 0 (not the case here)
    c1 = [0] * 5
    for coef, (a, b, c) in zip(f, m4):
        if coef and a == 0:
            c1[c] = (c1[c] + int(coef)) % p
    total += sum(1 for z in range(p) if sum(c1[k] * pow(z, k, p) for k in range(5)) % p == 0)
    return total


def main():
    repo, sha, outp = sys.argv[1], sys.argv[2], sys.argv[3]
    primes = None
    count = "--count-points" in sys.argv
    if "--primes" in sys.argv:
        primes = {int(x) for x in sys.argv[sys.argv.index("--primes") + 1].split(",")}
    rows = []
    for fn in FILES:
        for r in git_rows(repo, sha, fn):
            r["_file"] = fn
            rows.append(r)
    runs = [r for r in rows if r.get("kind") == "run" and r.get("family") == "quartic"]
    curves = {}
    for r in runs:
        curves.setdefault((r["p"], r["curve_seed"]), []).append(r)
    inst_rows = {(r["p"], r["curve_seed"]): r for r in rows
                 if r.get("kind") == "instance" and r.get("family") == "quartic"}
    report = {"sha": sha, "files": FILES, "curves": []}
    for (p, cs) in sorted(curves):
        if primes and p not in primes:
            continue
        t0 = time.time()
        rs = curves[(p, cs)]
        ref = rs[0]
        res = {"p": p, "curve_seed": cs, "n_rows": len(rs),
               "arms": sorted({r["arm"] for r in rs})}
        # C2: identical instance on every row (keys must exist: index with [])
        res["C2_same_instance"] = all(all(r[k] == ref[k] for k in INST_KEYS) for r in rs)
        ins = inst_rows.get((p, cs))
        res["C2_instance_row_agrees"] = bool(ins) and ins["f"] == ref["f"] and \
            ins["order"] == ref["order"] and ins["r"] == ref["r"] and ins["h"] == ref["h"]
        C = pa.Quartic(p, ref["f"], ref["p0"])
        res["C1_smooth"] = C.is_smooth()
        res["C1_p0_on_curve"] = C.on_curve(tuple(ref["p0"]))
        r_, t = ref["r"], ref["t_secret"]
        res["C3_r_prime"] = is_prime(r_)
        res["C3_h1_order_eq_r"] = ref["h"] == 1 and ref["order"] == r_
        res["C3_hw"] = hw_bounds_ok(p, r_)
        G = C.class_from_record(ref["g_class"])
        T = C.class_from_record(ref["target_class"])
        res["C4_G_record"] = C.check_typ_record(G[1]) if G[0] == "Typ" else {"exc": G[1]}
        res["C4_T_record"] = C.check_typ_record(T[1]) if T[0] == "Typ" else {"exc": T[1]}
        rng = random.Random(p * 1000003 + cs)
        res["C5_G_nonzero"] = not C.eq(G, C.zero())
        res["C5_rG_zero"] = C.eq(C.mul(G, r_, rng), C.zero())
        tG = C.mul(G, t, rng)
        res["C6_tG_eq_T"] = C.eq(tG, T)
        row_checks = []
        for r in rs:
            rec = r["recovered"]
            ok_rec = rec == t
            if not ok_rec and rec is not None:
                ok_rec = C.eq(C.mul(G, rec, rng), T)
            row_checks.append({"arm": r["arm"], "seed": r["seed"], "file": r["_file"],
                               "recovered_eq_t": rec == t, "cert_ok": bool(ok_rec),
                               "certified": r["certified"], "ok": r["ok"]})
        res["C6_rows"] = row_checks
        res["C6_all_rows_ok"] = all(x["cert_ok"] and x["certified"] and x["ok"] and x["recovered_eq_t"]
                                    for x in row_checks)
        # C7 negative controls
        t1G = C.add(tG, G, rng)
        res["C7_t_plus_1_rejected"] = not C.eq(t1G, T)
        if T[0] == "Typ":
            bad = T[1].copy()
            bad[0, 3] = (bad[0, 3] + 1) % p
            chk = C.check_typ_record(bad)
            res["C7_tampered_T_rejected"] = (not chk["F_in_ideal"]) or (not C.eq(("Typ", bad), tG))
            res["C7_tampered_T_detail"] = chk
        # C8 random classes
        pts = C.random_points(6, rng)
        xs = [C.class_of_points(pts[0:3]), C.class_of_points(pts[3:6])]
        res["C8_rX_zero"] = [C.eq(C.mul(x, r_, rng), C.zero()) for x in xs]
        if count:
            n1 = count_points(p, ref["f"])
            rec_n = {r.get("n_points") for r in rs if "n_points" in r}
            if ins:
                rec_n.add(ins["n_points"])
            res["C9_n_points"] = {"validator": n1, "recorded": sorted(rec_n),
                                  "match": rec_n == {n1}}
        # C10 paired arms: each solver seed has both arms once, with the same recovered log
        by_seed = {}
        for r in rs:
            by_seed.setdefault(r["seed"], []).append(r)
        res["C10_pairs"] = {str(s): {"arms": sorted(x["arm"] for x in v),
                                     "same_log": len({x["recovered"] for x in v}) == 1}
                            for s, v in sorted(by_seed.items())}
        res["C10_ok"] = all(v["arms"] == sorted(ARMS) and v["same_log"]
                            for v in res["C10_pairs"].values())
        res["seconds"] = round(time.time() - t0, 2)
        allok = (res["C10_ok"] and res["C1_smooth"] and res["C1_p0_on_curve"] and res["C2_same_instance"]
                 and res["C2_instance_row_agrees"] and res["C3_r_prime"] and res["C3_h1_order_eq_r"]
                 and all(res["C3_hw"].values()) and res["C5_G_nonzero"] and res["C5_rG_zero"]
                 and res["C6_tG_eq_T"] and res["C6_all_rows_ok"] and res["C7_t_plus_1_rejected"]
                 and res.get("C7_tampered_T_rejected", True) and all(res["C8_rX_zero"])
                 and (not count or res["C9_n_points"]["match"])
                 and all(v for k, v in res["C4_G_record"].items() if k != "ideal4_codim")
                 and all(v for k, v in res["C4_T_record"].items() if k != "ideal4_codim"))
        res["ALL_OK"] = bool(allok)
        report["curves"].append(res)
        print(json.dumps({k: res[k] for k in ("p", "curve_seed", "n_rows", "ALL_OK", "seconds")}),
              flush=True)
    report["summary"] = {
        "curves": len(report["curves"]),
        "curves_all_ok": sum(c["ALL_OK"] for c in report["curves"]),
        "rows_checked": sum(c["n_rows"] for c in report["curves"]),
        "rows_cert_ok": sum(sum(x["cert_ok"] for x in c["C6_rows"]) for c in report["curves"]),
    }
    json.dump(report, open(outp, "w"), indent=1, default=str)
    print(json.dumps(report["summary"]))


if __name__ == "__main__":
    main()
