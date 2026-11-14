# Idea 4 — XOR cubes on symmetric functions (pilot, 2026-11-14)

Pilot only: toy sizes, shared 4-core container, one core per run. Every number
below is an unverified pilot assertion, tagged `[pcc:unverified]`. No pcc
claim, benchmark record or leaderboard row was produced. The protocol
(`PROTOCOL.md`) was written before any measured run.

## What was run

m=3 point decomposition on `y^2+xy = x^3+x^2+1` over F_{2^n}, factor base
`x(P) in V`, `dim V = l`, target equation `S4(x1,x2,x3,x(R)) = 0` after Weil
descent to the 3l coordinates of x1,x2,x3 in a basis of V.

- Cells (l,n): (3,11), (4,13), (5,17) for SAT; plus (6,19), (7,23) for the
  algebraic part only. 8 targets per cell (seeds 1000*l+0..7; 0..3 planted,
  4..7 random; 4 targets at l=7). Truth by brute force over V^3:
  l=3: 4 SAT / 4 UNSAT; l=4: 5 SAT / 3 UNSAT (one random target is
  decomposable); l=5: 4 / 4.
- Part A, view X (`run_algebra.py`): exact ANF of the residual n bit
  equations after an XOR cube (x3_j = c_j + x1_j + x2_j for j<k, i.e. k
  coordinates of e1 fixed) or a literal cube (k coordinates of x1 fixed),
  k = 0..l, up to 16 cubes per (instance,k) (8 at l=7). Metric: F2-rank of
  the coefficients of monomials of degree >= 2 ("non-affine rank"; 0 =
  affine), same for degree >= 3, monomial counts.
- Part A, view E (`symbolic_e.py`): S4 rewritten exactly in (e1,e2,e3,xR,b)
  by symmetric reduction, checked numerically at 200 random points.
- Part B (`run_sat.py`): one CNF per instance (ANF encoding, one AND variable
  per nonlinear monomial, XOR rows chopped to 3-literal chunks, plus
  y_j <-> x1_j^x2_j^x3_j), CaDiCaL 1.5.3 (python-sat), arms differing only in
  assumptions: `plain` (untreated SAT arm), `lit-k` (all 2^k cubes on x1
  bits), `xor-k` (all 2^k cubes on e1 bits). All cubes solved, incremental,
  interleaved per instance. Fresh-solver-per-cube variant at l=3,4.
  CryptoMiniSat with native XOR rows at l=3,4 (wall time only; pycryptosat
  exposes no counters).
- Validation: planted tuple is a zero of S4; every SAT model decodes to a
  zero of S4 (asserted); SAT/UNSAT of every arm and cube set equals the
  brute-force truth (asserted, all passed); 243 of 275 CaDiCaL cube models
  also lift to real curve points summing to +-R (the rest are x-only zeros
  of S4, expected since S4 sees only x). CNF size: 1.6k vars/6.7k clauses
  (l=3), 7.8k/33k (l=4), 32.7k/138k (l=5).

Reproduce: `python3 symbolic_e.py symbolic_e.json; python3 run_algebra.py
algebra_rows.json; python3 run_sat.py sat_l3.json 3 11 8 --cms; ... ;
python3 analyze.py` (exact command lines in "Deviations" below).

## Results

### Part A — the residual never becomes affine

View X, non-affine rank (min..max over instances and cubes) `[pcc:unverified]`:

| l | n | XOR cube, k = 0..l-1 | XOR cube, k = l | literal cube, k = l | nonlinear monomials at k=l (xor / lit, mean) |
|---|---|---|---|---|---|
| 3 | 11 | 11..11 | 8..11 | 6..11 | 28.9 / 32.6 |
| 4 | 13 | 13..13 | 13..13 | 9..13 | 90.6 / 94.8 |
| 5 | 17 | 17..17 | 17..17 | 14..17 | 215.9 / 218.8 |
| 6 | 19 | 19..19 | 19..19 | 18..19 | 430.5 / 425.2 |
| 7 | 23 | 23..23 | 23..23 | 23..23 | 774.8 / 784.0 |

- The non-affine rank equals n (full) for every XOR cube with k < l in every
  cell, and stays full at k = l except for a few cubes at l=3. It never
  reaches 0. `[pcc:unverified]`
- For 0 < k < l the XOR-cube residual has MORE nonlinear monomials than the
  literal-cube residual on the same number of variables (e.g. l=6, k=3:
  6367 vs 3072; l=7, k=4: 12976 vs 5507); at k = l the counts are about
  equal. Maximum degree drops 6 -> 4 at k = l for both cube types.
  `[pcc:unverified]`
- The rank metric saturates at n because monomials outnumber equations by
  10x to 30x even at k = l; it shows "not affine", not "how far".

View E (exact, any n) `[pcc:unverified]`: S4 has 11 monomials in (e1,e2,e3):
`1, e3, e3^2, e3^3, e3^4, e2^2, e2^2 e3, e2^4, e1^2 e3, e1^2 e3^2, e1^4`.
With e1 fixed, `e3^3` and `e2^2 e3` remain, bit-degree 2 in (e2,e3): fixing
e1 completely does not make the system affine even in the symmetric
variables. Unplanned observation from the same list: e1 and e2 appear only
through squares and never multiply each other, so S4 is F2-affine in
(e1,e2) for fixed e3 (the analogue of "S3 is affine in (s,p)"). e3 is a
free n-bit field element, not an element of V, so cubing on e3 has about as
many cubes as there are tuples; this was not tested.

### Part B — conflicts: XOR cubes vs literal cubes vs plain SAT

Geometric-mean total conflicts over all cubes, CaDiCaL incremental
`[pcc:unverified]`:

| l | truth | N | plain | lit-2 | xor-2 | lit-l | xor-l |
|---|---|---|---|---|---|---|---|
| 3 | UNSAT | 4 | 482 | 512 | 515 | 467 | 557 |
| 4 | UNSAT | 3 | 4094 | 4573 | 4469 | 4658 | 4651 |
| 5 | UNSAT | 4 | 36430 | 34867 | 33449 | 35456 | 34598 |
| 3 | SAT | 4 | 136 | 280 | 397 | 301 | 500 |
| 4 | SAT | 5 | 387 | 3162 | 3534 | 4173 | 4378 |
| 5 | SAT | 4 | 6546 | 24324 | 27412 | 33007 | 33611 |

(`plain` on SAT targets stops at the first model; cube arms solve all cubes.)
Full k = 1..l tables for l=3,4 are in `results.json` / `analyze.py` output.

Paired ratio xor-k / lit-k, same instance (geometric mean, min..max)
`[pcc:unverified]`:

| l | k | conflicts, UNSAT | conflicts, SAT | conflicts, all 8 | propagations, all 8 | wall time, all 8 |
|---|---|---|---|---|---|---|
| 3 | 3 | 1.19 (1.03..1.44) | 1.66 (1.59..1.80) | 1.41 | 1.91 | 1.67 |
| 4 | 4 | 1.00 (0.97..1.04) | 1.05 (0.86..1.16) | 1.03 | 1.70 | 1.78 |
| 5 | 2 | 0.96 (0.94..0.98) | 1.13 (1.06..1.19) | 1.04 | 1.18 | 1.19 |
| 5 | 5 | 0.98 (0.97..0.99) | 1.02 (0.99..1.05) | 1.00 (0.997) | 1.70 | 1.64 |

- At the largest cell (l=5) every arm is within 8% of the untreated plain SAT
  arm in conflicts on UNSAT targets (xor-5/plain 0.95, lit-5/plain 0.97,
  xor-2/plain 0.92, lit-2/plain 0.96). `[pcc:unverified]`
- XOR cubes cost 1.2x to 1.9x the propagations and wall time of literal cubes
  on the same instance. `[pcc:unverified]`
- Fresh solver per cube (l=3,4): xor/lit conflict ratio 0.82..0.89 on UNSAT
  and 0.87..1.54 on SAT at k = l, with xor decisions 2x to 4x higher; both
  cube arms are 1.5x to 2x above plain at k = l. `[pcc:unverified]`
- CryptoMiniSat native XOR, k = l: xor/lit wall-time ratio 1.06..1.30 (l=3),
  2.2..2.4 (l=4). `[pcc:unverified]`

Slopes, log2(geometric-mean conflicts) against l, UNSAT targets, l = 3,4,5
`[pcc:unverified]`:

| arm | bits per l |
|---|---|
| plain SAT (untreated) | 3.12 |
| literal cubes, k = l | 3.12 |
| XOR cubes on e1, k = l | 2.98 |
| literal cubes, k = 2 | 3.05 |
| XOR cubes on e1, k = 2 | 3.01 |
| enumeration 2^{3l}/6 (count) | 3.00 |
| MITM 2^{2l}/2 (count) | 2.00 |
| linear 2+1 scan 2^l (count, repository baseline) | 1.00 |

Three points per fit and 3 to 4 UNSAT targets per cell; the 0.14 gap between
xor-l and lit-l comes from XOR cubes being worse at l=3 (ratio 1.19), not
from a gain at l=5. All SAT arms sit at about 3 bits per l, i.e.
about 2^{3l} conflicts (36k at l=5 vs 2^15 = 32.8k), against 2^l leaf solves
for the linear 2+1 scan baseline.

## Verdict against the pre-written kill criterion

- Clause 1 (non-affine rank does not reach 0 for k <= l-1): holds in every
  cell, l = 3..7. The prediction "residual becomes affine for some k < l" is
  not supported; view E shows it cannot be, even at k = l. `[pcc:unverified]`
- Clause 2 (XOR-cube total conflicts >= literal-cube total conflicts, judged
  at the largest l): the ratio is 0.997 over all 8 targets at k = l (0.98 on
  UNSAT, 1.02 on SAT) and 1.04 at k = 2 (0.96 UNSAT, 1.13 SAT). On the
  letter, ">= 1.0" holds on SAT targets and on the k=2 pooled figure and
  misses by 0.3% (pooled) to 4% (UNSAT only) elsewhere. No A/A arm was run,
  so a 2..4% difference cannot be separated from solver noise.
  `[pcc:unverified]`

Recorded verdict: **inconclusive on the letter of clause 2, prediction
refuted.** Reading it plainly: XOR cubes on e1 leave the conflict count where
plain SAT and literal cubes have it (about 2^{3l}), with 1.2x to 1.9x more
propagations, and the algebraic residual is never affine. Nothing measured
here moves the scaling toward the 2^l linear 2+1 baseline.

## Deviations from PROTOCOL.md

- l=3 generator: V is redrawn (same seeded stream) when it holds fewer than
  3 liftable x; found when the first algebra run crashed on one l=3 seed.
  Seeds whose first draw was fine are unchanged.
- l=5 SAT used k in {2,5} only (timing of the first instance: about 20 s per
  arm), the protocol's fallback subset minus the middle k. l=6 SAT was not
  run (one arm would exceed the 5-minute cap on this encoding). The slope
  therefore has 3 points.
- CryptoMiniSat arm only at l=3,4 and only plain / k=l.
- A timing probe on seeds 3000, 3001, 4000, 4001, 5000 was run before the
  measured runs to choose the k subset; those rows were regenerated in the
  measured runs, not reused.
- Command lines: `run_sat.py sat_l3.json 3 11 8 --cms`,
  `run_sat.py sat_l3_fresh.json 3 11 8 --fresh`,
  `run_sat.py sat_l4.json 4 13 8 --cms`,
  `run_sat.py sat_l4_fresh.json 4 13 8 --fresh`,
  `run_sat.py sat_l5.json 5 17 8 2,5`.
- The repository session log (`docs/session-logs/`) was not written, because
  this task may write only inside this directory.

## What a full study would need

- An A/A arm (same cubes, permuted cube order or solver seed) to put a noise
  floor under the 2..4% conflict ratios; more targets per cell.
- l = 6,7 SAT cells on a lighter encoding (the dense ANF encoding has
  2^{3l}-scale monomial counts), and the repository's own Semaev-chain CNF.
- A graded measure instead of the saturated rank: e.g. rank of the degree-2
  Macaulay/linearisation matrix, or the first fall degree of the residual.
- If the e3 observation is pursued: S4 is affine in (e1,e2) for fixed e3,
  but the constraint "x^3+e1 x^2+e2 x+e3 splits over V" is not linear and e3
  has about 2^{3l}/6 reachable values, so it needs its own protocol and a
  comparison against the 2^l linear 2+1 scan.
