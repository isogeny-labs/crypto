"""Width of the prime-field relation network vs bit size, measured and fitted,
with the 131-bit extrapolation against Pollard rho.  Writes scaling.svg/png."""
import json, math, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

H = os.path.dirname(os.path.abspath(__file__))
X = json.load(open(os.path.join(H, "extrapolation.json")))
pts = X["width_fit"]["points"]
s, b = X["width_fit"]["slope"], X["width_fit"]["intercept"]
fig, ax = plt.subplots(1, 2, figsize=(10, 4))
ds = [p["d"] for p in pts]; ws = [p["width"] for p in pts]
ax[0].plot(ds, ws, "o", color="#2a6fdb", label="measured (min-fill)")
ax[0].plot([3, 10], [b + s * 3, b + s * 10], "-", color="#888", label=f"fit {b:.1f} + {s:.2f}·d")
ax[0].axhline(28, color="#c0392b", ls=":", lw=1)
ax[0].text(3.1, 29.5, "about the dense limit in 15 GB", color="#c0392b", fontsize=8)
ax[0].set_xlabel("field bits d"); ax[0].set_ylabel("contraction width (log2 table entries)")
ax[0].set_title("Relation network width over F_p"); ax[0].legend(fontsize=8)
dd = list(range(3, 132))
ax[1].plot(dd, [b + s * d for d in dd], "-", color="#2a6fdb", label="BTN: one exact contraction (table entries)")
ax[1].plot(dd, [0.5 * math.log2(math.pi * 2 ** d / 4) for d in dd], "-", color="#27ae60",
           label="Pollard rho: whole DLP (group operations)")
ax[1].axvline(131, color="#888", ls=":", lw=1)
ax[1].text(124, 400, "ECCp-131", rotation=90, fontsize=8, color="#555")
ax[1].set_xlabel("field bits d"); ax[1].set_ylabel("log2 operations"); ax[1].set_title("Extrapolated to 131 bits")
ax[1].legend(fontsize=8)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(H, "scaling." + ext), dpi=130)
print("wrote scaling.svg, scaling.png")
