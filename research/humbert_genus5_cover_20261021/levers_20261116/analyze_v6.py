#!/usr/bin/env python3
"""Frozen analysis of levers_20261116/PROTOCOL.md: reads campaign/q*/results/v6.jsonl and the
la_solver v5 rows (C0 and rho reference), writes results/summary_v6.md, results/*.csv and figures/."""
import csv, glob, json, math, os, statistics as st, sys

HERE = os.path.dirname(os.path.abspath(__file__))
V5 = os.path.join(HERE, "..", "la_solver_20261116", "campaign")
OUT = os.path.join(HERE, "results"); FIG = os.path.join(HERE, "figures")
os.makedirs(OUT, exist_ok=True); os.makedirs(FIG, exist_ok=True)
COUNTED = ["rel_candidates", "relations", "la_cols", "la_rows", "la_nnz", "la_mulmods", "x_recovered"]
TAGS = ["base", "rel", "la4", "comb", "comb_aa", "lp_comb", "dlpxs10", "dlpxs05", "dlpxs02"]

rows = [json.loads(l) for f in sorted(glob.glob(os.path.join(HERE, "campaign/q*/results/v6.jsonl"))) for l in open(f)]
v5 = [json.loads(l) for f in sorted(glob.glob(os.path.join(V5, "q*/results/v5.jsonl"))) for l in open(f)]
V = {(r["p"], r["seed"], r["tag"]): r for r in v5}
R = {(r["p"], r["seed"], r["tag"]): r for r in rows}
qs = sorted({r["p"] for r in rows})
inst = lambda q: sorted({s for (p, s, t) in R if p == q})
md = ["# Summary v6 (levers_20261116)", "", f"{len(rows)} runs; q = {qs}", ""]

def med(x): return st.median(x) if x else float("nan")
def ratio(q, a, b, f):
    out = []
    for s in inst(q):
        if (q, s, a) in R and (q, s, b) in R and R[(q, s, b)][f] > 0:
            out.append(R[(q, s, a)][f] / R[(q, s, b)][f])
    return out

# correctness
bad = [(r["p"], r["seed"], r["tag"]) for r in rows if not (r["solved"] and r["e2e_ok"])]
md += [f"**Correctness:** {len(rows) - len(bad)}/{len(rows)} solved with e2e_ok; failures: {bad}", ""]

# C0 and R0
def cmp(a, b): return [c for c in COUNTED if a.get(c) != b.get(c)]
c0 = [(q, s, cmp(R[(q, s, "base")], V[(q, s, "dlpf50_l1")])) for q in qs for s in inst(q) if (q, s, "base") in R and (q, s, "dlpf50_l1") in V]
md += [f"**C0:** base == v5 dlpf50_l1 on {sum(1 for x in c0 if not x[2])}/{len(c0)} instances; diffs: {[x for x in c0 if x[2]]}"]
r0 = []
for q in qs:
    for s in inst(q):
        for t in ["rel", "la4", "comb", "comb_aa"]:
            if (q, s, t) in R and (q, s, "base") in R:
                r0.append((q, s, t, cmp(R[(q, s, t)], R[(q, s, "base")])))
        if (q, s, "lp_comb") in R and (q, s, "lp50_l1") in V:
            r0.append((q, s, "lp_comb", cmp(R[(q, s, "lp_comb")], V[(q, s, "lp50_l1")])))
md += [f"**R0:** {sum(1 for x in r0 if not x[3])}/{len(r0)} flagged runs counted-identical; diffs: {[x for x in r0 if x[3]]}", ""]

# ratios table
md += ["| q | R1 rel/base t_rel | L1 la4/base t_LA | K1 comb/base t_ic | A0 comb_aa/comb t_ic | G1' lp_comb/comb | X4 dlpxs10/comb | dlpxs05/comb | dlpxs02/comb |", "|" + ":--|" * 9]
rat = []
for q in qs:
    cells = {
        "R1": ratio(q, "rel", "base", "t_relations"), "L1": ratio(q, "la4", "base", "t_linear_algebra"),
        "K1": ratio(q, "comb", "base", "t_ic_total"), "A0": ratio(q, "comb_aa", "comb", "t_ic_total"),
        "G1": ratio(q, "lp_comb", "comb", "t_ic_total"), "X10": ratio(q, "dlpxs10", "comb", "t_ic_total"),
        "X05": ratio(q, "dlpxs05", "comb", "t_ic_total"), "X02": ratio(q, "dlpxs02", "comb", "t_ic_total"),
    }
    f = lambda x: f"{med(x):.3f} [{min(x):.3f}, {max(x):.3f}] n={len(x)}" if x else "–"
    md.append(f"| {q} | " + " | ".join(f(v) for v in cells.values()) + " |")
    for k, v in cells.items():
        rat.append({"q": q, "metric": k, "median": med(v), "min": min(v) if v else "", "max": max(v) if v else "", "n": len(v)})
md.append("")

# X3 safety
x3 = [(r["p"], r["seed"], r["tag"], r["precheck_extensions"], r["t_filter"]) for r in rows if r["tag"].startswith("dlpxs")]
x3bad = [x for x in x3 if x[3] > 5 or x[4] >= 1.0]
md += [f"**X3:** {len(x3) - len(x3bad)}/{len(x3)} dlpxs runs with precheck_extensions <= 5 and t_filter < 1 s; violations: {x3bad}", ""]

# X1-style counted: dlpxs mulmods / comb mulmods, candidates
md += ["| q | dlpxs10 mulmods/comb | dlpxs05 | dlpxs02 | dlpxs10 cand/comb | dlpxs05 | dlpxs02 |", "|" + ":--|" * 7]
for q in qs:
    a = [med(ratio(q, t, "comb", "la_mulmods")) for t in ["dlpxs10", "dlpxs05", "dlpxs02"]]
    b = [med(ratio(q, t, "comb", "rel_candidates")) for t in ["dlpxs10", "dlpxs05", "dlpxs02"]]
    md.append(f"| {q} | " + " | ".join(f"{x:.3f}" for x in a + b) + " |")
md.append("")

# phases and gap to rho
ph = []
md += ["| q | tag | t_setup | t_rel | t_filter | t_LA | t_desc | t_ic | ic/rho | ic/rho_neg |", "|" + ":--|" * 10]
for q in qs:
    for t in TAGS:
        rs = [R[(q, s, t)] for s in inst(q) if (q, s, t) in R]
        if not rs: continue
        g, gn = [], []
        for r in rs:
            pre = V.get((q, r["seed"], "pre_l1"))
            if pre and pre["rho_steps"] > 0:
                sps = pre["rho_seconds"] / pre["rho_steps"]
                g.append(r["t_ic_total"] / (pre["rho_expected_steps"] * sps))
                gn.append(r["t_ic_total"] / (pre["rho_expected_steps_negation"] * sps))
        d = {"q": q, "tag": t, "n": len(rs)}
        for k in ["t_setup", "t_relations", "t_filter", "t_linear_algebra", "t_descent", "t_ic_total"]:
            d[k] = med([r[k] for r in rs])
        d["ic_over_rho"] = med(g); d["ic_over_rho_neg"] = med(gn)
        d["rel_candidates"] = med([r["rel_candidates"] for r in rs]); d["la_mulmods"] = med([r["la_mulmods"] for r in rs])
        ph.append(d)
        md.append(f"| {q} | {t} | {d['t_setup']:.2f} | {d['t_relations']:.2f} | {d['t_filter']:.3f} | {d['t_linear_algebra']:.2f} | {d['t_descent']:.2f} | {d['t_ic_total']:.2f} | {d['ic_over_rho']:.3f} | {d['ic_over_rho_neg']:.3f} |")
md.append("")
if len(qs) >= 2:
    for t in ["base", "comb", "lp_comb", "dlpxs05"]:
        y = {d["q"]: d["t_ic_total"] for d in ph if d["tag"] == t}
        if qs[0] in y and qs[-1] in y:
            md.append(f"- F6 slope {t}: {math.log(y[qs[-1]] / y[qs[0]]) / math.log(qs[-1] / qs[0]):.2f}")
md.append(f"- load1 median {med([r['load1'] for r in rows]):.2f}, max {max(r['load1'] for r in rows):.2f}")

open(os.path.join(OUT, "summary_v6.md"), "w").write("\n".join(md) + "\n")
for name, data in [("phases_v6.csv", ph), ("ratios_v6.csv", rat)]:
    with open(os.path.join(OUT, name), "w", newline="") as fh:
        w = csv.DictWriter(fh, fieldnames=list(data[0])); w.writeheader(); w.writerows(data)
with open(os.path.join(OUT, "instances_v6.csv"), "w", newline="") as fh:
    keys = ["p", "seed", "tag", "order", "load1", "rel_candidates", "relations", "la_mulmods", "la_cols", "la_nnz", "precheck_extensions",
            "filter_cliques", "t_setup", "t_relations", "t_filter", "t_linear_algebra", "t_descent", "t_ic_total", "solved", "e2e_ok"]
    w = csv.writer(fh); w.writerow(keys)
    for r in sorted(rows, key=lambda r: (r["p"], r["seed"], r["order"])): w.writerow([r.get(k) for k in keys])
print("\n".join(md))

# figures
try:
    import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
except ImportError:
    sys.exit(0)
plt.rcParams.update({"font.size": 9, "svg.hashsalt": "levers"})
COL = {"base": "#7f7f7f", "rel": "#1f77b4", "la4": "#ff7f0e", "comb": "#2ca02c", "lp_comb": "#9467bd", "dlpxs05": "#d62728"}
fig, ax = plt.subplots(figsize=(6.4, 3.6))
for t, c in COL.items():
    pts = [(d["q"], d["t_ic_total"]) for d in ph if d["tag"] == t]
    if pts: ax.plot(*zip(*pts), "o-", color=c, label=t)
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("median t_ic (s, L0)"); ax.legend(fontsize=7); ax.grid(alpha=.3)
ax.set_title("IC time per arm (same instances)")
fig.tight_layout(); [fig.savefig(os.path.join(FIG, f"levers_tic.{e}"), dpi=200) for e in ("svg", "png")]; plt.close(fig)
fig, ax = plt.subplots(figsize=(7.2, 3.6))
keys = ["t_setup", "t_relations", "t_filter", "t_linear_algebra", "t_descent"]
labs = [f"{d['q']}\n{d['tag']}" for d in ph if d["tag"] in ("base", "comb", "lp_comb", "dlpxs05")]
sel = [d for d in ph if d["tag"] in ("base", "comb", "lp_comb", "dlpxs05")]
bottom = [0.0] * len(sel)
for k, c in zip(keys, ["#bbbbbb", "#1f77b4", "#8c564b", "#ff7f0e", "#2ca02c"]):
    v = [d[k] for d in sel]; ax.bar(range(len(sel)), v, bottom=bottom, color=c, label=k); bottom = [a + b for a, b in zip(bottom, v)]
ax.set_xticks(range(len(sel))); ax.set_xticklabels(labs, fontsize=6); ax.set_ylabel("median s (L0)"); ax.legend(fontsize=7)
ax.set_title("Phase medians"); fig.tight_layout(); [fig.savefig(os.path.join(FIG, f"levers_phases.{e}"), dpi=200) for e in ("svg", "png")]; plt.close(fig)
fig, ax = plt.subplots(figsize=(6.4, 3.4))
for t, c in [("base", "#7f7f7f"), ("comb", "#2ca02c"), ("lp_comb", "#9467bd")]:
    pts = [(d["q"], d["ic_over_rho"]) for d in ph if d["tag"] == t]
    if pts: ax.plot(*zip(*pts), "o-", color=c, label=t)
ax.axhline(1, color="k", lw=.8); ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("median t_ic / expected rho time")
ax.legend(fontsize=7); ax.grid(alpha=.3); ax.set_title("Distance to this crate's rho (no negation map)")
fig.tight_layout(); [fig.savefig(os.path.join(FIG, f"levers_gap_to_rho.{e}"), dpi=200) for e in ("svg", "png")]; plt.close(fig)
