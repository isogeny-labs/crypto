"""Layout only: draw already-priced representative phase fractions from phase-grid.csv."""
import csv
from pathlib import Path
from xml.sax.saxutils import escape

HERE = Path(__file__).resolve().parent
with (HERE / "phase-grid.csv").open(newline="") as handle:
    rows = list(csv.DictReader(handle))
picked = {}
for row in rows:
    if (row["method"], row["S_per_M"], row["I_per_M"], row["native_GAE_per_logical"]) != ("candidate", "0.5", "10", "0.1"):
        continue
    if row["slice"] != ("p739" if row["lead"] == "pencil_collector" else "all"):
        continue
    picked.setdefault((row["lead"], row["arm"]), []).append(row)

width, height = 1150, 130 + 43 * len(picked)
parts = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
         '<rect width="100%" height="100%" fill="white"/>',
         '<text x="20" y="32" font-family="sans-serif" font-size="22">Candidate cold-cost phase fractions</text>',
         '<text x="20" y="54" font-family="sans-serif" font-size="13">Representative price: S=0.5 M, I=10 M, native=0.1 GAE/logical; full cohort except pencil p=739 (60 targets)</text>']
colors = ["#225ea8", "#41b6c4", "#7fcdbb", "#c7e9b4"]
short = {"relations_and_signed_elimination": "relations",
         "native_logical_unallocated": "unallocated",
         "unallocated_setup_and_solver": "unallocated",
         "collector_logical_unallocated": "unallocated",
         "candidate_verification": "candidate verify",
         "walk_generation": "walk",
         "recover_ops": "recover",
         "walk_ops": "walk"}
for i, ((lead, arm), items) in enumerate(sorted(picked.items())):
    y = 81 + 43 * i
    items.sort(key=lambda r: float(r["phase_fraction"]), reverse=True)
    parts.append(f'<text x="20" y="{y+17}" font-family="sans-serif" font-size="16">{escape(lead)} / {escape(arm)}</text>')
    x = 310
    for j, row in enumerate(items[:4]):
        fraction = float(row["phase_fraction"])
        w = 780 * fraction
        parts.append(f'<rect x="{x:.2f}" y="{y}" width="{w:.2f}" height="25" fill="{colors[j]}"/>')
        if w > 100:
            label = f'{short.get(row["phase"], row["phase"])} {fraction:.1%}'
            parts.append(f'<text x="{x+4:.2f}" y="{y+17}" font-family="sans-serif" font-size="15" fill="#102020">{escape(label)}</text>')
        x += w
    parts.append(f'<rect x="{x:.2f}" y="{y}" width="{max(0,1090-x):.2f}" height="25" fill="#d9d9d9"/>')
    parts.append(f'<text x="1100" y="{y+17}" font-family="sans-serif" font-size="14">n={items[0]["targets"]}</text>')
parts.append(f'<text x="20" y="{height-24}" font-family="sans-serif" font-size="12">Each bar sums to 100%; gray combines remaining source-named phases. Source: phase-grid.csv, phase-accounting/v1.</text>')
parts.append('</svg>')
(HERE / "phase-fractions.svg").write_text("\n".join(parts) + "\n")
