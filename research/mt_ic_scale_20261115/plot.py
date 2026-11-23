#!/usr/bin/env python3
"""fig_scale.svg/.png/.pdf from summary.json, plus results/phase_split.json (descriptive, outside the frozen
analysis): IC (mu = 0.85) precompute phase split from the ms_at_* timestamps, median over repeats and instances."""
import json, os, statistics as st, collections
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
HERE = os.path.dirname(os.path.abspath(__file__))
summ = json.load(open(os.path.join(HERE, "summary.json")))
rows = [json.loads(l) for l in open(os.path.join(HERE, "results", "scale", "runs.jsonl"))]

ph = collections.defaultdict(lambda: collections.defaultdict(list))
for r in rows:
    if r.get("key") != "ic:4:0.85" or "error" in r: continue
    c = ph[f"{r['pb']}:{r['T']}"]
    c["fb"].append(r["ms_at_fb"]); c["combos"].append(r["ms_at_combos"] - r["ms_at_fb"])
    c["table"].append(r["ms_at_table"] - r["ms_at_combos"]); c["relations"].append(r["ms_at_relations"] - r["ms_at_table"])
    c["targets"].append(r["ms_arm"] - r["ms_at_relations"]); c["whole"].append(r["ms_arm"])
split = {c: {k: st.median(v) for k, v in d.items()} for c, d in ph.items()}
json.dump(split, open(os.path.join(HERE, "results", "phase_split.json"), "w"), indent=1)

Ts = [1024, 4096, 16384]; sizes = [32, 34, 36]
COL = {1024: "#2a6fdb", 4096: "#d1495b", 16384: "#3a9d5d"}
fig, (a, b, c) = plt.subplots(1, 3, figsize=(15, 4.3))
for T in Ts:
    med = [summ["cells"][f"{s}:{T}"]["ic:4:0.85/best_rho"]["median"] for s in sizes]
    a.plot(sizes, med, "o-", color=COL[T], label=f"T = {T}")
    for s in sizes:
        a.plot([s + (Ts.index(T) - 1) * 0.15] * 4, summ["cells"][f"{s}:{T}"]["ic:4:0.85/best_rho"]["per_instance"],
               "o", color=COL[T], alpha=0.35, ms=4)
a.axhline(1, color="#888", lw=0.8, ls="--")
a.set_xticks(sizes); a.set_xticklabels([f"2^{s}" for s in sizes]); a.set_xlabel("group size")
a.set_ylabel("IC (μ = 0.85) / best rho, cell median"); a.set_title("Distance to rho grows with the group")
a.legend(frameon=False, fontsize=8)
MCOL = {"0.7": "#999999", "0.85": "#d1495b", "1.0": "#2a6fdb"}
cells = [f"{s}:{T}" for s in sizes for T in Ts]
for j, mu in enumerate(["0.7", "0.85", "1.0"]):
    b.plot([x + (j - 1) * 0.25 for x in range(len(cells))], [summ["cells"][c_][f"ic:4:{mu}/best_rho"]["median"] for c_ in cells],
           "o", color=MCOL[mu], label=f"μ = {mu}")
b.axhline(1, color="#888", lw=0.8, ls="--")
b.set_xticks(range(len(cells))); b.set_xticklabels([c_.replace(":", "\nT=") for c_ in cells], fontsize=7)
b.set_ylabel("IC / best rho, cell median"); b.set_title("μ = 0.85 lowest in 8 of 9 cells"); b.legend(frameon=False, fontsize=8)
PH = [("table", "#d1495b"), ("targets", "#2a6fdb"), ("combos", "#e3a72f"), ("relations", "#3a9d5d"), ("fb", "#777777")]
bottom = [0] * len(cells)
for name, col in PH:
    v = [split[c_][name] / split[c_]["whole"] for c_ in cells]
    c.bar(range(len(cells)), v, bottom=bottom, color=col, label=name); bottom = [x + y for x, y in zip(bottom, v)]
c.set_xticks(range(len(cells))); c.set_xticklabels([c_.replace(":", "\nT=") for c_ in cells], fontsize=7)
c.set_ylabel("share of whole solve (medians)"); c.set_title("IC phase split (μ = 0.85)"); c.legend(frameon=False, fontsize=8)
fig.suptitle("Multi-target IC vs tuned rho at 2^32, 2^34, 2^36, local Xeon [pcc:unverified]", fontsize=11)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(os.path.join(HERE, f"fig_scale.{ext}"), dpi=150 if ext == "png" else None)
print(json.dumps({c_: {k: round(v / split[c_]["whole"], 3) for k, v in split[c_].items()} for c_ in cells}, indent=0))
