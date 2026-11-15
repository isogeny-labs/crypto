"""Draw recorded values from report.json (written by `dedupe-measure report`).
Layout only: every plotted number is read from the report.  Writes
dedupe.svg and dedupe.png (150 dpi)."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

H = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(H, "report.json")))
fig, ax = plt.subplots(1, 3, figsize=(16, 4.4))
names = {"skip_only": "skip O+A1 only", "memo_only": "check/op reuse only", "after": "both (this change)"}
colors = {"skip_only": "#e69f00", "memo_only": "#56b4e9", "after": "#2a6fdb"}

for k, slots in enumerate((2, 3)):
    per = R["counts"][f"slots_{slots}"]
    for j, v in enumerate(("skip_only", "memo_only", "after")):
        ratios = [row["ratio"] for row in per[v]["table"]]
        xs = [j + (i - len(ratios) / 2) * 0.02 for i in range(len(ratios))]
        ax[0].scatter([x + 4 * k for x in xs], ratios, s=10, color=colors[v],
                      label=names[v] if k == 0 else None)
ax[0].axhline(1, color="#888", lw=1, ls=":")
ax[0].set_xticks([1, 5]); ax[0].set_xticklabels(["m = 2 (24 counts)", "m = 3 (12 counts)"])
ax[0].set_ylabel("charged steps: before / variant")
ax[0].set_title("Exact count, every row = brute force"); ax[0].legend(fontsize=8)

s = R["sizes"]
for slots, mk in ((2, "o"), (3, "s")):
    rows = [r for r in s if r["slots"] == slots]
    ax[1].plot([r["d"] for r in rows], [r["edges_before"] for r in rows], mk + "--", color="#888", label=f"m={slots} before")
    ax[1].plot([r["d"] for r in rows], [r["edges"] for r in rows], mk + "-", color="#2a6fdb", label=f"m={slots} after")
for e in R["eccp131"]:
    ax[1].plot([131], [e["edges_before"]], "x", color="#888")
    ax[1].plot([131], [e["edges"]], "x", color="#2a6fdb")
    ax[1].annotate(f"ECCp-131 m={e['slots']}: {e['edges_before']:,} → {e['edges']:,}",
                   xy=(131, e["edges"]), xytext=(40, e["edges"] * (1.6 if e["slots"] == 3 else 0.45)), fontsize=7,
                   arrowprops=dict(arrowstyle="->", color="#2a6fdb"))
ax[1].set_xscale("log", base=2); ax[1].set_yscale("log")
ax[1].set_xlabel("field bits d"); ax[1].set_ylabel("network edges")
ax[1].set_title("Decomposition network size"); ax[1].legend(fontsize=7)

rows = R["solves"]["rows"]
lab = [f"p={r['p']}\nseed {r['seed']}" for r in rows]
x = range(len(rows))
ax[2].bar([i - 0.2 for i in x], [r["steps_before"] for r in rows], 0.4, color="#888", label="before")
ax[2].bar([i + 0.2 for i in x], [r["steps"] for r in rows], 0.4, color="#2a6fdb", label="after")
for i, r in enumerate(rows):
    ax[2].text(i, r["steps_before"] * 1.05, f"{r['ratio']:.2f}×", ha="center", fontsize=8)
ax[2].set_xticks(list(x)); ax[2].set_xticklabels(lab, fontsize=8)
ax[2].set_yscale("log"); ax[2].set_ylabel("charged steps, whole solve")
ax[2].set_title("reduction::solve, same scalar, verified"); ax[2].legend(fontsize=8)

fig.suptitle("hashed-ecdl-tensor: emit each check and modular operation once; start the sum at A1 "
             "(research/hashed_ecdl_dedupe_20261115/runs)", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(H, "dedupe.svg"))
fig.savefig(os.path.join(H, "dedupe.png"), dpi=150)
print("wrote dedupe.svg, dedupe.png")
