#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md (research/quartic_diem_la_20261123).

Usage: python3 analyze.py raw/run.jsonl [more.jsonl ...] > analysis.json

Arms (paired on the same instance and solver seed):
  Q_diem_2lp_sfb      the ladder's sampled-factor-base arm (baseline, unchanged)
  Q_diem_2lp_sfb_sge  the same relation phase; linear algebra = trim + SGE +
                      square coordinate Wiedemann (QLinAlg::SgeCoord)
Reference: ideal negation rho sqrt(pi r / 4) group additions (nothing unpriced
charged to rho), as in research/quartic_diem_ladder_20261121.
Reproduction check C0: the Q_diem_2lp_sfb rows against the ladder's raw rows.
Every number is [pcc:unverified].
"""
import json
import math
import sys
from collections import defaultdict

import numpy as np

SFB = "Q_diem_2lp_sfb"
SGE = "Q_diem_2lp_sfb_sge"
ARMS = [SFB, SGE]
WS = [0.0, 0.1, 1.0]
B = 4000
SEED = 20261123
LADDER = ["../quartic_diem_ladder_20261121/raw/run.jsonl",
          "../quartic_diem_ladder_20261121/raw/run131101_odd.jsonl",
          "../quartic_diem_ladder_20261121/raw/run131101_even.jsonl"]
C0_FIELDS = ["certified", "ok", "recovered", "group_ops", "cantor_mul", "cantor_inv",
             "rel_mul", "rel_add", "la_mul", "la_add", "relations", "rels_by_lp",
             "decomp_trials", "attempts", "core_rows", "core_cols", "idx_lookups",
             "idx_inserts", "idx_points", "fb_samples"]
SPLIT = ["rows", "dsu", "peel", "tscan", "trim", "sge", "spmv", "bm", "vec"]
# ratio of the pooled paired GAE_1 below which the change is a material saving
KILL_LO = 0.90
W0_TIE = 1.01


def native_all(r):
    return (r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]
            + r.get("idx_lookups", 0) + r.get("idx_inserts", 0))


def gae(r, w):
    return r["group_ops"] + w * native_all(r)


def gm(v):
    v = [x for x in v if x is not None]
    return math.exp(sum(math.log(x) for x in v) / len(v)) if v else None


def pct(a, lo=2.5, hi=97.5):
    return [float(np.percentile(a, lo)), float(np.percentile(a, hi))]


def ols(x, y):
    x = np.asarray(x, float)
    y = np.asarray(y, float)
    b = ((x - x.mean()) * (y - y.mean())).sum() / ((x - x.mean()) ** 2).sum()
    return b, y.mean() - b * x.mean()


def load(paths):
    runs, inst = [], {}
    for path in paths:
        for line in open(path):
            r = json.loads(line)
            if r.get("kind") == "instance":
                inst[(r["p"], r["curve_seed"])] = r
            elif r.get("kind") == "run" and r.get("arm") in ARMS:
                runs.append(r)
    return runs, inst


def main():
    runs, inst = load(sys.argv[1:])
    out = {"protocol": "research/quartic_diem_la_20261123/PROTOCOL.md",
           "label": "[pcc:unverified]"}
    by = defaultdict(dict)  # (arm, p, cs) -> {seed: run}
    bad = []
    for r in runs:
        ok = r["certified"] and r["ok"] and r.get("recovered") == r.get("t_secret")
        if not ok:
            bad.append({k: r.get(k) for k in ("arm", "p", "curve_seed", "seed", "recovered")})
        by[(r["arm"], r["p"], r["curve_seed"])][r["seed"]] = r
    out["uncertified_or_wrong_log"] = bad
    out["n_runs"] = {a: sum(1 for r in runs if r["arm"] == a) for a in ARMS}
    primes = sorted({p for (p, _) in inst})
    curves = {p: sorted(c for (q, c) in inst if q == p) for p in primes}
    pairs = defaultdict(list)  # p -> [(cs, seed, sfb, sge)]
    unpaired = []
    for p in primes:
        for c in curves[p]:
            for s in sorted(set(by[(SFB, p, c)]) | set(by[(SGE, p, c)])):
                a, b = by[(SFB, p, c)].get(s), by[(SGE, p, c)].get(s)
                if a is None or b is None:
                    unpaired.append([p, c, s])
                    continue
                pairs[p].append((c, s, a, b))
    out["unpaired"] = unpaired
    out["same_log_both_arms"] = all(a["recovered"] == b["recovered"]
                                    for p in pairs for (_, _, a, b) in pairs[p])

    # ── C0: the baseline arm reproduces the ladder's rows ──
    lad = {}
    for path in LADDER:
        try:
            fh = open(path)
        except OSError:
            continue
        for line in fh:
            r = json.loads(line)
            if r.get("kind") == "run" and r.get("arm") == SFB:
                lad[(r["p"], r["curve_seed"], r["seed"])] = r
    c0 = {"compared": 0, "identical": 0, "mismatches": []}
    for p in pairs:
        for (c, s, a, _) in pairs[p]:
            ref = lad.get((p, c, s))
            if ref is None:
                continue
            c0["compared"] += 1
            diff = [f for f in C0_FIELDS if ref.get(f) != a.get(f)]
            if diff:
                c0["mismatches"].append({"p": p, "curve_seed": c, "seed": s, "fields": diff})
            else:
                c0["identical"] += 1
    out["C0_reproduction"] = c0

    rng = np.random.default_rng(SEED)

    def curve_vals(p, f):
        """curve-level geo-mean over seeds of f(sfb, sge)."""
        d = defaultdict(list)
        for (c, s, a, b) in pairs[p]:
            d[c].append(f(a, b))
        return [gm(v) for c, v in sorted(d.items())]

    def boot(vals_by_prime):
        per = {}
        for p, v in vals_by_prime.items():
            lv = np.log(np.asarray(v, float))
            bs = [math.exp(rng.choice(lv, len(lv)).mean()) for _ in range(B)]
            per[str(p)] = {"gm": math.exp(lv.mean()), "ci95": pct(bs), "n_curves": len(v)}
        # pooled: equal weight per prime, curves resampled within prime
        ps = sorted(vals_by_prime)
        logs = {p: np.log(np.asarray(vals_by_prime[p], float)) for p in ps}
        pt = math.exp(np.mean([logs[p].mean() for p in ps]))
        bs = [math.exp(np.mean([rng.choice(logs[p], len(logs[p])).mean() for p in ps]))
              for _ in range(B)]
        # slope in ln p
        xs = [math.log(p) for p in ps for _ in logs[p]]
        ys = [y for p in ps for y in logs[p]]
        b, _ = ols(xs, ys)
        bb = []
        for _ in range(B):
            x2, y2 = [], []
            for p in ps:
                for i in rng.integers(0, len(logs[p]), len(logs[p])):
                    x2.append(math.log(p))
                    y2.append(logs[p][i])
            bb.append(ols(x2, y2)[0])
        return {"per_prime": per, "pooled": {"gm": pt, "ci95": pct(bs)},
                "slope_ln_p": {"slope": float(b), "ci95": pct(bb)}}

    # ── paired ratios sge / sfb ──
    paired = {}
    for w in WS:
        paired[f"gae_{w}"] = boot({p: curve_vals(p, lambda a, b, w=w: gae(b, w) / gae(a, w))
                                   for p in pairs})
    for name, f in [("la_add", lambda r: r["la_add"]), ("la_mul", lambda r: r["la_mul"]),
                    ("la_total", lambda r: r["la_add"] + r["la_mul"]),
                    ("native_all", native_all)]:
        paired[name] = boot({p: curve_vals(p, lambda a, b, f=f: f(b) / f(a)) for p in pairs})
    out["paired_sge_over_sfb"] = paired

    # ── IC / ideal negation rho, per arm ──
    icr = {}
    verdict = {}
    for arm, k in [(SFB, 2), (SGE, 3)]:
        icr[arm] = {}
        for w in WS:
            vals = {}
            for p in pairs:
                d = defaultdict(list)
                for t in pairs[p]:
                    run = t[k]
                    ideal = math.sqrt(math.pi * inst[(p, t[0])]["r"] / 4)
                    d[t[0]].append(gae(run, w) / ideal)
                vals[p] = [gm(v) for c, v in sorted(d.items())]
            icr[arm][f"all_{w}"] = boot(vals)
        verdict[arm] = {}
        for p in sorted(pairs):
            ups = [icr[arm][f"all_{w}"]["per_prime"][str(p)]["ci95"][1] for w in WS]
            los = [icr[arm][f"all_{w}"]["per_prime"][str(p)]["ci95"][0] for w in WS]
            v = ("BELOW-RHO CANDIDATE" if all(u < 1 for u in ups) else
                 "ABOVE RHO" if all(lo > 1 for lo in los) else
                 "UNRESOLVED (order changes inside the band)")
            verdict[arm][str(p)] = {"verdict": v, "upper95": ups, "lower95": los}
    out["ic_over_rho"] = icr
    out["ic_over_rho_verdict"] = verdict

    # ── la breakdown (geo-mean over runs of each bucket's share; arithmetic sums) ──
    brk = {}
    for p in pairs:
        brk[str(p)] = {}
        for arm, k in [(SFB, 2), (SGE, 3)]:
            rs = [t[k] for t in pairs[p]]
            tot_add = sum(r["la_add"] for r in rs)
            tot_mul = sum(r["la_mul"] for r in rs)
            e = {"la_add_mean": tot_add / len(rs), "la_mul_mean": tot_mul / len(rs),
                 "native_all_mean": sum(native_all(r) for r in rs) / len(rs),
                 "la_share_of_native": (tot_add + tot_mul) / sum(native_all(r) for r in rs),
                 "attempts_mean": sum(r["attempts"] for r in rs) / len(rs),
                 "la_fallbacks": sum(r.get("la_fallbacks", 0) for r in rs),
                 "group_ops_mean": sum(r["group_ops"] for r in rs) / len(rs)}
            if all(f"la_{s}_add" in r for r in rs for s in SPLIT):
                e["split_add_share"] = {s: sum(r[f"la_{s}_add"] for r in rs) / tot_add for s in SPLIT}
                e["split_mul_share"] = {s: sum(r[f"la_{s}_mul"] for r in rs) / max(tot_mul, 1)
                                        for s in SPLIT}
                e["split_sums_ok"] = all(
                    sum(r[f"la_{s}_add"] for s in SPLIT) == r["la_add"]
                    and sum(r[f"la_{s}_mul"] for s in SPLIT) == r["la_mul"] for r in rs)
            if arm == SGE:
                e["sq_dim_mean"] = sum(r.get("sq_dim", 0) for r in rs) / len(rs)
                e["sq_nnz_mean"] = sum(r.get("sq_nnz", 0) for r in rs) / len(rs)
                e["core_nnz_mean"] = sum(r.get("core_nnz", 0) for r in rs) / len(rs)
                e["core_rows_mean"] = sum(r["core_rows"] for r in rs) / len(rs)
                e["core_cols_mean"] = sum(r["core_cols"] for r in rs) / len(rs)
            brk[str(p)][arm] = e
    out["la_breakdown"] = brk

    # ── frozen decision rule (PROTOCOL.md section 6) ──
    g1, g01, g0 = paired["gae_1.0"]["pooled"], paired["gae_0.1"]["pooled"], paired["gae_0.0"]["pooled"]
    integrity = (not bad and not unpaired and out["same_log_both_arms"]
                 and c0["compared"] > 0 and c0["identical"] == c0["compared"])
    if not integrity:
        dec = "VOID (integrity check failed)"
    elif g1["ci95"][1] < 1 and g01["ci95"][1] < 1 and g0["ci95"][1] <= W0_TIE:
        dec = "IMPROVEMENT (ranked below 2lp-sfb across the band)"
    elif g1["ci95"][0] >= KILL_LO:
        dec = "KILL (no material saving at w = 1)"
    else:
        dec = "INCONCLUSIVE"
    out["decision"] = {"rule": "PROTOCOL.md section 6", "integrity": integrity, "verdict": dec,
                       "pooled_gae_1": g1, "pooled_gae_0.1": g01, "pooled_gae_0": g0}
    json.dump(out, sys.stdout, indent=1, default=float)
    print()


if __name__ == "__main__":
    main()
