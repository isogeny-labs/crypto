#!/usr/bin/env python3
"""Render the tables of RESULTS.md from control.json (read-only; prints markdown)."""
import json
import sys

path = sys.argv[1] if len(sys.argv) > 1 else "control.json"
d = json.load(open(path))
S = d["summary"]
ARMS = ["A_full", "B_single_lp", "C_double_lp", "R_rho_negation"]
SHORT = {"A_full": "A", "B_single_lp": "B", "C_double_lp": "C", "R_rho_negation": "R"}


def f(x):
    if x is None:
        return "-"
    if abs(x) >= 1e5:
        return f"{x:.3e}"
    return f"{x:,.0f}"


print("### Table 1. Counts (mean over runs)\n")
print("| p | runs | A grp | A nat.mul | B grp | B nat.mul | C grp | C rel.mul | C LA.mul | C LA.add | R grp (walk + other) | R nat |")
print("|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|")
for s in S:
    a = s["arms"]
    g = lambda arm, k: a.get(arm, {}).get(k)
    rr = a["R_rho_negation"]
    print(
        f"| {s['p']} | {s['n_curves']}c | {f(g('A_full','group_ops'))} | {f(g('A_full','native_mul'))} | "
        f"{f(g('B_single_lp','group_ops'))} | {f(g('B_single_lp','native_mul'))} | "
        f"{f(g('C_double_lp','group_ops'))} | {f(g('C_double_lp','rel_mul'))} | {f(g('C_double_lp','la_mul'))} | {f(g('C_double_lp','la_add'))} | "
        f"{f(rr['group_ops'])} ({f(rr['walk_ops'])} + {f(rr['setup_and_other_ops'])}) | {f(rr['native_mul'])} |"
    )

for key, label in [("total_w0", "w = 0"), ("total_w01", "w = 0.1"), ("total_w1", "w = 1"), ("total_cal", "calibrated (informational)")]:
    print(f"\n### Table: total cost in GAE at {label}, and ratio to R\n")
    print("| p | A | B | C | R | A/R | B/R | C/R |")
    print("|--:|--:|--:|--:|--:|--:|--:|--:|")
    for s in S:
        a = s["arms"]
        r = a["R_rho_negation"][key]
        row = [s["p"]]
        vals = [a.get(x, {}).get(key) for x in ARMS]
        row += [f(v) for v in vals]
        row += [("-" if v is None else f"{v / r:.2f}") for v in vals[:3]]
        print("| " + " | ".join(str(x) for x in row) + " |")

print("\n### Table: sensitivity, native additions included (mean)\n")
print("| p | C native mul | C native mul+add | C total w=1 (mul) | C total w=1 (mul+add) | R total w=1 |")
print("|--:|--:|--:|--:|--:|--:|")
for s in S:
    c = s["arms"]["C_double_lp"]
    r = s["arms"]["R_rho_negation"]
    print(
        f"| {s['p']} | {f(c['native_mul'])} | {f(c['native_all'])} | {f(c['total_w1'])} | {f(c['group_ops'] + c['native_all'])} | {f(r['total_w1'])} |"
    )

print("\n### Table: calibration (informational)\n")
print("| p | ns / Cantor add | ns / mulmod mod p | ns / mulmod mod r | w_rel | w_la |")
print("|--:|--:|--:|--:|--:|--:|")
for p, c in d["calibration_ns_informational"].items():
    print(f"| {p} | {c['ns_per_group_add']:.0f} | {c['ns_per_mulmod_mod_p']:.2f} | {c['ns_per_mulmod_mod_r']:.2f} | {c['w_rel']:.4f} | {c['w_la']:.4f} |")

print("\n### Table: fitted exponents (least squares of ln cost on ln p)\n")
print("| series | slope | coefficient | points p |")
print("|--|--:|--:|--|")
for k, v in d["fits"].items():
    print(f"| {k} | {v['slope']:.3f} | {v['coef']:.3g} | {', '.join(str(int(x)) for x in v['points_p'])} |")

print("\n### Crossover\n")
print("```json")
print(json.dumps(d["crossover"], indent=1))
print("```")
print("strict_all_prices_first_p:", d["strict_all_prices_first_p"])
print("all_certified:", d["all_certified"])
rows = d["rows"]
print("runs:", len(rows), "ic:", sum(1 for r in rows if r["kind"] == "ic"), "rho:", sum(1 for r in rows if r["kind"] == "rho"))
print("splits/trials per arm and p (smoothness sanity):")
seen = {}
for r in rows:
    if r["kind"] == "ic":
        seen.setdefault((r["p"], r["arm"]), []).append(r["splits"] / r["trials"])
for (p, arm), v in sorted(seen.items()):
    print(f"  p={p} {arm}: {sum(v)/len(v):.4f}")
