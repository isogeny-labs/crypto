"""Marked rendering of the n71/n83 hosted single-target panel (SVG + 150 dpi PNG).

Run from the repository root. Reads the recorded values read-only from the
hashed evidence copy (figures/data.json) and, for N83, the precomputed-rho
control records. The empty-table rho series is drawn hollow, grey and dashed
and labelled SUPERSEDED (validation sweep B 104.1, 2026-10-20). No new
measurement; no value is recomputed beyond what the records store.
"""
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

E = "research/sat_factor_base_review_20260908/autolab/evidence/20261014-koblitz-n71-n83-hosted-single-target"
C = "research/20261016-n83-precomputed-rho-control/records/run-38855595549/k600/analysis.json"
OUT = "benchmarks/20261014-n71-n83-hosted-single-target/n71-n83-hosted-online-ms"
data = json.load(open(f"{E}/figures/data.json"))
ctl = json.load(open(C))["arms"]

fig, axes = plt.subplots(1, 2, figsize=(11, 5.8))
for ax, tag, rid in zip(axes, ("n71", "n83"), ("gha-38585913645-1", "gha-38597929540-1")):
    rows = [r for r in data[tag] if "seed" in r]
    seeds = [r["seed"] for r in rows]
    ic = [r["ic_online_ms"] for r in rows]
    rho = [r["rho_online_ms"] for r in rows]
    ax.plot(seeds, rho, "o--", mfc="none", color="#888", alpha=0.8,
            label="rho, empty table (SUPERSEDED)")
    ax.plot(seeds, ic, "s-", color="#146", label="IC compact orbit online ms (setup excluded)")
    if tag == "n83":
        for key, col, lab in (("bl_a_time_matched", "#11a579", "Bernstein-Lange rho, time-matched table (control)"),
                              ("bl_b_memory_matched", "#3969ac", "Bernstein-Lange rho, memory-matched table (control)")):
            ax.plot(seeds, ctl[key]["online_ms"], "^-", color=col, ms=4, lw=1, label=lab)
        note = "superseded: empty-table rho; IC setup excluded (sweep B 104.1)\nprecomputed rho control (K600, rerun of same 16 targets) is faster"
    else:
        note = "superseded: empty-table rho; IC setup excluded (sweep B 104.1)\nno precomputed-rho control measured"
    ax.text(0.02, 0.985, note, transform=ax.transAxes, va="top", fontsize=7, color="#b00",
            bbox=dict(boxstyle="round", fc="#fff3f3", ec="#b00"))
    ax.set_yscale("log")
    ax.set_xlabel("one-target eval workload seed")
    ax.set_ylabel("online wall (ms, log scale)")
    ax.set_title(f"{tag}: single-target online, hosted EPYC, run {rid}, K=600, W=16\n"
                 "superseded ratio, not an improvement", fontsize=9, color="#b00")
    ax.grid(alpha=0.3)
    lo, hi = ax.get_ylim()
    ax.set_ylim(lo, hi * 12)
    ax.legend(fontsize=6.5, loc="upper center", bbox_to_anchor=(0.5, -0.14))
fig.suptitle("Koblitz compact-orbit single-target panels, hosted reruns (AMD EPYC 7763); marked 2026-10-20", fontsize=9)
fig.tight_layout()
fig.savefig(OUT + ".svg")
fig.savefig(OUT + ".png", dpi=150)
print("written")
