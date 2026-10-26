"""Lay out recorded values as Markdown tables (medians and ranges of the
recorded per-run numbers; no new measurement).  Usage: python summarize.py"""
import json, os
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(l) for l in open(os.path.join(HERE, "results", "attack.jsonl"))]

def med(v):
    v = sorted(v); n = len(v)
    return v[n // 2] if n % 2 else 0.5 * (v[n // 2 - 1] + v[n // 2])

def rho_exp(d):  # derived: sqrt(pi r / 2) x measured seconds per rho step (same run)
    return d["rho_expected_steps"] * d["rho_seconds"] / d["rho_steps"]

by = defaultdict(list)
for d in rows:
    by[(d["p"], d["arm"])].append(d)
ps = sorted({d["p"] for d in rows})
print("runs: %d, solved: %d, rho solved: %d" % (len(rows), sum(d["solved"] for d in rows), sum(d["rho_solved"] for d in rows)))
print("all checks (pushforward 8P, T(P) != 0, iota pairs constant, X_oo = 4c): %d/%d" % (
    sum(d["pushforward_8P"] and d["T_P_nonzero"] and d["iota_pairs_constant"] and d["x_inf_eq_4c"] for d in rows), len(rows)))
print()
print("| q | log2 r | arm | runs | unknowns | relations (quadric rows) | rel. candidates | t_rel s | t_LA s | t_descent s | IC total s | rho measured s | rho expected-steps s | IC / rho measured | IC / rho expected |")
print("|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
import math
for p in ps:
    for arm in ["full", "iota"]:
        v = by[(p, arm)]
        if not v:
            continue
        print("| %d | %.1f | %s | %d | %d | %d (%d) | %d | %.3f | %.3f | %.3f | %.2f | %.3f | %.3f | %.1f [%.1f, %.1f] | %.2f [%.2f, %.2f] |" % (
            p, med([math.log2(d["r"]) for d in v]), arm, len(v), med([d["unknowns"] for d in v]),
            med([d["relations"] for d in v]), med([d["quad_rows"] for d in v]),
            med([d["rel_candidates"] + d["quad_candidates"] for d in v]),
            med([d["t_relations"] for d in v]), med([d["t_linear_algebra"] for d in v]), med([d["t_descent"] for d in v]),
            med([d["t_ic_total"] for d in v]), med([d["rho_seconds"] for d in v]), med([rho_exp(d) for d in v]),
            med([d["ic_over_rho_time"] for d in v]), min(d["ic_over_rho_time"] for d in v), max(d["ic_over_rho_time"] for d in v),
            med([d["t_ic_total"] / rho_exp(d) for d in v]), min(d["t_ic_total"] / rho_exp(d) for d in v), max(d["t_ic_total"] / rho_exp(d) for d in v)))
print()
# iota vs full, paired by (p, seed): same instance, same seed
print("| q | unknowns full/iota | relation candidates full/iota | LA time full/iota | IC total full/iota |")
print("|--:|--:|--:|--:|--:|")
for p in ps:
    pr = []
    for d in by[(p, "full")]:
        e = [x for x in by[(p, "iota")] if x["seed"] == d["seed"]]
        if e:
            pr.append((d, e[0]))
    if not pr:
        continue
    f = lambda key: med([a[key] / b[key] for a, b in pr if b[key] > 0])
    print("| %d | %.2f | %.2f | %.2f | %.2f |" % (p, f("unknowns"), med([(a["rel_candidates"] + a["quad_candidates"]) / (b["rel_candidates"] + b["quad_candidates"]) for a, b in pr]),
          f("t_linear_algebra") if all(b["t_linear_algebra"] > 0 for _, b in pr) else float("nan"), f("t_ic_total")))

# CSV of the same medians for the Typst report (layout only)
with open(os.path.join(HERE, "results", "summary.csv"), "w") as f:
    f.write("q,log2r,arm,runs,unknowns,relations,t_rel,t_la,t_desc,t_ic,rho_meas,rho_exp,ratio_meas,ratio_exp,ratio_exp_lo,ratio_exp_hi\n")
    for p in ps:
        for arm in ["full", "iota"]:
            v = by[(p, arm)]
            if not v:
                continue
            ex = [d["t_ic_total"] / rho_exp(d) for d in v]
            f.write("%d,%.1f,%s,%d,%d,%d,%.3f,%.3f,%.3f,%.2f,%.3f,%.3f,%.1f,%.2f,%.2f,%.2f\n" % (
                p, med([math.log2(d["r"]) for d in v]), arm, len(v), med([d["unknowns"] for d in v]), med([d["relations"] for d in v]),
                med([d["t_relations"] for d in v]), med([d["t_linear_algebra"] for d in v]), med([d["t_descent"] for d in v]),
                med([d["t_ic_total"] for d in v]), med([d["rho_seconds"] for d in v]), med([rho_exp(d) for d in v]),
                med([d["ic_over_rho_time"] for d in v]), med(ex), min(ex), max(ex)))
