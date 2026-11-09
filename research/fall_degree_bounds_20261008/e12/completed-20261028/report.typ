#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false)
#set heading(numbering: none)
= E12b: completed Spot continuation
_Observed records and frozen scoring · 28 October 2026_

This report records solver outputs and applies a frozen arithmetic rule. It is
not an independent closure replay, hypothesis promotion, or proof of a general
fall-degree bound. Four draws completed for $(m,n,ell)=(3,24,7)$, $N=45$,
48 equations, seed 20261012 and degree cap 4.

#image("draws.svg", width: 100%)
#text(size: 8pt)[Four recorded labels; upward arrows denote censoring, not exact degree 5.
Job: 20261028-031317-fd-e12b-spot-20261028-r3-24148b.
Raw SHA-256: 055811b722f441ab681172a605f9a540b174734dbb4f23ff9f03b931d948c81b.]

#table(columns: (1fr, 1.4fr, 2.4fr, 1fr), inset: 6pt,
  table.header([Draw], [Recorded label], [Degree-4 dimension / columns], [Rounds]),
  [0], [≥5], [157926 / 164221], [9],
  [1], [4], [164221 / 164221], [8],
  [2], [≥5], [158254 / 164221], [9],
  [3], [4], [164221 / 164221], [7],
)

== Frozen reading: “mixed”
Amendments 12/15 score censored rows as cap plus one and take the second-smallest
of four values. Sorting $[5,4,5,4]$ gives $[4,4,5,5]$, so the lower median is 4.
Two rows score at least 5; the rule requires at least three. The other three
carried-forward r2 cells each retain four values of 4. Earlier partial draw 0 is
an identity reference, not an additional sample. No significance test is claimed.

== Execution and checks
Four requested draws completed; none failed or were interrupted in this attempt.
All record zero solutions. The m7i.4xlarge Spot host had 16 vCPU and 64 GiB in
us-east-1. Job status records exit 0; the completed instance terminated.

Gate logs record three counter tests, 24 gate-1 agreements, 16 gate-2/3 matching
rows, and draw-0 equality to r2 after removing only seconds fields. A separate
record audit checked these against frozen references, without executing a solver.
Diagnostic solver intervals sum to 1203.0 seconds, excluding build and gates.
The whole-command rusage records 24916.64 user CPU seconds, 505.06 system CPU
seconds and 30:57.68 elapsed, including build and gates. Timing is not scored or compared.

== Limits and provenance
The two censored degrees remain unknown above the cap. Record consistency does
not independently establish unsatisfiability, saturation, or engine correctness.
No new Lean verification is claimed; the prior 282-row table excludes these rows.
General first-fall assumptions and asymptotic last-fall bounds remain unresolved.

#text(size: 8pt)[
Source commit: 371035216942660c4615dfbb88d85a14afaf5c16. \
Frozen protocol: 020f03193c46d975935a065c966c81941cbdbf6d, Amendments 12/15. \
Archive SHA-256: b06c3745486ecf1e2e5fce7818e1495de06cb4bcfaa09e644098db379d235d9d. \
See accompanying README.md, provenance.json and verification.json for S3 location,
exact commands, identities, gates and limitations. verify_records.py reproduces
archive consistency and scoring; plot.py regenerates this graph from the raw rows.
]
