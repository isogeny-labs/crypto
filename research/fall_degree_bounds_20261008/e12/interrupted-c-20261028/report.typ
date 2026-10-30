#set page(paper: "a4", margin: 14mm)
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false)
#set heading(numbering: none)
= E12c: interrupted Spot continuation
_Observed partial records and durable storage · 28 October 2026_

One of four requested records completed for $(m,n,ell)=(4,28,7)$, $N=84$,
seed 20261012. Its restricted degree-4 window was uncertified. This is an
observational preservation report, not independent closure verification,
a last-fall-degree lower bound, or an official hypothesis decision.

#image("flow.svg", width: 100%)

#table(columns: (1.3fr, 2fr), inset: 3pt,
  table.header([Recorded field], [Value]),
  [Completed records], [1 of 4; draw 0],
  [Solutions], [0, producer reported],
  [Window certification], [`false`],
  [Degree-result fields], [`d_f_window=null`; `d_f_le3=null`],
  [Window dimension / columns], [447,015 / 817,860, producer reported],
  [Terminal state], [Spot reclaim; interrupted; exit 143],
)

== Execution and interruption
The wide-variable hky_window binary ran
`newcells --cells 4:28:7 --seed 20261012 --keep 4 --policies w1x`.
The r7i.4xlarge Spot host in us-east-1 had 128 GiB allocated and 16 recorded
threads. Producer logs record passing gates and five empty comparison diffs.
The output status contains only `gate_passed`, with no final `done`; cells_status
is absent. Gates were inspected, not replayed.

The log records a Spot notice at 07:21:44 UTC, exit 143, upload, then shutdown.
The terminal receipt was updated at 07:22:48 UTC. Three requested completed
records are missing; their individual attempt states are unknown. No resumable
solver-memory checkpoint or automatic replacement is supplied by Amendment 13.

== Meaning and limits
The window is a restricted subspace: reaching 1 can certify an upper bound;
failure does not establish failure of the unrestricted degree-4 closure.
The saved row gives no degree-at-least-5 conclusion. The frozen E12c rule needs
two finished degree-4 certifications for its positive reading; this attempt has
none, leaving “no reading.” No exact degree, unbounded growth, first-fall
conclusion, Lean proof, performance comparison or hypothesis promotion is claimed.

== Durable record and remaining checks
The 7,678-byte archive has 24 regular output files plus host.txt and rusage.txt:
26 files total. All 24 final output objects matched the archive. Original and
canonical downloads passed hash checks; canonical PUT/GET and byte-comparison
receipts are retained. The raw 775-byte JSONL, factor-base field basis, target,
identities and terminal/snapshot/readback receipts accompany this report.
These output snapshots omit the roughly 30 GB closure basis and full in-flight
state; they cannot resume an interrupted closure exactly. Complete intermediate
persistence remains a gap. No restart or retry is authorized here.

Formal independent experiment review, solution recount and closure replay were
not performed. No schema-validated experiment.export/v1 bundle was supplied by
the historical runner; this report does not claim that gate. Earlier reports
and unrestricted degree plots remain unchanged. Three E12c records, E12a and
the general fall-degree questions remain unfinished.

#text(size: 7.5pt)[
Job: 20261028-021036-fd-e12c-20261028-r3-914d46. \
Source: 371035216942660c4615dfbb88d85a14afaf5c16. \
Protocol: 640b58b211e464939f077ef3722f3f4c00dd8bbb, Amendments 11/13. \
Archive: fa4edf72399ec45b293b783d8582d9a26406c1670a3612a75b6110dd1e824f09. \
Raw row: 4370d536c0a53743b4098926be2479bea8453be91204204f50d7a94dfbfe9ea6. \
]
