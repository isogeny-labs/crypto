# Instrument fixes and reruns: protocol (2026-11-12)

Step 2 of `research/chained_prime_transfer_20261111/NEXT_STEPS_20261112.md`
(owner-approved). Frozen by `frozen.sha256` before the first run of
`instrument_rerun`. Every prediction and number here is `[pcc:unverified]`.

## Question

Three defects of the chained-transfer instrument touch every recorded distance
to the matched rho. How far does each recorded distance move when they are
fixed, and does any recorded verdict change?

## The defects and the fixes

1. **Genus-2 linear-algebra counter (v3 path, `measure.rs`).** The recorded
   `ic_jac_la_mulmods` is the `ops` of `solve_linear_inner`: one tick per
   element update of a dense Gauss-Jordan. V3.md observes it equals
   `(fb+2) x rows` in all four rows, a matrix-size count. It does not count the
   modular inversion of each pivot (about `2 log r` mulmods) or any addition.
   Fix: `instrument::solve_linear_counted` counts every mulmod and every
   add/sub of the same elimination (inversions included). The old counter keeps
   its name and values (`genus2_index_calculus`, `solve_linear_free`).
   Two new arms isolate effects:
   - **L** = the old relations (same collector, same random stream) with the
     counted elimination. This is pure accounting.
   - **A, B, C** = `ic_control::run_ic` (walk collection, structured peeling,
     counted Wiedemann on `A^T D A`, graph reduction on large primes) on the same
     instance: A = every degree-1 place, B = single large prime
     `nb = ceil(2 p^(1-1/(g+1)))`, C = double large prime
     `nb = ceil(2 p^(1-1/g))`. The same formula gives `2 p^(3/4)` and
     `2 p^(2/3)` at `g = 3` (the frozen positive-control values). A, B, C are
     algorithm changes (a walk costs one group operation per trial, the old
     collector two scalar multiplications), not accounting.
2. **Matched rho.** Reported as walk operations, set-up and other operations
   separately, native canonicalisations separately. Primary reference: the
   negation-map walk (`RhoWalk::negation()` through `rho_reference_walk`), total
   = every group operation (set-up, walk, verification), the same accounting as
   the IC arms. Second column: plain rho total. Third: negation walk operations
   alone (a lower bound that gives rho the set-up for free).
3. **Double large primes** for genus 2 and genus 3: arm C above. `ic_control`'s
   `run_ic` hard-wired `deg == 3`; it now reads `deg == genus` (a one-line edit,
   genus-3 behaviour unchanged, checked by the existing tests).

Not ported to `small_de` (elliptic `m = 2, 3` decomposition, no divisor class
group): large primes. Its linear algebra is already a counted sparse
elimination (`solve_sparse`, multiply-adds of real entries) and its matched rho
already includes the negation map and, for `D = -3, -4`, the automorphism walk.
For `small_de` only fix 2 (the rho split) applies.

## Pricing

Each total is `group_ops + w * native`, with native the IC's relation-phase and
linear-algebra multiplications (`rel.mul + la.mul`; additions as a labelled
sensitivity) and the rho's canonicalisations, at `w = 0, 0.1, 1` (AGENTS.md
5a). The order of two arms is stated only where it holds at all three. The
host-calibrated point is informational and never enters a verdict:
`w = 0.005` for relation-phase work (mod `p`), `0.013` for linear algebra
(mod `r`), `0.005` for rho canonicalisations, the values the positive control
measured relative to one Cantor addition. Unit: one group operation (Cantor or
elliptic addition) = 1 GAE; a Cantor step costs several elliptic additions, so
every IC/rho ratio across group types understates the gap.

The transfer (tower build, pull-back) is one-time and in no per-instance total.

## Reruns (binary `src/bin/instrument_rerun.rs`, three subcommands)

- **v3** (`chained_transfer_bench` instances): `p = 211, 419, 821, 1619`, the
  identical candidate search and seeds (`0xC0FFEE + 101 i + s`). Gate: the
  binary must reproduce the recorded `rho_src_ops` (428, 357, 370, 847), IC
  group ops (3,419; 8,475; 11,353; 34,521) and LA mulmods (14,633; 51,976;
  164,828; 603,721) exactly, else it aborts. Then: arm L (old relations, seed
  as recorded), arms A, B, C with 8 algorithm seeds, rho plain and negation
  with 32 seeds each, on E1. The solved instance is the real pulled-back pair
  `(phi^*(P), phi^*([x]P))`, loaded into `H64Curve` (conversion checked by
  random operations against `Genus2Curve`).
- **tower** (`tower_large` instances): `p = 251, 503, 1009, 2003, 4001, 8009,
  16001` (the recorded primes up to 16001; 32003 and 64007 are not rerun, to
  stay under about 60 minutes), 3 instances per prime with the recorded
  construction (`build64(p, 1000 + 17 i + p mod 997, p/4, 64)`), both rungs
  (g = 2, 3), arms A, B, C with 4 seeds each, rho on E plain and negation with
  64 seeds each. Rungs whose `#J/r` shares a factor with `r` are reported as
  skipped, not dropped silently.
- **smallde**: the `small_de_ic` cells at 24 and 28 bits (`m = 2, 3`,
  `D = -3, -4, -7, -8, -11, 0`), seed 20261112, 16 targets, repetitions
  `24:6, 28:3`. IC arms unchanged; rho plain, negation, automorphism with
  walk/set-up split.

Every recovered log (IC and rho) is certified by an independent `[k]G == T`
(`mul_rtl` on a Jacobian rung, `Curve::mul` on `E`); an uncertified log counts
as a failure and is listed.

## Old values compared

v3: `results_v3.json` (ratio of IC group ops, LA mulmods priced, to `rho_src_ops`).
tower: `tower_large.json`, `ratio_la` (IC + w LA mulmods over `rho_ops`, plain
rho, set-up included). small_de: `small_de_derived.json`, single-instance IC over
the best matched rho and the amortised ratio at `w = 0.1`.

## Frozen predictions (before any new data)

P1. V3 pure accounting (arm L): the real elimination count is larger than the
recorded counter in every row, by a factor between 2 and 60 (a dense
Gauss-Jordan with fill-in costs about `n^3`, the recorded count `n^2`).
Probability 0.75. Falsified if any row has a ratio below 1.5 or above 100.

P2. V3 relation phase: arms A, B, C need fewer group operations than the old
collector (about 1 per trial against about `2 log2 r + 1`). C over negation
rho at `w = 0` falls from 8.0, 23.7, 30.7, 40.8 into 1.0 to 8.0 at every `p`.
Probability 0.7. Falsified if any p lies outside.

P3. V3 verdicts: IC (arm C, and the best arm of A, B, C) stays above the
negation-map rho at all three prices at every p, as recorded ("above rho at
every price"). Probability 0.9.

P4. V3 at `w = 1`: C over negation rho stays above 10 at every p (native work
per trial of order 100 mulmods against rho's one canonicalisation).
Probability 0.85.

P5. Rho: negation total is within 0.7 to 1.3 of plain total at every v3 and
tower prime (set-up dominates both). Probability 0.8. Walk-only share of the
negation total is below 0.6 at all tower primes up to 16001. Probability 0.8.

P6. Tower g = 3 at `w = 0` and p <= 16001: A uses fewer group operations than
C (positive control: A/R below C/R through 32771); at `w = 1` C is below A.
Probability 0.8.

P7. Tower g = 2: with double large primes, C over A group operations is
between 0.3 and 1.2 at p >= 4001 (genus 2 is `O~(p)` with and without).
Probability 0.6.

P8. Tower: the recorded verdict "IC3 above rho at every weight" is unchanged at
all seven primes; recorded IC2 "unresolved" at p = 251 stays unresolved or
becomes above (it never becomes below at all prices). Probability 0.85. IC3/IC2
at `w = 0` stays within 2 to 4 for the best arm. Probability 0.7.

P9. small_de: the new distance to the matched rho differs from the recorded one
by less than 5% in every 24- and 28-bit arm-cell, because the recorded
reference was already the best of plain/negation/automorphism with set-up
included. No verdict changes. Probability 0.95. Walk-only rho is lower than
the total by 5 to 60% at 24 bits and by under 10% at 28 bits. Probability 0.6.

P10. Overall: no row, in any of the three reruns, is below the matched rho at
all three prices. Probability 0.95.

## Decision rule

For each row classify the order against the primary reference (arm C, then
best-of-A,B,C reported beside it) as `below` (below at all three prices),
`above` (above at all three) or `unresolved` (order changes inside the band).
A recorded verdict **changes** if the class differs from the recorded class
of that row. Report every change with old and new class and ratios. A distance
that falls because of a relabelling or accounting change is stated as such and
is not counted as an improvement (CLAUDE.md success criterion); a distance that
falls because the algorithm does less work (walk collection, large primes) is
a measured improvement of the index-calculus baseline and is reported as such,
with its gap to the matched rho as the distance remaining. A prediction is
scored refuted if its falsifier holds.
