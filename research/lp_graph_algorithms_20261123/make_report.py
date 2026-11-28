"""Summary table, figure (SVG) and PDF for the lp-graph-eval/v1 records in results/."""
import csv, glob, json, os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

here = os.path.dirname(os.path.abspath(__file__))
recs = []
for f in sorted(glob.glob(os.path.join(here, "results", "*.json"))):
    e = json.load(open(f))
    e["_name"] = os.path.basename(f)[:-5]
    recs.append(e)
recs.sort(key=lambda e: (e["rows"], e["_name"]))

rows = []
for e in recs:
    r, g, pf, mf, t = e["rank"], e["rank"]["gain_dsu"], e["pivot_filter"], e["mst_filter"], e["trees"]
    parts = {p["parts"]: p for p in e["partitions"]}
    rows.append({
        "stream": e["_name"], "R": e["rows"], "C": e["lp_columns"], "I": e["incidences"],
        "two_entry_only": r["exact_for_stream"], "dsu_rank": g["rank"], "dsu_left_nullity": g["left_nullity"],
        "balanced_cycles": g["balanced_cycles"], "unbalanced_cycles": g["unbalanced_cycles"],
        "dense_rank_matches": r["dense_matches"], "structural_bound_lower": r["structural_lower_bound"],
        "hopcroft_karp_rank": r["matching_rank"], "dsu_ms": round(r["gain_dsu_ns"] / 1e6, 3),
        "L_bfs": t["bfs"]["total_length"], "L_low_stretch": t["low_stretch"]["total_length"], "cycles": t["bfs"]["cycles"],
        "pivot_out": pf["report"]["complete_relations"], "pivot_N": pf["residual_columns"], "pivot_Z": pf["residual_nonzeros"],
        "pivot_sources": pf["source_terms"], "pivot_ms": round(pf["ns"] / 1e6, 2), "pivot_replay_failures": pf["replay_failures"],
        "mst_refused": mf["refused"], "mst_out": mf["report"]["complete_relations"], "mst_Z": mf["residual_nonzeros"],
        "mst_sources": mf["source_terms"], "mst_ms": round(mf["ns"] / 1e6, 2), "mst_replay_failures": mf["replay_failures"],
        "mindeg_fill": e["residual_elimination"]["fill_edges"], "mindeg_width": e["residual_elimination"]["width"],
        "mindeg_truncated": e["residual_elimination"]["truncated"],
        **{f"lambda_block_k{k}": parts[k]["block_connectivity_minus_one"] for k in parts},
        **{f"lambda_fm_k{k}": parts[k]["fm_connectivity_minus_one"] for k in parts},
    })
with open(os.path.join(here, "summary.csv"), "w", newline="") as fh:
    w = csv.DictWriter(fh, fieldnames=list(rows[0].keys()))
    w.writeheader(); w.writerows(rows)

names = [r["stream"] for r in rows]
x = range(len(rows))
fig, ax = plt.subplots(1, 3, figsize=(15, 4.6))
ax[0].bar([i - 0.2 for i in x], [r["L_bfs"] for r in rows], 0.4, label="BFS tree")
ax[0].bar([i + 0.2 for i in x], [r["L_low_stretch"] for r in rows], 0.4, label="low-stretch (AKPW heuristic)")
ax[0].set_yscale("log"); ax[0].set_title("Fundamental-cycle length L (lower is better)")
for k, mk in [(2, "o"), (4, "s"), (8, "^")]:
    ax[1].plot(list(x), [r[f"lambda_fm_k{k}"] / r[f"lambda_block_k{k}"] for r in rows], mk + "-", label=f"k={k}")
ax[1].axhline(1, color="grey", lw=0.8); ax[1].set_ylim(0, 1.1)
ax[1].set_title("Σ(λ−1): FM / contiguous rows (model, not timed)")
ran = [r for r in rows if not r["mst_refused"]]
xr = range(len(ran))
ax[2].bar([i - 0.2 for i in xr], [r["mst_Z"] / r["pivot_Z"] for r in ran], 0.4, label="Z  MST / pivot")
ax[2].bar([i + 0.2 for i in xr], [r["mst_ms"] / r["pivot_ms"] / 10 for r in ran], 0.4, label="wall MST / pivot ÷ 10")
ax[2].axhline(1, color="grey", lw=0.8)
ax[2].set_xticks(list(xr)); ax[2].set_xticklabels([r["stream"] for r in ran], rotation=60, ha="right", fontsize=7)
ax[2].set_title("MST vs pivot filter (streams MST accepted)")
for a in ax[:2]:
    a.set_xticks(list(x)); a.set_xticklabels(names, rotation=60, ha="right", fontsize=7)
for a in ax:
    a.legend(fontsize=7)
fig.suptitle("Large-prime graph algorithms on frozen synthetic streams (single local runs; not benchmarks)", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(here, "figure.svg"))
with PdfPages(os.path.join(here, "report.pdf")) as pdf:
    pdf.savefig(fig)
    tf = plt.figure(figsize=(11, 8.5)); tf.text(0.02, 0.98, open(os.path.join(here, "README.md")).read()[:6000],
        va="top", family="monospace", fontsize=5.5); pdf.savefig(tf)
print("wrote summary.csv figure.svg report.pdf", len(rows), "streams")
