#!/usr/bin/env python3
"""Sample size for the P4 confirmatory study, from the OLD run's variance only.

research/prime_medium_rho_p4_confirm_20261108 (PROTOCOL.md section 7).

Allowed use of the old data (owner decision, 2026-11-08): variance for the
power calculation. NOT allowed: any pooling of the old ratios into the new
estimate. This script reads only per-workload means of rho-neg (A) and
rho-plain (B) from the six committed comparisons of
research/prime_medium_tier_rho_20261107 and reports:

  1. curve-centred pooled per-run standard deviations of S for each arm
     (sigma_run^2 = pairs * within-curve variance of workload means, which
     assumes no target effect: rho on a prime-order group is randomly
     self-reducible, so the target should not matter), and the pooled
     within-curve correlation of the two arms' workload means;
  2. the delta-method standard error of the curve-equal ratio of means for
     a balanced design of N pairs, at the pooled correlation and at its
     lower 95 % end (a negative correlation widens the interval);
  3. a parametric simulation of the planned design (lognormal S per run,
     correlated within a pair at the pooled correlation and at its lower end, the old pooled means and CVs, workload clusters of `rounds`
     pairs) run through the frozen estimator of p4_confirm.py, for truths
     theta = 1.285, 1.35 and sqrt 2: the distribution of the 95 % half-width
     and the probability of each named outcome.

Run:  python3 -I analysis/power_from_old_variance.py --old-root \
          research/prime_medium_tier_rho_20261107 --out analysis/power.json
Standard library only. Deterministic (seed in the arguments).
"""
import argparse
import importlib.util
import json
import math
import os
import random
import statistics as st
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def load_estimator():
    spec = importlib.util.spec_from_file_location("p4_confirm", os.path.join(HERE, "p4_confirm.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def old_workloads(root):
    rows = []
    for j in ["J1", "J2", "J3", "J4", "J5", "J6"]:
        path = os.path.join(root, "sessions", j, "comparisons", "rho-neg__rho-plain.json")
        with open(path) as fh:
            d = json.load(fh)
        for w in d["workloads"]:
            rows.append({"job": j, "slug": w["slug"], "r": w["r"], "pairs": w["pairs"],
                         "a": w["a_mean_s"], "b": w["b_mean_s"]})
    return rows


def pooled_stats(rows):
    by = {}
    for r in rows:
        by.setdefault(r["slug"], []).append(r)
    ssa = ssb = sab = 0.0
    dof = 0
    means_a, means_b = [], []
    pairs = {r["pairs"] for r in rows}
    assert len(pairs) == 1, pairs
    k = pairs.pop()
    for slug, L in by.items():
        a = [x["a"] for x in L]
        b = [x["b"] for x in L]
        ma, mb = st.mean(a), st.mean(b)
        means_a.append(ma)
        means_b.append(mb)
        ssa += sum((x - ma) ** 2 for x in a)
        ssb += sum((x - mb) ** 2 for x in b)
        sab += sum((x - ma) * (y - mb) for x, y in zip(a, b))
        dof += len(L) - 1
    va, vb, cab = ssa / dof, ssb / dof, sab / dof
    rho = cab / math.sqrt(va * vb)
    # Fisher z interval for the pooled correlation (dof - 1 effective).
    z = math.atanh(rho)
    se = 1.0 / math.sqrt(max(dof - 2, 1))
    rho_lo, rho_hi = math.tanh(z - 1.96 * se), math.tanh(z + 1.96 * se)
    mu_a, mu_b = st.mean(means_a), st.mean(means_b)
    sd_a, sd_b = math.sqrt(k * va), math.sqrt(k * vb)
    return {
        "workloads": len(rows), "curves": len(by), "pairs_per_workload": k,
        "within_curve_dof": dof,
        "mean_s_neg": mu_a, "mean_s_plain": mu_b,
        "run_sd_neg": sd_a, "run_sd_plain": sd_b,
        "cv_neg": sd_a / mu_a, "cv_plain": sd_b / mu_b,
        "pair_correlation": rho, "pair_correlation_ci95": [rho_lo, rho_hi],
        "note": "run SD = sqrt(pairs * pooled within-curve variance of workload means); assumes no target effect",
    }


def delta_halfwidth(theta, cv_a, cv_b, rho, n):
    var_ln = (cv_a ** 2 + cv_b ** 2 - 2 * rho * cv_a * cv_b) / n
    return 1.959964 * theta * math.sqrt(var_ln)


def n_for_halfwidth(theta, cv_a, cv_b, rho, h):
    return math.ceil((1.959964 * theta / h) ** 2 * (cv_a ** 2 + cv_b ** 2 - 2 * rho * cv_a * cv_b))


def lognormal_pair(rng, mean_a, cv_a, mean_b, cv_b, rho):
    """Two lognormal S values with the given means, CVs and correlation rho."""
    sa, sb = math.sqrt(math.log1p(cv_a ** 2)), math.sqrt(math.log1p(cv_b ** 2))
    # correlation of the underlying normals that gives corr(X, Y) = rho
    arg = 1.0 + rho * math.sqrt(math.expm1(sa ** 2) * math.expm1(sb ** 2))
    rl = max(-0.999, min(0.999, math.log(arg) / (sa * sb)))
    z1 = rng.gauss(0.0, 1.0)
    z2 = rl * z1 + math.sqrt(1 - rl * rl) * rng.gauss(0.0, 1.0)
    return (mean_a * math.exp(sa * z1 - sa * sa / 2), mean_b * math.exp(sb * z2 - sb * sb / 2))


def simulate(est, stats, theta, curves, targets, rounds, sims, boot, seed, rho):
    rng = random.Random(seed)
    mu_neg = stats["mean_s_neg"]
    out = {"theta_true": theta, "pair_correlation": rho, "sims": sims, "bootstrap_resamples": boot,
           "halfwidths": [], "outcomes": {}, "distribution": "lognormal pairs, old pooled means/CVs"}
    for s in range(sims):
        pairs = []
        for c in range(curves):
            for t in range(targets):
                for k in range(rounds):
                    a, b = lognormal_pair(rng, mu_neg, stats["cv_neg"], mu_neg * theta,
                                          stats["cv_plain"], rho)
                    pairs.append(("c%02d" % c, "c%02dw%03d" % (c, t), k, b, a))
        res = est.ratio_with_interval(pairs, resamples=boot, seed=seed * 1000003 + s)
        hw = (res["ci95"][1] - res["ci95"][0]) / 2
        out["halfwidths"].append(hw)
        v = est.primary_outcome(res["ci95"][0], res["ci95"][1])
        out["outcomes"][v["verdict"]] = out["outcomes"].get(v["verdict"], 0) + 1
    hws = sorted(out["halfwidths"])
    out["halfwidth_median"] = st.median(hws)
    out["halfwidth_p90"] = hws[int(0.9 * (len(hws) - 1))]
    out["p_halfwidth_le_0.05"] = sum(1 for h in hws if h <= 0.05) / len(hws)
    out["outcome_probability"] = {k: v / sims for k, v in sorted(out["outcomes"].items())}
    del out["halfwidths"]
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--old-root", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--curves", type=int, default=10)
    ap.add_argument("--targets", type=int, default=24)
    ap.add_argument("--rounds", type=int, default=6)
    ap.add_argument("--sims", type=int, default=200)
    ap.add_argument("--boot", type=int, default=2000)
    ap.add_argument("--seed", type=int, default=20261108)
    a = ap.parse_args()
    est = load_estimator()
    rows = old_workloads(a.old_root)
    stats = pooled_stats(rows)
    n = a.curves * a.targets * a.rounds
    design = {"curves": a.curves, "targets_per_curve": a.targets, "rounds": a.rounds, "pairs": n}
    delta = {}
    for th in (1.285, 1.35, math.sqrt(2)):
        delta["%.4f" % th] = {
            "halfwidth_at_pooled_rho": delta_halfwidth(th, stats["cv_neg"], stats["cv_plain"], stats["pair_correlation"], n),
            "halfwidth_at_rho_lo": delta_halfwidth(th, stats["cv_neg"], stats["cv_plain"], stats["pair_correlation_ci95"][0], n),
            "halfwidth_at_rho_0": delta_halfwidth(th, stats["cv_neg"], stats["cv_plain"], 0.0, n),
            "n_for_0.05_at_rho_lo": n_for_halfwidth(th, stats["cv_neg"], stats["cv_plain"], stats["pair_correlation_ci95"][0], 0.05),
            "n_for_0.05_at_rho_0": n_for_halfwidth(th, stats["cv_neg"], stats["cv_plain"], 0.0, 0.05),
        }
    # Check of the delta method against the old J1 (288 pairs): reported for calibration only.
    j1_check = delta_halfwidth(1.2855, stats["cv_neg"], stats["cv_plain"], stats["pair_correlation"], 288)
    rho_lo = stats["pair_correlation_ci95"][0]
    sims = [simulate(est, stats, th, a.curves, a.targets, a.rounds, a.sims, a.boot, a.seed + i, rho)
            for i, (th, rho) in enumerate([(t, r) for r in (stats["pair_correlation"], rho_lo)
                                           for t in (1.285, 1.35, math.sqrt(2))])]
    doc = {
        "schema": "prime_medium_rho_p4_confirm.power/v1",
        "inputs": "per-workload mean S of rho-neg and rho-plain, research/prime_medium_tier_rho_20261107/sessions/J1..J6/comparisons/rho-neg__rho-plain.json (variance only; no old ratio enters the new estimate)",
        "evidence_class": "derived (from committed [pcc:unverified] counts)",
        "old_variance": stats,
        "design": design,
        "delta_method": delta,
        "delta_check_old_J1_288_pairs_halfwidth": j1_check,
        "delta_check_old_J1_observed_halfwidth": (1.404 - 1.177) / 2,
        "simulation": sims,
        "python": sys.version.split()[0],
        "args": vars(a),
    }
    with open(a.out, "w") as fh:
        json.dump(doc, fh, indent=1, sort_keys=True)
        fh.write("\n")
    print(json.dumps({"old_variance": stats, "delta": delta, "sim": [
        {k: s[k] for k in ("theta_true", "pair_correlation", "halfwidth_median", "halfwidth_p90", "p_halfwidth_le_0.05", "outcome_probability")}
        for s in sims]}, indent=1))


if __name__ == "__main__":
    main()
