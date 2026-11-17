import json, os
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = os.path.dirname(os.path.abspath(__file__))
rows = json.load(open(os.path.join(here, "levels.json")))
fig, ax = plt.subplots(figsize=(8, 3.8))
xs = range(len(rows))
ax.bar(xs, [r["bound_before"] for r in rows], color="#d9d9d9", label="bound before (level at most)")
ax.scatter(xs, [r["level"] for r in rows], color="#2f6fb0", zorder=3, label="level from descending walks (= PARI)")
ax.set_xticks(list(xs))
ax.set_xticklabels([f"{r['slug'][5:9]} ℓ={r['ell']}" for r in rows], rotation=70, fontsize=6)
ax.set_ylabel("volcano level v_ℓ(f)")
ax.set_title("17 open End(E) levels on 47- and 53-bit prime-field curves (depth 12-24)", fontsize=9)
ax.legend(fontsize=7)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(here, f"levels.{ext}"), dpi=150)
