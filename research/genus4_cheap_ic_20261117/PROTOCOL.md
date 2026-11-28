# Genus 4: cheaper IC arms against window-free plain rho at w = 1

Frozen 2026-11-17 before any measured run (`frozen.sha256`; binary hash in `binary.sha256`, written
after the build and before the first run). Every number is `[pcc:unverified]` until a validator
replays it.

## 1. Question

`research/genus4_lean_rho_20261116` found arm w at 2.075 [1.675, 2.657] (p = 8209) and
1.612 [1.180, 2.409] (p = 16411) times the cheapest rho at w = 1 in reading Q3; the cheapest rho is
plain rho without its cycle window (arm Q). At w = 1 arm w's cost is almost all native work
(relation search plus linear algebra). This study measures how far the cheaper IC variants built
in this program reduce that native work, and where they leave the gap to rho.

## 2. Instances, targets, seeds

Exactly those of the lean-rho study: p = 8209 curve seeds 363, 393, 407, 457, 474, 519;
p = 16411 curve seeds 172, 180, 2640; 3 targets per curve (target seeds `cs + k·1000003`,
k = 0..2); solver seeds 1-2. 27 targets, 54 runs per arm.

## 3. Arms (binary `genus_ladder`, this PR's source)

- w: double large prime + parity prefilter + coordinate Wiedemann + capped unit-T walk (baseline).
- s: w + structured Gaussian elimination before coordinate Wiedemann.
- f: w + Ferrari quartic splitting test.
- x: w + Ferrari + SGE (s and f together).
- z: sieved relation search, up to two large primes (`genus_ladder_sieve::run_sieve_ic`).

Rho references are the lean-rho study's measured rows (B, Q; and R from the earlier studies on the
same keys). No rho is re-run.

## 4. Gates

- G1: every IC run certified, recovered log equal to the planted value.
- G2: arm w rows equal the lean-rho study's arm w rows for the same keys in every count field
  (`group_ops`, `rel_mul`, `rel_add`, `la_mul`, `la_add`, `trials`, `relations`): same instrument.

## 5. Readings and statistics

Q3 primary, Q4 co-primary at w = 1, both as in the lean-rho protocol (G priced 1 GAE; every emitted
native operation w GAE; plain rho's canonicalisation count is its actual 0). IC native work is
`rel_mul + rel_add + la_mul + la_add` (Q1 multiplications only). Per prime and arm: ratio of the mean IC
cost to the cheapest mean rho cost (B, Q, R), bootstrap over targets then seeds (2000, 95%,
seed 20261122). Arm-to-arm: ratio of mean native work to arm w's, paired by key, same bootstrap.

## 6. Predictions (before data)

- P1: arm z's native work / arm w's in [0.60, 0.85] at both primes.
- P2: arm s's LA native work / arm w's in [0.35, 0.70]; its total native work / arm w's in [0.85, 1.00].
- P3: arm f's relation-phase native work / arm w's < 1 at both primes (point estimate).
- P4: at w = 1 (Q3) the best arm stays above rho at p = 8209 (lower bound > 1); at p = 16411 its
  point estimate is in [0.9, 1.5].
- P5: every arm is below rho at w = 0.1 (Q3 and Q4 upper bounds < 1) at both primes.

## 7. Decision rule

An arm is "below rho at w = 1" at a prime only when its upper bound is < 1 in both Q3 and Q4. The
arm that minimises the Q3 w = 1 point estimate is reported as the new baseline candidate, with its
paired native-work ratio to arm w. Positive control of a known asymptotic effect; not an ECDLP result.

## 8. Deviations known at freezing

Local host (no AWS identity, so not isolab-eks); session kept active during the run (owner choice);
no S3 upload. IC runs only; counts are deterministic.
