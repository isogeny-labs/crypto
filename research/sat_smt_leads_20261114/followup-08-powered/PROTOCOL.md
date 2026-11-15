# Follow-up to idea 8: powered run of target-parametric incremental SAT

Written 2026-11-14 before any measured run of this directory. Before this file
was written the following had been executed here, and nothing else: the
exhaustive validation at (l,n)=(3,11) (`validate.py`, verdicts only, no cost
metric) and one calibration smoke of 4 targets each at l=4 and l=5 drawn with
target-seed offset 99 (not the measured targets, offset 1), used only to size
the run (about 19 s per l=5 target for six arms). The smoke rows are in the
scratch directory and are not analysed. There is no version control in this
workspace; `PREREG.sha256` records the hashes of this file and of the code
before the first measured run.

Everything produced is a pilot-scale assertion, tagged [pcc:unverified].

## Claim under test

m=3 point decomposition on y^2+xy=x^3+1 over F_{2^n}, factor base = points with
x in a random l-dimensional F_2-subspace V. One CNF with the target abscissa as
n assumption bits. Claim: one incremental CDCL solver that solves a sequence
of never-seen targets needs fewer conflicts per target than the best
fresh-solver arm on the same target, and the gap grows along the sequence.

## Prediction (before data)

No reuse against the best fresh arm: the primary ratio at l=5 is between 0.85
and 1.15, the same within noise as the null-reference ratio (a fresh arm in the
place of INC). Position slope indistinguishable from 0. The ratio does not fall
with l. If so the pilot's 0.699 was the handicap of its baseline arm (FRESH-A).

## Instances

- Generator and encoding: `pdp.py`, copied from `../08-target-parametric/`
  (S4 resultant encoding, AND gates, 3-input XOR chains, lex symmetry breaking
  c1<=c2<=c3 in every arm). One fix: the clause OR_j c_ij for each summand
  forbids x_i = 0, the 2-torsion point that the ground truth excludes.
- Cells (l,n) = (3,11), (4,13), (5,17), n as in the pilot. l=6 (n=19) does not
  fit: the pilot's one calibration solve took 106 s; a 40-target cell with these
  arms would be hours. Largest l is therefore 5.
- Seeds: master 2026111408 (new; the pilot used 20261114). Per cell: setup seed
  (V) = master + 1000 l, target seed = setup + 1, clause-shuffle seed = setup + 2.
  One V, one target order per cell. Seeds are not changed after seeing data.
- Targets per cell: 50 non-decomposable (random rational points, x(R) not in V,
  verified by the real-point 2+1 meet-in-the-middle) + 10 planted decomposable,
  all distinct, one seeded shuffle; 60 per cell, same list and order for every
  arm. Second half = positions 30..59.
- Table-exhaustion control: targets used against the exact number of
  decomposable abscissae outside V (computed by the runner and printed in the
  table). From the validation and the smoke: l=3: 19, l=4: 231, l=5: 2,679.
  So 60 targets (10 decomposable) is 2.2% (0.4%) at l=5, 26% (4.3%) at l=4, and
  exceeds the count at l=3. l=3 FAILS this control and is run only as the small
  end of the l-trend; it is flagged wherever it appears. No target is ever
  repeated within a sequence.

## Arms (CaDiCaL 1.5.3 through python-sat, default options, one core)

- INC: one solver for the whole sequence; target by assumptions. Treatment.
- FRESH-A: new solver per target, same CNF, same assumptions.
- FRESH-U: new solver, parametric CNF plus target bits as unit clauses.
- SPEC: new solver, CNF regenerated with the target as a constant.
- FRESH-A2 (A/A arm): FRESH-A on a copy of the CNF with the clause list
  shuffled once (shuffle seed). Same formula, same assumptions.
- INC2: INC on the shuffled copy (replicate of the treatment, descriptive).
- BEST fresh per target = min(FRESH-A, FRESH-U, SPEC) conflicts on that target.
  FRESH-A2 is NOT in the minimum.

Execution: a run is one process on one core holding a group of arms that are
interleaved per target. At l=3 and l=4 all six arms are in one process. At l=5
the six arms are split over three processes (INC,FRESH-A | FRESH-U,FRESH-A2 |
SPEC,INC2) so that no process exceeds 10 minutes; the processes generate the
identical target list from the seeds (checked by the analysis). The metric is
conflicts, which these solvers reproduce exactly run to run (pilot skeptic
rerun), so the split does not touch the metric; wall time is recorded but not
analysed. Every verdict is compared with ground truth and every SAT model is
lifted to curve points; mismatches are counted and reported, never dropped.

## Decision rule (fixed)

Per-target ratio rho_i = max(INC_i,1) / max(BEST_i,1) in conflicts.

PRIMARY: geometric mean of rho_i over the non-decomposable (UNSAT) targets at
positions 30..59 of the l=5 cell. 95% interval: percentile bootstrap over those
targets, 10,000 resamples, numpy-free `random.Random(808)`.
- ALIVE if the upper bound is below 0.8.
- KILL if the lower bound is above 0.8.
- INCONCLUSIVE otherwise.
The verdict is this rule and nothing else.

Reference for reading it (not part of the rule): the minimum of three fresh arms
is biased low, so a no-reuse arm scores above 1 against BEST. The same statistic
with FRESH-A2 in the place of INC is reported as the null reference.

SECONDARY (reported, no verdict attached):
1. Position: OLS slope of ln rho_i on position, UNSAT targets, positions 1..59
   (position 0 has INC = FRESH-A by construction), per cell; 95% interval by
   pairs bootstrap (10,000, seed 809). Reported per target and per 10 targets.
   Same fit for FRESH-A2/BEST as the control slope. Reading: "reuse grows" only
   if the INC interval is entirely below 0.
2. Trend in l: the primary statistic at l=3 and l=4; difference of ln GM between
   l=5 and l=4 and between l=5 and l=3 with bootstrap interval (independent
   resampling per cell, seed 810). "Falls with l" only if an interval is
   entirely below 0, "rises" if entirely above, otherwise "holds / not resolved".
3. A/A noise floor: geometric mean and 95% bootstrap interval of
   FRESH-A2/FRESH-A on the primary set, and the standard deviation of the
   per-target ln ratio, per cell.
4. Descriptive: GM of INC over FRESH-A, FRESH-U, SPEC separately; INC2/BEST;
   the same on the first half, on all UNSAT targets, and on SAT targets; ratio
   of summed conflicts; median conflicts per arm; propagations for INC/BEST.
5. Next to every table: the non-SAT reference counts per target, not run here:
   linear 2+1 scan 2^l leaf solves (8, 16, 32), MITM 2^(2l)/2 + 2^l, enumeration
   2^(3l)/6. Units differ from conflicts; no ratio is formed.

## Validation before measurement (done, `validation_l3.json`)

(3,11), exhaustive over all 1,054 rational abscissae outside V: independent
triple enumeration = MITM ground truth; fixed CNF verdict = ground truth in the
parametric and in the constant form (0 mismatches each; the unfixed encoding
disagrees on 9); every SAT model lifts to rational points with
x(+-P1+-P2+-P3) = x(R); 20 of 20 planted targets SAT and lifted.

## Not covered

l >= 6; m = 4; other curves, other V, other target orders; Glucose 4, MiniSat,
CryptoMiniSat (native XOR); an incremental solver with learnt clauses deleted
between targets; assumption cores; learnt-clause statistics; wall time; any
comparison in common units with the linear 2+1 scan.
