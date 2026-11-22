#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set table(inset: 4pt, stroke: 0.4pt + gray)
#show table: set text(size: 7.5pt)
= Flat-presentation exponent study at m \= 3 and m \= 4, with wall time
as a co-primary unit (run 2026-11-15 to 2026-11-22)
<flat-presentation-exponent-study-at-m--3-and-m--4-with-wall-time-as-a-co-primary-unit-run-2026-11-15-to-2026-11-22>
#strong[Class.] An engine measurement in two units: counted F\_p
multiplications of the F4 row reduction (`F4Report.field_ops`, the
parent\'s T) and per-target wall time (W), from
`curve25519_family f4sub` with the flat subgroup presentation
(`subflat`: one variable y per summand with y^D \= 1). It measures the
cost T(E, m, L) of one symmetrised decomposition system against the base
size |V| \= D under the parent\'s frozen estimator and lever rules
(#link("../curve25519_exponent_study_20261108/README.md")[`../curve25519_exponent_study_20261108/`],
section 5). It is not an end-to-end solve, not an ecbench session, and
not a leaderboard or `benchmarks/` row. No pcc claim was registered;
every measured number below is an unverified assertion. Rho remains
ahead on every track (imported from the studies cited under Distance
remaining; this study has no rho arm) \[pcc:unverified\].

#strong[Question (owner, 2026-11-15: \"Ok do it\").] The exponent
studies left the m \= 3 and m \= 4 verdicts \"undetermined\" because the
chain presentation censored the series before the estimator could apply.
Wave-6 lead L3 showed the flat presentation 10 to 360x cheaper in
counted operations at m \= 3. Does the flat presentation let the
preregistered estimator apply at m \= 3 and m \= 4, what exponent does
it give in each unit, and do the two units agree? Protocol frozen before
data: #link("PROTOCOL.md")[`PROTOCOL.md`] (commit a297c73259).

#strong[Answer.] The m \= 3 verdict moves from \"undetermined\" to
#strong[no lever]: the preregistered estimator applies for the first
time at m \= 3 (13 complete cells, D \= 9 to 57, span 6.3x) and gives
s\_T \= 4.60 \[4.40, 4.79\] in the counted unit, far above the lever
threshold s \< 0, and s\_W \= 4.98 \[4.39, 5.56\] in wall time
\[pcc:unverified\]. The m \= 4 verdict stays \"undetermined\": the flat
presentation completes D \= 12 and 14 (the chain did not) but censors at
D \= 18 on memory, so the D \>\= 9 cells span only 1.6x; the two-point
slopes among them are 11.0 and 14.1 and rising \[pcc:unverified\]. Five
of the six predictions held \[pcc:unverified\]. P2 failed: the wall-time
slope exceeds the counted slope by 0.38, not by the predicted 0.5 or
more \[pcc:unverified\]. Post-hoc, not preregistered and not tested: the
wall-time excess appears concentrated at D \>\= 42 (per-operation time
stays between 6.0 and 9.8 ns for D \<\= 38 without a trend, is 11.2 ns
at D \= 42 and 32.1 ns at D \= 54, and 44.0 ns at the load-flagged,
second-container D \= 57); whether a power law or a bend describes it
better is a hypothesis for a future preregistered test
\[pcc:unverified\].

Distance remaining. This measurement changes no leaderboard row and no
cost against rho. On this route the counted cost of one system grows
faster than |V|^4 at m \= 3, so the parent\'s derived balance 2/(m + 1 -
s) has no admissible value (s \> m + 1): under the parent\'s
relation-count model, enlarging the base does not pay at m \= 3
(derived, not measured: s fitted over D \= 9..57 at k \= 20 and
extrapolated by the parent\'s model) \[pcc:unverified\]. This study has
no rho arm; the distance of the algebraic track is imported from
`research/curve25519_ic_baseline_20261102/README.md` as cited in
`research/curve25519_leads_wave5_20261112/lanes/pipeline.md`: 7.8x,
10.6x, 18.1x and 24.7x rho-neg at k \= 20, 24, 28, 32, unchanged by this
study \[pcc:unverified\].

== Cells (k \= 20, p \= 1,048,573, A \= 926; N \= 6 targets per cell;
binary 5abc70ba)
<cells-k--20-p--1048573-a--926-n--6-targets-per-cell-binary-5abc70ba>
T \= mean `field_ops` per target; W \= mean wall ms per target (engine
`ms`); SD \= highest productive pair degree; wall s \= whole-cell wall
time (6 targets, including setup); chain T \= the recorded chain cell at
the same (m, D) from the exponent study and its extension (frozen binary
dc2c30a4, same seeds). Full table with loads and container:
#link("analysis.md")[`analysis.md`]; machine-readable:
#link("analysis.json")[`analysis.json`]; figure:
#link("flat_exponent.svg")[`flat_exponent.svg`].

#align(center)[#table(
  columns: 9,
  align: (col, row) => (auto,auto,auto,auto,auto,auto,auto,auto,auto,).at(col),
  inset: 6pt,
  [m], [D], [T], [W ms], [ns per op], [SD], [wall s], [chain T
  (recorded)], [chain / flat T],
  [3],
  [9],
  [7.589e6],
  [74.6],
  [9.8],
  [10],
  [0],
  [1.476e8],
  [19.5],
  [3],
  [12],
  [2.726e7],
  [198],
  [7.3],
  [13],
  [1],
  [7.748e8],
  [28.4],
  [3],
  [14],
  [4.686e7],
  [347],
  [7.4],
  [15],
  [2],
  [7.766e8],
  [16.6],
  [3],
  [18],
  [1.424e8],
  [950],
  [6.7],
  [19],
  [6],
  [2.428e10],
  [170.5],
  [3],
  [19],
  [2.204e8],
  [1,331],
  [6.0],
  [20],
  [8],
  [2.204e8],
  [1.0],
  [3],
  [21],
  [2.649e8],
  [1,795],
  [6.8],
  [22],
  [11],
  [2.788e9],
  [10.5],
  [3],
  [27],
  [7.266e8],
  [5,864],
  [8.1],
  [28],
  [36],
  [2.635e11],
  [362.6],
  [3],
  [28],
  [1.003e9],
  [6,966],
  [6.9],
  [29],
  [43],
  [2.016e11],
  [201.0],
  [3],
  [36],
  [4.133e9],
  [29,780],
  [7.2],
  [37],
  [180],
  [censored],
  [],
  [3],
  [38],
  [6.720e9],
  [44,990],
  [6.7],
  [39],
  [272],
  [not run],
  [],
  [3],
  [42],
  [8.231e9],
  [92,510],
  [11.2],
  [43],
  [558],
  [not run],
  [],
  [3],
  [54],
  [2.506e10],
  [804,100],
  [32.1],
  [55],
  [4,832],
  [not run],
  [],
  [3],
  [57],
  [3.168e10],
  [#strike[1,393,000] (load-flagged, excluded from the wall fit)],
  [44.0],
  [58],
  [8,367],
  [censored],
  [],
  [3],
  [63],
  [censored: allocation failure (exit -6) after 13,904 s],
  [],
  [],
  [],
  [13,904],
  [not run],
  [],
  [4],
  [3],
  [4.233e5],
  [9.15],
  [21.6],
  [8],
  [2],
  [4.233e5],
  [1.0],
  [4],
  [7],
  [2.498e7],
  [369],
  [14.8],
  [8],
  [4],
  [2.498e7],
  [1.0],
  [4],
  [9],
  [1.333e8],
  [1,432],
  [10.7],
  [10],
  [11],
  [1.421e11],
  [1,065.8],
  [4],
  [12],
  [3.133e9],
  [17,830],
  [5.7],
  [13],
  [110],
  [censored],
  [],
  [4],
  [14],
  [2.738e10],
  [124,100],
  [4.5],
  [15],
  [750],
  [not run],
  [],
  [4],
  [18],
  [censored: allocation failure (exit -6) after 1,287 s],
  [],
  [],
  [],
  [1,287],
  [not run],
  [],
)
]

All 6 targets completed in every complete cell, and every system was
inconsistent (no decomposition of these targets), as in the parent: T is
the cost of deciding a system, not of finding a relation
\[pcc:unverified\]. At the prime D \= 19 and at the m \= 4 boundary
cells D \= 3, 7 the chain and flat presentations coincide and the
recorded chain T is reproduced exactly (ratio 1.0, identical per-target
`field_ops`), which ties this binary and these seeds to the parent\'s
\[pcc:unverified\]. The largest complete system (m \= 3, D \= 57)
reached a 6,920,436 x 6,957,419 sparse matrix \[pcc:unverified\].

== Estimator (parent section 5; OLS of ln unit on ln D over complete
cells with D \>\= 9; 95 % t interval)
<estimator-parent-section-5-ols-of-ln-unit-on-ln-d-over-complete-cells-with-d--9-95--t-interval>
#align(center)[#table(
  columns: 6,
  align: (col, row) => (auto,auto,auto,auto,auto,auto,).at(col),
  inset: 6pt,
  [fit], [n], [D range], [slope], [95 % interval], [2/(m + 1 - s),
  derived],
  [m \= 3, T],
  [13],
  [9..57],
  [4.598],
  [\[4.402, 4.794\]],
  [no admissible value (s \> 4)],
  [m \= 3, W (load-flagged D \= 57 excluded)],
  [12],
  [9..54],
  [4.977],
  [\[4.391, 5.563\]],
  [no admissible value],
  [m \= 3, T, first container only (D \<\= 54)],
  [12],
  [9..54],
  [4.597],
  [\[4.366, 4.827\]],
  [],
  [m \= 4, T and W],
  [3],
  [9, 12, 14],
  [not applicable (span 1.6x)],
  [],
  [],
)
]

Two-point local slopes. m \= 3, T: 4.45, 3.51, 4.42, 8.09, 1.84, 4.01,
8.87, 5.63, 8.99, 2.03, 4.43, 4.33 (D \= 9 to 57 in order); the scatter
follows the arithmetic of D (the 18 -\> 19 and 27 -\> 28 steps are
steep, 19 -\> 21 and 38 -\> 42 flat), as in the parent
\[pcc:unverified\]. m \= 3, W: 3.39, 3.64, 4.01, 6.24, 2.99, 4.71, 4.74,
5.78, 7.63, 7.20, 8.60, 10.16: the wall slopes rise from 5.8 (28 -\> 36)
to 8.6 (42 -\> 54) with one dip (7.63, then 7.20); the 54 -\> 57 value
(10.16) uses the load-flagged D \= 57 wall time; the counted slopes show
no such rise \[pcc:unverified\]. m \= 4, T: 4.81 (3 -\> 7), 6.66, 10.97,
14.06 (12 -\> 14); W: 4.36, 5.39, 8.77, 12.58 \[pcc:unverified\].

Fit residuals at m \= 3 (T over fitted T) range from 0.72 (D \= 27) to
1.38 (D \= 38) \[pcc:unverified\].

== Solving degree
<solving-degree>
SD \= D + 1 in every complete flat cell with D \>\= 9, at both m (and 8
at the m \= 4 boundary cells D \= 3, 7) \[pcc:unverified\]. The chain
presentation\'s SD tracked the largest prime factor of D (wave-5 law);
the flat presentation pays D + 1 regardless of the factorisation of D
\[pcc:unverified\]. At prime D the two presentations are the same system
(ratio 1.0, identical per-target `field_ops`). At composite D the chain
has the lower SD and the smaller largest matrices (at D \= 18, 27, 28:
SD 7, 8, 11 against 19, 28, 29; 14,548, 38,639, 49,575 rows against
35,987, 220,400, 259,774) and yet 10.5 to 363x more counted operations
(12 variables at 3 levels against 6 at 1), so the solving degree does
not explain the ratio; the mechanism is not determined by these data
\[pcc:unverified\].

== Controls on the faithful k \= 24 member (p \= 16,777,141, A \= 7,842;
m \= 3; seeds by the parent rule)
<controls-on-the-faithful-k--24-member-p--16777141-a--7842-m--3-seeds-by-the-parent-rule>
#align(center)[#table(
  columns: 6,
  align: (col, row) => (auto,auto,auto,auto,auto,auto,).at(col),
  inset: 6pt,
  [D], [T], [SD], [W ms], [k \= 20 fitted T], [ratio to fit],
  [10],
  [1.490e7],
  [11],
  [111],
  [1.053e7],
  [1.415],
  [12],
  [2.726e7],
  [13],
  [215],
  [2.435e7],
  [1.119],
  [15],
  [7.043e7],
  [16],
  [477],
  [6.795e7],
  [1.037],
  [20],
  [2.022e8],
  [21],
  [1,568],
  [2.551e8],
  [0.793],
  [30],
  [1.917e9],
  [31],
  [11,510],
  [1.646e9],
  [1.165],
)
]

At the shared D \= 12 the two primes give T equal to 1.6e-6 (2.726398e7
against 2.726394e7; the four-digit test of P5 holds) \[pcc:unverified\].
At D \= 10, 15, 20, 30 the k \= 24 values lie within 0.79 to 1.42x of
the k \= 20 law, close to the k \= 20 residual range (0.72 to 1.38x),
and the SD values are D + 1 throughout: consistent with T depending on
(m, D) and not on p, tested at one other prime and at m \= 3 only
\[pcc:unverified\]. Per-operation wall time on k \= 24 (6 to 8 ns)
matches k \= 20 \[pcc:unverified\].

== Predictions (written before data; a failed prediction stays failed)
<predictions-written-before-data-a-failed-prediction-stays-failed>
- #strong[P1 held] \[pcc:unverified\]. The m \= 3 estimator applies;
  s\_T \= 4.60 \[4.40, 4.79\] lies in \[4.0, 4.8\] with lower bound
  above 3. Verdict: no lever at m \= 3 \[pcc:unverified\].
- #strong[P2 failed] \[pcc:unverified\]. s\_W \= 4.98 \[4.39, 5.56\]
  lies in \[4.5, 7\], but s\_W - s\_T \= 0.38 \< 0.5 \[pcc:unverified\].
  The failure depends on the protocol\'s exclusion of the load-flagged D
  \= 57 wall time; including it (not permitted) would give s\_W \= 5.25
  \[4.62, 5.87\] and a difference of 0.65; on the matched D \= 9..54 set
  the difference is 0.38 \[pcc:unverified\]. Post-hoc description, not a
  tested finding: per-operation time varies within 1.6x (6.0 to 9.8 ns)
  with no trend for D \<\= 38, then rises to 1.56x the D \<\= 42 median
  (7.2 ns) at D \= 42, 4.5x at D \= 54 and 6.1x at D \= 57
  (load-flagged), where the matrix exceeds 5 million rows
  \[pcc:unverified\].
- #strong[P3 held] \[pcc:unverified\]. The m \= 3 run sequence reached D
  \= 57 and censored at D \= 63 (\<\= 84), by inference on memory, not
  time (see Censoring).
- #strong[P4 held] \[pcc:unverified\]. m \= 4: D \= 3, 7, 9 complete; D
  \= 12 completes under 12 GB (the chain-presentation cell did not), and
  D \= 14 too; the run sequence censors at D \= 18 (\<\= 28); the
  estimator is not applicable; both local slopes with D \>\= 9 (10.97,
  14.06) exceed 2.
- #strong[P5 held] \[pcc:unverified\]. Four-digit agreement at D \= 12;
  maximum deviation from the k \= 20 law 1.415x (\<\= 1.5).
- #strong[P6 held] \[pcc:unverified\]. sha256 5abc70ba... in both launch
  lines of `run.log` and in all 25 cell records.

== Wall-time regime disclosure
<wall-time-regime-disclosure>
The container restarted between 2026-11-15 07:34Z (D \= 54 written) and
2026-11-21 19:23Z (session resumed); the D \= 57 attempt in flight was
lost and rerun from scratch with the same seed in a fresh container
(kernel 6.18.44-fc-v726; note in `run.log`). Cells D \<\= 54 (m \= 3),
all m \= 4 cells and no control ran in the first container; D \= 57, 63
and the k \= 24 controls ran in the second. The relaunch coincided with
the D \= 57 start, so its 1-minute load before the cell was 1.88 (\>
1.5): under the protocol its wall time is excluded from the wall fit and
shown struck in the table \[pcc:unverified\]. Its counted T is
unaffected: T is deterministic given the seed, shown by the validator\'s
replay of the k \= 20 m \= 3 D \= 12 cell in the second container (all
six per-target `field_ops` and targets identical;
`validator-report.json`, `replay_m3_D12`) and by the identical
per-target `field_ops` of the 8 cells shared with wave-6 L3
\[pcc:unverified\]. The counted fit with the first container only is
4.597 \[4.366, 4.827\], indistinguishable from the full fit
\[pcc:unverified\]. The load averages of the other cells were 0.79 to
1.09, i.e. this process alone on a shared host whose other tenants are
not observable from inside the container \[pcc:unverified\]. The wall
unit has no A/A interval; its observed run-to-run variation on identical
systems is 6 to 12 % (validator replay of D \= 12: 209.9 against 198.0
ms; wave-6 L3 against this study on the 8 shared cells: -12 to +12 %),
which moves a close-D two-point wall slope by about 2
\[pcc:unverified\].

== Censoring
<censoring>
Both censored cells ended with exit -6 under `ulimit -v` 12,000,000 kB,
not on the 7,200 s per-target cap: m \= 4, D \= 18 after 1,287 s
(stderr: \"memory allocation of 575648 bytes failed\"; location not
recorded) and m \= 3, D \= 63 after 13,904 s (SIGABRT in
`RawVec::grow_one` inside `f4_with_arithmetic`; the allocation message
is cut off by the runner\'s 400-character stderr tail, so allocation
failure is inferred: a panic would exit 101 and a per-target timeout
would have produced a record) \[pcc:unverified\]. The engine writes its
record at the end of a cell, so how many of the 6 targets had finished
is not recorded; the D \= 63 process ran longer than one cap because the
cap is per target. The binding limit on this route is memory, by
inference, as in the extension study \[pcc:unverified\].

== What changed and what did not
<what-changed-and-what-did-not>
- The m \= 3 verdict is settled under the preregistered rule: no lever
  (s\_T and s\_W \>\> 0) \[pcc:unverified\]. It was \"undetermined\"
  since 2026-11-08.
- The m \= 4 verdict is not settled; the flat presentation moves the
  memory wall from D \= 12 to D \= 18 and shows accelerating local
  slopes, consistent with no lever but not an estimator verdict
  \[pcc:unverified\].
- No component speed-up is claimed here \[pcc:unverified\]: the chain /
  flat ratios reproduce L3 (which already reported them, with a
  validator report) and the same binary; nothing in this study is new
  engine work.
- The counted unit and wall time agree per operation within 1.6x through
  D \= 38 and then separate \[pcc:unverified\]; that the separation
  comes from matrices outgrowing cache-friendly sizes is a hypothesis,
  not measured here (no cache or memory-bandwidth measurement exists).

== Disclosures
<disclosures>
No Conductor task, EKS run, S3 checkpoint, benchrec record,
VALIDATION.json or pcc claim was reachable from this runtime (as in
every study of this series). The research knowledge base (MCP and CLI)
and the lead-atlas checkout were unreachable in this container; sources
checked are the repository files below. Single host, single process; the
host is shared and its other tenants are not observable. The binary is
the wave-6 build 5abc70ba (validated against the frozen dc2c30a4 on
control cells in `curve25519_wave6_runs_20261115`), not the frozen
binary itself. Six-day gap in the run.

Sources checked: `research/curve25519_exponent_study_20261108/`
(PROTOCOL section 5, cells),
`research/curve25519_exponent_study_ext_20261110/` (cells),
`research/curve25519_wave6_runs_20261115/L3_flat/` (analyse\_l3.py,
validator report), `research/curve25519_leads_wave5_20261112/README.md`
(SD law), `src/bin/curve25519_family.rs` (`f4sub`, `subflat`).

#figure(image("flat_exponent.svg", width: 100%), caption: [T and W against D (log-log), the m = 3 fits, the m = 4 cells, the k = 24 controls, the recorded chain cells, and per-operation time.])

== Files
<files>
- `PROTOCOL.md`: frozen protocol. `run_cells.py`: resumable runner.
  `run.log`: launch lines, cell lines, restart note. `cells/*.json`: 25
  cell records with `ext` (cap, memory limit, loads, wall, start time,
  binary sha256, exit code, command). The engine\'s own `study` field
  reads `curve25519_exponent_study_20261108` in every record (hard-coded
  in `f4sub`); `ext.study` names this study. The two censored records
  are the runner\'s fallback records and have no `seed` field; their
  seeds are in `ext.cmd`.
- `analyse.py` -\> `analysis.json`, `analysis.md`. `figure.py` -\>
  `flat_exponent.svg`, `.png`.
- `REPORT.typ`, `REPORT.pdf`. `validator-report.json`: independent
  validation (see Errata).

== Errata (from `validator-report.json`, verdict pass\_with\_errata; all
applied above, no verdict changed)
<errata-from-validator-reportjson-verdict-pass_with_errata-all-applied-above-no-verdict-changed>
- E1, E2: the Answer and P2 overstated the per-operation agreement
  (\"flat at 6 to 8 ns through D \= 42\", \"within 1.2x\"); the measured
  spread is 6.0 to 9.8 ns (1.6x) for D \<\= 38, 11.2 ns at D \= 42.
- E3: \"not a power law but a bend\" was a post-hoc reading presented as
  a finding; now labelled post-hoc and untested.
- E4: the solving-degree section attributed the chain/flat ratio to SD;
  the chain has the lower SD and the smaller matrices at D \= 18, 27,
  28, so the mechanism is undetermined.
- E5: the wall local slopes are not monotone (7.63 then 7.20), and the
  last uses the load-flagged cell.
- E6: three double-rounded table entries (6.9, 10.7, 43 s).
- E7: the k \= 24 D \= 12 cell is not a rerun of the k \= 20 cell;
  determinism is now shown by the validator\'s replay and the L3
  identity.
- E8: the censoring location is recorded for D \= 63 only, and the
  allocation message is truncated; now stated as inferred.
- E9: the D \= 57 wall time is now struck in the table as the text said.
- E10: the \"18x rho-neg\" figure was untraceable; replaced by the cited
  source\'s values per k.
- E11: \[pcc:unverified\] added to every result sentence.
- E12: P2\'s failure depends on the protocol\'s exclusion of the D \= 57
  wall time; disclosed.
- E13: \"cost depends on (m, D) and not on the prime\" narrowed to what
  was tested.
- E14: \"never pays\" marked derived and extrapolated.
- E15: wall-time run-to-run variation (6 to 12 %) disclosed; the cache
  sentence is a hypothesis.
- E16: \"chain\" disambiguated where the run sequence was meant; engine
  `study` field and missing `seed` in the censored records noted; REPORT
  rebuilt from the corrected README.
