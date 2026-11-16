# PROTOCOL: N4 v2, wider-range plane-quartic index calculus vs plain and negation-map rho on E0(K)

Frozen 2026-10-20 in the commit that adds this file, **before any data** (pilot or grid). This file is
not edited afterwards; revisions are recorded in the README as deviations. Parameters that this protocol
says are set by the pilot stage are written to `PARAMS.md` and committed before the first grid unit,
by the rules in "Pilot stage" below. Branch `claude/quartic-n4v2-20261020`, cut from
`origin/claude/quartic-pic-canonical-20261018` (which contains N4, #2283).

## Question

N4 (`research/quartic_n4_20261019/`, frozen B0) measured, on the GHS-weak family `E0/F_{q^3}`
(`#E0(K) = 4 ell`), IC-B0 cold cost exponent 1.083 [1.04, 1.13] against plain rho 1.485 over
q = 2^12..2^18, with linear algebra 57-74% of the cost, a hash-order-nondeterministic relation stage, and
no negation map in rho. This study asks, on the same family:

1. Over a wider range (q up to 2^22), does the IC cost exponent bend toward 1 (double large primes,
   model `O~(q)`, Diem) or toward 6/5 (single large prime, model `q^{6/5}`)?
2. Where is the crossover between IC and rho, in F_q-multiplication equivalents (M-eq, with the
   section-5a band) and in wall time, against plain rho **and** a negation-map rho?
3. What does each of four changes buy against the frozen B0, measured as its own arm: (a) double
   large primes with graph-cycle recombination; (b) structured Gaussian elimination (SGE) then block
   Lanczos instead of plain Lanczos on `A^T D A`; (c) sorted bucketing so a unit replays bit for bit;
   (d) rho with the negation map alongside plain rho?

## Instances

`q_k` = the largest prime `< 2^k` with `q = 7 (mod 12)`, `k = 8..22` (N4's rule; N4's list for
k = 8..18 is reproduced). `k = 22` is the top because N4's builder and solver keep `#E0(K)` and `ell`
in 64-bit words (`ell ~ q^3/4 < 2^64` requires `q < 2^22`); the new code (not B0) makes the mod-`ell`
additions overflow-safe for `ell` up to `2^64`. Per `(q, seed)` the instance is `ghs.rs::Inst::search`
of N4, unmodified (first `u` in the seeded stream with all of N4's conditions: `#E0(K) = 4 ell`, `ell`
prime, the order confirmed on a second point, quartic over `F_q`, smoothness certificate). Seeds 1..5
for `k <= 16`, 1..3 for `k >= 17` (N4's seeds, so for k <= 18 the instances are N4's).

The instance list (`q, seed, ell, order, f, tries`) is generated once by `v2 instances`, written to
`results/instances.jsonl` and committed with its sha256 before the grid. The W2a list of
`origin/codex/diem-durable-checkpoints-20261019` (`research/diem_quartic_relations_20261018/`) is used
only if its frozen list is complete; at the time of writing that branch has no `instances.jsonl` and no
`FROZEN.sha256`, and its partial-validation record lists `(k, i) = (20, 0..2), (21, 0..2)` missing, and
its construction differs (`q = 3 mod 4` smallest prime `>= 2^k`, SplitMix seeds, cubic modulus). It is
**not** reused; this is recorded in the README. That branch is not modified.

The planted target `k` and the rho targets are drawn from the seed as in N4 (`Rng(seed ^ 0x1234)`).

## Arms

All arms share N4's counted primitives (`common.rs`: `F_q` mul/inv/add, `K` mul = 11, mod-`ell`
mul `lmul`, add `ladd`, unpriced `hash`/`cmp`) and N4's `Pic^0` library (`plane_quartic_pic.rs`).

* **B0** (frozen baseline): N4's `solver::ic_solve` and N4's unit retry loop, code included by path
  from `research/quartic_n4_20261019/` unmodified. `N = ceil(3.0 q^{3/5})`; lines through two
  factor-base points bucketed in a global `HashMap`; 3 points = one-large-prime relation, 4 = full;
  rows `ceil(1.2 n) + 30`, 60 hold-out; Lanczos on `A^T D A`; descent; verify. Retry `c += 0.4`, up
  to 6 attempts, every attempt charged. Run for `k <= 21` only (its global hash table holds `N^2/2`
  entries: about 4.6 GB at k = 21 and 9 GB at k = 22; with about 14 GB free disk and a shared 48 GB
  host, k = 22 is not run for B0, and k = 21 B0 units run one at a time).
* **S** (change c, sorted bucketing): B0 with the relation stage replaced by per-point sorted
  bucketing: for each `i`, the normalised lines `(i, j > i)` (same cross product and batch inversion,
  so the same counted `F_q` multiplications) are sorted by `(key, j)`; a run of equal keys with at
  least two `j` is a line with at least 3 factor-base points, of which `i` is the smallest; the
  4th point of a 3-point line is computed as in B0 and the line is dropped if it is a factor-base point
  (it was found from a smaller index), exactly B0's relation set. Large-prime grouping uses ordered
  maps; no hash-map iteration anywhere. Memory `O(N)` per point instead of `O(N^2)`. Linear algebra:
  B0's Lanczos (same function, overflow-safe mod-`ell` adds). Same `c`, rows, hold-out, retry rule.
* **L** (change b): S's relations and matrix, then SGE (below) then **block Lanczos** with block width
  `b = 4` on `B^T D B` of the reduced matrix `B`, multiplications by matrix coefficients `+-1` replaced
  by additions; eliminated columns recovered by back substitution. Decomposition arms on the **same**
  matrix (reported as attribution, not as separate improvements): **L0** = scalar Lanczos with the
  `+-1` skip and no SGE; **L1** = SGE + scalar Lanczos with the `+-1` skip.
  SGE: (1) singleton columns removed with their row (cascading); (2) heaviest rows dropped until the
  excess `rows - cols` is `max(20, ceil(0.02 cols))`, interleaved with (1); (3) greedy merges: a column
  of weight `w <= 32` is eliminated with its lightest row as pivot while the model cost
  `J = n * nnz + gamma * n^2` decreases (`gamma` from PARAMS.md, default 4); the `kappa` column is
  never eliminated. Every row operation is counted (`lmul` for coefficients outside `{+-1}`, `ladd`).
* **D** (change a, double large primes): factor base `N = ceil(c_D q^{1/2})` (`c_D` from the pilot),
  `P0` as in B0. For every pair `i < j` the residual quadratic of the line is computed from the
  gradients and Hessians of `F` at the factor-base points (precomputed once per point), its roots
  by one square root in `F_q`; a pair gives a relation `P_i + P_j + L1 + L2 = kappa` when both
  residual points are rational. Lines with 3 or 4 factor-base points are emitted once (from their
  two smallest factor-base indices); multiplicities from tangency are kept. Large primes are vertices
  of a graph with one root vertex standing for "known"; a relation with 2 large primes is an edge,
  with 1 large prime an edge to the root, with 0 a full row. A BFS spanning forest (root component
  first, then other components in vertex order) is built; every non-tree edge gives one factor-base
  relation (even cycle: the cycle; odd cycle in the root tree: cycle plus the path to the root; in a
  non-root tree the first odd cycle anchors the tree and later non-tree edges give relations through
  the anchor). Rows are taken in increasing estimated weight (path length) until `ceil(1.2 n) + 30`
  rows plus 60 hold-out rows. Linear algebra: B0's Lanczos. Logs of every large prime in the root
  component and in anchored components are computed by propagation along the tree; descent as in B0
  with this larger known set. Retry on failure with `c_D += 0.25`, up to 6 attempts, all charged.
  Deterministic (no hash-map iteration; hash maps are used only for lookups).
* **DL** (a + b): D's relations and matrix, then SGE + block Lanczos (`b = 4`) as in L. Decomposition
  on the same matrix: **DL1** = D + SGE + scalar Lanczos. **DL is the pre-registered headline IC arm
  for the crossover.**
* **rho** (plain, N4's): N4's `solver::rho` walk (one trail, 16-adding walk, partition by a hash of
  `x`, distinguished points `2^dbits ~ sqrt(ell)/64`, restart after `20 * 2^dbits` steps without a
  distinguished point), re-implemented with overflow-safe mod-`ell` adds (identical walk for
  `ell < 2^62`).
* **rho-neg** (change d): the same trail/DP design on classes `{P, -P}`: canonical representative
  (the `y` whose first nonzero coordinate is `< q/2`), 32-adding walk with multipliers
  `a_j G + b_j Q`, the look-ahead rule (if the next point has the same partition index, use the next
  index, up to 32 times) against fruitless 2-cycles, a cycle check against a snapshot taken every 128
  steps, and escape from a detected cycle by doubling its minimal point (deterministic in the cycle).
  Every look-ahead addition, cycle walk and doubling is counted. Canonicalisation and cycle checks are
  counted as unpriced `cmp` operations.

Arms that share a deterministic phase execution (L0, L1, L share S's relations and matrix; DL1, DL
share D's) are evaluated on **one** execution of the shared phases; each arm is charged the full counted
cost and wall time of every phase it would run (cold), plus its own linear algebra. If two arms' kernel
vectors, normalised to `eta = 1`, agree, the log recovery, descent and verification run once and are
charged to both; otherwise each runs its own. A failing arm retries alone (shared phases re-run and
charged to it).

## Cost unit and pricing (unchanged from N4)

`M-eq = mul + r_inv inv + r_lmul lmul`, with `r_inv` and `r_lmul` the measured time ratios of N4's
`ratios()` (median over the units of a size). `F_q` and mod-`ell` additions are counted, not priced.
`GAE` = counted M-eq of one `E0(K)` addition. **Section-5a band:** unpriced native operations (IC: hash
inserts/lookups, sort keys, factor-base and large-prime lookups; rho: distinguished-point/partition
tests, canonicalisation, cycle checks, table operations) priced at **w = 0, 0.1, 1 GAE** each, for every
arm. For S, L and DL the number of sort comparisons is also recorded (sensitivity, not in the band).
An order between two arms is stated only where it holds at all three weights with the bootstrap 95%
interval of the ratio excluding 1 ("band-resolved").

## Measurements, fits and crossovers

* Per arm and size: mean cold M-eq over units (every attempt charged; failed units are listed and
  their cost reported separately), phase means and shares, wall time; rho: mean over runs. Ratio
  IC/rho with a 95% bootstrap interval (2000 resamples of units and runs), at the three band weights.
* **Exponent fits** `ln cost = a + e ln q` on size means, 95% bootstrap over units and runs, for
  three ranges: **full** `q >= 2^12` (k = 12..22, B0 to 21), **low** k = 12..16, **high** k = 17..22
  (B0: 17..21). Also the curvature `e_high - e_low` with its bootstrap interval, and per-phase
  exponents.
* **Bend decision rule** (per IC arm, on `e_high`, at w = 0 and w = 1): *toward 1* if the 95%
  interval contains 1.0 and excludes 1.2; *toward 6/5* if it contains 1.2 and excludes 1.0; *below 1*
  or *above 6/5* or *between* (interval inside (1.0, 1.2)) are stated as such; *unresolved* if it
  contains both.
* **Crossover in M-eq** (each IC arm vs rho and vs rho-neg, at each band weight): (i) the resolved
  crossover = the smallest grid `q` from which the arm is band-resolved lower at every larger grid size;
  (ii) the interpolated `q*` where the log of the mean ratio crosses 0 (linear in `ln q` between the
  bracketing sizes), with a bootstrap interval from the per-size resamples. If the ratio never crosses
  1 in the grid, `q*` is reported as below `q_min` (or above `q_max`) with an extrapolation from the
  fitted exponents, labelled as extrapolation.
* **Crossover in wall time**: the same on mean wall seconds per cold solve (single thread), with the
  host load recorded; wall time is a practicality note (L0 isolation), never a claim of speed.
* rho secondary: per size, the per-step M-eq and the pooled calibration `steps / sqrt(ell)` (all runs of
  all sizes) give a model rho cost; it is reported next to the measured mean, and the headline uses the
  measured mean.

## Controls

* Correctness: every IC unit's `k` and every rho run's `k` is verified by scalar multiplication
  `kG = Q` on `E0`; a failure counts 0 of 1. Sampled relations (10 full, 30 large-prime; for D also 30
  cycle rows) are checked through the canonical `Pic` group law (B0's check; for D cycle rows: the
  row's combination of point classes is checked to be zero). Hold-out rows in the kernel
  (`holdout_bad = 0`).
* rho calibration: plain `steps / sqrt(ell)` per size in [1.0, 1.5] (theory 1.25); rho-neg in
  [0.75, 1.15] (theory `sqrt(pi/4) = 0.886`); outside: that rho arm is not used at that size.
* Replay control (arm c): a second execution of one unit per size (k <= 18) and of one unit at
  k >= 19 for S/L/D/DL must produce identical records (all counts, relation counts, recovered `k`;
  wall times excluded). B0 replays are recorded for comparison.
* Instance control: N4's `check` (N1 = q + 1 by sampled points above 8191, transfer homomorphism,
  `#Jac * x = 0`) for one seed per size.

## Host and execution

Apple M4 Pro (10P + 4E cores), 48 GB, shared with other jobs (load average about 70 when this was
written): L0 isolation. At most 4 single-thread processes under `nice -n 10`. Native Rust built with
`rustc -O` (as N4); `CARGO_TARGET_DIR=/Users/adamburan/pq-work/cargo-target` for any cargo command.
Raw outputs compressed. Disk guard: if `df` on `/System/Volumes/Data` shows < 5 GB free, the run stops.

## Pilot stage (before PARAMS.md, data not used in results)

Seeds 101, 102 (not grid seeds). (1) Timing pilots of B0, S-family, D-family and both rho arms at
k = 14, 16, 18 and one unit of S/D at k = 20; extrapolate wall time per unit and per rho run to k = 22
with the fitted exponents. (2) `c_D` = the value in {1.5, 2.0, 2.5, 3.0} with the lowest mean D cold
M-eq (w = 0) over the pilot units at k = 16 and 18 that verified. (3) `gamma`: 4 unless L1 at k = 18
pilot fails; then 1. (4) rho runs per instance: the largest counts (N4's per-k counts for k <= 18, at
least 2 per instance at k >= 19 for each rho arm) such that the estimated grid total is below 20 h wall
on 4 processes; if not possible at k = 22, k = 22 runs with 1 instance's rho runs, or is dropped, stated
in PARAMS.md. PARAMS.md records the pilot numbers behind each choice.

## Pre-registered predictions

* **P1** every unit of every IC arm verifies; `relations_bad = 0`, `holdout_bad = 0`.
* **P2** plain rho exponent `1.50 +- 0.10`; rho-neg exponent `1.50 +- 0.10`; rho-neg / rho cost
  (pooled over sizes k >= 12) in [0.60, 0.85] (theory `1/sqrt(2) = 0.71` plus look-ahead and cycle
  overheads).
* **P3** (c) S, L, D, DL replay bit for bit at every replayed size; S's counted `F_q` multiplications
  in the relation phase equal B0's within 1% on the same instance; S's relation-phase wall time is
  below B0's at k >= 18.
* **P4** (b) SGE reduces the linear-algebra dimension by a factor >= 2 at k >= 16; L1's LA-phase M-eq
  is below S's by a factor >= 2 at k >= 16; L's (block, b = 4) LA M-eq is between 1x and 3x L1's
  (block vector operations cost `b^2` per entry in a large prime field) and below S's.
* **P5** (a) D is below S in cold M-eq at every k >= 14; D's and DL's `e_high` is in [0.95, 1.15]
  (toward 1); B0's and S's `e_high` is in [1.10, 1.30] (toward 6/5), i.e. B0 steepens from N4's 1.083.
* **P6** crossover (M-eq, band-resolved): DL vs rho-neg below `q = 2^12`; B0 vs rho-neg between `2^12`
  and `2^14`. Wall time: B0 vs rho-neg between `3e4` and `3e5`; DL vs rho-neg below `2^15`.

## Decision rules

* Each change is an **improvement over B0** (success criterion, AGENTS.md) at a size where its arm's
  cold M-eq is band-resolved lower than B0's on the same instances, with every answer verified.
  (c) is an improvement in replayability if P3's replay holds; a cost-neutral change is recorded as
  engineering, not as a cost improvement. The ratchet baseline after this study is the lowest
  band-resolved arm at the top size.
* The lead N4 stays **open** iff P1 holds and the exponent gap `e_rho-neg - e_IC` (full range, best
  arm, w = 0 and w = 1) has a bootstrap interval above 0; **closed** otherwise (owner's stop rule).
* Bend: the rule above, per arm.

## Claim hygiene

GHS-weak family only (`E0/F_{q^3}`, `4 | #E0(K)`, special `m = 2` form); no generic-curve,
prime-order, deployed-curve, standard-curve or key-recovery claim. A crossover or exponent is a
measured fit over the stated range, not an asymptotic theorem. No external implementation is
compared (none installed): no fastest-implementation claim. rho is single-trail with one inversion per
step (no simultaneous inversion): a batched-inversion rho would be cheaper in M-eq; this is stated as a
limit, not measured.
