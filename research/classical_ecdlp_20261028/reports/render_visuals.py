#!/usr/bin/env python3
"""Render already-recorded historical counts; no research computations.

Input: ../historical/guess_determine/probability-audit.json (frozen evidence).
The Rust archive replay separately checks both finite count populations.
Only document layout is performed here. No Python algorithm, verifier,
probability model, benchmark, or measurement-generation path is invoked.
"""
import json
import os
from pathlib import Path

os.environ.setdefault("MPLCONFIGDIR", "/tmp/classical-ecdlp-matplotlib")
import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = Path(__file__).resolve().parent
receipt = json.loads(
    (HERE.parent / "historical/guess_determine/probability-audit.json").read_text()
)
rows = receipt["cases"]

plt.rcParams.update({
    "font.family": "DejaVu Sans", "font.size": 10,
    "svg.hashsalt": "classical-ecdlp-archive-20261028",
})
fig, ax = plt.subplots(figsize=(7.2, 3.5), layout="constrained")
positions = list(range(len(rows)))
for offset, field, color, label in [
    (-0.19, "candidate_occurrences", "#536a87", "Candidate occurrences"),
    (0.19, "distinct_candidate_subsets", "#158b83", "Distinct candidate sets"),
]:
    bars = ax.bar(
        [x + offset for x in positions],
        [row[field] for row in rows],
        width=0.35,
        color=color,
        label=label,
    )
    ax.bar_label(bars, padding=3)
ax.set_xticks(positions, [f'h={row["h"]}, d={row["defect"]}' for row in rows])
ax.set_ylabel("Candidate count (exact finite enumeration)")
ax.set_ylim(0, 1950)
ax.set_title("One outer guess: repeated candidates in two finite fixtures", loc="left", pad=15)
ax.legend(frameon=False, loc="upper left")
ax.spines[["top", "right"]].set_visible(False)
ax.grid(axis="y", alpha=0.15)
ax.set_axisbelow(True)
svg_path = HERE / "candidate-counts.svg"
fig.savefig(svg_path, metadata={"Date": None})
svg_path.write_text("\n".join(line.rstrip() for line in svg_path.read_text().splitlines()) + "\n")
plt.close(fig)

# Editable declarative SVG: a source/contribution map, not an empirical graph.
(HERE / "classical-routes.svg").write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="850" height="320" viewBox="0 0 850 320">
<defs><marker id="arrow" markerWidth="7" markerHeight="7" refX="6" refY="3.5" orient="auto"><path d="M0,0 L7,3.5 L0,7" fill="#536a87"/></marker></defs>
<rect width="850" height="320" fill="white"/>
<g font-family="DejaVu Sans,sans-serif" fill="#18283b">
<text x="20" y="26" font-size="17" font-weight="bold">Classical routes and evidence status · 2026-10-28</text>
<g fill="#edf3f7" stroke="#a5b5c4">
<rect x="20" y="48" width="235" height="75" rx="8"/>
<rect x="306" y="48" width="249" height="75" rx="8"/>
<rect x="600" y="48" width="230" height="75" rx="8"/>
<rect x="20" y="154" width="235" height="75" rx="8"/>
<rect x="306" y="154" width="249" height="75" rx="8"/>
<rect x="600" y="154" width="230" height="75" rx="8"/>
</g>
<g font-size="13">
<text x="34" y="71" font-weight="bold">APS · one labelled relation</text>
<text x="34" y="92">Accepted manuscript read</text><text x="34" y="110">Relation search still required</text>
<text x="320" y="71" font-weight="bold">McGuire–Mueller</text>
<text x="320" y="92">Evaluation / signed point sums</text><text x="320" y="110">Six author-code CAS checks</text>
<text x="614" y="71" font-weight="bold">Kudo et al. · hybrid search</text>
<text x="614" y="92">Publisher/precursor abstracts</text><text x="614" y="110">Full chapter access remains open</text>
<text x="34" y="177" font-weight="bold">Adjacent zero-minor route</text>
<text x="34" y="198">Uses matrix algebra</text><text x="34" y="216">Separate from APS acceleration</text>
<text x="320" y="177" font-weight="bold">Guess and determine · 2026</text>
<text x="320" y="198">60 saved known-answer cases</text><text x="320" y="216">Six complete CAS witnesses</text>
<text x="614" y="177" font-weight="bold">Existing native W12 engine</text>
<text x="614" y="198">Rust archive and witness replay</text><text x="614" y="216">Six public-target regressions</text>
</g>
<g stroke="#536a87" stroke-width="2" fill="none" marker-end="url(#arrow)">
<path d="M255,85 H299"/><path d="M137,123 V139 H715 V130"/>
<path d="M255,191 H299"/><path d="M555,191 H593"/>
</g>
<text x="20" y="265" font-size="13">JMC 2020: conditional limits. Lanteri: restricted implementation evidence. MultiSolve: possible solver technology.</text>
<text x="20" y="290" font-size="13" font-weight="bold">Correctness receipts do not establish a runtime advantage. Historical timing metadata are incomplete.</text>
</g></svg>
''')
