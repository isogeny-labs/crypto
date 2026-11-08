# P3 round 2: rerun the rho outcome with a corrected walk (protocol, frozen 2026-11-08)

Follows round 1 (`PROTOCOL.md`, `RESULTS.md`, `problems/p3census-rho-walk-bimodal.md`):
the round-1 rho walk was defective, so outcome Y1 measured that walk, not rho. Round 2
reruns Y1 only, with a corrected walk, on the **same curves, covariates and yield files**
(inputs from `runs/run-20261107T2220Z/{p24,p28}/{curves,cov,yield}.jsonl`, pinned by
sha256 in the card) and the **same seeds** SA, SB, SZ (round 1 Addendum A table).

## Instrument gate (before the claim rerun)

The corrected walk (Addendum A of this file) must pass, at p ≤ 2^22 on 50 curves:
per-curve mean ops/√n unimodal with mean between 0.8 and 1.2; flat in n (slope of the
mean against log n within ±5 % across 2^16..2^22); A/A between-curve variance ratio ≤ 2.
If it does not, the rerun does not start and the problem record stays open.

## Claim rerun

Per prime: `rho` runs A and B (T per curve pinned in Addendum A, at least 256), then
`analyze` with round 1's arguments (covariates, family size 16, perms, planted +2 %).

## Decision rules (frozen; rho outcome only)

1. **Instrument still defective:** A/A between-curve variance ratio > 2 at either claim
   prime, or per-curve mean ops/√n outside 0.7 to 1.3 → Y1 "not informative".
2. **Coordinate lead:** a covariate with |effect| ≥ 1 % and Bonferroni-significant with
   the same sign at both primes for Y1.
3. **Null:** neither, with the planted control informative.
The coverage outcome (Y2) is not rerun; its round-1 null stands.

## Addendum A: instrument (pinned before any claim cell)

Pending the builder's diagnosis, fix and validation.
