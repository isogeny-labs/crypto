"""Route S pipeline diagram (layout only; labels name the code that runs)."""
import os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

HERE = os.path.dirname(os.path.abspath(__file__))
plt.rcParams.update({"font.size": 7, "svg.fonttype": "path"})
boxes = [
    ("E0/F_{q³}:\nQ = kG planted,\nG of prime order ℓ", "smith_route_s::\nroute_s"),
    ("H/F_q: w² = h(z),\ndeg 7 (JV m=3,\nr=5, split arm)", "build/fwd/frc +\nCantor: φ =\n(1+Fr+Fr²)∘π0*"),
    ("Props. 2 and 4\non H (rational S,\nlc(s) a square)", "smith_data_\nwith_subgroups"),
    ("plane quartic\nC ≅ X (canonical\nimage; T1 checks)", "fit_quartic,\ncertify_smooth,\ncount_points"),
    ("Ψ(φG), Ψ(φQ)\n∈ Pic⁰(C) as W1\ncanonical keys", "psi_class_fixed,\nW1Bridge::\nclass_psi"),
    ("dlog in ⟨Ψ(φG)⟩\n⊂ J_X(F_q), brute\nforce on keys;\nverify k'G = Q", "PlaneQuartic::add,\nPicClass::key"),
]
fig, ax = plt.subplots(figsize=(7.4, 1.9))
ax.set_xlim(0, 6)
ax.set_ylim(0, 1.6)
ax.axis("off")
for i, (top, bot) in enumerate(boxes):
    ax.add_patch(FancyBboxPatch((i + 0.06, 0.62), 0.86, 0.9, boxstyle="round,pad=0.02", fc="#e8eef7", ec="#2a6fbb", lw=0.8))
    ax.text(i + 0.49, 1.07, top, ha="center", va="center", fontsize=5.3)
    ax.text(i + 0.49, 0.32, bot, ha="center", va="center", fontsize=5.0, color="#444", family="monospace")
    if i < len(boxes) - 1:
        ax.annotate("", xy=(i + 1.06, 1.07), xytext=(i + 0.94, 1.07), arrowprops=dict(arrowstyle="->", lw=0.8))
fig.tight_layout()
fig.savefig(os.path.join(HERE, "pipeline.svg"))
fig.savefig(os.path.join(HERE, "pipeline.png"), dpi=150)
print("ok")
