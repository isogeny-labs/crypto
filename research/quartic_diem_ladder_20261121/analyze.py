#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md (quartic_diem_ladder_20261121).

Usage: python3 analyze.py raw/run.jsonl raw/run131101_odd.jsonl raw/run131101_even.jsonl > analysis.json

Arms: Q_diem_2lp_new (enumerated factor base, the pre-check-2 PRIMARY) and
Q_diem_2lp_sfb (sampled factor base, step A).  Reference: the ideal negation
rho sqrt(pi r / 4) group additions (L10-corrected, nothing unpriced charged to
rho), and, at the primes where it ran, the measured Q_rho_neg (calibration).
Every number is [pcc:unverified].
"""
import json
import math
import sys
from collections import defaultdict

import numpy as np

NEW = "Q_diem_2lp_new"
SFB = "Q_diem_2lp_sfb"
RHO = "Q_rho_neg"
ARMS = [NEW, SFB]
WS = [0.0, 0.1, 1.0]
B = 4000
SEED = 20261122
# per-prime curve limits for the split 131101 search (odd process 3, even 2)
SPLIT_KEEP = {"odd": 3, "even": 2}


def native_all(r):
    return (r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]
            + r.get("idx_lookups", 0) + r.get("idx_inserts", 0))


def native_mul(r):
    return r["rel_mul"] + r["la_mul"]


def metrics(r):
    go = r["group_ops"]
    m = {
        "fpm_mid": r["cantor_mul"] + 10 * r["cantor_inv"] + r["rel_mul"] + 9 * r["la_mul"],
        "rel_mul": r["rel_mul"],
        "rel_enum_mul": r["rel_enum_mul"],
        "la_mul": r["la_mul"],
        "la_add": r["la_add"],
        "native_all": native_all(r),
        "group_ops": go,
        "relations": r["relations"],
        "attempts": max(r["attempts"], 1),
        "decomp_trials": max(r["decomp_trials"], 1),
    }
    for w in WS:
        m[f"gae_all_{w}"] = go + w * native_all(r)
        m[f"gae_mul_{w}"] = go + w * native_mul(r)
    return m


def ols(x, y):
    x, y = np.asarray(x, float), np.asarray(y, float)
    xm = x.mean()
    b = ((x - xm) * (y - y.mean())).sum() / ((x - xm) ** 2).sum()
    return b, y.mean() - b * xm


def gm(v):
    return math.exp(float(np.mean([math.log(x) for x in v])))


def pct(a, lo=2.5, hi=97.5):
    return [float(np.percentile(a, lo)), float(np.percentile(a, hi))]


def load(paths):
    runs, inst, cells = [], {}, []
    for path in paths:
        tag = "odd" if "odd" in path else "even" if "even" in path else "main"
        kept = defaultdict(list)
        for line in open(path):
            r = json.loads(line)
            k = r.get("kind")
            if k == "instance" and r.get("family") == "quartic":
                p, cs = r["p"], r["curve_seed"]
                kept[p].append(cs)
                if tag in SPLIT_KEEP and len(kept[p]) > SPLIT_KEEP[tag]:
                    continue
                inst[(p, cs)] = r
            elif k == "cell" and r.get("family") == "quartic":
                r["_file"] = path
                cells.append(r)
            elif k == "run" and r.get("family") == "quartic":
                runs.append(r)
    runs = [r for r in runs if (r["p"], r["curve_seed"]) in inst]
    return runs, inst, cells


def main():
    runs, inst, cells = load(sys.argv[1:])
    out = {"protocol": "research/quartic_diem_ladder_20261121/PROTOCOL.md",
           "label": "[pcc:unverified]", "cells": cells}
    ic = defaultdict(dict)       # (arm, p, cs) -> {seed: run}
    rho = defaultdict(dict)
    bad = []
    for r in runs:
        key = (r["p"], r["curve_seed"])
        if r["arm"] == RHO:
            rho[key][r["seed"]] = r
        elif r["arm"] in ARMS:
            if not (r["certified"] and r["ok"]):
                bad.append({k: r[k] for k in ("arm", "p", "curve_seed", "seed")})
                continue
            ic[(r["arm"],) + key][r["seed"]] = r
    out["uncertified_runs"] = bad
    primes = sorted({p for (p, _) in inst})
    curves = {p: sorted(cs for (q, cs) in inst if q == p) for p in primes}
    out["grid"] = {str(p): {"curves": curves[p],
                            "runs": {a: sum(len(ic[(a, p, c)]) for c in curves[p]) for a in ARMS},
                            "rho_runs": sum(len(rho[(p, c)]) for c in curves[p])}
                   for p in primes}

    # per-curve reference
    ref = {}
    for (p, cs), rec in inst.items():
        r = rec["r"]
        ideal = math.sqrt(math.pi * r / 4)
        num = den = 0
        for a in ARMS:
            for run in ic[(a, p, cs)].values():
                num += run["cantor_mul"] + 10 * run["cantor_inv"]
                den += run["group_ops"]
        a_c = num / den if den else float("nan")
        ref[(p, cs)] = {"r": r, "ideal_gae": ideal, "A_fpm_per_op": a_c,
                        "ideal_fpm": ideal * a_c}

    # curve-level metrics (geo-mean over seeds)
    def curve_metric(arm, p, cs, name):
        v = [metrics(run)[name] for run in ic[(arm, p, cs)].values()]
        return gm([max(x, 1e-300) for x in v]) if v else None

    def ratio_metric(arm, p, cs, w, kind):
        rr = ref[(p, cs)]
        if kind == "fpm":
            m = curve_metric(arm, p, cs, "fpm_mid")
            return None if m is None else m / rr["ideal_fpm"]
        if kind == "cnt":   # count-calibrated weight w_cnt = 1 / A (times w)
            wc = w / rr["A_fpm_per_op"]
            v = [run["group_ops"] + wc * native_all(run) for run in ic[(arm, p, cs)].values()]
            return gm(v) / rr["ideal_gae"] if v else None
        m = curve_metric(arm, p, cs, f"gae_{kind}_{w}")
        return None if m is None else m / rr["ideal_gae"]

    rng = np.random.default_rng(SEED)

    def boot_prime(vals_by_prime):
        """vals_by_prime: {p: [curve values]} -> point gm, CI per prime, + slope data."""
        res = {}
        for p, v in vals_by_prime.items():
            v = np.log(np.asarray(v, float))
            pt = math.exp(v.mean())
            bs = [math.exp(rng.choice(v, len(v)).mean()) for _ in range(B)] if len(v) > 1 else [pt]
            res[p] = {"gm": pt, "ci95": pct(bs), "n_curves": len(v)}
        return res

    def boot_slope(vals_by_prime):
        ps = sorted(vals_by_prime)
        xs = [math.log(p) for p in ps for _ in vals_by_prime[p]]
        ys = [math.log(x) for p in ps for x in vals_by_prime[p]]
        b, a = ols(xs, ys)
        bb, aa = [], []
        for _ in range(B):
            x2, y2 = [], []
            for p in ps:
                v = vals_by_prime[p]
                for i in rng.integers(0, len(v), len(v)):
                    x2.append(math.log(p))
                    y2.append(math.log(v[i]))
            b2, a2 = ols(x2, y2)
            bb.append(b2)
            aa.append(a2)
        return {"slope": float(b), "intercept": float(a), "ci95": pct(bb),
                "_boot": (np.asarray(bb), np.asarray(aa))}

    def usable(arm):
        # a prime enters only with >= 2 curves that have a certified run
        d = {}
        for p in primes:
            cs = [c for c in curves[p] if ic[(arm, p, c)]]
            if len(cs) >= 2:
                d[p] = cs
        return d

    # ── IC / rho tables ──
    tables = {}
    verdict = {}
    for arm in ARMS:
        use = usable(arm)
        tables[arm] = {}
        for kind in ["all", "mul", "fpm", "cnt"]:
            wl = [None] if kind == "fpm" else ([1.0, 10.0] if kind == "cnt" else WS)
            for w in wl:
                lab = f"{kind}" if w is None else f"{kind}_{w}"
                vals = {p: [ratio_metric(arm, p, c, w, kind) for c in use[p]] for p in use}
                bp = boot_prime(vals)
                sl = boot_slope(vals)
                bb, aa = sl.pop("_boot")
                # extrapolated crossover ln p* = -a / b (only if slope < 0)
                cross = None
                if sl["slope"] < 0:
                    xs = [-a / b for a, b in zip(aa, bb) if b < 0]
                    pstar = math.exp(-sl["intercept"] / sl["slope"])
                    cross = {"p_star": pstar, "log2_p_star": math.log2(pstar),
                             "ci95_log2": [math.log2(math.exp(x)) for x in pct(xs)] if xs else None,
                             "frac_boot_negative_slope": float(np.mean(bb < 0))}
                tables[arm][lab] = {"per_prime": {str(p): bp[p] for p in sorted(bp)},
                                    "slope_ln_ratio": sl, "crossover": cross}
        # frozen verdict per prime: band all-native, w = 0, 0.1, 1
        verdict[arm] = {}
        for p in sorted(use):
            ups = [tables[arm][f"all_{w}"]["per_prime"][str(p)]["ci95"][1] for w in WS]
            los = [tables[arm][f"all_{w}"]["per_prime"][str(p)]["ci95"][0] for w in WS]
            if all(u < 1 for u in ups):
                v = "BELOW-RHO CANDIDATE"
            elif all(lo > 1 for lo in los):
                v = "ABOVE RHO"
            else:
                v = "UNRESOLVED (order changes inside the band)"
            verdict[arm][str(p)] = {"verdict": v, "upper95": ups, "lower95": los}
    out["ic_over_rho"] = tables
    out["verdict"] = verdict

    # ── exponents of the IC cost itself ──
    expo = {}
    for arm in ARMS:
        use = usable(arm)
        expo[arm] = {}
        for name in ["fpm_mid", "rel_mul", "rel_enum_mul", "la_mul", "native_all",
                     "gae_all_0.0", "gae_all_0.1", "gae_all_1.0", "gae_mul_0.1", "gae_mul_1.0"]:
            vals = {p: [curve_metric(arm, p, c, name) for c in use[p]] for p in use}
            sl = boot_slope(vals)
            sl.pop("_boot")
            bp = boot_prime(vals)
            expo[arm][name] = {"slope": sl, "per_prime_gm": {str(p): bp[p]["gm"] for p in bp}}
        # six- and seven-prime sensitivity (drop the top prime)
        if len(use) > 2:
            top = max(use)
            vals = {p: [curve_metric(arm, p, c, "fpm_mid") for c in use[p]] for p in use if p != top}
            sl = boot_slope(vals)
            sl.pop("_boot")
            expo[arm]["fpm_mid_without_top_prime"] = sl
    out["exponents"] = expo
    out["rho_ideal_exponent_note"] = "sqrt(pi r/4) with r ~ p^3: slope 1.5 in ln p by construction"

    # ── step A paired effect sfb / new (same curve, same solver seed) ──
    paired = {}
    for name in ["fpm_mid", "rel_mul", "rel_enum_mul", "la_mul", "native_all", "gae_all_0.1",
                 "gae_all_1.0", "gae_mul_1.0", "relations", "attempts", "decomp_trials", "group_ops"]:
        vals = {}
        for p in primes:
            v = []
            for c in curves[p]:
                rs = [metrics(ic[(SFB, p, c)][s])[name] / metrics(ic[(NEW, p, c)][s])[name]
                      for s in ic[(SFB, p, c)] if s in ic[(NEW, p, c)]
                      and metrics(ic[(NEW, p, c)][s])[name] > 0
                      and metrics(ic[(SFB, p, c)][s])[name] > 0]
                if rs:
                    v.append(gm(rs))
            if len(v) >= 2:
                vals[p] = v
        if not vals:
            continue
        bp = boot_prime(vals)
        allv = [x for p in vals for x in vals[p]]
        pooled = []
        for _ in range(B):
            s = []
            for p in vals:
                vv = vals[p]
                s.extend(vv[i] for i in rng.integers(0, len(vv), len(vv)))
            pooled.append(gm(s))
        sl = boot_slope(vals)
        sl.pop("_boot")
        paired[name] = {"per_prime": {str(p): bp[p] for p in sorted(bp)},
                        "pooled": {"gm": gm(allv), "ci95": pct(pooled)}, "slope_ln_ratio": sl}
    out["paired_sfb_over_new"] = paired

    # ── descriptors per prime ──
    desc = {}
    for arm in ARMS:
        desc[arm] = {}
        for p in primes:
            rs = [run for c in curves[p] for run in ic[(arm, p, c)].values()]
            if not rs:
                continue
            g = lambda k: gm([max(run.get(k, 0), 1) for run in rs])
            desc[arm][str(p)] = {
                "runs": len(rs), "fpm_mid": gm([metrics(x)["fpm_mid"] for x in rs]),
                "group_ops": g("group_ops"), "rel_mul": g("rel_mul"), "rel_add": g("rel_add"),
                "rel_enum_mul": g("rel_enum_mul"), "la_mul": g("la_mul"), "la_add": g("la_add"),
                "idx_lookups": g("idx_lookups"), "idx_inserts": g("idx_inserts"),
                "idx_points": g("idx_points"), "n_points": g("n_points"),
                "fb_samples": g("fb_samples"), "fb_cols": g("fb_cols"),
                "relations": g("relations"),
                "attempts_mean": float(np.mean([x["attempts"] for x in rs])),
                "attempts_max": max(x["attempts"] for x in rs),
                "decomp_fallbacks": sum(x.get("decomp_fallbacks", 0) for x in rs),
                "la_skipped": sum(x.get("la_skipped", 0) for x in rs),
                "enum_share_of_rel_mul": float(np.mean([x["rel_enum_mul"] / x["rel_mul"] for x in rs])),
                "native_all": gm([native_all(x) for x in rs]),
                "wall_s_mean": float(np.mean([x["wall_ns"] for x in rs])) * 1e-9,
            }
    out["descriptors"] = desc
    out["reference"] = {f"{p}/{c}": ref[(p, c)] for (p, c) in sorted(ref)}

    # ── measured rho calibration ──
    cal = {}
    for p in primes:
        rows = []
        for c in curves[p]:
            for s, run in rho[(p, c)].items():
                if not (run.get("kind") == "run" and run["certified"] and run["ok"]):
                    continue
                rr = ref[(p, c)]
                fpm = run["cantor_mul"] + 10 * run["cantor_inv"]
                rows.append({"curve": c, "seed": s, "steps_over_ideal": run["group_ops"] / rr["ideal_gae"],
                             "fpm_over_ideal_fpm": fpm / rr["ideal_fpm"],
                             "fpm_per_step": fpm / run["group_ops"]})
        if rows:
            cal[str(p)] = {"runs": len(rows),
                           "steps_over_ideal_gm": gm([x["steps_over_ideal"] for x in rows]),
                           "fpm_over_ideal_fpm_gm": gm([x["fpm_over_ideal_fpm"] for x in rows]),
                           "fpm_per_step_over_A": gm([x["fpm_per_step"] / ref[(p, x["curve"])]["A_fpm_per_op"]
                                                      for x in rows]),
                           "rows": rows}
            # IC / measured rho (sensitivity): per curve, IC gm over seeds / rho gm over seeds
            for arm in ARMS:
                for w in WS:
                    v = []
                    for c in curves[p]:
                        rr = [x for x in rho[(p, c)].values() if x.get("kind") == "run" and x["ok"]]
                        m = curve_metric(arm, p, c, f"gae_all_{w}")
                        if rr and m:
                            v.append(m / gm([x["group_ops"] for x in rr]))
                    if v:
                        cal[str(p)][f"{arm}_over_measured_rho_all_{w}"] = gm(v)
    out["rho_calibration"] = cal
    json.dump(out, sys.stdout, indent=1, default=float)


if __name__ == "__main__":
    main()
