#!/usr/bin/env python3
"""Write REPORT.typ from the committed result JSON and compile REPORT.pdf (typst wheel; document rendering only).
usage: build_pdf.py DIR"""
import json, sys
from pathlib import Path
import typst
d = Path(sys.argv[1])
Bv = json.load(open(d / "B/validator-report.json")); Cv = json.load(open(d / "C/validator-report.json"))
g = json.load(open(d / "raw/replay_all/grouped.json"))
cB = json.load(open(d / "B/replay-certificate.json")); cC = json.load(open(d / "C/replay-certificate.json"))
def esc(s): return s.replace("_", "\\_").replace("#", "\\#").replace("*", "\\*").replace("$", "\\$").replace("@", "\\@")
rowsB = "\n".join(f'  [B], [{esc(r["slug"])}], [pass], [{r["analysis"]["S_IC"]:.4g}], [{r["analysis"]["S_ref"]:.4g}], [{r["analysis"]["x_reference"]:.2f}], '
                  f'[{min(b["ratio"] for b in r["analysis"]["band"]):.2f} to {max(b["ratio"] for b in r["analysis"]["band"]):.2f}],' for r in Bv["rows"])
rowsC = "\n".join(f'  [C], [{esc(r["slug"])}], [fail R4], [{r["recomputed"]["S_IC"]:.4g}], [{r["recomputed"]["S_ref"]:.4g}], [{r["recomputed"]["x_reference"]:.2f}], [no band],' for r in Cv["rows"])
src = f'''#set page(paper: "a4", margin: 16mm)
#set text(size: 9pt)
#set heading(numbering: none)
= IC validation gate, Tables B and C (2026-10-24, Conductor T-2)

*Question.* Can the existing rows of leaderboard Tables B and C pass the validation gate (replay certificate, validator report,
matched rho reference with the section 5a band)? Decision rule R1 to R4 frozen in PROTOCOL.md before any replay.
Nothing here is an improvement; it decides which numbers may be shown.

*Result (measured and derived, this study).*
- Table B: 13 of 13 rows pass R1 to R4. {cB["counts"]["runs_compared"]} counted runs replay identically in a fresh process; {cB["counts"]["scalar_runs_checked_archive"]} archive and {cB["counts"]["scalar_runs_checked_replay"]} replay scalar runs verify as [d]G = Q in an independent verifier ({cB["counts"]["negative_control_rejected"]} of {cB["counts"]["negative_control_total"]} negative controls rejected); all 52 headline numbers recompute to 1e-9. IC stays above rho at every weight of the section 5a band. The manifest is staged for docs/ic/runs (outside this task's write scope).
- Table C: 0 of 9 pass. R1 holds ({cC["runs_identical"]} of {cC["runs_replayed"]} sampled replays identical), R3 holds (36 numbers), R2 cannot be checked (no target points archived) and R4 fails: the reference is a different study's batch rho, not a same-process arm, and no unpriced counts exist for a band.
- Gap to matched rho (distance remaining): Table B 5.47x to 38.21x (4.05x to 54.76x across the band); Table C 3.62x to 14.90x.

#figure(image("figures/row_outcome.svg", width: 100%), caption: [Per-row outcome of R1 to R4 (recorded verdicts; x reference from the board).])

#pagebreak()
== Per row (S in GAE per sqrt r for B, units per sqrt r per target for C)
#table(columns: (auto, 1fr, auto, auto, auto, auto, auto), align: (left, left, left, right, right, right, right), inset: 3pt, stroke: 0.4pt,
  [table], [row], [standing], [S_IC], [S_ref], [x ref], [IC/rho over band w = 0, 0.1, 1],
{rowsB}
{rowsC}
)

== The 96 root-finder replay mismatches
`ecbench verify --replay-all` on session ECBS1hfd5bf1885227, current binary: {g["identical"]} of {g["replayed"]} identical, {g["differs"]} differ, same scalars. By arm: ic-f4-m2 24, ic-f4-m3 24, ic-rootfind-m2 24, ic-rootfind-m3 24 differ; rho-strong, rho-strong-aa, rho-frob 24 of 24 identical each.
Field level (each flagged record re-executed and diffed): the 48 F4 rows differ in relations.native.solver\\_ops (m2 300,531 to 301,961; m3 2,832,171 to 3,732,225) and are repriced (word XORs now charged). The 48 root-finder rows differ only by two new counters in oracle\\_setup (pair\\_table\\_kind 0, pair\\_table\\_points 33); every existing counter, every GAE and every scalar is equal.
So the statement that the 96 are inherited-F4 rows and not root-finder rows is refuted as stated (half are root-finder rows), while the substantive point stands: no root-finder number changes. Table B and C rows read ic boundary and ic price oracle counts, which R1 re-executes identically; they are not affected.

== Limits
Binaries were built from the working tree with other agents' uncommitted edits (koblitz\\_index\\_calculus.rs, ic price.rs, workflow.rs); all replays agree with the archive and with sweep A's clean-build count (72 of 168). Table C scalars are not independently checked. Table A and the v4 run are not covered here. L0 shared host; no timing is used for B.
'''
(d / "REPORT.typ").write_text(src)
typst.compile(str(d / "REPORT.typ"), output=str(d / "REPORT.pdf"))
print("ok")
