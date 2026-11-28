# Results: the closure degree is at least 5 from m·ℓ = 21, and no engine in this container reaches degree 5 at 40 unknowns

Scored natively by `dreg_score slope runs/*.D*.jsonl` (`score-output.txt`)
against `PREREGISTRATION.md` (commit `8c7e498fc`). Every value below comes
from a committed record in `runs/`, `f4_probe/` or the two previous closure
studies. Run report with graph and PDF: `reports/run-20261018T0618Z/`.

**Registered verdict: undecided.** No cell above the degree-4 anchors
resolved to an exact degree, so the registered slope statistic has no
measured point above `m·ℓ = 18` and the registered thresholds (`s < 0.03`,
`s ≥ 0.10`) are not decided by this study. What the study does establish is
a feasibility wall and a lower bound, both stated below.

## What was measured

| m | (n, ℓ) | m·ℓ | N | degrees run | outcome per draw | cell |
|--:|---|--:|--:|---|---|---|
| 3 | (19, 7) | 21 | 40 | 5, 6 (sparse) | all four draws: capped at 350 M entries at degree 5 (168–173 s) and at degree 6 (102–104 s) | unmeasured above the committed `> 4` |
| 3 | (21, 7) | 21 | 42 | 5, 6 (sparse) | all four draws: capped at degree 5 and at degree 6 (about 2 min each) | unmeasured above the committed `> 4` |
| 3 | (22, 8) | 24 | 46 | 4, 5, 6 (sparse) | draw 2: degree 4 capped at 350 M entries after 8,466 s, degree 5 capped after 82 s, degree 6 aborted on allocation under the 7 GB address-space limit (exit 134); draws 4 and 5: degree 4 killed when the queue was stopped, after 155 and 13 CPU-minutes; draw 7: not started | unmeasured |
| 3, 4 | (24, 8), (25, 9), (27, 9), (28, 10), (30, 10); (18, 5), (20, 5) | 20–30 | 46–60 | — | not started: the queue was stopped after the first fresh cell (below) | not run |
| 4 | (24, 6) | 24 | 72 | — | cannot be built: the Boolean monomial type caps at 64 variables | not buildable |

- **P4 (instrument check).** The rewritten sparse closure refuted the
  committed dense degree-4 refutation of m3 (18, 6) draw 0 at degree 4
  (rank 64,433 of 66,712 columns, 6,863 falls, 400,372 products, 1,325 s).
  The second check draw, m4 (13, 4) draw 1 (124,314 columns), was still
  running when the worker restarted and is recorded as unmeasured. P4 holds
  on the one measured replay and is incomplete.
- **P5 (soundness).** Not run. The registered controls are the first
  satisfiable draw of each fresh cell, all at N ≥ 46, where the sparse
  closure's degree-4 run itself caps after more than two hours; a capped
  control would be unmeasured by construction. The previous study's
  control (345 satisfiable draws, none refuted) covers the dense engine;
  the sparse engine is covered by its agreement test and by P4's one
  measured replay.
- **P1–P3.** Unmeasured: no draw at `m·ℓ = 20` or `21` resolved, and no
  cell above the anchors resolved.

## The feasibility wall (measured)

Degree 5 of the chained `S₃` system at 40 Boolean unknowns is out of this
container's reach with every engine the repository has:

| engine | setting | result on m3 (19, 7) draw 0, N = 40 |
|---|---|---|
| sparse MutantXL closure (rewritten: colex-ranked rows, dense bit accumulator) | cap 350 M stored entries, 7 GB address space | cap reached after 168–173 s at degree 5, 102–104 s at degree 6, on all 8 `m·ℓ = 21` draws |
| Boolean F4 (`pq_f4_f2`), probe only | step-matrix cap 1 GiB (default) | stopped by the cap at the degree-5 step after 507 s; the returned 3,096-element set is not a basis |
| Boolean F4, probe only | cap raised to 8 GiB words, 13 GB address space | killed by the memory cgroup at 12.6 GB RSS in the degree-5 step |
| dense closure (previous study's engine) | — | not attempted: a degree-5 Macaulay matrix at N = 40 has 760 k columns, about 72 GB as a dense bit matrix |

The same engines at degree 4 are slow but finish: 1,325 s (sparse) and
418 s (F4, solving degree 4, agreeing with the dense closure) on the 36-unknown
anchor; the first 46-unknown fresh cell did not finish degree 4 within the entry
cap: 8,466 s to the cap on draw 2, 155 CPU-minutes without a verdict on
draw 4 when the queue was stopped. The F4 probe is a second instrument that was never registered; its
records are in `f4_probe/` and enter no statistic here.

## What the data say about the slope (derived, labelled)

The registered statistic needs resolved cells and has none above the
anchors. Two facts are nevertheless fixed by committed records:

- At `m·ℓ ≤ 18` the closure degree is 4 on every draw (16 anchor cells over
  the two previous studies), and at `m·ℓ = 21` it is above 4 on every draw
  (8 of 8, previous study; confirmed here by degree-5 and degree-6 runs that
  did not refute before capping, which is unmeasured, not confirmation).
- So the local slope between `m·ℓ = 18` and `21` is at least
  `(5 − 4)/(21 − 18) = 0.33` degrees per summand unknown. For the fitted
  slope over the registered range to fall below `0.10`, the degree would
  have to hold at exactly 5 from `m·ℓ = 21` out to about `m·ℓ = 28`; to fall
  below `0.03`, out to about `m·ℓ = 51`. Both are consistent with the data
  and neither is tested by it. This is arithmetic on the registered
  thresholds, not a measurement.

Under the cost model of the preregistration at `w = 1` (one touch per
column, the repository's lower-bound convention; `dreg_score slope` prints
the table), the admissible Macaulay degree on the chained system at
`n = 131` is 3 for `m = 3`, 4 for `m = 4` and 5 for `m = 5`; at `w = 2.2`
(the exponent the closure's own counted work shows, Result 11) it is 2 for
`m ≥ 4` and none for `m = 3`. The scorer's "gap" column extrapolates from
the fitted slope, which this study left at the anchors' value; with the
measured lower bound (degree ≥ 5 at `m·ℓ = 21`, far below the priced
`m·ℓ ≈ 131`) the gap at `n = 131` is at least `+1` degree for `m ≤ 4` at
`w = 1` and at least `+3` at `w = 2.2`. The slope decides only whether some
larger `n` admits the route. Derived, not measured.

## Classification and the success criterion

Class: **stage diagnostic**, negative on reach. No `S`, no rho ratio, no
speed claim. The index-calculus baseline is unchanged. Two engineering
changes landed with the study and are not improvements to the baseline: the
exact counters solve each chain link directly (draw generation at `n ≤ 30`
in seconds instead of hours; tested equal to the field scan), and the sparse
closure stores colex-ranked rows and reduces in a dense bit accumulator
(same saturated span, tested; the previous sparse engine had not finished the
36-unknown anchor after 33 CPU-minutes, the new one finishes it in 1,325 s).

## Deviations from the preregistration

- The m4 (24, 6) cell cannot be built (64-variable cap); recorded, not
  measured.
- The measurement queue was stopped after the first fresh cell: every
  further cell would have produced, at best, the same "capped at degree 5"
  record at 1–3 hours per degree-4 run, and the engine agent that was to
  speed the closure up was cut off by an API spend limit after delivering
  the rewrite above.
- The P4 check is incomplete (one of four replays measured) for the same
  reason: the worker restart killed the second replay after 80 minutes.
- The F4 probe was run outside the registered instrument and is reported as
  a probe only.

## Successor

The question is unchanged and now has a measured price: resolving the
closure degree at `m·ℓ = 21` needs a degree-5 step on 40 Boolean unknowns,
which is more than 12.6 GB for F4 and more than 350 M stored entries for the
sparse closure here. The next run belongs on a host with 64 GB or more (the
dense closure at degree 5, N = 40, is about 72 GB; F4's degree-5 step
somewhat less), or behind an out-of-core closure. The registered cells,
draws and scorer are in place for that run; nothing in the protocol needs
to change except the resource caps.
