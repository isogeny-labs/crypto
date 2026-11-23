"""Figure and PDF for the follow-up runs: block Wiedemann schedule and real streams."""
import glob, json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

here = os.path.dirname(os.path.abspath(__file__))
fig, ax = plt.subplots(1, 3, figsize=(15, 4.6))
for f in sorted(glob.glob(os.path.join(here, "block_wiedemann", "bw_n*.jsonl"))):
    rows = [json.loads(l) for l in open(f)]
    n = rows[0]["dimension"]
    b = [r["b"] for r in rows]
    ax[0].plot(b, [r["sequence_spmv"] / n for r in rows], "o-", label=f"products / N, N={n}")
    ax[0].plot(b, [r["sequence_length"] / n for r in rows], "s--", label=f"sequential rounds / N, N={n}")
    ax[1].plot(b, [r["wall_ns"] / 1e9 for r in rows], "o-", label=f"wall s, N={n}")
for a in ax[:2]:
    a.set_xscale("log", base=2); a.set_xlabel("block size b (m = n = b)"); a.legend(fontsize=7)
ax[0].set_title("Block Wiedemann: work vs rounds")
ax[1].set_yscale("log"); ax[1].set_title("Single-thread wall time (orientation only)")
recs = []
for f in sorted(glob.glob(os.path.join(here, "results_real", "*.json"))):
    e = json.load(open(f)); g = e["rank"]["gain_dsu"]
    tot = g["balanced_cycles"] + g["unbalanced_cycles"]
    recs.append((os.path.basename(f)[:-5], g["balanced_cycles"] / tot if tot else 0))
ax[2].barh([r[0] for r in recs], [r[1] for r in recs])
ax[2].set_xlim(0, 1); ax[2].set_title("Real streams: balanced share of cycles")
ax[2].tick_params(axis="y", labelsize=7)
fig.suptitle("Large-prime graph follow-ups (single local runs; not benchmarks)", fontsize=10)
fig.tight_layout()
fig.savefig(os.path.join(here, "figure_followup.svg"))
with PdfPages(os.path.join(here, "report_followup.pdf")) as pdf:
    pdf.savefig(fig)
    text = open(os.path.join(here, "README.md")).read()
    i = text.index("## Real relation streams")
    tf = plt.figure(figsize=(11, 8.5))
    tf.text(0.02, 0.98, text[i:i + 7000], va="top", family="monospace", fontsize=5.2)
    pdf.savefig(tf)
print("wrote figure_followup.svg report_followup.pdf")
