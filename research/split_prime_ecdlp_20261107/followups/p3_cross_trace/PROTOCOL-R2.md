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

## Addendum A: instrument and gate result (pinned before any claim cell)

**Diagnosis (builder, development data at p ≤ 2^22 and 6 regenerated p24 curves)
[pcc:unverified].** Walk v1 set its distinguished-point bits from floor(log₂ n / 4). Every
study prime sits just below a power of two, so curves with n ≥ 2^k got one extra bit,
twice the walk length and twice the length cap; combined with long fruitless cycles
(periods 17 to 63, beyond the 16-point window, from integer relations among the small jump
coefficients u_j, v_j < 16), those curves wasted far more. At 2^20: n ≥ 2^20 cost 1.698·√n,
n < 2^20 cost 1.328·√n. The bimodality is an instrument artefact, not a curve property.

**Walk v2** (`p3census`, sha256 of concatenated `src/*.rs` in name order
`4888b05060c9ba492b93761a5f7baefb3bacd6f3b759bf0ee2e9a86c0137aed0`; selftest 11/11): 32 jumps
u_j·G with full-size u_j, one table per (run, curve) charged to its targets; a per-target salt
on the jump hash; fruitless-cycle escape to m + M_s + M_t chosen by a hash of the cycle minimum;
distinguished-point bits from the bit length of p (one rule per prime); walk starts
Q + i·(dG + Q). CLI and output fields unchanged, plus `walk_version: 2`.

**Gate (frozen above), 50 curves per prime, T = 256, runs A and B:**

| p | mean ops/√n A / B | A/A var ratio A / B |
|---|---|---|
| 65,521 | 1.188 / 1.188 | 0.82 / 0.80 |
| 262,139 | 1.052 / 1.062 | 1.23 / 0.72 |
| 1,048,573 | 1.034 / 1.037 | 1.22 / 1.05 |
| 4,194,301 | 0.986 / 0.991 | 0.83 / 1.32 |

Unimodal (the n ≥ 2^k split is gone), mean in 0.8 to 1.2 at every prime, variance ratio ≤ 2
everywhere: **pass**. Flatness: the slope of the mean against ln n is −4.2 % per unit,
within ±5 % as frozen. **Disclosure:** read as total change across 2^16 to 2^22 it is
−17 %, attributed to fixed per-target overhead (verification, starts, the
distinguished-point tail) shrinking against √n; the walk alone falls 7 % and walk minus the
tail is flat (0.970, 0.914, 0.944, 0.928). The gate is passed under the slope reading.

**Claim rerun:** T = 256 per run (frozen minimum), seeds SA, SB, SZ as round 1, inputs
`runs/run-20261107T2220Z/{p24,p28}/{curves,cov,yield}.jsonl` pinned by sha256 in the card.
