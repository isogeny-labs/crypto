# Idea 8 pilot: target-parametric incremental solving (m=3 PDP, binary Koblitz curve)

Pilot, 2026-11-14. Toy sizes, one shared 4-core container, python-sat
(CaDiCaL 1.5.3, Glucose 4). Every number below is an unverified pilot
assertion, tagged [pcc:unverified]. Protocol and its pre-measurement
amendment: `PROTOCOL.md`. Raw rows: `raw_l{3,4,5}.jsonl`, `cores_l{3,4}.jsonl`,
aggregated in `results.json`; full printed tables in `analysis_stdout.txt`.

## Verdict against the pre-written kill criterion

**Inconclusive, leaning negative.** The kill criterion needs both (1) INC /
FRESH-A conflicts > 0.7 on the second half of the sequence at the largest l
and (2) smallest minimal core >= 0.75 n at the largest l.

- (1) is 0.699 at l=5 (8 targets), i.e. on the threshold, not above it. By
  the letter the criterion is not met. The ratio does not shrink with l or
  along the sequence, the fitted exponent is the same in both arms, and
  against the untreated constant-target arm (SPEC) the ratio is 0.80 to 0.89.
- (2) was not measured at l=5 (the core run did not finish one target in the
  budget). At l=3 and l=4 the minimal cores are well below 0.75 n, but they
  sit exactly on the counting null (random set of matched density), so they
  give no evidence of a cheap necessary condition.

No arm comes near the non-SAT baseline: the linear 2+1 scan needs 2^l leaf
solves per target (32 at l=5); the INC arm needs a median of 8,360 conflicts
per non-decomposable target at l=5.

## What was run

- Curve y^2+xy=x^3+1 over F_{2^n}; V = random l-dimensional subspace; cells
  (l,n) = (3,11), (4,13), (5,17). Targets: planted sums of three base points
  (decomposable) and random rational points verified non-decomposable by a
  real-point 2+1 meet-in-the-middle; all distinct, x(R) not in V; one fixed
  shuffled order per cell. Seeds: 20261114 + 1000 l (setup), +1 (targets).
- Encoding (all SAT arms): S4 as the resultant in t of the two S3 quadratics,
  AND gates for bit products, 3-input XOR chains, lex symmetry breaking
  c1<=c2<=c3. Parametric form: x(R) is n Boolean variables set by assumptions.
- Arms per target, interleaved: INC (one solver for the sequence), FRESH-A
  (new solver, same CNF, same assumptions: isolates "solver state kept"),
  FRESH-U (new solver, target bits as unit clauses), SPEC (new solver, CNF
  regenerated with constant target: the untreated SAT arm). Glucose 4 ran INC
  and FRESH-A only, FRESH-A with DRUP logging.
- Validation: l=3 exhaustive triple enumeration equals the MITM ground truth
  on all 1,057 rational abscissae (48 decomposable); solver verdict equals
  ground truth on 44 targets in both parametric and constant form; all SAT
  models lift to rational points with x(+-P1+-P2+-P3)=x(R). In the measured
  cells: 52 targets x 6 arms, 0 verdict mismatches, 0 lift failures.
  [pcc:unverified]

## (a) Reuse: conflicts per target, same targets, same order

Median conflicts per target, CaDiCaL. [pcc:unverified]

| l | n | targets (dec+non) | class | INC | FRESH-A | FRESH-U | SPEC | enumeration 2^(3l)/6 | MITM 2^(2l)/2+2^l | linear 2+1 leaf solves 2^l |
|---|---|---|---|---|---|---|---|---|---|---|
| 3 | 11 | 6+6 | UNSAT | 276 | 348.5 | 430.5 | 283.5 | 85 | 40 | 8 |
| 3 | 11 | 6+6 | SAT | 182.5 | 209.5 | 233.5 | 159.5 | | | |
| 4 | 13 | 12+12 | UNSAT | 1377 | 1953 | 1920 | 1514 | 683 | 144 | 16 |
| 4 | 13 | 12+12 | SAT | 720 | 1385 | 1496.5 | 626.5 | | | |
| 5 | 17 | 8+8 | UNSAT | 8359.5 | 10854 | 9566.5 | 9087 | 5461 | 544 | 32 |
| 5 | 17 | 8+8 | SAT | 5833 | 7204.5 | 5772.5 | 5900.5 | | | |

Ratios of summed conflicts, CaDiCaL (Glucose 4 in brackets). [pcc:unverified]

| l | INC/FRESH-A first half | INC/FRESH-A second half (kill metric) | second half, UNSAT only | INC/SPEC second half | INC/SPEC all | FRESH-A/SPEC all |
|---|---|---|---|---|---|---|
| 3 | 0.842 (0.656) | 0.685 (0.767) | 0.712 (0.734) | 0.842 | 0.919 | 1.218 |
| 4 | 0.689 (0.653) | 0.666 (0.665) | 0.649 (0.660) | 0.909 | 0.982 | 1.449 |
| 5 | 0.887 (0.695) | 0.699 (0.536) | 0.716 (0.760) | 0.802 | 0.890 | 1.114 |

Readings:

- The INC/FRESH-A ratio is about 0.65 to 0.75 at every l and does not fall
  with l. Part of it is the baseline, not the treatment: a fresh solver under
  assumptions needs 1.11 to 1.45 times the conflicts of the constant-target
  encoding (FRESH-A/SPEC). Against SPEC the incremental arm is at 0.89 to 0.98
  over all targets and 0.80 to 0.91 on the second half.
- No decline along the sequence: the per-position curve (in `results.json`,
  `reuse_curve_cadical`) is flat around 0.6 to 0.9 on UNSAT targets from the
  second target on; the large excursions (0.085, 4.0, 5.8) are SAT targets,
  where the time to the first model is luck.
- Glucose 4's 0.536 at l=5 is driven by SAT targets (second-half SAT 0.281,
  UNSAT 0.760, 3 to 5 targets each).
- Propagations: INC/FRESH-A second half 0.777, 0.534, 0.643 (CaDiCaL).
- Table-exhaustion control: decomposable targets used against decomposable
  abscissae available: l=3 6 of 48 (fails the control, flagged), l=4 12 of
  about 341, l=5 8 of about 2,731 (heuristic 2^(3l)/12). All targets were
  never seen before by the solver.

Slopes: see the last section.

## (b) Target bits in learnt clauses (Glucose 4, FRESH-A, DRUP log)

Literals on the n target variables r per learnt clause. [pcc:unverified]

| l | class | learnt clauses | share with 0 r-literals | share with <=2 | median | p90 | share with 0 target-cone literals | mean 2^-(r-literals) |
|---|---|---|---|---|---|---|---|---|
| 3 | UNSAT | 2,215 | 0.441 | 0.583 | 1 | 7 | 0.255 | 0.509 |
| 4 | UNSAT | 29,647 | 0.343 | 0.390 | 6 | 10 | 0.250 | 0.370 |
| 5 | UNSAT | 118,070 | 0.280 | 0.304 | 8 | 12 | 0.257 | 0.293 |
| 5 | SAT | 66,487 | 0.430 | 0.450 | 6 | 10 | 0.272 | 0.443 |

The distribution is bimodal: 28 to 44% of learnt clauses carry no target bit
(about a quarter touch no target-cone gate at all, at every l), the rest carry
roughly half of the target (median 8 of 17 at l=5). The target-free share
falls with l on UNSAT targets (0.44, 0.34, 0.28). The target-free quarter is
consistent in size with the constant-factor reuse seen in (a); it does not
change the exponent.

## (c) Assumption cores over target bits (UNSAT targets, Glucose 4)

The solver's first core is the full target in 25 of 26 UNSAT solves (one core
of 10 of 11 at l=3). Deletion-minimal cores, two bit orders, against the
counting null (greedy deletion on a random set of matched density, 300
trials, `core_null.py`). [pcc:unverified]

| l | n | targets | minimal core sizes (best of two orders) | smallest / n | null median (p10..p90) | density used for null |
|---|---|---|---|---|---|---|
| 3 | 11 | 6 | 6, 5, 6, 6, 5, 5 | 0.455 | 6 (5..6) | 0.053 (solver, 300 uniform abscissae) |
| 4 | 13 | 4 | 8, 8, 8, 8 | 0.615 | 8 (7..9) | 0.080 (solver, 150 uniform abscissae) |
| 5 | 17 | 0 (not finished) | n/a | n/a | 11 (10..11) | 0.042 (heuristic) |

Minimal cores are smaller than n, but by exactly what a structureless set
of that density gives: a core of size k only says that 2^(n-k) abscissae in
one affine family are all non-decomposable (about 32 at these densities). No
core is smaller than the null range. Nothing here is a decomposability filter.

## Slopes (bits per l, least squares on log2 median UNSAT conflicts, l=3,4,5)

[pcc:unverified] CaDiCaL: INC 2.46, FRESH-A 2.48, FRESH-U 2.24, SPEC 2.50.
Glucose 4: INC 2.66, FRESH-A 2.64. Three sizes, n changes with l (11, 13, 17),
so the slope mixes l and n. Reference slopes: enumeration 3.0, MITM 2.0,
linear 2+1 scan 1.0. One calibration solve at l=6, n=19 (not a measured cell)
took 112,195 conflicts and 106 s (FRESH-A, CaDiCaL, UNSAT), which is 3.4 bits
above the l=5 median.

## Deviations and limits

- The first encoding (chain through t) was replaced before any measured row;
  l=6, 7 were dropped for time; symmetry breaking added in all arms
  (PROTOCOL.md, Amendment 1). Largest measured l is 5, so the kill criterion
  is evaluated there with 8 second-half targets.
- Core minimisation at l=5 did not complete a single target; at l=4 it
  completed 4 of the 6 planned.
- The run.py draw-based density estimate was useless at these sample sizes
  (0 decomposable among 6 to 12 draws); the table uses the exact l=3 count and
  the 2^(3l)/12 heuristic for l=4, 5.
- One sequence order, one V, one curve per cell; no A/A arm (two fresh solvers
  on the same target are deterministic, so the noise floor is across targets
  only).
- The repository session log, knowledge-base lookup and Conductor task were
  not done: the task restricted writes to this directory and those tools are
  not installed here.

## What a full study would need

- l up to 7 or 8 with native XOR + Gauss (CryptoMiniSat or WDSat) so that one
  solve is about one conflict per tuple, and at least 30 targets per class.
- A second baseline for (a) that removes the assumption handicap: an
  incremental solver whose learnt clauses are deleted between targets.
- Sequences long enough to see whether INC/SPEC keeps falling (here 12 to 24
  targets), still far below the number of decomposable abscissae.
- Cores over other coordinates of the target (a different basis, the bits of
  x(R)^2 and other powers that enter the equations linearly) and exact
  minimum cores instead of deletion-minimal ones, each against the same null.
- m=4.

## Rerun

`python3 validate.py 11 3` (validation, about 3 s), `python3 run.py 11 3 6 6
raw_l3.jsonl` (smallest cell, about 3 s), `python3 run.py 13 4 12 12
raw_l4.jsonl`, `python3 run.py 17 5 8 8 raw_l5.jsonl` (about 9 min),
`python3 cores.py 11 3 6 6 6 cores_l3.jsonl`, `python3 core_null.py`,
`python3 analyze.py`.
