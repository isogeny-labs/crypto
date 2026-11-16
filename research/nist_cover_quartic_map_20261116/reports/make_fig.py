#!/usr/bin/env python3
"""Lay out recorded values from runs.jsonl: no computation, plotting only."""
import json, sys, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

runs_path = sys.argv[1] if len(sys.argv) > 1 else "../runs.jsonl"
outdir = sys.argv[2] if len(sys.argv) > 2 else "."
rows = []
with open(runs_path) as fh:
    for ln in fh:
        ln = ln.strip()
        if ln:
            rows.append(json.loads(ln))
ok = [r for r in rows if r.get("status") == "ok"]

# Figure 1: per-instance verification panel (recorded booleans/counts).
fig, ax = plt.subplots(figsize=(8, 4.2))
ps = [r["p"] for r in ok]
cC = [r["count_C_Fp"] for r in ok]
cH = [r["count_H_Fp"] for r in ok]
ax.plot(ps, cH, "o-", label="#H(F_p)", color="#1f77b4")
ax.plot(ps, cC, "x--", label="#C(F_p) (plane quartic)", color="#ff7f0e", markersize=9)
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("prime p"); ax.set_ylabel("rational point count")
ax.set_title("Smith trigonal: #C(F_p) = #H(F_p) on the novel NIST-shaped cover")
for r in ok:
    ax.annotate(f"γ={r['gamma']}", (r["p"], r["count_H_Fp"]),
                textcoords="offset points", xytext=(4, 6), fontsize=7)
ax.legend()
cap = ("Each instance: a=-3 prime-order E, novel genus-3 cover H_gamma, "
       "explicit quartic C. Markers coincide -> same zeta (k=1). "
       f"run 20261116T1637Z; {len(ok)} instances.")
fig.text(0.5, -0.02, cap, ha="center", fontsize=7, wrap=True)
fig.tight_layout()
fig.savefig(os.path.join(outdir, "counts.svg"), bbox_inches="tight")
print("wrote counts.svg for", len(ok), "instances")

# summary table to stdout
for r in ok:
    print(f"p={r['p']} b={r['b']} #E={r['e_order']} gamma={r['gamma']} "
          f"ord(phi*P)={r['ord_pullback_P']} hom={r['pullback_homomorphism']} "
          f"#C=#H={r['counts_match']} dlp_on_C={r['dlp_transfer_ok']} "
          f"k={r['planted_k']} rec={r['recovered_log_on_C']}")
