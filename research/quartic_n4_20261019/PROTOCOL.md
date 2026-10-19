# PROTOCOL: N4, relation-collection index calculus on the GHS plane quartic vs a matched rho on E0(K)

Frozen 2026-10-19 in the commit that adds this file, before any grid data. Revisions are recorded
in the README as deviations; this file is not edited afterwards. Branch `claude/quartic-ic-n4-20261019`
(from #2227). Lead N4 of the plane-quartic program
(`research/plane_quartic_program_20261018/PROGRAM.md`, W2a/W2b).

## Question

For `E0/K`, `K = F_{q^3}`, with `#E0(K) = 4 ell` (`ell` prime) and its genus-3 GHS plane quartic `C/F_q`,
what is the measured end-to-end cold cost of solving one ECDLP in the `ell`-subgroup with a relation
collection on `C` and descent through canonical `Pic^0(C)(F_q)` arithmetic, with **no subgroup
enumeration**, against a matched Pollard rho on `E0(K)`, and how do the two cost curves scale in `q`?

## Why this replaces the old vehicle (T1, already measured, `results/t1_q*.jsonl`)

`ghs_cubic_quartic_ic_20261012/quartic_ic.rs` built its dlog table by looping `n = 1..ell` over every
multiple of the generator (99.8 to 99.99% of its cost, `results/t1_q*.jsonl`, record `s1`), discarded the
relation rows (`let _ = rows`), and its relations numbered 0, 1 and 3 at q = 5, 7, 11. Its "pure IC"
numbers are subgroup enumeration. T1 re-ran it with the canonical keys of #2227 and records which of
its numbers survive. Its Zech-table field also caps at q of about 200, so N4 uses a new builder.

## Instances (frozen)

`q` = the largest prime `< 2^k` with `q = 7 (mod 12)` for `k = 8..18`:
223, 499, 991, 2011, 4051, 8191, 16363, 32719, 65479, 131071, 262111. `K = F_q[x]/(x^3 - c)`, `c` the
least non-cube. Per `(q, seed)`: the first `u = t^2 + b t + c0` in the seeded xorshift64* stream
(`ghs.rs::Inst::search`) with six distinct roots, `u, u', u''` independent, `#E0(K) = 4 ell`, `ell` prime
(order by Hasse-interval BSGS, confirmed on a second point), the `F_q` quartic with `F_q` coefficients and a
passing smoothness certificate (`plane_quartic_pic::smoothness_certificate`). Seeds 1..5 for `k <= 16`,
1..3 for `k = 17, 18`. The planted `k` is drawn from the same seed. Instance search is not charged to
either arm: both arms receive the same instance (the cover data `u` for IC, the Weierstrass model and
generator for rho).

## Arms

* **IC-B0** (the pipeline under test). Factor base `F` = `N = ceil(c q^{3/5})` rational points of `C`
  sampled on random lines, `c = 3.0`; base point `P0 = F[0]`. Relations: all lines through two factor-base
  points are bucketed (batch-inverted line coordinates, hash table); a bucket with 4 factor-base points
  gives a full relation `sum (P_i - P0) = kappa`, a bucket with 3 gives a relation with one large prime
  (the fourth point, from the residual quadratic). Large-prime relations sharing a prime are subtracted
  (`kappa` cancels); full relations keep `kappa` as a column (anchors it). Rows used: `ceil(1.2 n) + 30`
  of the available rows in a seeded shuffle (3 full rows first); 60 further rows are held out. Linear
  algebra: Lanczos on `A^T D A` mod `ell` (random `D`), kernel vector verified `A z = 0` and on the hold-out
  rows. Logs of the large primes from their first relation. Descent of `h(r0), h(r0+G), h(r0+Q)` for the
  first random `r0` with all three valid transfers: `x <- x + [P_j - P0]` for random known `P_j`, until the
  canonical degree-3 representative (`PlaneQuartic::add`) splits (base locus of its net of conics) into
  three distinct known points. `k = (y2 - y0)/(y1 - y0) mod ell`, verified `k G = Q` on `E0`. On failure
  (too few relations) the run is repeated with `c += 0.4`, up to 6 attempts; **every attempt is charged**.
* **IC-A0** (the baseline pipeline, for the improvement statement). The same code with `F` = all `q + 1`
  rational points (full relations only, no large primes). `k <= 12`, seeds 1..3.
* **rho** (matched). Plain Pollard rho on the same `E0(K)` `ell`-subgroup, same `ec.add`/counters:
  one trail, 16-adding walk (`M_j = a_j G` or `b_j Q`), partition by a hash of `x`, distinguished points
  with `2^dbits ~ sqrt(ell)/64` stored by full point, trail restart after `20 * 2^dbits` steps without a
  distinguished point. No negation map (sensitivity below). `n_rho` independent targets per instance:
  30 for k <= 12, 24 for k = 13, 14, 16 for k = 15, 16, 8 for k = 17, 6 for k = 18.

## Cost unit and pricing

Everything runs through the same counters (`common.rs`): `F_q` multiplications (a `K` multiplication is
11, a `K` Frobenius 2), inversions (priced at the measured time ratio `inv/mul`, median over the units of
a size), additions (counted, not priced). The canonical `Pic` library's own counter is added to the
same counters. Mod-`ell` multiplications (linear algebra, rho exponent bookkeeping) are priced at the
measured time ratio `lmul/mul` (median per size). **M-eq = mul + r_inv inv + r_lmul lmul.** One group addition
equivalent `GAE` = the counted M-eq of one `E0(K)` addition (about 70). `S = M-eq / (GAE sqrt(ell))`.

**Unpriced native work (AGENTS.md section 5a)**, listed with counts for both arms: IC: hash-table inserts
and lookups (line buckets, factor-base membership, log lookups); rho: distinguished-point/partition tests
(one per step) and table inserts/lookups. Priced at **w = 0, 0.1, 1 GAE per operation** for both arms;
`F_q` additions are reported but outside the band. Two arms are ranked only where the order holds at all
three weights with the bootstrap interval of the ratio excluding 1.

## Measurements and fits

Per size: mean over units of the cold IC M-eq (all phases: factor base, relations, matrix, linear algebra,
log recovery, descent, verify; sum over attempts) and mean over all rho runs; ratio IC/rho with a 95%
bootstrap interval (2000 resamples of units and runs). Exponent fits `ln cost = a + e ln q` on the size means,
bootstrapped over units and runs. **Primary range: q >= 4051 (k >= 12, 7 sizes); secondary: all sizes.**
Phase exponents are reported next to the prediction. Marginal per-target cost (descent and verify, the
target-independent phases excluded) is reported as secondary; the headline is the cold single-target cost.

## Controls

* Correctness: every IC run's `k` is verified against the planted target on `E0`; a run that fails
  verification is a failure and counts as 0 of 1 correct. Sampled relations (10 full, 30 large-prime per
  run) are verified through the canonical `Pic` group law (their classes sum to one constant, `relations_bad = 0`);
  hold-out rows must be in the kernel (`holdout_bad = 0`).
* Rho calibration (positive control for the baseline): mean steps / `sqrt(ell)` per size must be within
  [1.0, 1.5] (theory `sqrt(pi/2) = 1.25`); all rho runs verified. Out of range: the rho arm is not used.
* Instance control: `N1 = q + 1`, homomorphic transfer and `#Jac * x = 0` (the `n4 check` mode, run
  for q <= 65479 before the grid; brute-force point count for q <= 700).
* Host: one 4-core Intel Xeon 2.1 GHz VM, 4 single-thread processes, no isolation tier above L0. Counts are
  the metric; wall time is a practicality note.

## Pre-registered predictions

* **P1** every IC unit verifies (correct 100%).
* **P2** rho exponent `1.50 +- 0.10`; calibration holds.
* **P3** IC-B0 cold exponent in `[1.0, 1.4]` on the primary range (model: pair stage `N^2`, linear algebra
  `~ N^2`, both `q^{6/5}`; descent and factor-base sampling lower).
* **P4** IC-A0 exponent about 2 and above IC-B0 at every common size `q >= 991`.
* **P5** there is a crossover size `q*` below which rho is cheaper; IC-B0 is band-resolved lower than rho
  for all sizes above `q*` in the grid.

## Decision rules (stop rule included)

* Lead N4 stays **open** iff P1 holds and the exponent gap `e_rho - e_IC` has a bootstrap 95% interval
  entirely above 0 at both w = 0 and w = 1 on the primary range.
* Lead N4 is **closed** if P1 fails, or if the gap interval contains 0 or is negative at w = 0 or w = 1
  on the primary range with at least 3 sizes (the owner's stop rule: measured exponent not below q^{3/2}).
* Improvement statements (success criterion): IC-B0 is an improvement over IC-A0 where its cost is lower
  with the interval excluding 1; the improvement over rho is stated only where band-resolved, with the
  distance to rho given everywhere else.

## Claim hygiene

Public synthetic instances; the GHS-weak family only (`4 | #E0(K)`, `E0/F_{q^3}`, special form), not
generic curves, not prime-order `E/F_{q^3}`, not any deployed curve or the binary/prime standard curves.
No key-recovery claim. A crossover is a measured fit over the stated range, never an asymptotic theorem.
Not compared with an external implementation (Magma, Sage, PARI, msolve not installed): no
fastest-implementation claim. Sensitivities reported, not hidden: a negation map in rho would lower the rho
cost by at most a factor `sqrt(2)`; the IC arm uses no automorphism.
