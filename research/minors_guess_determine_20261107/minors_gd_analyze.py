#!/usr/bin/env python3
"""Pre-specified analysis of minors_gd panel records (PROTOCOL.md section 7).

Run as:  python3 -I minors_gd_analyze.py RECORDS.jsonl [RECORDS2.jsonl ...] [--label dev|panel] [--boot 2000] [--seed 20261107]

Stdlib only. Input: one `minors_gd.run/v1` JSON object per line. Output: a JSON report on
stdout (and a short text table on stderr). Nothing here is a measurement: it is the
estimator applied to whatever records it is given; the report carries the label.

Estimators
  rho / rho_aa : cost_b = mean over instances of charged work (each run solves).
  other arms   : cost_b = (sum of charged work over all runs) / (number of successes)
                 (censored Poisson-process / exponential MLE; a run that hits its cap
                 contributes its work and no success). With 0 successes only the
                 one-sided 95% lower bound  exposure / 2.996  is reported.
Exponent: ordinary least squares slope of log2(cost_b) on bits over the sizes with an
estimate (>= 3 sizes needed), bootstrapped over instances (same resampled instance
indices for every arm, so the ratio to rho is paired by planted instance).
Pricing: charged = muls + w*invs with w = 1.5*bits(p) (primary). Sensitivity: w = 3 and
w = 10 multiplications, and the unpriced-work band lam in {0, 0.1, 1} multiplications per
uncharged operation (adds + hash inserts for the minors arms; adds + 3*steps for rho).
"""
import json, math, random, sys, collections


def bits_p(p):
    return p.bit_length()


def price(r, w=None, lam=0.0):
    w = 1.5 * bits_p(r["p"]) if w is None else w
    unp = r["adds"] + r["hash_inserts"]
    if r["solver"].startswith("rho"):
        unp += 3 * r["extra"].get("steps", 0)
    return r["muls"] + w * r["invs"] + lam * unp


def garwood_upper_rate(k, alpha=0.025):
    """upper (1-alpha... two-sided level) Poisson mean bound for k events: P(X<=k | mu)=alpha."""
    lo, hi = 0.0, 10.0 + 10 * k + 50
    for _ in range(200):
        mid = (lo + hi) / 2
        c = sum(math.exp(-mid + i * math.log(mid) - math.lgamma(i + 1)) for i in range(k + 1)) if mid > 0 else 1.0
        if c > alpha:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def garwood_lower_rate(k, alpha=0.025):
    if k == 0:
        return 0.0
    lo, hi = 0.0, float(k)
    for _ in range(200):
        mid = (lo + hi) / 2
        c = 1.0 - sum(math.exp(-mid + i * math.log(mid) - math.lgamma(i + 1)) for i in range(k)) if mid > 0 else 0.0
        if c < alpha:
            lo = mid
        else:
            hi = mid
    return (lo + hi) / 2


def ols_slope(xs, ys):
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    if sxx == 0:
        return None
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx


def pct(v, q):
    v = sorted(v)
    if not v:
        return None
    k = (len(v) - 1) * q
    f = int(math.floor(k)); c = min(f + 1, len(v) - 1)
    return v[f] + (v[c] - v[f]) * (k - f)


def size_cost(runs, solver, w=None, lam=0.0):
    """runs: list of records of one (bits, solver). returns (cost or None, successes, exposure)."""
    exp = sum(price(r, w, lam) for r in runs)
    k = sum(1 for r in runs if r["success"])
    if solver.startswith("rho"):
        return (exp / len(runs) if runs else None), k, exp
    return (exp / k if k else None), k, exp


def fit(costs, bits_list, min_sizes=3):
    pts = [(b, math.log2(costs[b])) for b in bits_list if costs.get(b)]
    if len(pts) < min_sizes:
        return None
    return ols_slope([p[0] for p in pts], [p[1] for p in pts])


def main(argv):
    files = []
    skip = False
    for i, a in enumerate(argv):
        if skip:
            skip = False
        elif a.startswith("--"):
            skip = True
        else:
            files.append(a)
    opt = lambda n, d: (argv[argv.index(n) + 1] if n in argv else d)
    label = opt("--label", "UNLABELLED")
    nboot = int(opt("--boot", "2000"))
    rng = random.Random(int(opt("--seed", "20261107")))
    recs = []
    for f in files:
        for line in open(f):
            line = line.strip()
            if line:
                recs.append(json.loads(line))
    by = collections.defaultdict(lambda: collections.defaultdict(dict))  # solver -> bits -> idx -> rec
    for r in recs:
        by[r["solver"]][r["bits"]][r["idx"]] = r
    solvers = sorted(by)
    bits_all = sorted({b for s in by for b in by[s]})
    out = {"schema": "minors_gd.analysis/v1", "label": label, "files": files, "n_records": len(recs), "bootstrap": nboot, "sizes": bits_all, "arms": {}}
    # common idx set per size (matched instances)
    common = {b: sorted(set.intersection(*[set(by[s][b]) for s in solvers if b in by[s]])) for b in bits_all}
    out["matched_instances_per_size"] = {str(b): len(v) for b, v in common.items()}

    def arm_table(w=None, lam=0.0):
        t = {}
        for s in solvers:
            t[s] = {}
            for b in bits_all:
                if b not in by[s]:
                    continue
                runs = [by[s][b][i] for i in common[b]]
                c, k, e = size_cost(runs, s, w, lam)
                row = {"cost": c, "successes": k, "runs": len(runs), "exposure": e}
                if not s.startswith("rho"):
                    if k:
                        row["cost_ci95_garwood"] = [e / garwood_upper_rate(k), (e / garwood_lower_rate(k)) if garwood_lower_rate(k) > 0 else None]
                    else:
                        row["cost_lower_bound_95_onesided"] = e / 2.996
                else:
                    xs = [price(r, w, lam) for r in runs]
                    if len(xs) > 1:
                        m = sum(xs) / len(xs); sd = math.sqrt(sum((x - m) ** 2 for x in xs) / (len(xs) - 1))
                        row["cost_ci95_normal"] = [m - 1.96 * sd / math.sqrt(len(xs)), m + 1.96 * sd / math.sqrt(len(xs))]
                t[s][str(b)] = row
        return t

    out["costs_primary"] = arm_table()
    out["costs_inv_weight_3"] = arm_table(w=3.0)
    out["costs_inv_weight_10"] = arm_table(w=10.0)
    out["costs_unpriced_0.1"] = arm_table(lam=0.1)
    out["costs_unpriced_1"] = arm_table(lam=1.0)

    # ratios to rho, point estimates
    ratios = {}
    for s in solvers:
        if s.startswith("rho"):
            continue
        ratios[s] = {}
        for b in bits_all:
            a = out["costs_primary"].get(s, {}).get(str(b)); r = out["costs_primary"].get("rho", {}).get(str(b))
            if a and r and r["cost"]:
                if a["cost"]:
                    ratios[s][str(b)] = a["cost"] / r["cost"]
                else:
                    ratios[s][str(b)] = {"lower_bound": a["cost_lower_bound_95_onesided"] / r["cost"]}
    out["ratio_to_rho_point"] = ratios

    # bootstrap exponent and ratios, matched resampling of instance indices
    fits = {s: {"all_sizes": [], "top3": []} for s in solvers}
    ratio_boot = {s: {b: [] for b in bits_all} for s in solvers if not s.startswith("rho")}
    undetermined = {s: 0 for s in solvers}
    dslope = {s: {"all_sizes": [], "top3": []} for s in solvers if not s.startswith("rho")}
    big = sorted(bits_all)[-3:]
    for _ in range(nboot):
        draw = {b: [rng.choice(common[b]) for _ in common[b]] for b in bits_all}
        costs = {s: {} for s in solvers}
        for s in solvers:
            for b in bits_all:
                if b not in by[s]:
                    continue
                runs = [by[s][b][i] for i in draw[b]]
                c, k, e = size_cost(runs, s)
                if c:
                    costs[s][b] = c
        for s in solvers:
            sl = fit(costs[s], bits_all)
            if sl is None:
                undetermined[s] += 1
            else:
                fits[s]["all_sizes"].append(sl)
            sl3 = fit(costs[s], big, min_sizes=3)
            if sl3 is not None:
                fits[s]["top3"].append(sl3)
            if s in dslope and costs.get("rho"):
                a1, r1 = fit(costs[s], bits_all), fit(costs["rho"], bits_all)
                a3, r3 = fit(costs[s], big), fit(costs["rho"], big)
                if a1 is not None and r1 is not None:
                    dslope[s]["all_sizes"].append(a1 - r1)
                if a3 is not None and r3 is not None:
                    dslope[s]["top3"].append(a3 - r3)
            if s in ratio_boot and "rho" in costs:
                for b in bits_all:
                    if b in costs[s] and b in costs["rho"]:
                        ratio_boot[s][b].append(costs[s][b] / costs["rho"][b])
    out["exponent"] = {}
    for s in solvers:
        pts = {b: out["costs_primary"][s][str(b)]["cost"] for b in bits_all if b in by[s] and out["costs_primary"][s][str(b)]["cost"]}
        point_all = fit(pts, bits_all)
        point_top3 = fit(pts, big)
        row = {"point_all_sizes": point_all, "point_top3": point_top3,
               "ci95_all_sizes": [pct(fits[s]["all_sizes"], 0.025), pct(fits[s]["all_sizes"], 0.975)] if fits[s]["all_sizes"] else None,
               "ci95_top3": [pct(fits[s]["top3"], 0.025), pct(fits[s]["top3"], 0.975)] if fits[s]["top3"] else None,
               "bootstrap_undetermined_fraction": undetermined[s] / nboot, "sizes_with_estimate": sorted(pts)}
        out["exponent"][s] = row
    out["slope_minus_rho_slope_ci95"] = {s: {k: [pct(v, 0.025), pct(v, 0.975)] if v else None for k, v in d.items()} for s, d in dslope.items()}
    out["ratio_to_rho_ci95"] = {s: {str(b): [pct(v, 0.025), pct(v, 0.975)] for b, v in d.items() if v} for s, d in ratio_boot.items()}

    # decision rule (PROTOCOL.md section 8), reported mechanically, not interpreted
    dec = {}
    for s in solvers:
        if s.startswith("rho"):
            continue
        e = out["exponent"][s]
        hi = e["ci95_all_sizes"][1] if e["ci95_all_sizes"] else None
        hi3 = e["ci95_top3"][1] if e["ci95_top3"] else None
        above = []
        for b in bits_all:
            r = out["ratio_to_rho_point"].get(s, {}).get(str(b))
            if r is None:
                continue
            above.append(r if isinstance(r, float) else r["lower_bound"])
        beats_at_32 = [b for b in bits_all if b >= 32 and isinstance(out["ratio_to_rho_point"].get(s, {}).get(str(b)), float) and out["ratio_to_rho_point"][s][str(b)] < 1]
        dec[s] = {"exponent_upper95_all": hi, "exponent_upper95_top3": hi3,
                  "min_ratio_to_rho_over_sizes": min(above) if above else None,
                  "sizes_ge_32_where_arm_cheaper_than_rho": beats_at_32,
                  "rule_closes_lead_for_this_reconstruction": (hi is not None and hi >= 0.5 and (min(above) > 1 if above else False)),
                  "rule_real_lead_flag": (hi is not None and hi < 0.5) or bool(beats_at_32)}
    out["decision_rule_mechanical"] = dec
    # A/A
    if "rho" in by and "rho_aa" in by:
        aa = {}
        for b in bits_all:
            if b in by["rho"] and b in by["rho_aa"] and common[b]:
                a = out["costs_primary"]["rho"][str(b)]["cost"]; c = out["costs_primary"]["rho_aa"][str(b)]["cost"]
                vals = []
                for _ in range(nboot):
                    dr = [rng.choice(common[b]) for _ in common[b]]
                    x = sum(price(by["rho"][b][i]) for i in dr); y = sum(price(by["rho_aa"][b][i]) for i in dr)
                    vals.append(y / x)
                aa[str(b)] = {"ratio": c / a, "ci95": [pct(vals, 0.025), pct(vals, 0.975)]}
        out["aa_rho_vs_rho_aa"] = aa
    out["sanity"] = {"all_successes_correct": all(r["correct"] for r in recs if r["success"]),
                     "bad_candidates_total": sum(r["bad_candidates"] for r in recs)}
    json.dump(out, sys.stdout, indent=1, default=str)
    sys.stdout.write("\n")
    # text table
    sys.stderr.write("label=%s  records=%d\n" % (label, len(recs)))
    sys.stderr.write("%-10s" % "arm" + "".join("%12d" % b for b in bits_all) + "\n")
    for s in solvers:
        row = "%-10s" % s
        for b in bits_all:
            c = out["costs_primary"].get(s, {}).get(str(b))
            if not c:
                row += "%12s" % "-"
            elif c["cost"]:
                row += "%12.3g" % c["cost"]
            else:
                row += "%12s" % (">%.2g" % c["cost_lower_bound_95_onesided"])
        sys.stderr.write(row + "\n")
    for s, e in out["exponent"].items():
        sys.stderr.write("exponent %-10s all=%s ci=%s | top3=%s ci=%s\n" % (s, "%.3f" % e["point_all_sizes"] if e["point_all_sizes"] is not None else None, e["ci95_all_sizes"], "%.3f" % e["point_top3"] if e["point_top3"] is not None else None, e["ci95_top3"]))


if __name__ == "__main__":
    main(sys.argv[1:])
