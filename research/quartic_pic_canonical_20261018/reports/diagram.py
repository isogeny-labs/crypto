"""Draw the add pipeline (text layout only; no computation)."""
import os, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import FancyBboxPatch

out = os.path.dirname(os.path.abspath(__file__)) if len(sys.argv) < 2 else sys.argv[1]
fig, ax = plt.subplots(figsize=(11, 6.4))
ax.set_xlim(0, 11); ax.set_ylim(0, 6.4); ax.axis("off")
def box(x, y, w, h, t, fc="#eef3fb", ec="#2a78d6"):
    ax.add_patch(FancyBboxPatch((x, y), w, h, boxstyle="round,pad=0.04,rounding_size=0.08", fc=fc, ec=ec, lw=1.2))
    ax.text(x + w / 2, y + h / 2, t, ha="center", va="center", fontsize=8, color="#0b0b0b")
def arr(x1, y1, x2, y2):
    ax.annotate("", xy=(x2, y2), xytext=(x1, y1), arrowprops=dict(arrowstyle="->", color="#52514e", lw=1.2))
box(0.1, 5.1, 3.6, 1.0, "operands x, y\ntypical: conics through D (3x6 echelon)\nexceptional Exc(P): cubics through a\nline section minus P (7 cubics)")
box(4.0, 5.1, 3.2, 1.0, "Z6 = D_x + D_y: products of\ngenerating spaces (lift the degree\nif the span is not complete)")
box(7.5, 5.1, 3.4, 1.0, "4 cubics through Z6\n(e * R_1 inside the complete\ndegree-4 space)")
arr(3.7, 5.6, 4.0, 5.6); arr(7.2, 5.6, 7.5, 5.6)
box(7.5, 3.9, 3.4, 0.9, "V: 3 cubics through E7 = Z6 + P0\n(one linear condition at P0;\nproduct with lines if P0 in Z6)")
arr(9.55, 5.1, 9.55, 4.8)
box(7.5, 2.5, 3.4, 0.9, "conic e with e*R_1 inside V ?", fc="#fff4e5", ec="#eb6834")
arr(9.55, 3.9, 9.55, 3.4)
box(3.9, 2.5, 3.2, 0.9, "yes: E7 ~ 2K - P\nP = base point (colon with\nlines); result Exc(P)", fc="#fdeaf1", ec="#d55181")
arr(7.5, 2.95, 7.1, 2.95)
box(7.5, 1.0, 3.4, 1.0, "no: cubic c in V; conic q' with\nq' v in c*R_2 (residual R5);\ng_v = q' v / c for v in V", fc="#e9f7f1", ec="#1baf7a")
arr(9.55, 2.5, 9.55, 2.0)
box(3.9, 1.0, 3.2, 1.0, "conics through D_z = span{q', g_v}\nreduced echelon form:\ncanonical key, 3x6 matrix", fc="#e9f7f1", ec="#1baf7a")
arr(7.5, 1.5, 7.1, 1.5)
box(0.1, 2.3, 3.6, 1.7, "Class x <-> x + D_inf in Pic^3, D_inf ~ K - P0\nh0 = 1: unique effective D (not collinear)\n   key = conics through D\nh0 = 2: class K - P, key = P (about q of ~q^3)\nzero = Exc(P0)", fc="#f6f5f1", ec="#52514e")
box(0.1, 0.3, 3.6, 1.4, "neg: conic through D_x + 2 P0, residual\n(pencil of conics gives Exc(P), P = l1 x l2)\nscalar mul: double-and-add\neverything is F_q linear algebra", fc="#f6f5f1", ec="#52514e")
ax.text(5.5, 6.3, "W1: Pic^0 addition on a smooth plane quartic (library path)", ha="center", fontsize=11, fontweight="bold")
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(os.path.join(out, f"add_pipeline.{ext}"), **kw)
print("ok")
