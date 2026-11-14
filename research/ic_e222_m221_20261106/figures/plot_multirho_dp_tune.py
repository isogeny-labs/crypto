"""dp-bit sweep (held-out instances) and rho/IC before/after tuning. Reads committed rows only."""
import json, math, statistics as st, pathlib
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
here = pathlib.Path(__file__).resolve().parent
d = here.parent / "results" / "pf_j0"
def L(f): return [json.loads(l) for l in open(d / f)]
IC26 = {20: 8.7, 200: 20.2, 1000: 70.4}  # logged medians (26-bit IC rows not committed)
cells = [(26, (20, 200, 1000)), (28, (200, 1000, 3000)), (30, (200, 1000, 3000))]
fig, ax = plt.subplots(2, 3, figsize=(11, 6.4))
for j, (b, Ts) in enumerate(cells):
    a = ax[0][j]
    for T, c in zip(Ts, ("#7fb3d5", "#2e86c1", "#1b4f72")):
        W = math.sqrt(2 * (2**b / 6) * T)
        ms = [st.median(x["folded_multi_ms"] for x in L(f"dpsweep/b{b}_T{T}_dp{dp}.jsonl")) for dp in range(2, 13)]
        a.plot(range(2, 13), ms, "o-", ms=3, color=c, label=f"T={T}")
        a.axvline(round(math.log2(W / T) - 1.5), color=c, ls=":", lw=1)
    a.set_yscale("log"); a.set_title(f"{b} bits: held-out sweep (INST0=10)", fontsize=9)
    a.set_xlabel("dp bits"); a.legend(fontsize=7)
    a = ax[1][j]
    ic = [IC26[T] if b == 26 else st.median(x["sym_total_ms"] for x in L(f"symmulti_{b}_T{T}.jsonl")) for T in Ts]
    for f, lab, c, ls in (("rhoall", "rho, old restarts", "#c0504d", "--"),
                          ("rhoks", "rho, K-S restarts, old dp", "#e3a33b", "-"),
                          ("rhotuned", "rho, K-S restarts, tuned dp", "#1f6fb2", "-")):
        a.plot(Ts, [st.median(x["folded_multi_ms"] for x in L(f"{f}_{b}_T{T}.jsonl")) / i for T, i in zip(Ts, ic)], "o" + ls, color=c, label=lab)
    a.axhline(1, color="grey", lw=0.8); a.set_xscale("log"); a.set_yscale("log")
    a.set_xticks(Ts); a.set_xticklabels([str(T) for T in Ts])
    yt = [0.1, 0.2, 0.5, 1, 2]
    a.set_yticks(yt); a.set_yticklabels([str(v) for v in yt]); a.set_ylim(0.1, 3); a.minorticks_off()
    a.set_xlabel("targets T"); a.set_title(f"{b} bits: IC's own targets (INST0=0)", fontsize=9)
ax[0][0].set_ylabel("rho wall ms (median of 4)")
ax[1][0].set_ylabel("rho time / IC time (above 1: IC faster)")
ax[1][0].legend(fontsize=7, loc="upper left")
fig.suptitle("Multi-target rho dp-bit tuning (dotted: rule dp = round(log2(sqrt(2nT/6)/T) - 1.5)) and the resulting rho/IC", fontsize=10)
fig.tight_layout()
for e in ("svg", "png"): fig.savefig(here / f"multirho_dp_tune.{e}", dpi=150)
with PdfPages(here / "multirho_dp_tune.pdf") as p: p.savefig(fig)
