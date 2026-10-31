"""Layout only: plots leads.json (cost tier vs expected effect). No computation of results."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
d = json.load(open(os.path.join(here, "leads.json")))
# hand-placed offsets so points sharing a cell do not overlap
slots = {}
fig, ax = plt.subplots(figsize=(11, 5.4))
for L in d["leads"]:
    key = (L["cost"], L["effect"])
    k = slots.get(key, 0); slots[key] = k + 1
    x = L["cost"] + 0.22 * (k % 3) - 0.22
    y = L["effect"] + 0.2 * (k // 3) * 0 + (0.0)
    y = L["effect"] + [0.22, 0.0, -0.22][k % 3] * (1 if k < 3 else 1) + (0.0 if k < 3 else 0.0)
    x = L["cost"] - 0.15 + 0.3 * k
    y = L["effect"]
    face = "#1f6f8b" if L["ranked"] else "#999999"
    mk = "o" if L["verdict"] == "keep" else "s"
    ax.scatter([x], [y], s=150, c=face, marker=mk, edgecolors="black", zorder=3)
    ax.annotate(L["id"], (x, y), xytext=(12, -4), textcoords="offset points", fontsize=9)
ax.set_xlim(0.5, 3.6); ax.set_ylim(-0.6, 2.7)
ax.set_xticks([1, 2, 3]); ax.set_xticklabels(["tier 1\nminutes-1 h", "tier 2\nhours, 16 GB serial", "tier 3\ndays / big machine"], fontsize=8)
ax.set_yticks([0, 1, 2]); ax.set_yticklabels(["0 accounting /\nreference", "1 stage\ndiagnostic", "2 possible measured\nIC improvement"], fontsize=8)
ax.set_xlabel("cost tier (planning judgement, not measured)")
ax.set_ylabel("best-case effect class (judgement)")
ax.set_title("ECC2K-130 leads by cost tier vs best-case effect", fontsize=10)
ax.scatter([], [], c="#1f6f8b", marker="o", edgecolors="black", label="ranked, skeptic: keep")
ax.scatter([], [], c="#1f6f8b", marker="s", edgecolors="black", label="ranked, skeptic: rescope")
ax.scatter([], [], c="#999999", marker="s", edgecolors="black", label="not ranked (accounting), skeptic: rescope")
ax.legend(loc="upper left", fontsize=7, framealpha=0.95)
ax.grid(alpha=0.3)
key = "\n".join(f'{L["id"]}  {L["short"]}' for L in d["leads"])
fig.text(0.70, 0.5, key, fontsize=7, va="center", family="monospace")
fig.subplots_adjust(right=0.68, left=0.18, bottom=0.17)
fig.savefig(os.path.join(here, "leads_cost_vs_effect.svg"))
fig.savefig(os.path.join(here, "leads_cost_vs_effect.png"), dpi=150)
