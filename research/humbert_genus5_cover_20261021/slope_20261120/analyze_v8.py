#!/usr/bin/env python3
"""Frozen analysis of slope_20261120/PROTOCOL.md: campaign/q*/results/v8.jsonl (C0 against v7 d35).
Writes results/summary_v8.md, results/*.csv and figures/."""
import csv, glob, json, math, os, statistics as st, sys

HERE = os.path.dirname(os.path.abspath(__file__))
V7 = os.path.join(HERE, "..", "next_leads_20261120", "campaign")
OUT = os.path.join(HERE, "results"); FIG = os.path.join(HERE, "figures")
os.makedirs(OUT, exist_ok=True); os.makedirs(FIG, exist_ok=True)
COUNTED = ["rel_candidates", "relations", "la_cols", "la_rows", "la_nnz", "la_mulmods", "x_true", "x_recovered", "E_order", "r", "D_order"]
IC = ["best7", "best8", "best8_aa", "d25_8", "d50_8", "lp50_8"]

rows = [json.loads(l) for f in sorted(glob.glob(os.path.join(HERE, "campaign/q*/results/v8.jsonl"))) for l in open(f)]
v7 = [json.loads(l) for f in sorted(glob.glob(os.path.join(V7, "q*/results/v7.jsonl"))) for l in open(f)]
R = {(r["p"], r["seed"], r["tag"]): r for r in rows}
V = {(r["p"], r["seed"], r["tag"]): r for r in v7}
qs = sorted({r["p"] for r in rows})
inst = lambda q: sorted({s for (p, s, t) in R if p == q})
med = lambda x: st.median(x) if x else float("nan")
fmt = lambda x: f"{med(x):.3f} [{min(x):.3f}, {max(x):.3f}] n={len(x)}" if x else "–"
sps = lambda r: r["rho_steps"] / r["rho_seconds"]
rho_exp = lambda r: (r["rho_expected_steps_negation"] + r["rho_par"]["extra_group_ops"]) / sps(r)

def ratio(q, a, b, fa, fb=None):
    fb = fb or fa
    get = lambda r, f: f(r) if callable(f) else r[f]
    return [get(R[(q, s, a)], fa) / get(R[(q, s, b)], fb) for s in inst(q) if (q, s, a) in R and (q, s, b) in R and get(R[(q, s, b)], fb) > 0]

md = ["# Summary v8 (slope_20261120)", "", f"{len(rows)} runs; q = {qs}", ""]
ok = lambda r: (r.get("solved") and r.get("e2e_ok")) if "solved" in r else r.get("rho_solved")
bad = [(r["p"], r["seed"], r["tag"]) for r in rows if not ok(r)]
md += [f"**Correctness:** {len(rows) - len(bad)}/{len(rows)} solved; failures: {bad}", ""]
cmp = lambda a, b: [c for c in COUNTED if a.get(c) != b.get(c)]
c0 = [(q, s, cmp(R[(q, s, "best7")], V[(q, s, "d35")])) for q in qs for s in inst(q) if (q, s, "best7") in R and (q, s, "d35") in V]
c1 = [(q, s, t, cmp(R[(q, s, t)], R[(q, s, "best7")])) for q in qs for s in inst(q) for t in ("best8", "best8_aa") if (q, s, t) in R and (q, s, "best7") in R]
md += [f"**C0:** best7 == v7 d35 on {sum(not x[2] for x in c0)}/{len(c0)}; diffs {[x for x in c0 if x[2]]}",
       f"**C0':** best8/best8_aa == best7 on {sum(not x[3] for x in c1)}/{len(c1)}; diffs {[x for x in c1 if x[3]]}", ""]

rat = []
md += ["| q | L2 t_LA best8/best7 | t_ic best8/best7 | A/A best8_aa/best8 | d25_8/best8 | d50_8/best8 | lp50_8/best8 | M3 rho_m8/rho_m7 steps/s |", "|" + ":--|" * 8]
for q in qs:
    cells = {"L2": ratio(q, "best8", "best7", "t_linear_algebra"), "TIC": ratio(q, "best8", "best7", "t_ic_total"),
             "AA": ratio(q, "best8_aa", "best8", "t_ic_total"), "d25": ratio(q, "d25_8", "best8", "t_ic_total"),
             "d50": ratio(q, "d50_8", "best8", "t_ic_total"), "lp50": ratio(q, "lp50_8", "best8", "t_ic_total"),
             "M3": ratio(q, "rho_m8", "rho_m7", sps)}
    md.append(f"| {q} | " + " | ".join(fmt(v) for v in cells.values()) + " |")
    rat += [{"q": q, "metric": k, "median": med(v), "min": min(v) if v else "", "max": max(v) if v else "", "n": len(v)} for k, v in cells.items()]
md.append("")
rr = [r for r in rows if r["tag"].startswith("rho")]
md += [f"**M4:** rho solved {sum(r['rho_solved'] for r in rr)}/{len(rr)}, abandoned max {max((r['rho_par']['abandoned_walks'] for r in rr), default=0)}; "
       f"steps_over_expected median rho_m7 {med([r['rho_par']['steps_over_expected'] for r in rr if r['tag'] == 'rho_m7']):.3f}, "
       f"rho_m8 {med([r['rho_par']['steps_over_expected'] for r in rr if r['tag'] == 'rho_m8']):.3f}", ""]

ph = []
md += ["| q | tag | t_setup | t_rel | t_filter | t_LA | t_desc | t_ic | / rho_m8 exp | / rho_m8 actual | / rho_m7 exp |", "|" + ":--|" * 11]
for q in qs:
    for t in IC:
        xs = [R[(q, s, t)] for s in inst(q) if (q, s, t) in R]
        if not xs: continue
        g8 = [x["t_ic_total"] / rho_exp(R[(q, x["seed"], "rho_m8")]) for x in xs if (q, x["seed"], "rho_m8") in R]
        g8a = [x["t_ic_total"] / R[(q, x["seed"], "rho_m8")]["rho_seconds"] for x in xs if (q, x["seed"], "rho_m8") in R]
        g7 = [x["t_ic_total"] / rho_exp(R[(q, x["seed"], "rho_m7")]) for x in xs if (q, x["seed"], "rho_m7") in R]
        d = {"q": q, "tag": t, "n": len(xs)}
        for k in ["t_setup", "t_relations", "t_filter", "t_linear_algebra", "t_descent", "t_ic_total"]:
            d[k] = med([x[k] for x in xs])
        d.update(ic_over_rho_m8=med(g8), ic_over_rho_m8_actual=med(g8a), ic_over_rho_m7=med(g7),
                 native_ops=med([x.get("rel_fast_fp_mulmods", 0) + x["la_mulmods"] for x in xs]))
        ph.append(d)
        md.append(f"| {q} | {t} | {d['t_setup']:.2f} | {d['t_relations']:.2f} | {d['t_filter']:.3f} | {d['t_linear_algebra']:.2f} | {d['t_descent']:.2f} | {d['t_ic_total']:.2f} | {d['ic_over_rho_m8']:.3f} | {d['ic_over_rho_m8_actual']:.3f} | {d['ic_over_rho_m7']:.3f} |")
md.append("")
rho_med = {}
for t in ["rho_m7", "rho_m8"]:
    for q in qs:
        xs = [R[(q, s, t)] for s in inst(q) if (q, s, t) in R]
        if xs:
            rho_med[(t, q)] = med([rho_exp(x) for x in xs])
            md.append(f"- {t} q={q}: steps/s median {med([sps(x) for x in xs]):.3e}, expected time median {rho_med[(t, q)]:.2f} s, actual median {med([x['rho_seconds'] for x in xs]):.2f} s")

# slopes and crossover (G)
def fit(pts):
    xs, ys = [math.log(a) for a, b in pts], [math.log(b) for a, b in pts]
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    b = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
    return b, my - b * mx
md.append("")
best_by_q = {q: min((d for d in ph if d["q"] == q and d["tag"] != "best8_aa"), key=lambda d: d["t_ic_total"]) for q in qs}
if len(qs) >= 2 and all(("rho_m8", q) in rho_med for q in qs):
    bi, ai = fit([(q, best_by_q[q]["t_ic_total"]) for q in qs])
    br, ar = fit([(q, rho_med[("rho_m8", q)]) for q in qs])
    b8, a8 = fit([(q, next(d for d in ph if d["q"] == q and d["tag"] == "best8")["t_ic_total"]) for q in qs])
    md.append(f"- G slope (log-log LS over q): best IC arm per q {bi:.2f}; best8 {b8:.2f}; rho_m8 expected {br:.2f}")
    if bi < br:
        qx = math.exp((ai - ar) / (br - bi))
        md.append(f"- G crossover q* (exploratory extrapolation of the two fits): {qx:.0f}")
    md.append("- best arm per q: " + ", ".join(f"{q}: {best_by_q[q]['tag']} {best_by_q[q]['ic_over_rho_m8']:.3f}" for q in qs))

md += ["", "§5a band: (w * IC native mulmods) / (rho_m8 expected group operations), best IC arm per q:"]
band = []
for q in qs:
    b = best_by_q[q]
    ms = [R[(q, s, "rho_m8")] for s in inst(q) if (q, s, "rho_m8") in R]
    gops = med([m["rho_expected_steps_negation"] + m["rho_par"]["extra_group_ops"] for m in ms]) if ms else float("nan")
    cells = {w: b["native_ops"] * w / gops for w in (0.0, 0.1, 1.0)}
    md.append(f"- q={q} {b['tag']}: w=0 {cells[0.0]:.3f}, w=0.1 {cells[0.1]:.3f}, w=1 {cells[1.0]:.3f}")
    band.append({"q": q, "best": b["tag"], **{f"w{w}": v for w, v in cells.items()}})
md.append(f"- load1 median {med([r['load1'] for r in rows]):.2f}, max {max(r['load1'] for r in rows):.2f}")

open(os.path.join(OUT, "summary_v8.md"), "w").write("\n".join(md) + "\n")
for name, data in [("phases_v8.csv", ph), ("ratios_v8.csv", rat), ("band_v8.csv", band)]:
    if data:
        with open(os.path.join(OUT, name), "w", newline="") as fh:
            w = csv.DictWriter(fh, fieldnames=list(data[0])); w.writeheader(); w.writerows(data)
print("\n".join(md))

try:
    import matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
except ImportError:
    sys.exit(0)
plt.rcParams.update({"font.size": 9, "svg.hashsalt": "slope"})
fig, ax = plt.subplots(figsize=(6.4, 3.8))
for t, c in [("best7", "#7f7f7f"), ("best8", "#2ca02c"), ("d50_8", "#1f77b4"), ("lp50_8", "#9467bd")]:
    pts = [(d["q"], d["t_ic_total"]) for d in ph if d["tag"] == t]
    if pts: ax.plot(*zip(*pts), "o-", color=c, label=f"IC {t}")
for t, c, ls in [("rho_m7", "#ff7f0e", ":"), ("rho_m8", "#d62728", "--")]:
    pts = [(q, rho_med[(t, q)]) for q in qs if (t, q) in rho_med]
    if pts: ax.plot(*zip(*pts), "s" + ls, color=c, label=f"{t} expected")
ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("median seconds (L0)"); ax.grid(alpha=.3); ax.legend(fontsize=7)
ax.set_title("IC against the matched rho, q = 16001..65521")
fig.tight_layout(); [fig.savefig(os.path.join(FIG, f"v8_ic_vs_rho.{e}"), dpi=200) for e in ("svg", "png")]; plt.close(fig)
fig, ax = plt.subplots(figsize=(7.4, 3.6))
sel = [d for d in ph if d["tag"] in ("best7", "best8", "lp50_8")]
bottom = [0.0] * len(sel)
for k, c in zip(["t_setup", "t_relations", "t_filter", "t_linear_algebra", "t_descent"], ["#bbbbbb", "#1f77b4", "#8c564b", "#ff7f0e", "#2ca02c"]):
    v = [d[k] for d in sel]; ax.bar(range(len(sel)), v, bottom=bottom, color=c, label=k); bottom = [a + b for a, b in zip(bottom, v)]
ax.set_xticks(range(len(sel))); ax.set_xticklabels([f"{d['q']}\n{d['tag']}" for d in sel], fontsize=6); ax.set_ylabel("median s (L0)"); ax.legend(fontsize=7)
ax.set_title("Phase medians"); fig.tight_layout(); [fig.savefig(os.path.join(FIG, f"v8_phases.{e}"), dpi=200) for e in ("svg", "png")]; plt.close(fig)
fig, ax = plt.subplots(figsize=(6.4, 3.4))
for t, c in [("best8", "#2ca02c"), ("d50_8", "#1f77b4"), ("lp50_8", "#9467bd")]:
    pts = [(d["q"], d["ic_over_rho_m8"]) for d in ph if d["tag"] == t]
    if pts: ax.plot(*zip(*pts), "o-", color=c, label=f"{t} / rho_m8")
pts = [(d["q"], d["ic_over_rho_m7"]) for d in ph if d["tag"] == "best8"]
if pts: ax.plot(*zip(*pts), "o:", color="#7f7f7f", label="best8 / rho_m7")
ax.axhline(1, color="k", lw=.8); ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xlabel("q"); ax.set_ylabel("median t_ic / expected rho time")
ax.legend(fontsize=7); ax.grid(alpha=.3); ax.set_title("Distance remaining to the matched rho")
fig.tight_layout(); [fig.savefig(os.path.join(FIG, f"v8_gap_to_rho.{e}"), dpi=200) for e in ("svg", "png")]; plt.close(fig)
