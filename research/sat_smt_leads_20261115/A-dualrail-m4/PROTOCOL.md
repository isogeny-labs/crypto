# PROTOCOL: dual-rail encoding at m = 4 (lead A, 2026-11-15)

Written before any measured run (measured = cell seeds 2026111500 + l). Local pilot, one core,
shared 4-core container. Every number that follows from it is `[pcc:unverified]`. No benchmark
record, leaderboard row, `benchmarks/` entry or pcc claim.

## Claim under test

Point decomposition R = P1 + P2 + P3 + P4 (signs free), x(Pi) in V \ {0}, V = span(1, .., t^(l-1))
in F_{2^n}, n = 4l + 1, curve y^2 + xy = x^3 + x^2 + B. Chain with intermediate x-coordinates
u = x(P1 +- P2), v = x(R -+ Q): S3(x1, x2, u), S3(u, xR, v), S3(x3, x4, v) written in
(s, p) = (x3 + x4, x3 x4), then z^2 + sz + p = 0. Gates and rails as in `dr4.py`.

1. Parity: with (x1, x2) assigned, CryptoMiniSat on the dual-rail encoding needs O(1) conflicts
   per leaf (flat in l).
2. Sub-scan: a search over the 2l coordinates of (x1, x2) with learnt clauses kept costs fewer
   than 2^(2l)/2 leaves + conflicts, with a ratio falling in l.

## Prediction (written after development on seed 777000 + l, l = 3..5, before measured seeds)

1. Warm (one incremental solver): 1.5 to 5 conflicts per leaf, no growth in l. Cold (fresh solver
   per leaf): not O(1); hundreds to thousands, growing with l (development seed: about 100 at
   l = 3, 300 to 9 000 at l = 4, up to 54 000 at l = 5).
2. No arm below 1.0 at l = 6; budget-0 keep arm near 2.5 to 5; budget arms above it. Expected
   decision: KILL.

## Pre-registered rule

Primary metric per (arm, l): ratio = mean over the non-decomposable targets of
(leaf solver calls + conflicts) / (2^(2l)/2). For free CDCL, leaves = 0. Baseline named: the
pair scan, 2^(2l)/2 linear leaf solves. Trend = least-squares slope of log2(mean ratio) on l
over the three largest sizes. A run stopped by a conflict cap gives a lower bound and counts as
its lower bound; such arms cannot be "alive".

- ALIVE: some learning arm (keep-B3, keep-B10, keep-B100, core, free, any encoding/order variant
  of these) has ratio < 0.7 at l = 6, trend < 0, and ratio below its own discard control at l = 6.
- KILL: every arm (all keep, core and free arms) has ratio >= 1.0 at l = 6.
- otherwise INCONCLUSIVE.

Discard control: fresh solver per solver call. Full discard DFS runs at l = 2, 3 (all budgets) and
l = 4 (budgets 0 and 10). At l = 5, 6 the control is the sampled cold-leaf estimate
1 + mean cold conflicts per leaf (16 random leaves per target, cap 20 000 conflicts, lower bound if
capped); it stands in for every budget's discard arm and is labelled as such.

## Cells, targets, seeds

- l = 2, 3, 4, 5, 6; n = 9, 13, 17, 21, 25. Cell seed 2026111500 + l. B drawn from the seed,
  redrawn until sqrt(B) is not a product of two non-zero elements of V (see soundness).
  Amendment before any measured run: also redrawn until the factor base has at least
  2^(l-2) + 1 points up to sign (the first draw at l = 2 had an empty factor base and
  `validate.py` stopped before any solver call).
- 8 non-decomposable random targets and 4 planted targets per cell, the same for every arm.
  Targets: R != O, x(R) != 0, x(R) not in V, R not a sum of <= 2 signed base points.
  Planted (decomposable) targets are reported separately.
- CryptoMiniSat 5.18.0 via pycryptosat, one thread, options `autodisablegauss=0, sls=0`
  (the m = 3 pilot's options, unchanged).

## Arms (all on the same targets)

DFS over the 2l coordinates of (x1, x2); the driver drops nodes with x1 > x2 certain or
x1 = 0 / x2 = 0 fully assigned without a solver call (the pair scan's own symmetry), so the
budget-0 DFS visits 2^l (2^l - 1)/2 leaves. Leaf calls are unlimited up to a safety cap of
200 000 conflicts (a capped leaf is recorded, the run stops and is censored).

| arm | encoding | order | description |
|---|---|---|---|
| keep-B0 | dual | interleaved, MSB first | scan done by the solver, one incremental solver |
| keep-B3, keep-B10, keep-B100 | dual | interleaved | conflict budget at every internal node |
| keep-B0-blk, keep-B10-blk | dual | x1 then x2 | order control |
| keep-B0-d2, keep-B10-d2 | dual2 (rails on both gate inputs) | interleaved | encoding variant, added in development |
| core | dual | lex scan | decision-restricted learning, emulated: final-conflict core of each UNSAT leaf kept as a nogood over (x1, x2) coordinates; covered leaves are skipped |
| free | dual | none | one unrestricted solve, cap 100 * 2^(2l)/2 conflicts |
| free-d2, free-plain | dual2, plain AND gates | none | same cap |
| disc-B0, -B3, -B10, -B100 | dual | interleaved | fresh solver per call; sizes as above |
| cold | dual (16 leaves), dual2 (8 leaves) | sampled leaves | fresh solver per leaf, cap 20 000 |

Not available: decision restriction or priorities inside CryptoMiniSat (pycryptosat exposes
none); the core arm is the substitute. Planted targets: keep-B0, B3, B10, B100, core, free.

Reported as counts next to every table: pair scan 2^(2l)/2 leaf solves; meet in the middle
2^(2l) time and 2^(2l) memory; 3-scan enumeration 2^(3l)/24.

Per-run deadline: each process has a wall deadline below 10 minutes; arms not started are
written as NOT_RUN rows. Phase order: keep-B0, B3, B10, core, free, cold, B100, variants,
discard. l = 6 runs as two processes of 4 non-decomposable targets and one of planted targets,
one after the other.

## Soundness and validation (before measured runs, `validate.py 2 3` on the cell seeds)

- Defect fixed relative to the m = 3 pilot: intermediate x = 0. B is drawn so that
  S3(a, b, 0) = (ab)^2 + B has no zero with a, b in V; then neither a true decomposition nor a
  twist-point model can have u = 0 or v = 0. Clauses u != 0, v != 0, x1 != 0, x2 != 0, p != 0.
- Infinity: targets are not sums of <= 2 signed base points, so no intermediate sum of a true
  decomposition is the point at infinity; the encoding does not represent infinity.
- Checks: pair-table truth equals enumeration of all signed quadruples (all 24 targets at l = 2,
  12 of 24 at l = 3); free solves (plain, dual) agree with truth; every SAT model lifts to four
  curve points with signs summing to R; every leaf (x1, x2) answer equals the enumeration of
  decompositions using that pair; DFS (budgets 0, 3, 10; keep and discard; both orders) and core
  arms agree with truth. `run.py` asserts sat == truth and the lift on every run.

## Development before this file

Seed 777000 + l, l = 2..5: validation, timing probes, the cold-leaf observation in the
prediction, and the addition of the dual2 encoding. No solver option or budget was changed
after looking at measured seeds.
