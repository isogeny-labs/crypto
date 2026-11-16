"""Lay out recorded values (theorem bounds from README §1, joint nu_s from joint_nu.jsonl)."""
import json
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

rows = [json.loads(l) for l in open("joint_nu.jsonl")]
fig, (a, b) = plt.subplots(1, 2, figsize=(11, 4.4))
labels = ["S3, 3 unknowns, equal bounds\n(Brunn–Minkowski, PR #2612)", "S3, 2 unknowns, any bounds\n(theorem §1)", "S3, 3 unknowns, any bounds\n(theorem §1)"]
vals = [0.3293, 0.5, 0.75]
a.barh(range(3), vals, color=["#3b6ea8", "#2e8b57", "#2e8b57"])
for i, v in enumerate(vals):
    a.text(v + 0.01, i, f"< {v:.3f}", va="center", fontsize=9)
a.axvline(8 / 9, color="#b33", ls="--"); a.text(8 / 9 + 0.01, 2.35, "demand 8/9", color="#b33", fontsize=9)
a.set_yticks(range(3)); a.set_yticklabels(labels, fontsize=8); a.set_xlim(0, 1.05)
a.set_xlabel("proved upper bound on reach ε (any lattice from (p,f)^t)")
a.set_title("Single block: proved bounds vs demand", fontsize=10)
cells = sorted({(r["beta"], r["gamma_u"], r["gamma_v"]) for r in rows})
for c in cells:
    pts = [(r["s"], r["nu_s_over_s"]) for r in rows if (r["beta"], r["gamma_u"], r["gamma_v"]) == c and r["T"] == max(x["T"] for x in rows if x["s"] == r["s"])]
    dem = next(r["in_demand_region"] for r in rows if (r["beta"], r["gamma_u"], r["gamma_v"]) == c)
    b.plot([p[0] for p in pts], [p[1] for p in pts], marker="o", ls="-" if dem else ":", label=f"β={c[0]}, γ=({c[1]},{c[2]}){' demand' if dem else ''}")
b.axhline(1.0, color="#b33", ls="--"); b.text(1.05, 1.05, "joint lattice needs ν_s/s < 1", color="#b33", fontsize=9)
b.set_xticks([1, 2, 3]); b.set_xlabel("s (power of the ideal)"); b.set_ylabel("ν_s(T)/s at the largest T (upper bound)")
b.set_ylim(0, 2.6); b.legend(fontsize=7, loc="lower right")
b.set_title("Joint (4,2) lattice: least δ-weight in (f1,f2,f3)^s", fontsize=10)
fig.suptitle("Small-root follow-ups, 2026-10-26 (theorem_check.jsonl, joint_nu.jsonl)", fontsize=10)
fig.tight_layout(); fig.savefig("followups.svg"); fig.savefig("followups.png", dpi=150)
