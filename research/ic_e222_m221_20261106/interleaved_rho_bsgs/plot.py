"""rho/BSGS ratio per cell with 95% bootstrap intervals and the A/A arm (reads summary.json)."""
import json, pathlib
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
here = pathlib.Path(__file__).resolve().parent
S = json.loads((here / "summary.json").read_text())
fig, ax = plt.subplots(figsize=(9, 4))
x = range(len(S))
for i, o in enumerate(S):
    ax.errorbar(i - 0.12, o["rho_over_bsgs"], yerr=[[o["rho_over_bsgs"] - o["lo"]], [o["hi"] - o["rho_over_bsgs"]]], fmt="o", color="#1f6fb2", capsize=3, label="rho / BSGS" if i == 0 else None)
    ax.errorbar(i + 0.12, o["aa"], yerr=[[o["aa"] - o["aa_lo"]], [o["aa_hi"] - o["aa"]]], fmt="s", color="#999999", capsize=3, ms=4, label="rho A/A" if i == 0 else None)
ax.axhline(1, color="grey", lw=0.8)
ax.set_xticks(list(x)); ax.set_xticklabels([f"{o['bits']}b\nT={o['T']}" for o in S], fontsize=8)
ax.set_ylabel("time ratio (above 1: BSGS faster)")
ax.set_title("Interleaved multi-target rho (tuned dp) vs folded multi-target BSGS, j=0 curves\ngeometric mean of 6 paired rounds, 95% bootstrap interval", fontsize=9)
ax.legend(fontsize=8)
fig.tight_layout()
for e in ("svg", "png"): fig.savefig(here / f"rho_vs_bsgs.{e}", dpi=150)
with PdfPages(here / "rho_vs_bsgs.pdf") as p: p.savefig(fig)
