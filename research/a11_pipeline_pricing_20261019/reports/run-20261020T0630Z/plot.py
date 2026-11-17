"""Plot per-DLP GAE from ../../paper_bound.txt (the native example's output) only.
Rendering aid; no computation beyond parsing the printed values."""
import os, re
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
text = open(os.path.join(here, "..", "..", "paper_bound.txt")).read()

alg = {"A11": [], "A0": []}
for line in text.splitlines():
    m = re.match(r"\s+(A11|A0)\s+(\d+)\s+\d+\s+\S+\s+\S+\s+.*?\s{2,}(\S+)\s+(measured|extrapolated.*)$", line)
    if m:
        alg[m.group(1)].append((int(m.group(2)), float(m.group(3)), m.group(4).startswith("measured")))
ic = {float(w): float(v) for w, v in re.findall(r"w = (\S+)\s+ic-pkm.*?per DLP mean (\d+)", text)}
rho = [float(v) for v in re.findall(r"rho-neg \(matched reference\):.*?per DLP mean (\d+)", text)]

plt.rcParams.update({"font.size": 9})
fig, ax = plt.subplots(figsize=(7.5, 4.6))
for arm, col, mk in [("A11", "#008300", "P"), ("A0", "#2a78d6", "o")]:
    pts = alg[arm]
    meas = [(t, v) for t, v, ok in pts if ok]
    extr = [(t, v) for t, v, ok in pts if not ok]
    ax.plot([t for t, _ in meas], [v for _, v in meas], color=col, marker=mk, lw=1.4,
            label=f"{arm} algebraic oracle, lower bound (measured F4 counts)")
    if extr:
        ax.plot([t for t, _ in extr], [v for _, v in extr], color=col, marker=mk, mfc="none", ls="--", lw=1,
                label=f"{arm} lower bound, t > 5 held at the t = 5 count (extrapolation)")
lo, hi = min(ic.values()), max(ic.values())
ax.axhspan(lo, hi, color="#eb6834", alpha=0.25, label=f"group-law IC (PKM20, t = 8, measured): {lo:.0f}–{hi:.0f} GAE, w ∈ {{0, 0.1, 1}}")
ax.axhline(rho[0], color="#555", ls=":", label=f"matched rho (lower bound): {rho[0]:.0f} GAE")
ax.set_yscale("log")
ax.set_xlabel("t (factor base g·μ_{2^t}, N = 2^(t−1) abscissae), m = 3")
ax.set_ylabel("GAE per discrete logarithm")
ax.grid(color="#e5e5e5", lw=0.6)
ax.legend(frameon=False, fontsize=7, loc="center right")
ax.set_title("A11 as an algebraic oracle: paper lower bound vs measured arms (p = 946177)", fontsize=9)
fig.tight_layout()
fig.savefig(os.path.join(here, "bound_vs_t.svg"))
fig.savefig(os.path.join(here, "bound_vs_t.png"), dpi=150)
print("wrote bound_vs_t (svg, png)", {k: len(v) for k, v in alg.items()}, ic, rho)
