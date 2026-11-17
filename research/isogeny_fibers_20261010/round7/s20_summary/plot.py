"""Plot the verdict matrix of section 20 from verdicts.json (recorded verdicts only; no computation)."""
import json, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = pathlib.Path(__file__).resolve().parent
d = json.loads((here / "verdicts.json").read_text())
cols = d["columns"]
leads = d["leads"]
# same categorical slots and marker shapes as round6/s19_summary/plot.py
colors = ["#2a78d6", "#6250d6", "#eb6834", "#6b6a66", "#0b8a60"]
markers = ["o", "s", "^", "D", "P"]
ink, ink2, grid = "#0b0b0b", "#52514e", "#e4e3df"

plt.rcParams["svg.fonttype"] = "none"
plt.rcParams["font.family"] = "DejaVu Sans"
fig, ax = plt.subplots(figsize=(12.5, 5.2))
SP = 1.35
HX = len(cols) * SP - 0.45
n = len(leads)
for i, L in enumerate(leads):
    y = n - 1 - i
    c = L["col"]
    ax.plot([-0.6, HX - 0.1], [y, y], color=grid, lw=0.8, zorder=0)
    face = colors[c] if L["merged"] else "white"
    ax.scatter([c * SP], [y], s=170, marker=markers[c], facecolor=face, edgecolor=colors[c], linewidth=2, zorder=3)
    if not L["merged"]:
        ax.text(c * SP, y - 0.42, "UNMERGED", ha="center", va="center", fontsize=6.5, color=ink2)
    ax.text(-0.62, y, f'{L["lead"]}  {L["title"]}', ha="right", va="center", fontsize=9, color=ink)
    ax.text(HX, y, f'#{L["pr"]}: {L["headline"]}', ha="left", va="center", fontsize=7.4, color=ink2)
ax.set_xlim(-0.6, HX + 5.6)
ax.set_ylim(-0.9, n - 0.4)
ax.set_xticks([i * SP for i in range(len(cols))])
ax.set_xticklabels(["Falsifier\nnot fired", "Mixed:\nparts differ", "Falsifier fired\n/ refuted", "Open:\nnot decided", "No falsifier\n(errata, library)"], fontsize=8, color=ink)
ax.xaxis.tick_top()
ax.tick_params(axis="x", length=0, pad=6)
ax.set_yticks([])
for s in ax.spines.values():
    s.set_visible(False)
ax.text(HX, n - 0.1, "PR status and source headline", fontsize=8, color=ink, ha="left", va="bottom", weight="bold")
fig.text(0.01, 0.01,
         "Hollow marker: pull request open and UNMERGED; filled: merged. Status checked 2026-11-01 04:11 UTC; five merged (including R7.8 #3125 at 03:58 UTC), three open. One verdict class per lead; see section 20.9.\n"
         "R7.8 has no separate research verdict. R7.1: the ordering part of the falsifier fires in one accounting, the exponent part does not. Toy instances only. Source: verdicts.json.",
         fontsize=7.2, color=ink2, ha="left")
fig.subplots_adjust(left=0.22, right=0.995, top=0.86, bottom=0.11)
svg_path = here / "verdicts.svg"
fig.savefig(svg_path)
# Matplotlib leaves trailing spaces on some generated path lines; keep the
# committed derived SVG clean under git diff --check.
svg_path.write_text("\n".join(line.rstrip() for line in svg_path.read_text().splitlines()) + "\n")
