# Frozen experiment — batch-inverted affine complements for actual Dickson31 8A IC oracle
Date: 2026-11-17. Owner PR #4618, stacked on #4615. [pcc:unverified]

## Scientific scope
This is a known Montgomery simultaneous-inversion optimization of **a generic-equivalent 4A+4A membership control inside an index-calculus research pipeline**. It is NOT a non-generic decomposition mechanism, a complete discrete logarithm solver, or a new exponent. Do not confuse it with the sign-independent RR9/S9 target.

Use exactly the parent Rust native `research/prime31_ic_20261115/native/examples/prime31_s9_b31_half_sum_oracle.rs`, original arithmetic pinned at `1df714645aa683cf975bc76fcacb9842cb4780fd` on Rust 1.100, prime `p=2147483647`, `n=2147446831`, E: y^2=x^3+847514846*x+2087444724, G=(625037715,2100627251); `Base::build(PRIME31,6)` with 31 x-values. Keep the existing 4A index construction, ordering, 62 oriented tokens and 618,885 distinct four-sums unchanged.

## Arms and frozen test set
A: parent exact affine `FourSet::recover` calling `Arithmetic::add` once per public complement and replaying witnesses.
B: same lookup, same hash index, same first-hit witness order, but compute regular complements `R-L` in chunks of **512** using one Fermat inversion of the product of all nonzero denominators, two prefix/backward sweeps, and affine formula. Handle same-x cases with original `Arithmetic::add` and identity targets/halves separately. Count all lanes in the successful final chunk, not just probes before the first hit; no precomputation left free.
Run identical frozen 96 distinct public Q generated from SplitMix64 seed `0x533942484f4c4431` using `k=1+next % (n-1)`, and two historical negative-at-a=0 Q from k=1610011808 and 1264623843. For each Q try public R=Q+[a]G for a=0..8 and stop at the first true membership. Candidate sees R only; k is caller-only and never supplied to either oracle.

Assertions: exact equality of found/negative, first successful shift, probes and recovered signed token witness, original-curve witness replay; 88/7/1 fresh first shifts, two old a=0 misses then a=1 hits, 98 positives/11 negatives across 109 attempts; independently test R=O and a same-x exceptional complement. Report 4A setup wall, candidate/legacy attempt wall and operation counts including Fermat inverse, extra final-batch work, all failed probes, memory capacity (not RSS), and all exceptional fallbacks.

Do not change corpus/curve/shift cap/batch size after observing timings. If source fails compile, fix only syntax/correctness and preserve the initial exact protocol. Any improvement is arithmetic engineering, cannot establish a non-generic index-calculus speedup. Compare with the historical 1024-Q full-solver #4532 only as contextual baseline; no cross-cohort speed ratio.

## Prior research checked
- Parent #4615 exact Dickson31 98-Q holdout, 18,049,427 original complement probes.
- #4596 owns full 1024-Q batched IC vs shared BSGS; #4604 owns D9 public RR9; #4614 low-memory 3-sum.
- #4550/#4581/#4612 document B8 D9/quadratic quotient/rarity. Earlier batch IC/rho art: `research/notes/ecc2k130/compact_s3_batch_20260929/RESULT.md`, `research/lead_sweep_20261110/LEAD-RHO-batched-inversion-shared-pm/PROTOCOL.md`, `research/prime31_ic_20261115/native/APS_JOINT_SHAMIR_BATCH_RESULT_20261116.md`. This **does not claim inventing batching**.
- Petit–Kosters–Messeng PKC 2016 DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala FFA 2018 DOI 10.1016/j.ffa.2018.01.009.
- Montgomery simultaneous inverse trick, originally Montgomery 1987, reviewed in Brent–Zimmermann *Modern Computer Arithmetic*, §2.5.1. See https://algorithm.tamnd.com/batch-modular-inverse/.

Producer-only claims [pcc:unverified]. No paid compute/merge/leaderboard changes; unsigned test is not independent PCC.
