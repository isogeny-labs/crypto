#!/usr/bin/env python3
"""Render a frozen Git metadata inventory; no cryptographic computation."""

import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
data = json.loads((HERE / "inventory.json").read_text())
counts = data["counts"]
roots = ("research", "experiments", "benchmarks")
assert sum(counts["by_root"][r]["directories"] for r in roots) == counts["tracked_study_directories"]
assert len(data["study_directories"]) == counts["tracked_study_directories"]
assert len(data["lean_files"]) == counts["lean_files_total"]

scale = 600 / max(counts["by_root"][r]["directories"] for r in roots)
rows = []
for i, root in enumerate(roots):
    row = counts["by_root"][root]
    y = 100 + 75 * i
    yes = round(row["with_colocated_lean"] * scale, 2)
    no = round(row["without_colocated_lean"] * scale, 2)
    rows.append(f'<text x="30" y="{y-10}" font-family="DejaVu Sans" font-size="16" fill="#172b4d">{root}: {row["directories"]} directories; {row["with_colocated_lean"]} with a co-located .lean file</text>')
    rows.append(f'<rect x="30" y="{y}" width="{yes}" height="30" fill="#166a8c"/>')
    rows.append(f'<rect x="{30+yes}" y="{y}" width="{no}" height="30" fill="#b4bdc9"/>')

svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="850" height="390" viewBox="0 0 850 390">
<rect width="850" height="390" fill="#fff"/>
<text x="30" y="35" font-family="DejaVu Sans" font-size="20" fill="#172b4d">Tracked study directories at frozen crypto main</text>
<text x="30" y="62" font-family="DejaVu Sans" font-size="13" fill="#42526e">Git tree {data['source_commit'][:12]} · path co-location only · n={counts['tracked_study_directories']} directories</text>
{''.join(rows)}
<text x="30" y="340" font-family="DejaVu Sans" font-size="14" fill="#166a8c">■ with a co-located .lean file</text>
<text x="30" y="363" font-family="DejaVu Sans" font-size="14" fill="#536273">■ without a co-located .lean file; not a missing-proof count</text>
</svg>
'''
(HERE / "coverage.svg").write_text(svg)

typ = f'''#set page(width: 210mm, height: 297mm, margin: 17mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
= Crypto formal coverage: frozen path inventory
28 October 2026 · source commit `{data['source_commit']}` · tree `{data['source_tree']}`

This is a syntactic navigation aid. It does not certify mathematical claims, Lean builds, experimental evidence, or code correctness. Its chart shows only where tracked Lean files sit relative to study directories.

#image("coverage.svg", width: 100%)

== Method and observations
`tools/formal_coverage_inventory.py` reads the complete frozen Git tree with `git ls-tree -r -z` and all tracked `.lean` blobs with `git cat-file --batch`. The 165,588 tracked blobs and one Git submodule entry are recorded separately. The submodule contents are not traversed. `inventory.json` records exact paths, blob IDs, and regex-derived theorem or lemma line pointers.

- {counts['tracked_study_directories']} study directories: {counts['by_root']['research']['directories']} research, {counts['by_root']['experiments']['directories']} experiments, {counts['by_root']['benchmarks']['directories']} benchmarks.
- {counts['lean_files_total']} `.lean` files in the repository; {counts['lean_files_under_study_roots']} under those study roots.
- {counts['tracked_study_directories_with_colocated_lean']} study directories with a co-located `.lean` file; {counts['tracked_study_directories_without_colocated_lean']} without one.
- {counts['lexical_theorem_lemma_pointers']} lexical declaration pointers. The regex can include commented text or omit other declaration forms.

== Interpretation and next gate
Directory co-location is not a claim-to-theorem map. A benchmark may contain no theorem to prove; a theorem may live in a shared library or another repository. Review each substantive written claim against its exact Lean statement and hypotheses, run the pinned theorem build and axiom audit, then independently review the underlying experiment. Do not turn these path counts into a proof-coverage percentage.

== Reproduction
From a checkout containing the frozen commit, run `python3 tools/formal_coverage_inventory.py {data['source_commit']} docs/reports/2026-10-28-formal-coverage/inventory.json`, then `python3 docs/reports/2026-10-28-formal-coverage/render.py`. This metadata script does not run cryptographic research or performance measurements.
'''
(HERE / "report.typ").write_text(typ)
print(f"rendered {HERE / 'coverage.svg'} and {HERE / 'report.typ'}")
