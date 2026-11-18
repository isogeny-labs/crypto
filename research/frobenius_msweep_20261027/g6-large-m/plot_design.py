#!/usr/bin/env python3
"""Design figure for G6: the factor-base sizes the Frobenius-interleaved design can take at n = 23, versus the target window.
The plotted numbers are the HAND ARITHMETIC of PROTOCOL.md section 4.3 (recorded there), not measurements; the census confirms or refutes them."""
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

# m -> {(A,B): log2 mu}, a = 0, n = 23, E ~ 2^23; A = [beta on curve], B = [beta + beta^(2^m) on curve]
vals = {
    12: {(0, 0): -23.0, (0, 1): -5.6, (1, 0): 4.1, (1, 1): 9.5},
    14: {(0, 0): -23.0, (0, 1): -8.7, (1, 0): 5.8, (1, 1): 10.2},
    16: {(0, 0): -23.0, (0, 1): -11.9, (1, 0): 7.5, (1, 1): 10.9},
    18: {(0, 0): -23.0, (0, 1): -15.1, (1, 0): 9.2, (1, 1): 11.6},
    20: {(0, 0): -23.0, (0, 1): -18.2, (1, 0): 10.9, (1, 1): 12.4},
}
lo, hi = -10.0, 5.0
fig, ax = plt.subplots(figsize=(7.5, 4.6), constrained_layout=True)
ax.axhspan(lo, hi, color="tab:green", alpha=0.18, label="target window  log2 mu in [-10, 5]")
ax.axhline(0, color="k", lw=0.6)
cols = {(0, 0): "tab:grey", (0, 1): "tab:blue", (1, 0): "tab:orange", (1, 1): "tab:red"}
for m, d in vals.items():
    for ab, v in d.items():
        ax.plot(m, v, "o", color=cols[ab], label=f"A={ab[0]}, B={ab[1]}" if m == 12 else None)
ax.set_xlabel("m (number of slots), n = 23, a = 0")
ax.set_ylabel("log2 mu of the NI design (hand arithmetic)")
ax.set_xticks(list(vals))
ax.set_title("Frobenius-interleaved slots: four sizes per m, none in the window at m >= 16", fontsize=10)
ax.legend(fontsize=8, loc="center right")
fig.savefig("design_feasibility.svg")
fig.savefig("design_feasibility.png", dpi=140)
print("wrote design_feasibility.svg")
