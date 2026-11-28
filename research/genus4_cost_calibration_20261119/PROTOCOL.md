# Genus 4: measured price of native operations against a group operation (timing tier)

Frozen 2026-11-19 before any measured run of this study (`frozen.sha256`). Binary:
`genus_ladder` at ca72267899, the prime-order study's binary (sha256 493ee294…, `binary.sha256`).
Every number is `[pcc:unverified]`.

## 1. Question

The genus-4 results price every native operation (relation-phase F_p work, mod-r linear-algebra
work) at w ∈ {0, 0.1, 1} GAE, where 1 GAE is one Jacobian group operation. Is the w = 1 end
conservative or fragile? The prime-order validator found that x* at p = 16411 stays below the
ideal negation-rho bound only while a mod-r operation costs ≤ 1.25 GAE. This study measures the
actual time of each class on one host and reports:

- **R1 (price-free).** IC wall time over (measured seconds per rho group operation ×
  `sqrt(pi r / 4)`), for arms x (cc 2.0) and x* (cc 1.4).
- **R2 (effective prices).** w_rel = t_rel / t_G and w_la = t_la / t_G, the measured cost of one
  counted relation-phase operation and one counted linear-algebra operation, in GAE.

## 2. Host and isolation (timing tier, T-timing)

- 4-vCPU shared cloud VM, no hardware PMU (`perf` absent), so wall time only.
- One measuring process at a time, pinned with `taskset -c 2`, `--threads 1`. Nothing else is
  started during a block.
- Counts are deterministic. Only wall time is measured.

## 3. Instances

- **p = 16411:** curves 373, 455.
- **p = 32771:** curves 355, 882.
- These are prime-order curves from `research/genus4_prime_order_20261118`. Target k = 0.

## 4. Runs, in this order (blocks are separate commands, each under 2 h)

1. **IC pass 1.**
   - Arm x at cc ∈ {1.0, 1.4, 2.0, 2.8}, each with SGE settings (4, 1.5) and (8, 2): 8
     configurations.
   - Seeds 1–2 on both curves: 4 runs per configuration and prime. Both primes.
2. **Rho at p = 16411.** Window-free plain rho (arm Q), curve 373, k = 0, seeds 1–3, each run to
   collision.
3. **Rho at p = 32771.** Curve 355, k = 0, seed 1.
4. **IC pass 2.** A repeat of block 1, rows identical in counts, for the A/A timing noise.

## 5. Gates

- **G1.** Every IC run is certified and correct, and every rho run is certified.
- **G2 (instrument identity).** Rows for cc 1.4 / SGE (4, 1.5) and cc 2.0 / SGE (4, 1.5) equal the
  prime-order study's rows in every count field (`group_ops`, `rel_mul`, `rel_add`, `la_mul`,
  `la_add`, `trials`, `relations`) for the same keys.
- **G3.** Pass 2 counts equal pass 1 counts row for row.

## 6. Analysis (`analyze.py`)

- **t_G(p).** Σ rho wall_ns / Σ rho group_ops over the rho runs at p. Per-run values are reported.
- **R2.** Per prime, OLS of (wall_ns − t_G · group_ops) on [1, rel_ops, la_ops] over all IC rows
  of both passes, where rel_ops = rel_mul + rel_add and la_ops = la_mul + la_add. This gives t_rel
  and t_la. 95% intervals come from a bootstrap over (curve, seed, configuration) clusters, 2000
  resamples, seed 20261125. Then w_rel = t_rel / t_G and w_la = t_la / t_G.
- **R1.** Per prime and arm (x = cc 2.0 / SGE (4, 1.5); x* = cc 1.4 / SGE (4, 1.5)): mean IC wall
  over (t_G(p) × mean `sqrt(pi r / 4)` over the two curves). Intervals come from the same cluster
  bootstrap applied to the IC side. t_G's spread is reported beside the result, not folded into it.
- **A/A.** The median and maximum |ln(wall_pass2 / wall_pass1)| over identical rows.
- **Decision.**
  - The w = 1 headline is **conservative** if the upper bounds of w_rel and w_la are below 1 at
    both primes.
  - It is **fragile** if w_la's lower bound exceeds 1.25 at p = 16411.
  - Otherwise it is **unresolved**.

## 7. Predictions (before data)

- **P1.** t_G(16411) ∈ [1.5, 5] µs per group operation.
- **P2.** w_rel and w_la ∈ [0.002, 0.05] at both primes.
- **P3.** R1 for x* ∈ [0.003, 0.03] at both primes.
- **P4.** A/A median |ln ratio| < 0.05.
- **P5.** t_G(32771) / t_G(16411) ∈ [0.8, 1.3].

## 8. Scope and deviations known at freezing

- **One host, timing tier.** Wall time on a shared VM is a calibration of this implementation, not
  a statement about optimized code. Rho and IC share the Cantor arithmetic. Linear algebra uses
  u64/u128 mod-r arithmetic.
- **Not a new speed claim.** This calibrates the price convention under which the validated
  genus-4 results are reported.
- **Thin rho sample.** There is one rho run at p = 32771, because a run costs about 1 h; P5 tests
  whether t_G depends on p.
- **Host.** Local, not isolab-eks (no AWS identity). No S3.
