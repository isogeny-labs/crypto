"""rho/IC wall-time ratio before and after the Kuhn-Struik restart fix (reads committed rows only)."""
import json, statistics as st, pathlib
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
here = pathlib.Path(__file__).resolve().parent.parent / "results" / "pf_j0"
IC26 = {20: 8.7, 200: 20.2, 1000: 70.4}  # logged medians, RESULTS_prime_fast_j0_sym.md (rows not committed at 26 bits)
def med(f, k): return st.median(json.loads(l)[k] for l in open(here / f))
fig, axes = plt.subplots(1, 3, figsize=(11, 3.6), sharey=True)
for ax, b, Ts in zip(axes, (26, 28, 30), ((20, 200, 1000), (200, 1000, 3000), (200, 1000, 3000))):
    ic = [IC26[T] if b == 26 else med(f"symmulti_{b}_T{T}.jsonl", "sym_total_ms") for T in Ts]
    old = [med(f"rhoall_{b}_T{T}.jsonl", "folded_multi_ms") / i for T, i in zip(Ts, ic)]
    new = [med(f"rhoks_{b}_T{T}.jsonl", "folded_multi_ms") / i for T, i in zip(Ts, ic)]
    ax.plot(Ts, old, "o--", color="#c0504d", label="rho, old restarts (uniform over all targets)")
    ax.plot(Ts, new, "o-", color="#1f6fb2", label="rho, Kuhn-Struik restarts (unsolved only)")
    ax.axhline(1, color="grey", lw=0.8)
    ax.set_xscale("log"); ax.set_yscale("log"); ax.set_xticks(Ts); ax.set_xticklabels([str(T) for T in Ts]); ax.minorticks_off(); ax.set_title(f"{b}-bit j=0 curves"); ax.set_xlabel("targets T")
axes[0].set_ylabel("rho time / IC time  (above 1: IC faster)")
axes[0].legend(fontsize=7, loc="upper left")
fig.suptitle("Symmetric multi-target IC vs batched folded multi-target rho: the IC lead was the rho's restart bug", fontsize=10)
fig.tight_layout()
out = pathlib.Path(__file__).resolve().parent
fig.savefig(out / "multirho_restart_fix.svg"); fig.savefig(out / "multirho_restart_fix.png", dpi=150)
with PdfPages(out / "multirho_restart_fix.pdf") as pdf: pdf.savefig(fig)
