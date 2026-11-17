#!/usr/bin/env python3
"""Plot E3 from e3/summary.json (aggregated by examples/generator_effect_e3_summary.rs).

Lays out recorded and aggregated values only; computes nothing new.
Writes e3.svg / e3.png (S against B at each band weight, ratio to the
known-log MITM arm) and e3_f4.svg / e3_f4.png (decision cost per trial
against t).
"""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = Path(__file__).resolve().parent
doc = json.loads((HERE / "summary.json").read_text())
curves = [c for c in doc["curves"] if c["r"] > 1e6]  # the 24-bit population
MARK = {9061313: "o", 13040257: "s", 11223809: "^"}
NAME = {9061313: "e3-c1", 13040257: "e3-c2", 11223809: "e3-c3"}
COL = {
    "chain": "#c0392b",
    "full": "#e67e22",
    "grid": "#2471a3",
    "kl": "#1e8449",
    "rho": "#555555",
}
W = doc["weights"]

fig, axes = plt.subplots(2, 3, figsize=(15, 8.6), sharex=True)
for i, w in enumerate(W):
    ax = axes[0][i]
    rx = axes[1][i]
    for c in curves:
        p = c["p"]
        m = MARK[p]
        B = [row["B"] for row in c["rows"]]
        for key, lab in (("grid", "pkm-semaev grid reference (measured)"), ("kl", "known-log random + MITM m=3 (measured)")):
            s = [row[key]["by_w"][i]["mean_s"] for row in c["rows"]]
            lo = [row[key]["by_w"][i]["ci90"][0] for row in c["rows"]]
            hi = [row[key]["by_w"][i]["ci90"][1] for row in c["rows"]]
            ax.errorbar(B, s, yerr=[[a - b for a, b in zip(s, lo)], [b - a for a, b in zip(s, hi)]],
                        marker=m, color=COL[key], lw=1.2, ms=5, capsize=2,
                        label=lab if p == 9061313 else None)
        for pres in ("chain", "full"):
            pts = [(row["B"], f["by_w"][i]) for row in c["rows"] for f in row["f4"] if f["presentation"] == pres]
            if not pts:
                continue
            ax.plot([b for b, _ in pts], [x["mean_s"] for _, x in pts], marker=m, ls="--",
                    color=COL[pres], lw=1.2, ms=5, mfc="none",
                    label=f"pkm-semaev F4 {pres} (substituted)" if p == 9061313 else None)
            rx.errorbar([b for b, _ in pts], [x["over_kl"]["ratio"] for _, x in pts],
                        yerr=[[x["over_kl"]["ratio"] - x["over_kl"]["ci90"][0] for _, x in pts],
                              [x["over_kl"]["ci90"][1] - x["over_kl"]["ratio"] for _, x in pts]],
                        marker=m, ls="--", color=COL[pres], lw=1.2, ms=5, mfc="none", capsize=2,
                        label=f"F4 {pres} / known-log MITM" if p == 9061313 else None)
        g = [(row["B"], row["grid_vs_kl"][i]["grid_over_kl"]) for row in c["rows"]]
        rx.errorbar([b for b, _ in g], [x["ratio"] for _, x in g],
                    yerr=[[x["ratio"] - x["ci90"][0] for _, x in g], [x["ci90"][1] - x["ratio"] for _, x in g]],
                    marker=m, color=COL["grid"], lw=1.2, ms=5, capsize=2,
                    label="grid / known-log MITM" if p == 9061313 else None)
        if c["rho"]:
            ax.axhline(c["rho"]["by_w"][i]["mean_s"], color=COL["rho"], lw=0.8, ls=":",
                       label="rho-neg (measured, per curve)" if p == 9061313 else None)
    ax.set_yscale("log")
    ax.set_xscale("log", base=2)
    ax.set_title(f"w = {w:g} GAE per unpriced native op")
    ax.grid(True, which="both", alpha=0.25)
    rx.axhline(1.0, color="black", lw=0.8)
    rx.set_yscale("log")
    rx.set_xscale("log", base=2)
    rx.set_xlabel("B (columns; prime-kummer:t, t = 2..5, and known-log size)")
    rx.grid(True, which="both", alpha=0.25)
axes[0][0].set_ylabel("S = GAE / sqrt(r), full solve")
axes[1][0].set_ylabel("S ratio to known-log MITM (paired, 90% CI)")
axes[0][0].legend(fontsize=7, loc="upper right")
axes[1][0].legend(fontsize=7, loc="upper left")
sessions = ", ".join(f"{NAME[c['p']]} {c['session']}" for c in curves)
fig.suptitle("E3: Semaev/F4 on the Kummer tower against known-log:scalars=random with MITM at equal B, m = 3\n"
             f"8 planted_shared targets per curve; markers o/s/^ = e3-c1/c2/c3; sessions {sessions}", fontsize=10)
fig.tight_layout(rect=(0, 0, 1, 0.94))
fig.savefig(HERE / "e3.svg")
fig.savefig(HERE / "e3.png", dpi=150)

# Decision cost per trial against t.
fig, ax = plt.subplots(figsize=(8, 5.2))
for c in curves[:1]:
    ts = [row["t"] for row in c["rows"]]
    for pres in ("chain", "full"):
        pts = [(row["t"], f) for row in c["rows"] for f in row["f4"] if f["presentation"] == pres]
        ax.plot([t for t, _ in pts], [f["f4_muladds_per_call"] for _, f in pts], marker="o", color=COL[pres],
                label=f"F4 {pres}: multiply-adds per call (capped ecbench runs)")
        for t, f in pts:
            cr = f.get("f4cost_consistent")
            if cr:
                ax.vlines(t, cr["min"], cr["max"], color=COL[pres], lw=3, alpha=0.35)
    ax.plot(ts, [row["grid_mults_per_call"] for row in c["rows"]], marker="s", color=COL["grid"],
            label="grid reference: F_p multiplications per call")
    ax.plot(ts, [2 * row["B"] for row in c["rows"]], marker="^", color=COL["kl"],
            label="MITM m=3: 2B additions + 2B lookups per trial (folded table)")
ax.set_yscale("log")
ax.set_xticks([2, 3, 4, 5])
ax.set_xlabel("tower height t (|V| = 2^t, B ≈ 2^(t−1))")
ax.set_ylabel("native operations per decomposition attempt")
ax.set_title("Decision cost per target, e3-c1 (identical on e3-c2/c3 for F4 and grid)\n"
             "shaded bars: consistent (planted) systems, f4cost runs", fontsize=10)
ax.grid(True, which="both", alpha=0.25)
ax.legend(fontsize=8)
fig.tight_layout()
fig.savefig(HERE / "e3_f4.svg")
fig.savefig(HERE / "e3_f4.png", dpi=150)
print("wrote e3.svg e3.png e3_f4.svg e3_f4.png", file=sys.stderr)
