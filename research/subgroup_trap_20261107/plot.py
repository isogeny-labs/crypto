"""Layout only: draws numbers already computed by the Rust run (no research computation)."""
import json
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

run = Path(sys.argv[1])
rows = []
for k in (5, 7, 9, 11):
    p = run / f"c1_k{k}.jsonl"
    if p.exists():
        rows += [json.loads(l) for l in p.read_text().splitlines() if l.strip()]
mass = [json.loads(l) for l in (run / "mass.jsonl").read_text().splitlines() if l.strip()]

fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))

# (a) A0 std: where the decompositions are. Mean count per target on H against
# the identity's per-target prediction over all of E.
a0 = [r for r in rows if r["curve"] == "A0" and r["arm"] == "std"]
ks = [m["k"] for m in mass]
ax[0].bar([k - 0.4 for k in ks], [m["mean_on_H"] for m in mass], width=0.8, color="#3b6ea5", label="mean N(R) on H (exact)")
preds = {r["k"]: r["pred_excl"] for r in a0}
ax[0].bar([k + 0.4 for k in ks], [preds[k] for k in ks], width=0.8, color="#c9883a", label="identity prediction over E")
for m in mass:
    ax[0].annotate(f"|H| = {m['H']}\nof #E(F_Q²) = {m['E_FQ2']}", (m["k"], m["mean_on_H"]), textcoords="offset points", xytext=(0, 4), ha="center", fontsize=7)
ax[0].set_xticks(ks)
ax[0].set_yscale("log")
ax[0].set_xlabel("k (Q = 2^k, field F_{Q^4})")
ax[0].set_ylabel("decompositions per target")
ax[0].set_title("(a) A0 standard base: all m⁴ decompositions lie on H", fontsize=9)
ax[0].legend(fontsize=7, loc="upper left")
ax[0].set_ylim(1, 5e7)

# (b) every frozen row: zero off 2E, about 2x prediction on 2E
for r in rows:
    if r["curve"] == "A0" or r["mean_on_2E_over_pred"] is None:
        continue
    jit = {"std": -0.25}.get(r["arm"], 0.0) + (0.25 if r["arm"].startswith("tw_b") else 0.0)
    ax[1].scatter(r["k"] + jit, r["mean_on_2E_over_pred"], s=14, color="#3b6ea5" if r["arm"] != "std" else "#c9883a")
ax[1].set_xticks(sorted({r["k"] for r in rows}))
ax[1].axhline(2.0, color="black", lw=0.8, ls="--")
ax[1].axhline(1.0, color="grey", lw=0.6, ls=":")
nviol = sum(r["viol_2E"] for r in rows)
npairs = sum(round(r["T"] * (1 - r["frac_tr_eq_tr_a"])) for r in rows)
ax[1].set_title(f"(b) count on Tr(x_R)=Tr(a) half / prediction; {nviol} nonzero of {npairs} off-2E targets", fontsize=9)
ax[1].set_xlabel("k (orange: standard base, blue: twisted lines)")
ax[1].set_ylabel("mean on 2E / pred_excl")
ax[1].set_ylim(0, 3.5)

fig.tight_layout()
fig.savefig(run.parent.parent / "fig_subgroup_trap.svg")
fig.savefig(run.parent.parent / "fig_subgroup_trap.png", dpi=150)
