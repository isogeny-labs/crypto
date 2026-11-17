"""Plot recorded values only (research-visuals): no result is computed here.

Inputs: raw/*.jsonl written by examples/hec_rho_window_reprice.rs and
ir_spread/fit-*.json written by examples/ir_gate_fit.rs.  Ratios, shares and
fits are read from those files; this script only lays them out.
"""
import json
import math
import pathlib

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = pathlib.Path(__file__).resolve().parent
STUDY = HERE.parent


def rows(name):
    return [json.loads(l) for l in (STUDY / "raw" / name).read_text().splitlines() if l.strip()]


def save(fig, stem):
    fig.savefig(HERE / f"{stem}.svg")
    fig.savefig(HERE / f"{stem}.png", dpi=150)


# Figure 1: IC/rho per row, and the window's share of the default walk.
cf = rows("collide-first.jsonl")
colors = {2: "#1f77b4", 3: "#d62728", 4: "#2ca02c"}
fig, (a, b) = plt.subplots(1, 2, figsize=(11, 5.6))
for g in (2, 3, 4):
    rs = [r for r in cf if r["genus"] == g]
    x = [math.log2(int(r["n"])) for r in rs]
    a.plot(x, [r["committed_ic_over_rho"] for r in rs], "s--", color=colors[g], mfc="none",
           label=f"g{g}: committed, full-width rho")
    a.plot(x, [r["ic_over_rho_window_uncharged"] for r in rs], "o", color=colors[g], mfc="none", ms=8,
           label=f"g{g}: default walk, window uncharged")
    a.plot(x, [r["ic_over_rho"] for r in rs], "x", color=colors[g], ms=6,
           label=f"g{g}: default walk, window charged")
    b.plot(x, [100 * int(r["window_bitops_summed"]) / (r["rho_bitops_mean"] * r["runs"]) for r in rs],
           "o-", color=colors[g], label=f"genus {g}")
a.set_xlabel("log2 N")
a.set_ylabel("IC / rho, bitop/v1 means (committed IC)")
a.set_title("IC/rho per row (18 rows, 40 rho seeds each)")
fig.legend(*a.get_legend_handles_labels(), fontsize=6.5, ncol=3, loc="lower center")
b.set_xlabel("log2 N")
b.set_ylabel("window share of rho bitops (%)")
b.set_title("Cycle window's charge, default walk")
b.legend(fontsize=8)
fig.suptitle("hec_bitop_20261020 rows, rho re-derived (examples/hec_rho_window_reprice.rs); counts, no wall time",
             fontsize=8)
fig.tight_layout(rect=(0, 0.13, 1, 1))
save(fig, "ratio_and_window_share")

# Figure 2: Callgrind Ir against counted gates, rho.signed_frobenius_full, K1/F_2^17.
fit = json.loads((STUDY / "ir_spread" / "fit-ladder.json").read_text())
old = json.loads((STUDY / "ir_spread" / "fit-session-binary.json").read_text())
fig, ax = plt.subplots(figsize=(10.5, 4.6))
g = [r["gates"] for r in fit["records"]]
ax.plot(g, [r["solve_ir"] for r in fit["records"]], "o", color="#1f77b4",
        label="ladder, whole bracketed solve (12 records, binary 4ff2214e)")
ax.plot(g, [r["solve_ir"] - r["fixed_removed_ir"] for r in fit["records"]], "^", color="#2ca02c",
        label="same, minus classes set-up and harness re-pricing")
og = [r["gates"] for r in old["records"]]
ax.plot(og, [r["solve_ir"] for r in old["records"]], "s", color="#d62728", mfc="none", ms=9,
        label="ic-acct-7 seq 9, 11 (session binary bf8196c2)")
xs = [0, 7.0e6]
for f, c, s in ((fit["fit_whole_solve"], "#1f77b4", "-"), (fit["fit_without_fixed"], "#2ca02c", "-"),
                (old["fit_whole_solve"], "#d62728", ":")):
    ax.plot(xs, [f["intercept_ir"] + f["slope_ir_per_gate"] * x for x in xs], s, color=c, lw=1,
            label=f"Ir = {f['intercept_ir']:.3g} + {f['slope_ir_per_gate']:.4f}·gates")
ax.set_xlabel("counted bitop/v1 gates per solve")
ax.set_ylabel("Callgrind Ir per solve")
ax.set_xlim(0, 7.0e6)
ax.set_ylim(0, 3.2e6)
ax.set_title("rho.signed_frobenius_full on K1/F_2^17: fixed per-solve Ir dominates short walks", fontsize=9)
ax.legend(fontsize=7, loc="upper left", bbox_to_anchor=(1.01, 1.0))
fig.tight_layout()
save(fig, "ir_vs_gates")
