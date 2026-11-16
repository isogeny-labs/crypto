#set page(paper: "a4", margin: 15mm)
#set text(font: "DejaVu Sans", size: 9pt)
= Trace-zero solver: validated work reduction
2026-11-15 · PR3744 · TZF-20261115-cold-instruction-work · unsigned producer evidence

#image("instructions.svg", width: 100%)

#table(columns: (auto,auto,auto,auto,auto), inset: 4pt,
 [Field/case], [Baseline median Ir], [Candidate median Ir], [Removed], [Criterion],
[131/cancel], [182,008,866], [181,985,615], [0.01%], [inconclusive],
[131/planted], [269,704,933], [202,206,920], [25.03%], [pass],
[131/uniform], [619,422,026], [282,085,987], [54.46%], [pass],
[83/cancel], [153,463,249], [153,402,769], [0.04%], [inconclusive],
[83/planted], [292,813,150], [189,888,544], [35.15%], [pass],
[83/uniform], [561,510,012], [259,437,764], [53.80%], [pass],
)

*Observed:* all 90 planned query executions complete, each with a fresh native field-tally process and a fresh Callgrind process. All decisions, point indices, model diagnostics and witness order match. Six fixed d12 controls ×three arms ×five repetitions; these are paired disclosed anchors, not independent population samples. Error bars show the recorded five-repeat min/max; full A/A medians and ranges are in summary.json.

*Decision rule:* a cell passes when candidate median instruction count is at least 5% lower and baseline A/A median difference is at most 1%. Every inconclusive cell is retained. This is whole-process simulated instruction work, including parsing, setup, Gaussian elimination, allocation and replay. It is not native runtime, cycles, an external-best comparison or an asymptotic claim [pcc:unverified].

*Controls and resources:* every planted/cancellation query Found with original-group replay; each uniform query Refuted against separate arithmetic evidence. The 12-call largest-class pilot passed before freeze. CPU0, 2GiB, 600second combined pilot/freeze/primary cap, no retry or paid resources. Planning estimate 255.815seconds is resource planning, not timing evidence. No rows excluded or interrupted.

#pagebreak()
= What changed and how it was checked

*Implementation:* hoist squared V/W basis elements once; eliminate coefficients using three packed words, retaining arbitrary-width fallback; share one Montgomery inversion across at most 32 consecutive residuals; specialize monic quadratic roots. The first residual remains scalar to avoid charging unused batch work for an immediate answer. Search order, full nullspace enumeration, conservative caps, actual supplied signs and final original-group replay remain.

#table(columns: (auto,auto,auto,auto), inset: 4pt,
 [Case], [Field mul: old / new], [Field square: old / new], [Inv: old / new],
[83/planted], [52,159 / 48,994], [97,472 / 18,904], [695 / 31],
[83/uniform], [111,555 / 102,024], [253,877 / 24,224], [2,008 / 64],
[83/cancel], [21,344 / 21,333], [16,040 / 15,794], [11 / 10],
[131/planted], [49,549 / 47,732], [93,251 / 28,733], [408 / 21],
[131/uniform], [121,195 / 111,734], [358,139 / 37,484], [1,994 / 64],
[131/cancel], [31,604 / 31,593], [26,749 / 26,359], [10 / 9],
)

The field ledger includes cold fixture/variety construction through final replay; parsing is outside this field ledger and inside the instruction count. Inversions are informational: their internal multiplications/squarings are already counted. Unused residual-batch tail work is charged. Never add unlike field operations into a group-only ratio.

*Validation:* all 65 prior scale cases give exactly identical old/new decisions and witness order. New focused checks compare packed/generic solution kernels, batch/scalar sums across all small-curve points (including infinity, doubling and opposites), and every monic quadratic over F32 against exhaustive roots and the old routine. Prior linearized, terminal and 20-test suites pass; library type-check passes [pcc:unverified].

A separate Rust checker imports no production field/group/solver modules. Schoolbook polynomial arithmetic plus Euclid inversion rechecks all 65 known answers, including all 19 negatives; wide negatives require independent inconsistency of every necessary residual system. It validates all base coordinates/kernel admissions and replays the 416 saved tuples from 650 earlier rows, plus new saved tuples. Corrupted-point and false-refutation fixtures are both rejected. This producer-written check is stronger cross-arithmetic evidence, not independently signed verification [pcc:unverified].

*Scope:* E0 y²+xy=x³+1 over n83 and ECC2K-130 n131; existing exact curve/modulus/G identities remain in workload and prior study. Full-query old/new instruction comparisons do not establish best external performance against M4RI/FLINT/Magma, native-time gain, factor-base logarithms or a complete relation-collection pipeline. Canonical scoreboard/timeline/performance-gains values remain historical; no admitted runtime/full-cost record is changed. Existing ledger/main, whole-root and benchrec merge dependencies remain.

*Reproduction:* source 902f78ccfce8ab09c42b5dd39f3e9bde103c35aa; protocol 4643a05317e1266df27e06984a03d0d9d4a5146b5f616c7f5b855ec97228ee00; native binary 15b6156c1e95c9220915ca7eb85883190b4c03791679f3195caab4b1cd7479ea. README.md links commands, source pins, rows, summary, canonical export, grid audit, separate arithmetic checks, original inputs and the archived raw process tree. Every PDF page was visually inspected. Sources checked: existing S3/half-trace/Itoh–Tsujii modules and Montgomery-batching references in repository code; KB unavailable, wrapper127. No mathematical novelty assertion.
