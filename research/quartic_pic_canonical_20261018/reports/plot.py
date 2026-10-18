"""Plot recorded W1 M1/M2 values (results/m1_summary.jsonl, results/m2_fits.jsonl).
Python only lays out recorded values; nothing is computed here beyond axis scaling."""
import json, sys, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

here = os.path.dirname(os.path.abspath(__file__))
res = os.path.join(here, "..", "results")
out = here if len(sys.argv) < 2 else sys.argv[1]
rows = [json.loads(l) for l in open(os.path.join(res, "m1_summary.jsonl"))]
setup = {r["k"]: r for r in rows if r["rec"] == "setup"}
ops = [r for r in rows if r["rec"] == "op"]
ks = sorted(setup)
log2q = [float(np.log2(setup[k]["q"])) for k in ks]

COL = {"add_generic": "#2a78d6", "add_exceptional_operand": "#eb6834", "double": "#1baf7a", "negate": "#e87ba4"}
LAB = {"add_generic": "add, generic", "add_exceptional_operand": "add, one exceptional operand", "double": "double", "negate": "negate"}
MK = {"add_generic": "o", "add_exceptional_operand": "s", "double": "^", "negate": "D"}

def series(op, field, stat="median"):
    d = {r["k"]: r for r in ops if r["op"] == op}
    return [d[k][field][stat] if isinstance(d[k][field], dict) else d[k][field] for k in ks]

def style(ax):
    for s in ("top", "right"):
        ax.spines[s].set_visible(False)
    ax.grid(axis="y", color="#e2e1dd", lw=0.6)
    ax.set_axisbelow(True)

fig, axs = plt.subplots(1, 3, figsize=(13.5, 4.2))
# (a) multiplications
ax = axs[0]
for op in COL:
    y = series(op, "mul")
    lo, hi = series(op, "mul", "q1"), series(op, "mul", "q3")
    ax.errorbar(log2q, y, yerr=[np.array(y) - lo, np.array(hi) - y], color=COL[op], marker=MK[op], ms=5, lw=1.6, capsize=2, label=LAB[op])
ax.set_ylim(0, 7500); ax.set_xlabel("log2 q"); ax.set_ylabel("F_q multiplications per operation (median, IQR)")
ax.set_title("(a) multiplication count", fontsize=10); style(ax)
# (b) priced
ax = axs[1]
for op in COL:
    ax.plot(log2q, series(op, "priced_mul_equiv_median"), color=COL[op], marker=MK[op], ms=5, lw=1.6, label=LAB[op])
ax.set_ylim(0, 9000); ax.set_xlabel("log2 q"); ax.set_ylabel("mul + inv x (measured inv/mul time), median")
ax.set_title("(b) inversions priced at measured cost", fontsize=10); style(ax)
# (c) wall
ax = axs[2]
for op in COL:
    y = series(op, "ns"); lo = series(op, "ns", "q1"); hi = series(op, "ns", "q3")
    y = np.array(y) / 1e3; lo = np.array(lo) / 1e3; hi = np.array(hi) / 1e3
    ax.errorbar(log2q, y, yerr=[y - lo, hi - y], color=COL[op], marker=MK[op], ms=5, lw=1.6, capsize=2, label=LAB[op])
ax.set_ylim(0, 110); ax.set_xlabel("log2 q"); ax.set_ylabel("wall time per operation, microseconds (median, IQR)")
ax.set_title("(c) wall time, shared host", fontsize=10); style(ax)
axs[0].legend(fontsize=8, frameon=False, loc="lower right")
fig.suptitle("W1 plane-quartic Pic: operation cost vs q (synthetic quartics, 200 operations per point; 2 pinned cores, shared host; no external comparison)", fontsize=9.5)
fig.tight_layout(rect=(0, 0, 1, 0.94))
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(os.path.join(out, f"m1_cost_vs_q.{ext}"), **kw)
plt.close(fig)

# scalar multiplication + key encoding
fig, axs = plt.subplots(1, 2, figsize=(9.5, 3.8))
ax = axs[0]
y = np.array(series("scalar_mul_64bit", "mul")) / 1e3
lo = np.array(series("scalar_mul_64bit", "mul", "q1")) / 1e3; hi = np.array(series("scalar_mul_64bit", "mul", "q3")) / 1e3
ax.errorbar(log2q, y, yerr=[y - lo, hi - y], color="#6250d6", marker="o", ms=5, lw=1.6, capsize=2)
ax.set_ylim(0, 600); ax.set_xlabel("log2 q"); ax.set_ylabel("thousand F_q multiplications")
ax.set_title("scalar multiplication, random 64-bit scalar", fontsize=10); style(ax)
ax = axs[1]
y = series("key_encoding", "mul"); iv = series("key_encoding", "inv")
ax.plot(log2q, y, color="#2a78d6", marker="o", ms=5, lw=1.6)
ax.set_ylim(0, 60); ax.set_xlabel("log2 q"); ax.set_ylabel("F_q multiplications (3 inversions each)")
ax.set_title("key encoding (echelon form of a conic basis)", fontsize=10); style(ax)
fig.tight_layout()
for ext, kw in (("svg", {}), ("png", {"dpi": 150})):
    fig.savefig(os.path.join(out, f"m1_scalar_key.{ext}"), **kw)
plt.close(fig)
print("ok")
