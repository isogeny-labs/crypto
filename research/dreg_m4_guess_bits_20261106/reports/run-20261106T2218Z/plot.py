"""Layout only.  Draws plot-data.json, which the native analyser produces from
the committed records:

    target/release/examples/dreg_m4_guess_bits --plot-data score research > plot-data.json

Nothing is classified or counted here: the essential sets, per-guess counts
and phase-2 tallies are fields computed in Rust."""
import json
import os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
data = json.load(open(os.path.join(here, "plot-data.json")))
cells = data["cells"]
names = ["1·a", "1·b", "1·c", "1·d", "2·a", "2·b", "2·c", "3·a", "3·b", "3·c"]  # link · slot; arm I: s coordinates, then e₁+x₃, e₂+x₄; arm R: x₂, x₃, x₄ coordinates

fig, ax = plt.subplots(figsize=(10, 6.4))
for row, c in enumerate(cells):
    for i in range(10):
        colour = "#2e86ab" if i in c["essential"] else "#eeeeee"
        ax.add_patch(plt.Rectangle((i, row), 0.95, 0.9, color=colour))
    p2 = c["phase2"]
    if p2:
        note = f"E only: {p2['reduced_refuting']}/{p2['assignments']} refute; kept {p2['kept']}, lost {p2['lost']}"
    else:
        note = "E empty (f constant)"
    ax.text(10.2, row + 0.45, f"{c['refuting']}/1024 refute · {note}", va="center", fontsize=7)
ax.set_xlim(0, 17.5)
ax.set_ylim(len(cells), 0)
ax.set_xticks([i + 0.47 for i in range(10)])
ax.set_xticklabels(names, fontsize=7)
ax.set_yticks([r + 0.45 for r in range(len(cells))])
ax.set_yticklabels([f"{c['arm']} d{c['draw']}" for c in cells], fontsize=7)
for x in (4, 7):
    ax.axvline(x - 0.025, color="black", linewidth=0.8)
ax.text(2, -0.4, "link 1", ha="center", fontsize=8)
ax.text(5.5, -0.4, "link 2", ha="center", fontsize=8)
ax.text(8.5, -0.4, "link 3", ha="center", fontsize=8)
ax.set_title(
    "m = 4, ℓ = 5, k = 10, degree 3: guesses that decide refutation (blue)\n"
    f"essential on links 1/2/3 — arm I {data['by_link_i']}, arm R {data['by_link_r']}; "
    f"guessing only E keeps every refutation on {data['phase2_all_kept']} of {data['phase2_tested']}",
    fontsize=9, pad=18)
for s in ("top", "right", "left", "bottom"):
    ax.spines[s].set_visible(False)
fig.tight_layout()
fig.savefig(os.path.join(here, "essential.svg"))
fig.savefig(os.path.join(here, "essential.png"), dpi=150)
