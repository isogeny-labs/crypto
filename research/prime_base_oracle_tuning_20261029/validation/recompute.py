#!/usr/bin/env python3
"""Validator's independent recompute for research/prime_base_oracle_tuning_20261029.

Stdlib only.  Reads records.jsonl directly; shares no code with ecbench
(compare.rs / band.rs).  The pricing follows the definition stated in
PROTOCOL.md section 6 and the band.rs module comment:
    priced_S(w) = (G - C + w * (U + n_C)) / sqrt(r)
with C = n_C = 0 when a record carries no pinned calibration (checked), and U
the count behind each `cost.unpriced` counter X_uncharged (top-level counters,
else the per-phase native maps under X_uncharged or X).

Also re-verifies [d]G = Q for every measured record with its own affine
arithmetic, derives the curve coefficients from G and the targets, and checks
j(E) is neither 0 nor 1728 (Aut(E) = {+-1}).

Usage (cwd = repository root):
    python3 -I research/prime_base_oracle_tuning_20261029/validation/recompute.py OUT.json
"""
from __future__ import annotations

import json
import math
import random
import sys
from collections import defaultdict
from pathlib import Path

STUDY = Path("research/prime_base_oracle_tuning_20261029")
WEIGHTS = (0.0, 0.1, 1.0)
WKEY = {0.0: "0", 0.1: "0.1", 1.0: "1"}


def load(session: str):
    recs = []
    with open(STUDY / "sessions" / session / "records.jsonl") as fh:
        for line in fh:
            recs.append(json.loads(line))
    return recs


def unpriced_total(r) -> tuple[int, bool]:
    total, missing = 0, False
    for name in r["cost"].get("unpriced") or []:
        base = name[: -len("_uncharged")] if name.endswith("_uncharged") else name
        if name in (r.get("counters") or {}):
            total += int(r["counters"][name])
            continue
        found = False
        for p in r.get("phases") or []:
            nat = p.get("native") or {}
            if name in nat:
                total += int(nat[name]); found = True
            elif base in nat:
                total += int(nat[base]); found = True
        if not found:
            missing = True
    return total, missing


def calib_pinned(r) -> bool:
    d = r.get("detail")
    return bool(isinstance(d, dict) and d.get("calibration_pinned_units"))


def priced_s(r, w: float):
    g = r["cost"]["total_gae"]
    u, missing = unpriced_total(r)
    if missing and w > 0:
        return None
    return (g + w * u) / math.sqrt(r["workload"]["curve"]["r"])


# ---------------------------------------------------------------- curve checks

def inv(a, p):
    return pow(a % p, p - 2, p)


def ec_add(P, Q, a, p):
    if P is None:
        return Q
    if Q is None:
        return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if P == Q:
        lam = (3 * x1 * x1 + a) * inv(2 * y1, p) % p
    else:
        lam = (y2 - y1) * inv(x2 - x1, p) % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)


def ec_mul(k, P, a, p):
    R, A = None, P
    while k:
        if k & 1:
            R = ec_add(R, A, a, p)
        A = ec_add(A, A, a, p)
        k >>= 1
    return R


def is_prime(n: int) -> bool:
    if n < 2:
        return False
    i = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += 1
    return True


def curve_checks(recs):
    c0 = recs[0]["workload"]["curve"]
    icv = c0["icv1"].split(":")
    p = int(icv[1].split("-")[1])
    G = tuple(int(v, 16) for v in c0["generator"])
    r = int(c0["r"])
    pts = {}
    for x in recs:
        t = tuple(int(v, 16) for v in x["workload"]["target"])
        pts[x["workload"]["target_index"]] = t
    # a, b from G and one target with a different abscissa
    Q = next(t for t in pts.values() if t[0] != G[0])
    lhs = lambda P: (P[1] * P[1] - P[0] ** 3) % p
    a = (lhs(G) - lhs(Q)) * inv(G[0] - Q[0], p) % p
    b = (lhs(G) - a * G[0]) % p
    on_curve = all((y * y - (x ** 3 + a * x + b)) % p == 0 for (x, y) in list(pts.values()) + [G])
    disc = (4 * a ** 3 + 27 * b ** 2) % p
    j = None if disc == 0 else (1728 * 4 * a ** 3 * inv(disc, p)) % p
    t = p + 1 - r
    return {
        "p": p, "p_prime": is_prime(p), "a": a, "b": b, "r": r, "r_prime": is_prime(r),
        "cofactor": c0["cofactor"], "trace": t, "hasse_ok": t * t <= 4 * p,
        "all_targets_and_G_on_curve": on_curve, "nonsingular": disc != 0,
        "rG_is_identity": ec_mul(r, G, a, p) is None,
        "j_invariant": j, "j_is_0": j == 0, "j_is_1728": j == 1728 % p,
        "aut_group_order": 6 if j == 0 else (4 if j == 1728 % p else 2),
        "record_automorphisms_available": sorted({x["workload"]["curve"]["automorphisms_available"] for x in recs}),
        "p_mod_3": p % 3, "p_mod_4": p % 4,
        "_G": G, "_pts": pts,
    }


def verify_scalars(recs, cc):
    a, p, G = cc["a"], cc["p"], cc["_G"]
    bad, n = [], 0
    for x in recs:
        if x["warmup"]:
            continue
        n += 1
        d = int(x["outcome"]["recovered"])
        Q = cc["_pts"][x["workload"]["target_index"]]
        if ec_mul(d, G, a, p) != Q:
            bad.append(x["record_id"])
    return n, bad


# ---------------------------------------------------------------- statistics

def measured(recs, arm):
    return [x for x in recs if x["arm"] == arm and not x["warmup"]]


def pairs(recs, a, b, w):
    A = {(x["workload"]["workload_id"], x["round"]): x for x in measured(recs, a)}
    B = {(x["workload"]["workload_id"], x["round"]): x for x in measured(recs, b)}
    keys = sorted(set(A) & set(B))
    out = defaultdict(list)
    same_seed = True
    for k in keys:
        same_seed &= A[k]["algorithm_seed"] == B[k]["algorithm_seed"]
        out[k[0]].append((priced_s(A[k], w), priced_s(B[k], w)))
    return out, len(A) - len(keys) + len(B) - len(keys), same_seed


def ratio(by_w):
    sa = sum(p[0] for v in by_w.values() for p in v)
    sb = sum(p[1] for v in by_w.values() for p in v)
    return sb / sa


def boot(by_w, n=4000, seed=29102026):
    rng = random.Random(seed)
    groups = list(by_w.values())
    out = []
    for _ in range(n):
        sa = sb = 0.0
        for _ in range(len(groups)):
            g = groups[rng.randrange(len(groups))]
            for _ in range(len(g)):
                pa, pb = g[rng.randrange(len(g))]
                sa += pa; sb += pb
        out.append(sb / sa)
    out.sort()
    return [out[int(0.025 * n)], out[int(0.975 * n) - 1]]


def mean_s(recs, arm, w):
    v = [priced_s(x, w) for x in measured(recs, arm) if x["outcome"]["status"] == "verified"]
    return sum(v) / len(v)


def session_summary(recs):
    arms = sorted({x["arm"] for x in recs})
    s = {}
    for arm in arms:
        m = measured(recs, arm)
        s[arm] = {
            "measured": len(m),
            "verified": sum(1 for x in m if x["outcome"]["status"] == "verified"),
            "matches_target": sum(1 for x in m if x["outcome"]["matches_target"] is True),
            "statuses": sorted({x["outcome"]["status"] for x in m}),
            "any_uncounted_unpriced": any(unpriced_total(x)[1] for x in m),
            "any_calibration_pinned": any(calib_pinned(x) for x in m),
            "cost_s_equals_gae_over_sqrt_r": all(
                abs(x["cost"]["s"] - x["cost"]["total_gae"] / math.sqrt(x["workload"]["curve"]["r"]))
                <= 1e-12 * max(1.0, x["cost"]["s"]) for x in m),
            "mean_s": {WKEY[w]: mean_s(recs, arm, w) for w in WEIGHTS},
            "mean_max_rss_kib": sum(x["time"]["max_rss_kib"] for x in m) / len(m),
            "method_ids": sorted({x["method"]["method_id"] for x in m}),
            "factor_base": sorted({(x["method"].get("params") or {}).get("factor_base", "") for x in m}),
            "oracle": sorted({(x["method"].get("params") or {}).get("oracle", "") for x in m}),
            "fixed_params": sorted({json.dumps({k: v for k, v in (x["method"].get("params") or {}).items()
                                                if k in ("targets", "linalg", "max_trials", "solver",
                                                         "solver_budget_seconds")}, sort_keys=True)
                                    for x in m}),
        }
    return s


def aa_exact(recs, a, b):
    A = {(x["workload"]["workload_id"], x["round"]): x for x in measured(recs, a)}
    B = {(x["workload"]["workload_id"], x["round"]): x for x in measured(recs, b)}
    diff = [k for k in A if k not in B or A[k]["cost"]["total_gae"] != B[k]["cost"]["total_gae"]
            or unpriced_total(A[k]) != unpriced_total(B[k])
            or A[k]["outcome"]["recovered"] != B[k]["outcome"]["recovered"]]
    return {"pairs": len(A), "pairs_differing_in_gae_unpriced_or_scalar": len(diff)}


def compare_claim(b, a, arm_b, w):
    f = STUDY / "analysis" / f"compare-P{b}-{a}__{arm_b}-w{WKEY[w] if w else '0'}.json"
    j = json.loads(f.read_text())
    return j["ops"]["ratio_b_over_a"], j["ops"]["ci95_bootstrap"], str(f)


def main(out_path):
    res = {"schema": "xp-pbo.validator-recompute/v1", "weights": list(WEIGHTS), "sizes": {}}
    for b in (20, 24, 28, 32):
        recs = load(f"holdout-P{b}")
        slug = recs[0]["workload"]["curve"]["slug"]
        size = {"curve": slug, "records": len(recs), "arms": session_summary(recs)}
        pl = [("inc", "sel"), ("inc", "inc-aa"), ("rho-neg", "rho-neg-aa"), ("rho-neg", "sel"), ("rho-neg", "inc")]
        pl += [("inc", "size"), ("inc", "stride"), ("sel", "stride")]
        size["ratios"] = {}
        for a, arm_b in pl:
            for w in WEIGHTS:
                by_w, unpaired, same_seed = pairs(recs, a, arm_b, w)
                rr = ratio(by_w)
                ent = {"recomputed": rr, "pairs": sum(len(v) for v in by_w.values()),
                       "workloads": len(by_w), "unpaired": unpaired, "same_seeds": same_seed}
                try:
                    c, ci, f = compare_claim(b, a, arm_b, w)
                    ent.update({"claimed_compare": c, "claimed_ci95_bootstrap": ci, "claimed_source": f,
                                "rel_diff": abs(c - rr) / abs(rr)})
                except FileNotFoundError:
                    pass
                if (a, arm_b) in (("inc", "sel"), ("rho-neg", "sel"), ("rho-neg", "inc")) and b == 20:
                    ent["validator_bootstrap_ci95"] = boot(by_w)
                size["ratios"][f"{a}->{arm_b}@{WKEY[w]}"] = ent
        size["aa_exact"] = {"inc/inc-aa": aa_exact(recs, "inc", "inc-aa"),
                            "rho-neg/rho-neg-aa": aa_exact(recs, "rho-neg", "rho-neg-aa")}
        if b == 20:
            cc = curve_checks(recs)
            n, bad = verify_scalars(recs, cc)
            size["curve_checks"] = {k: v for k, v in cc.items() if not k.startswith("_")}
            size["scalar_reverification"] = {"measured_records": n, "failures": bad}
        res["sizes"][str(b)] = size

    # train selection, recomputed from train records (PROTOCOL.md 4.4)
    sel = {}
    for b in (20, 24, 28, 32):
        sdirs = sorted(p.name for p in (STUDY / "sessions").iterdir()
                       if p.name == f"train-P{b}" or p.name.startswith(f"train-ext-P{b}-"))
        cells = {}
        curves = set()
        for sd in sdirs:
            recs = load(sd)
            curves |= {x["workload"]["curve"]["slug"] for x in recs}
            for arm in sorted({x["arm"] for x in recs}):
                if arm == "rho-neg":
                    continue
                m = measured(recs, arm)
                ver = [x for x in m if x["outcome"]["status"] == "verified"]
                elig = len(m) == 8 and len(ver) == 8 and not any(unpriced_total(x)[1] for x in m)
                cells[arm] = {"session": sd, "runs": len(m), "verified": len(ver), "eligible": elig,
                              "s": [sum(priced_s(x, w) for x in ver) / len(ver) for w in WEIGHTS],
                              "oracle": (m[0]["method"].get("params") or {}).get("oracle")}
        fam = lambda arm: arm.split("-n")[0]
        n_of = lambda arm: int(arm.split("-n")[1])
        el = [k for k, v in cells.items() if v["eligible"]]
        key = lambda k: (cells[k]["s"][1], 0 if fam(k) == "mitm2" else 1, n_of(k))
        best = min(el, key=key)
        best_size = min([k for k in el if fam(k) == "mitm2"], key=key)
        claimed = json.loads((STUDY / "select" / f"selection-P{b}.json").read_text())
        ties = [k for k in el if cells[k]["s"][1] == cells[best]["s"][1] and k != best]
        sel[str(b)] = {"train_sessions": sdirs, "train_curves": sorted(curves),
                       "cells": len(cells), "eligible": len(el),
                       "recomputed_selected": best, "recomputed_size_only": best_size,
                       "claimed_selected": claimed["selected"]["arm"],
                       "claimed_size_only": claimed["size_only"]["arm"],
                       "selected_s": cells[best]["s"], "claimed_s": [claimed["selected"][k] for k in ("s0", "s1", "s2")],
                       "exact_ties_at_w0.1": ties,
                       "mitm2_cells": {k: cells[k]["s"] for k in sorted(cells, key=lambda z: (fam(z), n_of(z)))
                                       if fam(k) == "mitm2"}}
    res["selection"] = sel
    res["train_holdout_disjoint"] = {
        b: res["sizes"][b]["curve"] not in sel[b]["train_curves"] for b in sel}
    Path(out_path).write_text(json.dumps(res, indent=1, sort_keys=True) + "\n")
    print(json.dumps({b: {k: (v["recomputed"], v.get("claimed_compare"), v.get("validator_bootstrap_ci95"))
                          for k, v in res["sizes"][b]["ratios"].items() if k.startswith("inc->sel")}
                      for b in res["sizes"]}, indent=1))


if __name__ == "__main__":
    main(sys.argv[1])
