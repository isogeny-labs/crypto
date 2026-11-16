"""Lay out coverage.csv as a status matrix (families x field types). Plotting only."""
import csv, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.patches import Patch

rows = list(csv.DictReader(open("coverage.csv")))
cols = ["prime_field", "koblitz_prime_n", "binary_random_prime_n", "extension_small_n"]
collab = ["prime field F_p", "Koblitz, n prime", "random binary, n prime", "extension F_{q^n}, small n"]

def status(s):
    s = s.lower()
    if s.startswith("not applicable"): return 0
    if s.startswith("closed") or s.startswith("formula negative") or "measured negative" in s or s.startswith("measured (\"exhausted") : return 1
    if s.startswith("measured positive") or s.startswith("measured 31") or s.startswith("measured 30") or s.startswith("measured 416"): return 2
    if s.startswith("measured") or s.startswith("audited") or s.startswith("inconclusive") or s.startswith("geometry") or s.startswith("literature") or s.startswith("never"): return 3
    if s.startswith("open") or s.startswith("planned") or s.startswith("not run") or s.startswith("kernel") or s.startswith("not enumerated") or s.startswith("not examined"): return 4
    if s.startswith("new"): return 5
    return 3

colors = {0: "#f0f0f0", 1: "#9e9e9e", 2: "#4c9a2a", 3: "#7fb3d5", 4: "#f2c14e", 5: "#d1495b"}
labels = {0: "not applicable", 1: "closed / measured negative", 2: "measured positive (toy or weak class)",
          3: "measured or audited, no exponent change", 4: "open: planned, blocked or never examined", 5: "new in this search"}

fig, ax = plt.subplots(figsize=(11, 0.42 * len(rows) + 1.6))
for i, r in enumerate(rows):
    for j, c in enumerate(cols):
        st = status(r[c])
        ax.add_patch(plt.Rectangle((j, len(rows) - 1 - i), 1, 1, color=colors[st], ec="white"))
        txt = r[c]
        if len(txt) > 34: txt = txt[:33] + "…"
        ax.text(j + 0.5, len(rows) - 1 - i + 0.5, txt, ha="center", va="center", fontsize=6.2)
ax.set_xlim(0, len(cols)); ax.set_ylim(0, len(rows))
ax.set_xticks([j + 0.5 for j in range(len(cols))]); ax.set_xticklabels(collab, fontsize=8)
ax.set_yticks([len(rows) - 1 - i + 0.5 for i in range(len(rows))]); ax.set_yticklabels([r["family"] for r in rows], fontsize=7.5)
ax.tick_params(length=0)
for s in ax.spines.values(): s.set_visible(False)
ax.set_title("Cover and decomposition routes: coverage in aburan28/crypto as of 2026-10-18\n(status read from the repository inventory, appendices A and B; literature from C, D, E)", fontsize=9)
ax.legend(handles=[Patch(color=colors[k], label=labels[k]) for k in colors], loc="upper center", bbox_to_anchor=(0.5, -0.03), ncol=3, fontsize=7, frameon=False)
fig.tight_layout()
fig.savefig("coverage.svg"); fig.savefig("coverage.png", dpi=150)
print("ok", len(rows))
