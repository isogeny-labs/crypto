#!/usr/bin/env python3
"""Frozen analysis of next_leads_20261120/PROTOCOL.md: campaign/q*/results/v7.jsonl against the v6
rows (C0). Writes results/summary_v7.md, results/*.csv and figures/."""
import csv, glob, json, math, os, statistics as st, sys

HERE = os.path.dirname(os.path.abspath(__file__))
V6 = os.path.join(HERE, "..", "levers_20261116", "campaign")
OUT = os.path.join(HERE, "results"); FIG = os.path.join(HERE, "figures")
os.makedirs(OUT, exist_ok=True); os.makedirs(FIG, exist_ok=True)
COUNTED = ["rel_candidates", "relations", "la_cols", "la_rows", "la_nnz", "la_mulmods", "x_true", "x_recovered", "E_order", "r", "D_order"]
IC = ["comb6", "comb7", "comb7_aa", "d25", "d30", "d35", "lp40", "lp50"]

rows = [json.loads(l) for f in sorted(glob.glob(os.path.join(HERE, "campaign/q*/results/v7.jsonl"))) for l in open(f)]
v6 = [json.loads(l) for f in sorted(glob.glob(os.path.join(V6, "q*/results/v6.jsonl"))) for l in open(f)]
R = {(r["p"], r["seed"], r["tag"]): r for r in rows}
V = {(r["p"], r["seed"], r["tag"]): r for r in v6}
qs = sorted({r["p"] for r in rows})
inst = lambda q: sorted({s for (p, s, t) in R if p == q})
med = lambda x: st.median(x) if x else float("nan")
fmt = lambda x: f"{med(x):.3f} [{min(x):.3f}, {max(x):.3f}] n={len(x)}" if x else "–"

def rho_expected_time(r):
    """expected solve time from the run's own throughput (PROTOCOL: primary rho reference)"""
    sps = r["rho_steps"] / r["rho_seconds"]
    if "rho_par" in r:
        return (r["rho_expected_steps_negation"] + r["rho_par"]["extra_group_ops"]) / sps
    return r["rho_expected_steps"] / sps

def ratio(q, a, b, fa, fb=None):
    fb = fb or fa
    out = []
    for s in inst(q):
        if (q, s, a) in R and (q, s, b) in R:
            x, y = fa(R[(q, s, a)]) if callable(fa) else R[(q, s, a)][fa], fb(R[(q, s, b)]) if callable(fb) else R[(q, s, b)][fb]
            if y > 0: out.append(x / y)
    return out

md = ["# Summary v7 (next_leads_20261120)", "", f"{len(rows)} runs; q = {qs}", ""]
bad = [(r["p"], r["seed"], r["tag"]) for r in rows if not (r.get("e2e_ok", r.get("rho_solved")) and r.get("solved", r.get("rho_solved")))]
md += [f"**Correctness:** {len(rows) - len(bad)}/{len(rows)} solved (IC: solved and e2e_ok; rho: rho_solved); failures: {bad}", ""]
cmp = lambda a, b: [c for c in COUNTED if a.get(c) != b.get(c)]
c0 = [(q, s, cmp(R[(q, s, "comb6")], V[(q, s, "comb")])) for q in qs for s in inst(q) if (q, s, "comb6") in R and (q, s, "comb") in V]
c1 = [(q, s, t, cmp(R[(q, s, t)], R[(q, s, "comb6")])) for q in qs for s in inst(q) for t in ("comb7", "comb7_aa") if (q, s, t) in R and (q, s, "comb6") in R]
md += [f"**C0:** comb6 == v6 comb on {sum(not x[2] for x in c0)}/{len(c0)}; diffs {[x for x in c0 if x[2]]}",
       f"**C0':** comb7/comb7_aa == comb6 on {sum(not x[3] for x in c1)}/{len(c1)}; diffs {[x for x in c1 if x[3]]}", ""]

rat = []
md += ["| q | S1 setup comb7/comb6 | S2 t_ic comb7/comb6 | A/A comb7_aa/comb7 | d25/comb7 | d30/comb7 | d35/comb7 | lp40/lp50 | lp50/comb7 | M1 rho_m/rho_s steps/s |", "|" + ":--|" * 10]
for q in qs:
    cells = {
        "S1": ratio(q, "comb7", "comb6", "t_setup"), "S2": ratio(q, "comb7", "comb6", "t_ic_total"),
        "AA": ratio(q, "comb7_aa", "comb7", "t_ic_total"), "d25": ratio(q, "d25", "comb7", "t_ic_total"),
        "d30": ratio(q, "d30", "comb7", "t_ic_total"), "d35": ratio(q, "d35", "comb7", "t_ic_total"),
        "lp40_lp50": ratio(q, "lp40", "lp50", "t_ic_total"), "lp50_comb7": ratio(q, "lp50", "comb7", "t_ic_total"),
        "M1": ratio(q, "rho_m", "rho_s", lambda r: r["rho_steps"] / r["rho_seconds"]),
    }
    md.append(f"| {q} | " + " | ".join(fmt(v) for v in cells.values()) + " |")
    for k, v in cells.items():
        rat.append({"q": q, "metric": k, "median": med(v), "min": min(v) if v else "", "max": max(v) if v else "", "n": len(v)})
md.append("")

# M2
rm = [r for r in rows if r["tag"] == "rho_m"]
md += [f"**M2:** rho_m solved {sum(r['rho_solved'] for r in rm)}/{len(rm)}, abandoned_walks max {max((r['rho_par']['abandoned_walks'] for r in rm), default=0)}; "
       f"steps_over_expected median {med([r['rho_par']['steps_over_expected'] for r in rm]):.3f} [{min((r['rho_par']['steps_over_expected'] for r in rm), default=0):.3f}, {max((r['rho_par']['steps_over_expected'] for r in rm), default=0):.3f}]", ""]
rs = [r for r in rows if r["tag"] == "rho_s"]
if rs:
    md.append(f"- rho_s steps/expected median {med([r['rho_steps'] / r['rho_expected_steps'] for r in rs]):.3f}")

# phases, gap to rho
ph = []
md += ["", "| q | tag | t_setup | t_rel | t_filter | t_LA | t_desc | t_ic | / rho_m exp | / rho_m actual | / rho_s exp |", "|" + ":--|" * 11]
for q in qs:
    for t in IC:
        rr = [R[(q, s, t)] for s in inst(q) if (q, s, t) in R]
        if not rr: continue
        g = {"m": [], "ma": [], "s": []}
        for r in rr:
            m, sc = R.get((q, r["seed"], "rho_m")), R.get((q, r["seed"], "rho_s"))
            if m: g["m"].append(r["t_ic_total"] / rho_expected_time(m)); g["ma"].append(r["t_ic_total"] / m["rho_seconds"])
            if sc: g["s"].append(r["t_ic_total"] / rho_expected_time(sc))
        d = {"q": q, "tag": t, "n": len(rr)}
        for k in ["t_setup", "t_relations", "t_filter", "t_linear_algebra", "t_descent", "t_ic_total"]:
            d[k] = med([r[k] for r in rr])
        d.update(ic_over_rho_m=med(g["m"]), ic_over_rho_m_actual=med(g["ma"]), ic_over_rho_s=med(g["s"]),
                 native_ops=med([r.get("rel_fast_fp_mulmods", 0) + r["la_mulmods"] for r in rr]))
        ph.append(d)
        md.append(f"| {q} | {t} | {d['t_setup']:.2f} | {d['t_relations']:.2f} | {d['t_filter']:.3f} | {d['t_linear_algebra']:.2f} | {d['t_descent']:.2f} | {d['t_ic_total']:.2f} | {d['ic_over_rho_m']:.3f} | {d['ic_over_rho_m_actual']:.3f} | {d['ic_over_rho_s']:.3f} |")
md.append("")
for t in ["rho_m", "rho_s"]:
    for q in qs:
        rr = [R[(q, s, t)] for s in inst(q) if (q, s, t) in R]
        if rr:
            md.append(f"- {t} q={q}: steps/s median {med([r['rho_steps'] / r['rho_seconds'] for r in rr]):.3e}, expected time median {med([rho_expected_time(r) for r in rr]):.2f} s, actual median {med([r['rho_seconds'] for r in rr]):.2f} s")

# §5a band: IC native mulmods priced at w GAE each vs rho_m group operations
md += ["", "§5a band: median (w * IC native mulmods) / (rho_m expected group operations), best IC arm per q:", ""]
band = []
for q in qs:
    best = min((d for d in ph if d["q"] == q and d["tag"] != "comb7_aa"), key=lambda d: d["t_ic_total"])
    ms = [R[(q, s, "rho_m")] for s in inst(q) if (q, s, "rho_m") in R]
    gops = med([m["rho_expected_steps_negation"] + m["rho_par"]["extra_group_ops"] for m in ms]) if ms else float("nan")
    cells = {w: best["native_ops"] * w / gops for w in (0.0, 0.1, 1.0)}
    md.append(f"- q={q} best={best['tag']}: w=0 {cells[0.0]:.3f}, w=0.1 {cells[0.1]:.3f}, w=1 {cells[1.0]:.3f} (native {best['native_ops']:.3e}, rho group ops {gops:.3e})")
    band.append({"q": q, "best": best["tag"], **{f"w{w}": v for w, v in cells.items()}})
if len(qs) >= 2:
    md.append("")
    for t in ["comb7", "d30", "lp50"]:
        y = {d["q"]: d["t_ic_total"] for d in ph if d["tag"] == t}
        if qs[0] in y and qs[-1] in y:
            md.append(f"- slope t_ic {t}: {math.log(y[qs[-1]] / y[qs[0]]) / math.log(qs[-1] / qs[0]):.2f}")
    for t in ["rho_m"]:
        y = {q: med([rho_expected_time(R[(q, s, t)]) for s in inst(q) if (q, s, t) in R]) for q in qs}
        md.append(f"- slope expected time {t}: {math.log(y[qs[-1]] / y[qs[0]]) / math.log(qs[-1] / qs[0]):.2f}")
md.append(f"- load1 median {med([r['load1'] for r in rows]):.2f}, max {max(r['load1'] for r in rows):.2f}")

open(os.path.join(OUT, "summary_v7.md"), "w").write("\n".join(md) + "\n")
for name, data in [("phases_v7.csv", ph), ("ratios_v7.csv", rat), ("band_v7.csv", band)]:
    if data:
        with open(os.path.join(OUT, name), "w", newline="") as fh:
            w = csv.DictWriter(fh, fieldnames=list(data[0])); w.writeheader(); w.writerows(data)
print("\n".join(md))

try:
    import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
except ImportError:
    sys.exit(0)
plt.rcParams.update({"font.size": 9, "svg.hashsalt": "nextleads"})
fig, ax = plt.subplots(figsize=(6.4, 3.8))
for t, c in [("comb6", "#7f7f7f"), ("comb7", "#2ca02c"), ("d30", "#1f77b4"), ("lp50", "#9467bd")]:
    pts = [(d["q"], d["t_ic_total"]) for d in ph if d["tag"] == t]
    if pts: ax.plot(*zip(*pts), "o-", color=c, label=f"IC {t}")
for t, c, ls in [("rho_m", "#d62728", "--"), ("rho_s", "#ff7f0e", ":")]:
    pts = [(q, med([rho_expected_time(R[(q, s, t)]) for s in inst(q) if (q, s, t) in R])) for q in qs]
    pts = [p for p in pts if not math.isnan(p[1])]
    if pts: ax.plot(*zip(*pts), "s" + ls, color=c, label=f"{t} expected")
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("median seconds (L0)"); ax.grid(alpha=.3); ax.legend(fontsize=7)
ax.set_title("IC arms against the matched and the scalar rho")
fig.tight_layout(); [fig.savefig(os.path.join(FIG, f"v7_ic_vs_rho.{e}"), dpi=200) for e in ("svg", "png")]; plt.close(fig)
fig, ax = plt.subplots(figsize=(7.2, 3.6))
sel = [d for d in ph if d["tag"] in ("comb6", "comb7", "d30", "lp50")]
keys = ["t_setup", "t_relations", "t_filter", "t_linear_algebra", "t_descent"]
bottom = [0.0] * len(sel)
for k, c in zip(keys, ["#bbbbbb", "#1f77b4", "#8c564b", "#ff7f0e", "#2ca02c"]):
    v = [d[k] for d in sel]; ax.bar(range(len(sel)), v, bottom=bottom, color=c, label=k); bottom = [a + b for a, b in zip(bottom, v)]
ax.set_xticks(range(len(sel))); ax.set_xticklabels([f"{d['q']}\n{d['tag']}" for d in sel], fontsize=6); ax.set_ylabel("median s (L0)"); ax.legend(fontsize=7)
ax.set_title("Phase medians"); fig.tight_layout(); [fig.savefig(os.path.join(FIG, f"v7_phases.{e}"), dpi=200) for e in ("svg", "png")]; plt.close(fig)
fig, ax = plt.subplots(figsize=(6.4, 3.4))
for t, c in [("comb7", "#2ca02c"), ("d30", "#1f77b4"), ("lp50", "#9467bd")]:
    pts = [(d["q"], d["ic_over_rho_m"]) for d in ph if d["tag"] == t]
    if pts: ax.plot(*zip(*pts), "o-", color=c, label=f"{t} / matched rho")
pts = [(d["q"], d["ic_over_rho_s"]) for d in ph if d["tag"] == "comb7"]
if pts: ax.plot(*zip(*pts), "o:", color="#7f7f7f", label="comb7 / scalar rho")
ax.axhline(1, color="k", lw=.8); ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("median t_ic / expected rho time")
ax.legend(fontsize=7); ax.grid(alpha=.3); ax.set_title("Distance remaining to rho")
fig.tight_layout(); [fig.savefig(os.path.join(FIG, f"v7_gap_to_rho.{e}"), dpi=200) for e in ("svg", "png")]; plt.close(fig)
