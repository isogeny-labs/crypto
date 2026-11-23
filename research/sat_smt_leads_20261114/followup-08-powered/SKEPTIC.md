# Skeptic review: follow-up 08, powered run of target-parametric incremental SAT

Reviewer: independent skeptic pass, 2026-11-14, one core, about 25 minutes.
Every statement below is an unverified reviewer assertion [pcc:unverified]; no pcc claim exists
for the producer's numbers either.
Scratch (my scripts and outputs):
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-f08/`
(`indep.py`, `stats.py`, `rerun/`, `long_l4.jsonl`, `prod/` = copy of this directory used for
all runs). No producer file was edited; nothing else in the repository was written.

## Outcome

- Reproduced: **yes** for the smallest cell (bit-identical counters) and for every reported
  statistic recomputed from the raw rows, **except** the l-trend intervals, which are too narrow
  because of a bootstrap bug (section 3).
- **INCONCLUSIVE is the right label under the written rule.** It is a knife-edge label: it
  holds against BEST and against SPEC alone, and would be ALIVE against FRESH-A alone.
- The claim that matters is weaker than the README's framing in one direction and stronger in
  another: the conflict ratio is real and still falling slowly at 500 targets (my run, l=4),
  but INC does more propagations than SPEC at every position and takes about 2.2 times SPEC's
  wall time, with no improvement along the sequence. There is no evidence of a compounding gain.

## 1. Instance generator and encoding (own arithmetic)

Own code (`indep.py`): log/antilog field tables for the recorded moduli, points found by
solving z^2+z=c by table, own addition law (associativity and N·P=O checks pass; group orders
2,116 / 8,012 / 130,972). Only `build2` and `assumptions` (the CNF under test) were imported.

| check | l=3, n=11 | l=4, n=13 | l=5, n=17 |
|---|---|---|---|
| signed base size (meta) | 6 (6) | 14 (14) | 32 (32) |
| decomposable abscissae outside V, own triple enumeration (meta) | 19 (19) | 231 (231) | 2,679 (2,679) |
| label errors among the 60 measured targets | 0 | 0 | 0 |
| measured targets not rational, in V, zero or repeated | 0 | 0 | 0 |

CNF checks at l=3:

| check | result |
|---|---|
| 60 measured targets, CaDiCaL and Glucose 4 on the producer CNF | verdict = my label on 60 of 60; solvers agree; 10 SAT |
| 10 planted targets | all SAT; both solvers' models lift (own lift) to rational points with P1+P2+P3 = ±R exactly |
| exhaustive, all 1,054 rational abscissae outside V | 0 mismatches (19 SAT, 1,035 UNSAT) |
| all models projected on c, for all 19 decomposable abscissae | 19 models = my 19 sorted triples, set difference 0, no x_i = 0, no non-rational summand |
| old encoding (`nonzero=False`) | 9 mismatches, all 9 are exactly the (0,1)+P2+P3 targets not otherwise decomposable |
| fixed CNF with x_i = 0 forced on those 9 abscissae, each summand | 0 of 27 SAT |

The x=0 (2-torsion) defect of the pilot is fixed. One measured target at l=3 and one at l=5 lie
in the 2-torsion family and are labelled non-decomposable; with the fix that is consistent on
both sides.

Residual gap, not hit here: the CNF has no rationality (trace) constraint on the summands. A
model can use two twist points with P1+P2 = (0,1), i.e. x and 1/x both in V and both
non-liftable. None of the three V contains such a pair (checked), and the 1,080 measured rows
report 0 verdict and lift errors, so the measured rows are unaffected. The encoding is not
sound for an arbitrary V.

At l=3 every decomposable target has exactly one model and the signed base has 6 points. The
l=3 cell is a degenerate instance as well as failing the table control.

## 2. Rerun of the smallest cell

`rerun_smallest.sh` run in the scratch copy with `OUT` in scratch, 85 s: validation ok,
361 lines identical to `raw_l3.jsonl` in every field except wall time; `validation_l3.json`
identical. The 1,080 `raw_rows` in `results.json` equal the rows of the five `raw_*.jsonl`
files; 0 rows carry an error. l=4 and l=5 were not rerun.

## 3. Statistics recomputed from the raw rows (own code, own bootstrap seed)

UNSAT targets at positions 30..59, conflicts, geometric mean (95% percentile bootstrap).

| l | n targets | INC/BEST | INC/SPEC | INC/FRESH-A | INC/FRESH-U | SPEC/BEST | A/A FRESH-A2/FRESH-A, sd ln |
|---|---|---|---|---|---|---|---|
| 3 | 25 | 0.590 (0.556..0.629) | 0.529 (0.491..0.569) | 0.551 (0.511..0.589) | 0.409 | 1.116 | 1.074, 0.163 |
| 4 | 25 | 0.747 (0.720..0.775) | 0.747 (0.719..0.776) | 0.622 (0.600..0.647) | 0.602 | 1.000 | 1.007, 0.081 |
| 5 | 24 | **0.793 (0.773..0.812)** | 0.783 (0.755..0.808) | 0.675 (0.657..0.691) | 0.732 | 1.012 | 1.012, 0.054 |

All agree with the README. Medians (126 / 974 / 6,639 for INC), the position slope at l=5
(−0.0032, −0.0044..−0.0019), the null reference 1.189 and the propagation ratio INC/SPEC 1.36
also agree.

**Bug in the l-trend interval.** `boot_gm` returns the bootstrap means *sorted*, and the
l-trend subtracts the two sorted lists element by element. That pairs quantile with quantile
instead of resampling the cells independently, which the protocol specifies. Correct intervals:

| difference of ln GM | README | recomputed, independent resampling |
|---|---|---|
| INC/BEST, l=5 − l=4 | +0.060 (0.047..0.072) | +0.060 (0.014..0.106) |
| INC/BEST, l=5 − l=3 | +0.295 (0.256..0.335) | +0.295 (0.228..0.361) |
| INC/SPEC, l=5 − l=4 | not reported | +0.048 (−0.003..+0.097) |
| INC/FRESH-A, l=5 − l=4 | not reported | +0.081 (0.034..0.125) |

"Rises with l" survives against BEST and FRESH-A with intervals about 3.7 times wider than
stated. Against SPEC alone the l=4 to l=5 step is not resolved. The trend rests on one V per
cell with n moving with l; between-V variance is not measured.

Position slope by half, INC/BEST, ln ratio per target:

| l | positions 1..29 | positions 30..59 |
|---|---|---|
| 3 | −0.0086 (−0.0154..−0.0025) | −0.0018 (−0.0078..+0.0059) |
| 4 | −0.0098 (−0.0146..−0.0049) | +0.0002 (−0.0038..+0.0048) |
| 5 | −0.0059 (−0.0105..−0.0013) | −0.0019 (−0.0037..+0.0006) |

The single slope the README quotes ("0.969x per 10 targets") averages a fast early drop and a
second half whose slope is not separated from 0 in any cell. It must not be extrapolated
linearly.

## 4. Fairness

**Best-of baseline.** The minimum of three arms is biased low, but the size here is small
because the arms are not exchangeable: SPEC is the minimum on 25 of 25 primary targets at l=4
and 20 of 24 at l=5. Measured bias of BEST against the best single arm (SPEC/BEST):
1.000 at l=4, 1.012 at l=5, 1.116 at l=3 (where FRESH-A and SPEC are tied and noisy). For
comparison, three exchangeable arms with the A/A per-arm sd (0.054/√2 = 0.038 at l=5) would
give a 3.2% bias. So the penalty on INC at l=5 is about 1.2%: 0.793 against BEST, 0.783
against SPEC. That 1.2% equals the distance to the 0.8 threshold, but the SPEC interval
(0.755..0.808) also contains 0.8.

The README's sentence that FRESH-A2/BEST = 1.19 is "the bias of a minimum of three arms" is
wrong. That 1.19 is almost entirely FRESH-A being 16% to 20% worse than SPEC
(FRESH-A/SPEC = 1.16 at l=5, 1.20 at l=4). The null reference is therefore a weak-arm
reference, not a noise reference.

**Single pre-specified arm.** Against SPEC: 0.529 / 0.747 / 0.783. Against FRESH-A (same
formula, same assumptions, the only like-for-like arm): 0.551 / 0.622 / 0.675, which would be
ALIVE under the same rule. The verdict depends on which fresh arm is the baseline; the protocol
fixed BEST before the data, so INCONCLUSIVE stands.

**Are INC's conflicts comparable?** Only partly.
- INC and SPEC solve different formulas. Per conflict INC does about 1.7 times SPEC's
  propagations at l=5 (propagations 1.36, conflicts 0.78). In propagations INC loses to SPEC in
  every cell (1.14 / 1.21 / 1.36, rising with l). Summed wall time on the primary set, l=4,
  one process: INC 8.7 s, SPEC 4.3 s, and SPEC's time includes regenerating its CNF in Python.
  A conflict count is not a cost when the formulas differ; the pre-registered metric favours INC.
- INC's restart, reduce, rephase and inprocessing schedules run on cumulative counters, so by
  position k it is in a different scheduler state from a fresh solver. No arm separates learnt
  clause reuse from amortised inprocessing of the parametric formula or from scheduler state
  (for example INC with learnt clauses dropped between targets). The README lists this as not
  covered. The mechanism of the 0.6 to 0.8 is therefore unidentified.
- The delta of `accum_stats` around each incremental solve is the right way to count; I found
  no accounting error.

**Table-exhaustion control.** Met at l=4 and l=5, failed at l=3 as stated. The README's
"26%" at l=4 divides all 60 targets by the 231 decomposable abscissae, which mixes units. The
relevant fractions are: decomposable targets 10/19, 10/231 (4.3%), 10/2,679 (0.4%);
non-decomposable targets 50 of 1,035 / 3,767 / 62,790 non-decomposable rational abscissae
outside V (4.8%, 1.3%, 0.08%).

**Protocol before data.** Supported by file times and hashes only; there is no version
control. `PREREG.sha256` matches all six files. PROTOCOL.md, analyze.py and PREREG.sha256 were
written at 18:09:19.9; the l=3 run (9.6 s of solver time) finished at 18:09:29.7, so it was
launched within a second of the hash; l=4 finished 18:11:33; l=5 ran 18:11 to 18:22. An interim
analysis of l=3 and l=4 (`scratchpad/f08/an34.txt`, 18:11:40) existed before the l=5 data, and a
4-target smoke at l=5 with all six arms existed before the protocol. Neither changes the rule:
the decision rule, cells, seeds and primary set in the code equal the protocol, and the written
prediction (0.85..1.15) was wrong, which is not what a tuned threshold looks like. File times
can be set, so this is "consistent with", not "shown". Unreported deviation: the l-trend
bootstrap is not the independent resampling the protocol specifies (section 3).

## 5. Does the reuse compound?

Reviewer's exploratory run, not pre-registered, one V, one seed: `run.py 13 4 500 10 INC,SPEC`
(l=4, 510 distinct targets, 277 s, 0 errors). 500 non-decomposable targets are 13% of the 3,767
available, so this run is weaker on target-space coverage than the producer's.

| positions | UNSAT targets | INC/SPEC conflicts (±1.96 se) | INC/SPEC propagations | INC/SPEC wall time | INC median conflicts |
|---|---|---|---|---|---|
| 1..19 | 18 | 0.854 ± 0.054 | 1.36 | 2.15 | 1,062 |
| 20..39 | 20 | 0.793 ± 0.041 | 1.25 | 2.10 | 1,005 |
| 40..59 | 19 | 0.764 ± 0.043 | 1.21 | 2.11 | 956 |
| 60..99 | 40 | 0.724 ± 0.028 | 1.22 | 2.21 | 898 |
| 100..149 | 49 | 0.687 ± 0.022 | 1.19 | 2.23 | 855 |
| 150..199 | 50 | 0.650 ± 0.018 | 1.17 | 2.27 | 827 |
| 200..299 | 99 | 0.646, 0.635 | 1.18, 1.15 | 2.34, 2.25 | 811, 801 |
| 300..399 | 98 | 0.635, 0.640 | 1.14, 1.17 | 2.29, 2.44 | 788, 789 |
| 400..509 | 106 | 0.597, 0.608 | 1.14, 1.15 | 2.44, 2.39 | 760, 760 |

Slope of ln ratio per target: −0.00052 over 1..509, −0.00030 ± 0.00010 after position 100,
−0.00025 ± 0.00015 after position 200. The producer's l=4 slope over 1..59 was −0.0044.

What the data support:
- The conflict ratio keeps falling past 60 targets, so the README's "where it levels off is
  not measured" is right and my half-split in section 3 understates the tail. At l=4 it has not
  reached a plateau at 500 targets.
- The fall is strongly decelerating: roughly 0.85, 0.76, 0.69, 0.65, 0.60 at positions about
  10, 50, 125, 200, 450, close to position^−0.09. The slope per target dropped by a factor of
  about 17 between the first 60 and the last 300 targets. At that rate a factor of 2 over SPEC
  in conflicts would need on the order of thousands of targets and a factor of 3 more targets
  than the cell contains.
- In propagations INC never beats SPEC (1.14 at best), and in wall time it is 2.1 to 2.4 times
  slower with the ratio drifting up, not down. Nothing compounds in units of work.
- INC's own median conflicts per target fell from 1,062 to 760 over 500 targets (factor 0.72),
  while going from l=4 to l=5 multiplies it by 6.8, the same growth as the fresh arms (SPEC 6.5).
  The reuse is a slowly improving constant on top of an unchanged exponent in l.
- At equal position 30..59 the ratio is worse at larger l against BEST and FRESH-A. Since both
  l=4 and l=5 were measured in the transient, this cannot distinguish a higher plateau from a
  slower approach to the same plateau.

What the data cannot support: any regime where the factor grows with l; any plateau value at
l=5; any statement about l ≥ 6, m = 4, another V, another solver, or native XOR; any ranking
against the linear 2+1 scan (32 leaf solves against 6,639 conflicts at l=5 are different units,
and the producer forms no ratio, correctly). My long run is one sequence and exploratory.

Answer: on this evidence the reuse is a bounded, slowly improving constant factor in conflicts
(about 0.6 to 0.8), smaller at larger l for a fixed sequence length, and absent or negative in
propagations and time. No measured trend, extrapolated, gives a growing factor.

Smallest experiment that would settle it: pre-register INC against SPEC only, metric
propagations (and wall time on an isolated core) with conflicts secondary; l = 4, 5 and one l=6
point; 3 V seeds per l; at least 1,000 UNSAT targets per sequence at l=5 (about 2.5 core hours
per V at the measured 5.4 s + 3.7 s per target); add one control arm, INC with learnt clauses
dropped between targets, to separate lemma reuse from amortised inprocessing. Fit ln ratio
against ln position per l. The lead is alive only if the fitted exponent does not shrink with l
and the propagation ratio falls below 1; otherwise close it.

## Problems found

1. l-trend bootstrap subtracts sorted bootstrap lists; reported intervals are about 3.7 times
   too narrow and the procedure deviates from the protocol without being listed as a deviation.
   The reading "rises" survives against BEST, not against SPEC alone.
2. README attributes the 1.19 null reference to the bias of a minimum of three arms. The
   min-bias is 0% to 1.2% at l=4 and l=5; 1.19 is FRESH-A being a weaker arm than SPEC.
3. The primary metric (conflicts) compares solvers on different formulas. In propagations INC
   loses to SPEC in every cell and in time by about 2 times; this is in the README only as a
   post hoc note, while the headline numbers are the conflict ratios.
4. The quoted position slope is a linear fit to a decelerating curve; "about 3% per 10
   targets" does not hold beyond the window (my l=4 run: 0.3% per 10 targets after position 100).
5. The verdict is on a knife edge and baseline-dependent: INCONCLUSIVE against BEST and SPEC,
   ALIVE against FRESH-A. The window was fixed while the ratio was still moving, so the label
   says little about the asymptote in either direction.
6. Mechanism unidentified: no arm separates learnt-clause reuse from amortised inprocessing and
   scheduler state.
7. One V, one target order and one seed per cell; l and n move together; l=3 is degenerate
   (6 base points, one model per decomposable target, 10 of 19 decomposable abscissae used).
8. "Table control 26% at l=4" mixes non-decomposable targets with the decomposable table; the
   control is met at l=4 and l=5 (4.3% and 0.4%; 1.3% and 0.08% of non-decomposable abscissae).
9. The encoding has no rationality constraint on the summands; unsound for a V containing a
   non-liftable pair x, 1/x. Not triggered by these three V or these 180 targets.
10. Protocol-before-data rests on file times and a hash file in an unversioned directory; an
    interim l=3/l=4 analysis and an l=5 smoke preceded the l=5 data. No sign that the rule or
    cells changed.
11. Repository obligations not met by the producer (stated in the README): no pcc claim, no
    benchmark record, no validation-gate evidence, no visuals. Nothing here may be cited as a
    result or reach a leaderboard.

Not checked by me: l=4 and l=5 producer rows were not rerun (only recomputed from the stored
rows; labels re-derived independently); CaDiCaL's internal handling of assumptions was reasoned
about, not instrumented. Treat those points as weaker than stated.
