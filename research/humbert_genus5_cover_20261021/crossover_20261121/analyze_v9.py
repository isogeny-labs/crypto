#!/usr/bin/env python3
"""Frozen analysis of crossover_20261121/PROTOCOL.md: campaign/q*/results/v9.jsonl (C0 against v8 at q=65521).
Writes results/summary_v9.md, results/*.csv and figures/."""
import csv, glob, json, math, os, statistics as st, sys

HERE = os.path.dirname(os.path.abspath(__file__))
V8 = os.path.join(HERE, "..", "slope_20261120", "campaign")
OUT = os.path.join(HERE, "results"); FIG = os.path.join(HERE, "figures")
os.makedirs(OUT, exist_ok=True); os.makedirs(FIG, exist_ok=True)
COUNTED = ["rel_candidates", "relations", "la_cols", "la_rows", "la_nnz", "la_mulmods", "x_true", "x_recovered", "E_order", "r", "D_order"]
IC = ["d20", "d25", "d35", "d35_aa", "d50"]

rows = [json.loads(l) for f in sorted(glob.glob(os.path.join(HERE, "campaign/q*/results/v9.jsonl"))) for l in open(f)]
v8 = [json.loads(l) for f in sorted(glob.glob(os.path.join(V8, "q*/results/v8.jsonl"))) for l in open(f)]
R = {(r["p"], r["seed"], r["tag"]): r for r in rows}
V = {(r["p"], r["seed"], r["tag"]): r for r in v8}
qs = sorted({r["p"] for r in rows})
inst = lambda q: sorted({s for (p, s, t) in R if p == q})
med = lambda x: st.median(x) if x else float("nan")
fmt = lambda x: f"{med(x):.3f} [{min(x):.3f}, {max(x):.3f}] n={len(x)}" if x else "–"
sps = lambda r: r["rho_steps"] / r["rho_seconds"]
rho_exp = lambda r: (r["rho_expected_steps_negation"] + r["rho_par"]["extra_group_ops"]) / sps(r)
ok = lambda r: (r.get("solved") and r.get("e2e_ok")) if "solved" in r else (r.get("rho_solved") and r["rho_par"]["abandoned_walks"] == 0)

md = ["# Summary v9 (crossover_20261121)", "", f"{len(rows)} runs; q = {qs}", ""]
bad = [(r["p"], r["seed"], r["tag"]) for r in rows if not ok(r)]
md += [f"**Z:** {len(rows) - len(bad)}/{len(rows)} solved; failures: {bad}"]
capped = [(r["p"], r["seed"]) for r in rows if r["tag"] == "rho" and r["rho_steps"] >= 2e10]
md += [f"- rho runs at the cap: {capped}", ""]
cmp = lambda a, b: [c for c in COUNTED if a.get(c) != b.get(c)]
c0 = []
for s in inst(65521):
    for t, vt in (("d35", "best8"), ("d50", "d50_8")):
        if (65521, s, t) in R and (65521, s, vt) in V:
            c0.append((s, t, cmp(R[(65521, s, t)], V[(65521, s, vt)])))
aa = [(q, s, cmp(R[(q, s, "d35_aa")], R[(q, s, "d35")])) for q in qs for s in inst(q) if (q, s, "d35_aa") in R and (q, s, "d35") in R]
md += [f"**C0 (q=65521 vs v8):** identical {sum(not x[2] for x in c0)}/{len(c0)}; differing (instance changed or defect): {[x for x in c0 if x[2]]}",
       f"**C0' (d35_aa == d35):** {sum(not x[2] for x in aa)}/{len(aa)}; diffs {[x for x in aa if x[2]]}", ""]

# per-instance ratios to rho expected time
per = []
md += ["| q | arm | median t_ic | median IC/rho_exp | range | median IC/rho_actual | wins (IC < rho_exp) |", "|" + ":--|" * 7]
best_by_q = {}
for q in qs:
    rhos = {s: R[(q, s, "rho")] for s in inst(q) if (q, s, "rho") in R}
    for t in IC:
        xs = [(s, R[(q, s, t)]) for s in inst(q) if (q, s, t) in R and s in rhos]
        if not xs: continue
        g = [x["t_ic_total"] / rho_exp(rhos[s]) for s, x in xs]
        ga = [x["t_ic_total"] / rhos[s]["rho_seconds"] for s, x in xs]
        d = {"q": q, "tag": t, "n": len(xs), "t_ic": med([x["t_ic_total"] for _, x in xs]),
             "t_rel": med([x["t_relations"] for _, x in xs]), "t_la": med([x["t_linear_algebra"] for _, x in xs]),
             "t_setup": med([x["t_setup"] for _, x in xs]), "t_desc": med([x["t_descent"] for _, x in xs]),
             "ratio": med(g), "ratio_min": min(g), "ratio_max": max(g), "ratio_actual": med(ga), "wins": sum(v < 1 for v in g)}
        per.append(d)
        md.append(f"| {q} | {t} | {d['t_ic']:.2f} | {d['ratio']:.3f} | [{d['ratio_min']:.3f}, {d['ratio_max']:.3f}] | {d['ratio_actual']:.3f} | {d['wins']}/{d['n']} |")
    cand = [d for d in per if d["q"] == q and d["tag"] != "d35_aa"]
    if cand: best_by_q[q] = min(cand, key=lambda d: d["ratio"])
md.append("")
for q in qs:
    rr = [R[(q, s, "rho")] for s in inst(q) if (q, s, "rho") in R]
    if rr:
        md.append(f"- rho q={q}: steps/s median {med([sps(x) for x in rr]):.3e}, expected time median {med([rho_exp(x) for x in rr]):.2f} s, actual median {med([x['rho_seconds'] for x in rr]):.2f} s, "
                  f"steps/expected median {med([x['rho_par']['steps_over_expected'] for x in rr]):.2f}, r/q^4 median {med([x['r'] / q**4 for x in rr]):.4f}")
md.append("")
md.append("**X (q=131071, best arm median IC/rho_exp < 1?):** " + (f"{best_by_q[131071]['tag']} {best_by_q[131071]['ratio']:.3f} [{best_by_q[131071]['ratio_min']:.3f}, {best_by_q[131071]['ratio_max']:.3f}] -> {'holds' if best_by_q[131071]['ratio'] < 1 else 'falsified'}" if 131071 in best_by_q else "no data"))
md.append("- best arm per q: " + ", ".join(f"{q}: {d['tag']} {d['ratio']:.3f}" for q, d in sorted(best_by_q.items())))

def slope(pts):
    xs, ys = [math.log(a) for a, b in pts], [math.log(b) for a, b in pts]
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
if len(best_by_q) >= 2:
    rho_t = {q: med([rho_exp(R[(q, s, "rho")]) for s in inst(q) if (q, s, "rho") in R]) for q in best_by_q}
    md.append(f"- Y slopes over q: best-arm median t_ic {slope([(q, d['t_ic']) for q, d in best_by_q.items()]):.2f}; rho expected {slope(list(rho_t.items())):.2f}; "
              + "; ".join(f"{t} {slope([(d['q'], d['t_ic']) for d in per if d['tag'] == t]):.2f}" for t in ["d20", "d25", "d35", "d50"] if len([d for d in per if d['tag'] == t]) >= 2))
    md.append("- phase slopes (d35): rel " + f"{slope([(d['q'], d['t_rel']) for d in per if d['tag'] == 'd35']):.2f}, LA {slope([(d['q'], d['t_la']) for d in per if d['tag'] == 'd35']):.2f}")
md.append(f"- load1 median {med([r['load1'] for r in rows]):.2f}, max {max(r['load1'] for r in rows):.2f}")

open(os.path.join(OUT, "summary_v9.md"), "w").write("\n".join(md) + "\n")
if per:
    with open(os.path.join(OUT, "ratios_v9.csv"), "w", newline="") as fh:
        w = csv.DictWriter(fh, fieldnames=list(per[0])); w.writeheader(); w.writerows(per)
print("\n".join(md))

try:
    import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
except ImportError:
    sys.exit(0)
plt.rcParams.update({"font.size": 9, "svg.hashsalt": "crossover"})
cols = {"d20": "#8c564b", "d25": "#1f77b4", "d35": "#2ca02c", "d50": "#9467bd"}
fig, ax = plt.subplots(figsize=(6.4, 3.8))
for t, c in cols.items():
    pts = [(d["q"], d["t_ic"]) for d in per if d["tag"] == t]
    if pts: ax.plot(*zip(*pts), "o-", color=c, label=f"IC {t}")
pts = [(q, med([rho_exp(R[(q, s, "rho")]) for s in inst(q) if (q, s, "rho") in R])) for q in qs]
ax.plot(*zip(*pts), "s--", color="#d62728", label="SIMD rho expected")
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("median seconds (L0)"); ax.grid(alpha=.3); ax.legend(fontsize=7)
ax.set_title("IC arms and the SIMD rho, q = 65521..131071")
fig.tight_layout(); [fig.savefig(os.path.join(FIG, f"v9_ic_vs_rho.{e}"), dpi=200) for e in ("svg", "png")]; plt.close(fig)
fig, ax = plt.subplots(figsize=(6.4, 3.4))
for t, c in cols.items():
    pts = [(d["q"], d["ratio"]) for d in per if d["tag"] == t]
    if pts: ax.plot(*zip(*pts), "o-", color=c, label=t)
ax.axhline(1, color="k", lw=.8); ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("median per-instance t_ic / rho expected")
ax.legend(fontsize=7); ax.grid(alpha=.3); ax.set_title("Distance remaining to the SIMD rho")
fig.tight_layout(); [fig.savefig(os.path.join(FIG, f"v9_gap_to_rho.{e}"), dpi=200) for e in ("svg", "png")]; plt.close(fig)
fig, ax = plt.subplots(figsize=(7.4, 3.6))
sel = [d for d in per if d["tag"] in ("d25", "d35")]
bottom = [0.0] * len(sel)
for k, c in zip(["t_setup", "t_rel", "t_la", "t_desc"], ["#bbbbbb", "#1f77b4", "#ff7f0e", "#2ca02c"]):
    v = [d[k] for d in sel]; ax.bar(range(len(sel)), v, bottom=bottom, color=c, label=k); bottom = [a + b for a, b in zip(bottom, v)]
ax.set_xticks(range(len(sel))); ax.set_xticklabels([f"{d['q']}\n{d['tag']}" for d in sel], fontsize=6); ax.set_ylabel("median s (L0)"); ax.legend(fontsize=7)
ax.set_title("Phase medians"); fig.tight_layout(); [fig.savefig(os.path.join(FIG, f"v9_phases.{e}"), dpi=200) for e in ("svg", "png")]; plt.close(fig)
