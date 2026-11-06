import json, os
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = os.path.dirname(os.path.abspath(__file__))
rows = json.load(open(os.path.join(here, "certificates_table.json")))
fig, ax = plt.subplots(figsize=(7, 4))
xs = [r["degree"] * r["k0"] for r in rows]
ys = [r["verify_s"] if isinstance(r["verify_s"], float) else 0.5 for r in rows]
ax.scatter(xs, ys, color="#2f6fb0")
for r, x, y in zip(rows, xs, ys):
    ax.annotate(f"ℓ={r['ell']}", (x, y), fontsize=7, xytext=(3, 3), textcoords="offset points")
ax.set_xlabel("extension degree of the certificate's field over GF(p)")
ax.set_ylabel("seconds to check (≤ 0.5 drawn at 0.5)")
ax.set_title("19 depth-1 level certificates: check time in curve_traits build", fontsize=9)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(here, f"verify_time.{ext}"), dpi=120)
