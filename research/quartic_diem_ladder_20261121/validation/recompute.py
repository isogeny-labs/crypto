#!/usr/bin/env python3
"""Validator: independent recomputation of every headline number of RESULTS.md
from the committed raw rows (read from Git), without importing analyze.py.

Usage: python3 -I recompute.py REPO SHA OUT.json

Independent choices (on purpose different from analyze.py):
  * per-prime 95% intervals by EXACT enumeration of the 5^5 cluster-bootstrap
    resamples (multinomial weights), besides a Monte Carlo bootstrap with an
    independent generator (numpy default_rng(4242), 20000 replicates);
  * slopes refitted by closed-form OLS; slope bootstrap with its own generator;
  * every key read from a record is indexed with [] (lesson L-99ee952350b2).
"""
import itertools
import json
import math
import subprocess
import sys
from collections import Counter, defaultdict

import numpy as np

STUDY = "research/quartic_diem_ladder_20261121"
FILES = ["raw/run.jsonl", "raw/run131101_odd.jsonl", "raw/run131101_even.jsonl"]
NEW, SFB, RHO = "Q_diem_2lp_new", "Q_diem_2lp_sfb", "Q_rho_neg"
WS = [0.0, 0.1, 1.0]
NATIVE_KEYS = ["rel_mul", "rel_add", "la_mul", "la_add", "idx_lookups", "idx_inserts"]
IC_KEYS = NATIVE_KEYS + ["group_ops", "cantor_mul", "cantor_inv", "rel_enum_mul", "relations",
                         "attempts", "decomp_trials", "idx_points", "n_points", "fb_samples",
                         "decomp_fallbacks", "rels_by_lp", "rels_line", "lines_tried",
                         "certified", "ok", "recovered", "t_secret", "r", "h", "seed",
                         "curve_seed", "p", "fb_mode"]


def git_rows(repo, sha, path):
    out = subprocess.run(["git", "-C", repo, "show", f"{sha}:{STUDY}/{path}"],
                         check=True, capture_output=True).stdout
    return [json.loads(l) for l in out.decode().splitlines() if l.strip()]


def gm(v):
    return math.exp(sum(math.log(x) for x in v) / len(v))


def n_all(r):
    return sum(r[k] for k in NATIVE_KEYS)


def exact_boot_ci(vals, lo=0.025, hi=0.975):
    """Exact cluster-bootstrap distribution of the geometric mean of n values
    resampled with replacement: enumerate count vectors, multinomial weights."""
    n = len(vals)
    logs = [math.log(v) for v in vals]
    dist = []
    for combo in itertools.combinations_with_replacement(range(n), n):
        c = Counter(combo)
        w = math.factorial(n)
        for k in c.values():
            w //= math.factorial(k)
        dist.append((sum(logs[i] for i in combo) / n, w / n ** n))
    dist.sort()
    out, acc = [], 0.0
    targets = [lo, hi]
    j = 0
    for x, w in dist:
        acc += w
        while j < 2 and acc >= targets[j] - 1e-15:
            out.append(math.exp(x))
            j += 1
    return out


def mc_boot_ci(vals, rng, B=20000):
    v = np.log(np.asarray(vals, float))
    idx = rng.integers(0, len(v), (B, len(v)))
    bs = np.exp(v[idx].mean(axis=1))
    return [float(np.percentile(bs, 2.5)), float(np.percentile(bs, 97.5))]


def ols(x, y):
    x, y = np.asarray(x, float), np.asarray(y, float)
    b = ((x - x.mean()) * (y - y.mean())).sum() / ((x - x.mean()) ** 2).sum()
    return float(b), float(y.mean() - b * x.mean())


def slope_boot(by_prime, rng, B=20000):
    ps = sorted(by_prime)
    x = np.concatenate([np.full(len(by_prime[p]), math.log(p)) for p in ps])
    y = np.concatenate([np.log(by_prime[p]) for p in ps])
    b, a = ols(x, y)
    bb = []
    for _ in range(B):
        ys = np.concatenate([np.log(np.asarray(by_prime[p])[rng.integers(0, len(by_prime[p]), len(by_prime[p]))])
                             for p in ps])
        xm = x.mean()
        bb.append(((x - xm) * (ys - ys.mean())).sum() / ((x - xm) ** 2).sum())
    return {"slope": b, "intercept": a, "ci95": [float(np.percentile(bb, 2.5)), float(np.percentile(bb, 97.5))]}


def main():
    repo, sha, outp = sys.argv[1:4]
    rows = []
    for fn in FILES:
        for r in git_rows(repo, sha, fn):
            r["_file"] = fn
            rows.append(r)
    out = {"sha": sha}
    inst = [r for r in rows if r["kind"] == "instance" and r["family"] == "quartic"]
    cells = [r for r in rows if r["kind"] == "cell" and r["family"] == "quartic"]
    runs = [r for r in rows if r["kind"] == "run" and r["family"] == "quartic"]
    ic = [r for r in runs if r["arm"] in (NEW, SFB)]
    rho = [r for r in runs if r["arm"] == RHO]
    skips = [r for r in rows if r["kind"] == "skip"]
    # key existence (L-99ee952350b2): every key read below exists in every IC row
    missing = Counter(k for r in ic for k in IC_KEYS if k not in r)
    out["missing_keys_in_ic_rows"] = dict(missing)
    out["counts"] = {"instances": len(inst), "ic_runs": dict(Counter(r["arm"] for r in ic)),
                     "rho_runs": len(rho), "rho_skips": len(skips),
                     "instances_per_prime": dict(Counter(r["p"] for r in inst))}
    out["cells"] = [{k: c[k] for k in ("p", "tried", "admitted", "search_wall_s")} | {"file": c["_file"]}
                    for c in cells]
    out["all_ic_certified_ok"] = all(r["certified"] and r["ok"] and r["recovered"] == r["t_secret"] for r in ic)
    out["all_rho_certified_ok"] = all(r["certified"] and r["ok"] and r["recovered"] == r["t_secret"] for r in rho)
    out["decomp_fallbacks_total"] = sum(r["decomp_fallbacks"] for r in ic)
    out["max_attempts"] = max(r["attempts"] for r in ic)
    out["fb_mode"] = dict(Counter(r["arm"] + "|" + r["fb_mode"] for r in ic))
    out["h_values"] = dict(Counter(r["h"] for r in ic))

    by = defaultdict(dict)  # (arm, p, cs) -> seed -> row
    for r in ic:
        by[(r["arm"], r["p"], r["curve_seed"])][r["seed"]] = r
    primes = sorted({r["p"] for r in ic})
    curves = {p: sorted({r["curve_seed"] for r in ic if r["p"] == p}) for p in primes}
    out["curves"] = {str(p): curves[p] for p in primes}
    ideal = {}
    A = {}
    for p in primes:
        for c in curves[p]:
            r0 = next(iter(by[(NEW, p, c)].values()))
            ideal[(p, c)] = math.sqrt(math.pi * r0["r"] / 4)
            num = sum(x["cantor_mul"] + 10 * x["cantor_inv"] for a in (NEW, SFB) for x in by[(a, p, c)].values())
            den = sum(x["group_ops"] for a in (NEW, SFB) for x in by[(a, p, c)].values())
            A[(p, c)] = num / den

    def curve_val(arm, p, c, f):
        v = [f(x) for x in by[(arm, p, c)].values()]
        assert len(v) == 2, (arm, p, c, len(v))
        return gm(v)

    rng = np.random.default_rng(4242)
    # IC / rho, primary band
    tab = {}
    for arm in (NEW, SFB):
        for w in WS:
            byp = {}
            for p in primes:
                byp[p] = [curve_val(arm, p, c, lambda x: x["group_ops"] + w * n_all(x)) / ideal[(p, c)]
                          for c in curves[p]]
            cells_ = {}
            for p in primes:
                cells_[str(p)] = {"gm": gm(byp[p]), "exact_boot_ci95": exact_boot_ci(byp[p]),
                                  "mc_boot_ci95": mc_boot_ci(byp[p], rng), "per_curve": byp[p]}
            tab[f"{arm}|all_{w}"] = {"per_prime": cells_, "slope": slope_boot(byp, rng)}
        # mul-only sensitivity and FPM
        for w in (0.1, 1.0):
            byp = {p: [curve_val(arm, p, c, lambda x: x["group_ops"] + w * (x["rel_mul"] + x["la_mul"]))
                       / ideal[(p, c)] for c in curves[p]] for p in primes}
            tab[f"{arm}|mul_{w}"] = {"per_prime": {str(p): {"gm": gm(byp[p]), "exact_boot_ci95": exact_boot_ci(byp[p])}
                                                   for p in primes}, "slope": slope_boot(byp, rng)}
        byp = {p: [curve_val(arm, p, c, lambda x: x["cantor_mul"] + 10 * x["cantor_inv"] + x["rel_mul"] + 9 * x["la_mul"])
                   / (ideal[(p, c)] * A[(p, c)]) for c in curves[p]] for p in primes}
        tab[f"{arm}|fpm"] = {"per_prime": {str(p): {"gm": gm(byp[p]), "exact_boot_ci95": exact_boot_ci(byp[p])}
                                           for p in primes}, "slope": slope_boot(byp, rng)}
    out["ic_over_rho"] = tab
    # frozen verdict rule on both interval methods
    verdict = {}
    for arm in (NEW, SFB):
        for p in primes:
            for meth in ("exact_boot_ci95", "mc_boot_ci95"):
                ups = [tab[f"{arm}|all_{w}"]["per_prime"][str(p)][meth][1] for w in WS]
                los = [tab[f"{arm}|all_{w}"]["per_prime"][str(p)][meth][0] for w in WS]
                v = ("BELOW-RHO CANDIDATE" if all(u < 1 for u in ups) else
                     "ABOVE RHO" if all(l > 1 for l in los) else "UNRESOLVED")
                verdict[f"{arm}|{p}|{meth}"] = v
    out["verdict"] = verdict
    # rho charged one native op per step (band.rs-style canonicalisation), sensitivity
    out["rho_charged_1_native_per_step"] = {
        f"{arm}|{p}|w={w}": tab[f"{arm}|all_{w}"]["per_prime"][str(p)]["gm"] / (1 + w)
        for arm in (NEW, SFB) for p in (65537, 131101) for w in WS}
    # exponents of IC metrics
    expo = {}
    metrics = {
        "fpm_mid": lambda x: x["cantor_mul"] + 10 * x["cantor_inv"] + x["rel_mul"] + 9 * x["la_mul"],
        "rel_mul": lambda x: x["rel_mul"], "rel_enum_mul": lambda x: x["rel_enum_mul"],
        "la_mul": lambda x: x["la_mul"], "native_all": n_all, "group_ops": lambda x: x["group_ops"],
        "gae_mul_1.0": lambda x: x["group_ops"] + x["rel_mul"] + x["la_mul"],
    }
    for arm in (NEW, SFB):
        for name, f in metrics.items():
            byp = {p: [curve_val(arm, p, c, f) for c in curves[p]] for p in primes}
            expo[f"{arm}|{name}"] = slope_boot(byp, rng)
        byp = {p: [curve_val(arm, p, c, metrics["fpm_mid"]) for c in curves[p]] for p in primes if p != max(primes)}
        expo[f"{arm}|fpm_mid_without_top"] = slope_boot(byp, rng)
    out["exponents"] = expo
    # paired sfb / new
    paired = {}
    pm = dict(metrics)
    pm["relations"] = lambda x: x["relations"]
    pm["attempts"] = lambda x: max(x["attempts"], 1)
    for name, f in pm.items():
        byp = {}
        for p in primes:
            byp[p] = [gm([f(by[(SFB, p, c)][s]) / f(by[(NEW, p, c)][s]) for s in (1, 2)]) for c in curves[p]]
        allv = [v for p in primes for v in byp[p]]
        B = 20000
        pooled = []
        for _ in range(B):
            s = []
            for p in primes:
                v = np.asarray(byp[p])
                s.extend(v[rng.integers(0, len(v), len(v))])
            pooled.append(gm(s))
        paired[name] = {"pooled_gm": gm(allv), "pooled_ci95": [float(np.percentile(pooled, 2.5)),
                                                                 float(np.percentile(pooled, 97.5))],
                        "per_prime": {str(p): gm(byp[p]) for p in primes}, "slope": slope_boot(byp, rng)}
    out["paired_sfb_over_new"] = paired
    # descriptors at 131101 (and every prime) for sfb and new
    desc = {}
    for arm in (NEW, SFB):
        for p in primes:
            rs = [x for c in curves[p] for x in by[(arm, p, c)].values()]
            la = gm([x["la_add"] for x in rs])
            na = gm([n_all(x) for x in rs])
            lp_ops = [x["rels_by_lp"][1] + 2 * x["rels_by_lp"][2] for x in rs]
            desc[f"{arm}|{p}"] = {
                "gm_la_add": la, "gm_native_all": na, "ratio_of_gms": la / na,
                "mean_per_run_share_la_add": float(np.mean([x["la_add"] / n_all(x) for x in rs])),
                "gm_group_ops": gm([x["group_ops"] for x in rs]),
                "A_fpm_per_pic_op": gm([A[(p, c)] for c in curves[p]]),
                "gm_fb_samples": gm([max(x["fb_samples"], 1) for x in rs]),
                "idx_points_over_n_points": float(np.mean([x["idx_points"] / x["n_points"] for x in rs])),
                # uncounted hash work in push_rel / seen_lines (both arms): vid entry per large
                # prime occurrence, one seen_lines insert per split line (>= rels_line)
                "uncounted_hash_ops_lower_est_over_native_all": float(np.mean(
                    [(lp + x["rels_line"]) / n_all(x) for lp, x in zip(lp_ops, rs)])),
                "idx_ops_over_native_all": float(np.mean([(x["idx_lookups"] + x["idx_inserts"]) / n_all(x) for x in rs])),
                "rel_add_share": float(np.mean([x["rel_add"] / n_all(x) for x in rs])),
            }
    out["descriptors"] = desc
    # rho calibration
    cal = {}
    rho_by = defaultdict(list)
    for r in rho:
        rho_by[(r["p"], r["curve_seed"])].append(r)
    for p in primes:
        rr = [(c, x) for c in curves[p] for x in rho_by[(p, c)]]
        if not rr:
            continue
        cal[str(p)] = {
            "runs": len(rr),
            "steps_over_ideal_gm": gm([x["group_ops"] / ideal[(p, c)] for c, x in rr]),
            "steps_over_ideal_mean": float(np.mean([x["group_ops"] / ideal[(p, c)] for c, x in rr])),
            "fpm_per_step_over_A": gm([(x["cantor_mul"] + 10 * x["cantor_inv"]) / x["group_ops"] / A[(p, c)]
                                       for c, x in rr]),
            "group_ops_eq_steps": all(x["group_ops"] >= x["steps"] for c, x in rr),
        }
    allr = [x["group_ops"] / ideal[(x["p"], x["curve_seed"])] for x in rho]
    cal["pooled"] = {"runs": len(allr), "gm": gm(allr), "mean": float(np.mean(allr)),
                     "expected_gm_over_mean_if_rayleigh": math.exp(-0.5772156649 / 2) * math.sqrt(2) / math.sqrt(math.pi / 2)}
    out["rho_calibration"] = cal
    json.dump(out, open(outp, "w"), indent=1, default=float)


if __name__ == "__main__":
    main()
