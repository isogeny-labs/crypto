import json, os
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
s = json.load(open(os.path.join(here, "summary.json")))
rows = sorted(s["composites"], key=lambda r: r["digits"])
colour = {"FF": "#2f6fb0", "C": "#c0504d"}
fig, ax = plt.subplots(figsize=(7, 3.6))
for i, r in enumerate(rows):
    st = r["factordb_status"]
    c = "#3a9a5b" if st.startswith("C (") else colour.get(st, "#999999")
    ax.bar(i, r["digits"], color=c)
ax.set_xticks([])
ax.set_xlabel("unsplit composites from curve_traits, by size")
ax.set_ylabel("decimal digits")
ax.set_title("Composites checked against factordb (blue: fully factored there; "
             "green: C112, split here; red: still composite)", fontsize=8)
fig.tight_layout()
for ext in ("svg", "png"):
    fig.savefig(os.path.join(here, f"composites.{ext}"), dpi=150)
