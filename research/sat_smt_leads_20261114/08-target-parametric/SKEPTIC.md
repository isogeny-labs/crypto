# Skeptic review: idea 8, target-parametric incremental solving

Reviewer: independent skeptic pass, 2026-11-14, one core, about 15 minutes.
Everything below is an unverified reviewer assertion [pcc:unverified].
Scratch (my scripts and rerun output):
`/tmp/claude-0/-home-user/570c81c4-197a-558d-aa76-c9ff505d9731/scratchpad/skeptic-08-target-parametric/`
(`indep.py`, `indep2.py`, `raw_l3_rerun.jsonl`, `rerun.log`). No producer file was edited.

## Outcome

- Reproduced: yes (smallest cell, bit-identical counters).
- The producer's verdict "inconclusive" follows from the pre-written criterion. Corrected verdict: **inconclusive**.
- Two of the producer's supporting statements do not hold as written: "0 verdict mismatches"
  is true only of the sampled targets (the ground truth disagrees with the CNF on 1.5% of
  abscissae at l=3), and "no decline along the sequence" is contradicted by the UNSAT rows.
  "Leaning negative" in README.md is a reading, not a result of the criterion.

## 1. Instance generator (independent check, l=3, n=11)

Own code: log/antilog field arithmetic for the recorded modulus 2053, curve points found by
brute force over y, own addition law (300 associativity/on-curve checks pass). Only the
producer's `build2`/`assumptions` (the CNF under test) and `Setup` (to read V) were imported.

| check | result |
|---|---|
| signed base size | 8, equals meta |
| decomposable abscissae by triple enumeration | 48 (44 outside V), equals README |
| labels of the 12 measured l=3 targets against my enumeration | 12 of 12 agree; all rational, outside V |
| 20 own planted targets | all SAT; every model lifts (own lift) to rational points with x(+-P1+-P2+-P3)=x(R) |
| exhaustive: all 1,053 rational abscissae outside V, parametric CNF | 993 UNSAT, 44 SAT matching ground truth, **16 SAT where the producer's ground truth says non-decomposable** |
| same, constant-target CNF, every 4th abscissa (264) | 246 UNSAT, 14 SAT matching, **4 SAT against ground truth** |
| SAT models using a non-rational (twist) point | 0 |

**Defect found.** The CNF lets x_i = 0 (0 is in V), i.e. it uses the 2-torsion point (0,1) as
a factor-base point. `Curve.lift` returns None for x=0, so `Setup.base`, `decomposable()` and
`lift_model` exclude that point. All 16 disagreeing abscissae are exactly the targets of the
form (0,1)+P2+P3; the models are algebraically valid sums. So the encoding and the ground
truth define two different problems, differing on 16 of 1,053 abscissae (1.5%) at l=3.
With (0,1) admitted the decomposable counts outside V are 60 / 391 / 2,456 at l=3/4/5
against 44 / 327 / 2,241 without.

Effect on the measured rows: none found. I recomputed the label of every measured target at
l=3, 4 and 5 (12, 24, 16 rows) by my own enumeration: 0 label errors, and no measured target
lies in the 2-torsion family. A non-decomposable target drawn there would have been logged as
an error, not silently dropped. The README sentence "solver verdict equals ground truth" is
therefore a statement about 44 + 52 sampled targets, not about the generator. The core study
is affected in principle: a freed affine family can hit one of these abscissae (as it can hit
x(R) in V, which the amendment does disclose), which can only enlarge cores; the counting-null
density was taken from the solver, so it is consistent with what the solver sees.

Exact counts replace the README's heuristic: decomposable targets used / available outside V
are 6/44 (l=3), 12/327 (l=4, 3.7%), 8/2,241 (l=5, 0.36%). The table-exhaustion control holds
at l=4 and l=5 and fails at l=3, as the producer says.

## 2. Rerun of the smallest cell

`python3 run.py 11 3 6 6 <scratch>/raw_l3_rerun.jsonl`, 2.9 s. All 13 lines (meta + 12 rows)
equal `raw_l3.jsonl` in every field except wall time: conflicts, decisions, propagations,
core sizes and DRUP histograms for all six arms. Recomputing the tables from the three raw
files with my own lines reproduces the kill metric (0.685, 0.666, 0.699), the medians and the
slopes in `results.json` (INC 2.46, FRESH-A 2.48, FRESH-U 2.24, SPEC 2.50).
l=4 and l=5 were not re-run (l=5 is about 9 minutes); their raw rows were only re-aggregated.

## 3. Fairness of the comparison

- Same instances and seeds: yes. One target list per cell, every arm solved per target inside
  the same loop iteration, same assumption list. Position 0 has INC = FRESH-A exactly at every
  l, as it must.
- Baseline strength: **the pre-registered baseline FRESH-A is the weakest fresh arm**, and the
  kill metric depends on that. CaDiCaL, second half, conflicts, INC over:

  | l | FRESH-A (kill metric) | FRESH-U | SPEC | per-target best of the three fresh arms |
  |---|---|---|---|---|
  | 3 | 0.685 | 0.579 | 0.842 | 0.896 |
  | 4 | 0.666 | 0.634 | 0.909 | 0.987 |
  | 5 | 0.699 | 0.818 | 0.802 | 1.037 |

  Against the untreated constant-target arm the gain is 10 to 20%, and against an oracle that
  picks the best fresh arm per target there is none at l=5. The producer reports the SPEC
  ratios and says this; the headline number is still the one measured against the handicapped arm.
- Non-SAT baselines: enumeration, MITM and the linear 2+1 scan are quoted as operation counts,
  not run. The units differ (conflicts, each with tens of thousands of propagations, against
  leaf solves or table probes), so "8,360 conflicts against 32 leaf solves" is not a ratio.
  The direction is not in doubt at these sizes; the size of the gap is not measured.
- Uncounted work: none that favours the treatment. INC has no precomputation; symmetry breaking
  is in every arm; CNF build time for SPEC is not charged (that favours the baseline). Wall time
  is not analysed.
- SAT/UNSAT mixing: the kill metric pools them by protocol. At l=5 the second half is 5 SAT +
  3 UNSAT; pooled 0.699, UNSAT-only 0.716, SAT-only 0.681. The pooled value is below 0.7 only
  because of SAT targets, where time to first model is luck (idx 8: 811 against 9,569
  conflicts). Leave-one-out range 0.604 to 0.809; target bootstrap 95% interval 0.45 to 1.06,
  with half of the resamples above 0.7. The metric cannot distinguish 0.6 from 0.9.
- Slope: three sizes, but n moves with l (11, 13, 17), so "bits per l" mixes two variables;
  per-step increments are 2.3 to 2.6 bits per l, or 1.2 then 0.65 bits per bit of n. The one
  l=6 calibration solve (+3.4 bits) sits above the fitted line. The equality of INC and FRESH-A
  slopes (2.46, 2.48) is the supported part; the absolute slope is not.
- "No decline along the sequence" is not what the UNSAT rows show. INC/FRESH-A by position,
  UNSAT targets only: l=5: 0.84, 0.80, 0.73, 0.77, 0.75, 0.72, 0.76, 0.67; l=4 (after the
  first target): 0.89, 0.69, 0.69, 0.84, 0.62, 0.66, 0.72, 0.60, 0.67, 0.67, 0.56. Both drift
  down. With 8 and 11 points this is weak, but it is a decline, and the sequences are too short
  to say where it levels off. This is the quantity the idea is about and it is unresolved.
- Arms reported: all six arms that ran appear in the README and `results.json`. The discarded
  first encoding has no logged rows; the l=6 calibration solve is disclosed.
- Protocol before results: cannot be established. There is no version control here.
  `PROTOCOL.md` has modification time 09:06:22, the same second as `run.py` and two seconds
  before `raw_l3.jsonl` was completed, and it already contains Amendment 1. The amendment was
  written after unlogged smoke solves that included l=5 conflict counts, and it changed the
  encoding, added symmetry breaking, dropped l=6 and l=7 and so made l=5 the "largest l" that
  carries the threshold. The kill thresholds themselves (0.7, 0.75 n) are as in the task
  statement. I see no sign of tuning to the 0.699 value: the l=5 cell ran once, the seed is
  the stated formula, and my rerun shows the pipeline is deterministic.
- Cores: at l=3 and l=4 the minimal cores (5-6 of 11; 8 of 13) are inside the counting-null
  range; I did not re-run the core scripts or the null. Only 4 targets at l=4, deletion-minimal
  not minimum, one basis. At l=5 nothing was measured.

## 4. Does the verdict follow from the kill criterion?

Kill needs both (1) pooled second-half INC/FRESH-A > 0.7 at the largest l and (2) smallest
minimal core >= 0.75 n at the largest l.

- (1): 0.699 at l=5, not above 0.7, so by the letter not met. It is not distinguishable from
  0.7 (see the interval above) and is 0.716 on UNSAT targets alone.
- (2): unmeasured at l=5. At l=4 it is 0.615, below 0.75, so it would not be met there either,
  for a reason the null explains.

So the kill criterion is not met, and "alive" is not earned either: the protocol's own clause
sends "ratio <= 0.7 only on SAT targets" to inconclusive, and the core evidence matches the
null. **Inconclusive is the correct verdict.** "Leaning negative" rests on comparisons outside
the criterion (SPEC, slope equality), which are fair points, but the downward drift of the
UNSAT reuse curve and the missing l=5 cores point the other way and should be stated with it.

## What would settle it

1. Fix the x=0 inconsistency (forbid c_i = 0 in the CNF, or admit (0,1) in the ground truth)
   and re-run validation exhaustively at l=3.
2. At l=5: at least 30 UNSAT targets, UNSAT-only kill metric, against SPEC as well as FRESH-A,
   with a sequence long enough to see whether the UNSAT ratio keeps falling (still under a few
   percent of the 2,241 decomposable abscissae).
3. Core minimisation at l=5 with a time budget that finishes, against the same null.

## Not done by this review

l=4 and l=5 solver runs, `cores.py` and `core_null.py` were not re-executed. Glucose rows were
not examined beyond the rerun identity at l=3. Repository session log, knowledge-base lookup
and Conductor task were not done: this review may write only this file.
