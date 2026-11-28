# Frozen protocol — Prime31 RR9 seven-anchor algebraic completion and generic-equivalence falsifier
Date: 2026-11-17 UTC. PR #4547, stacked on #4546. Producer-only [pcc:unverified].
Status: registered *before* Rust implementation, no solver-speed or novelty claim.

## Prior lead review and revised mathematical observation
Read the existing #4546 exceptional Hermite, #4545 exhaustive B8 signed fibres, #4541 radical, #4539 signed RR9 chart, #4466 kernel, #4373 capped F4, #4228/#4349 quotient, #4513 mixed IC and #4532 cached BSGS (170.113 ms versus mixed IC 2221.474 ms on 1024 targets, historical one-run producer evidence). Cross-repo negatives: crypto-autoresearcher #3301/#3305. Primary published baseline: Amadori–Pintore–Sala, *Finite Fields Appl.* 51 (2018), 168–182, DOI 10.1016/j.ffa.2018.01.009. Research-KB MCP/CLI not available in this runtime; code search checked for seven-anchor RR9, not an exhaustive novelty claim.

Let E/F_p be the existing certified ordinary prime-order/cofactor-one curve, R != O, and S_1,...,S_7 be known signed finite leaf points. In L(9O), a function F that vanishes on the degree-eight effective divisor (S_1)+...+(S_7)+(-R) has ninth zero T when all nine zeros are finite, and the divisor class identity gives:
```
T = R - (S_1 + ... + S_7)
```
This is an EXACT **generic group-subtraction equivalence**: constructing T by interpolating seven supplied signed supports and taking the remaining norm root is not a novel non-generic relation extractor. It is still useful as an independent algebraic certificate and a filter against false novelty. If the ninth zero lies at O, the degree-nine norm lead vanishes and that branch must be explicitly rejected/handled. If an x-coordinate repeats, Hermite jets are required, not duplicated evaluation rows.

## Preregistered Rust experiment (reuse, no replacement ECC solver)
Implement `rr9_complete_seven_signed_leaves` in the existing native example `research/prime31_ic_20261115/native/examples/prime31_rr9_evaluation.rs`. Input = seven **supplied signed** finite leaves + public R. Use existing `rr9_leaves_kernel` with eighth divisor point `-R`, `rr9_norm`, `rr9_poly_div_rem`, and existing `Arithmetic::sqrt/rhs`. Divide the norm by `(X-x(R)) prod_7 (X-x(S_i))`; require exact linear quotient with nonzero top coefficient; obtain x(T), select the unique y sign by F(T)=0, and return finite T. The test-only oracle separately computes `R-sum_7 S_i` using existing EC additions. There must be NO secret logarithm or target scalar in the algebraic extractor.

Controls:
1. All 256 sign masks for the first distinct-x B8 base, computing public R by independent existing arithmetic, must recover each withheld eighth signed point, including x(R)=a base x if encountered; all polynomial quotient and original-curve checks asserted.
2. Both exceptional x=r B8 fixtures from #4546 must reconstruct the withheld eighth point with the correct signed tangent/Hermite multiplicity.
3. At least 32 deterministic eight-leaf positive trials for each of the B31 rational-x and Dickson-depth-six bases must recover the withheld point using only seven supplied leaf points and R; no planted scalar passed to completion.
4. Negative R+G offsets with eighth point absent from the B8 signed base are rejected as factor-base completions, even though a generic off-base T is algebraically reconstructed. Excluded exceptional/infinity/ambiguous cases counted, not silently marked success.
5. Counts (positive, negative, repeated-root, polynomial failure, ambiguity, given-support operations) and timings diagnostic only. Never compare these microbenchmarks to full cold solver or claim an ECDLP improvement. No changes to leaderboards, no paid compute.

## Validation and decision
Pinned Rust 1.100.0 existing .github/workflows/prime31-rr9-evaluation.yml must pass library + example tests, with exact-source GitHub Actions and raw JSON if available. A failed test must remain visible and be corrected without relaxing gates. Record observed outcomes and source hash. If positive, mark this seven-anchor route **generic-equivalent**, and prioritize B31 extraction of RR9 coefficients from *public R alone*, without any supplied leaf or sign, followed by complete blinded ECDLP timing against shared-precomputation generic BSGS, tuned rho and the published IC frontier. Producer checks are [pcc:unverified] until independent signed replay.
