# Frozen protocol: target-conditioned Prime31 F3 chart Jacobian regularity

**Frozen 2026-11-19; PR #4812 lane.** An exact mathematical nonsingularity argument is the primary result. The native Rust run is an *independent execution gate*, not the source of the theorem. No public-only factor-index recovery or whole-ECDLP improvement is presupposed. [pcc:unverified]

## Source and lead audit (before the experiment)
- Original unchanged Prime31 curve and Rust arithmetic: `research/prime31_ic_20261115/native/lib.rs`; pinned source `1df714645aa683cf975bc76fcacb9842cb4780fd`.
- The existing chart and 512 validated planted controls: `rr9/TRANSFORMED_SLOT_F3_RESULT_20261119.md` and #4812. Its 14 unknowns, eight degree-eight slot selectors and seven quartic S3 constraints are not yet an extractor.
- #4692 is the existing public-only weighted RR9/F4 owner; #4822 sparse Wiedemann; #4816 fixed-*unweighted* quotient baseline (not the same relation language); #4809 already tests the B2 quadratic root primitive, so do not duplicate it; #4807/#4808 already measure generic half-table collision counts.
- Mathematical background: Semaev (2004), Diem (2011, DOI:10.1112/S0010437X10005075), Kousidis–Wiemers (2019, DOI:10.1515/jmc-2017-0022), and the Kosters–Yeo caveat arXiv:1503.08001. Literature does not prove fast prime-field F4 extraction.
- Shared research-KB MCP not installed in this runtime; consulted accessible GitHub primary reports and the above published literature. This is not a novelty-priority claim.

## Exact mathematical claim

Let `E: y²=f(x)=x³+ax+b` over `F_p`, `p=2147483647`, `a=847514846`, `b=2087444724`. Its `E(F_p)` has odd prime order `n=2147446831`. For the four previously verified disjoint transformed B8 windows, let `T_i={x([2^i]P_j):j in window}` (eight pairwise distinct ordinary rational x values in each slot), `H_i(X)=prod_(t in T_i)(X-t)`. The finite signed-chain chart has unknowns `t_0,...,t_7,z_1,...,z_6`, with `z_7=t_7`, `z_0=x(Q)`, and equations
`H_i(t_i)=0` for `0<=i<=7`, `S3(t_i,z_(i+1),z_i)=0` for `0<=i<=6`.

**Claim:** At every geometric solution of this complete target-conditioned finite affine chart the Jacobian has column rank 14. Consequently its ideal (including the public-target constraint) is radical/zero-dimensional over `F_p` on this chart. This says *nothing* about its degree of regularity or fast F4 runtime.

**Proof:** The previously checked ordinary-chart induction shows that every finite-chain geometric solution has rational, ordinary `t_i,z_i` (all leaf roots are rational and each S3 step is a finite sum of ordinary `F_p` points). Hence `f(t_i)f(z_(i+1)) != 0`: an `F_p` zero of `f` would be a rational 2-torsion point on the odd-order curve. Choose the 14 constraints consisting of eight `H_i` and six `S3(t_i,z_(i+1),z_i)` for `1<=i<=6`. Ordered by the eight t-columns and six suffix z-columns, the Jacobian determinant is, up to sign,
`prod_(i=0)^7 H_i'(t_i) * prod_(i=1)^6 (partial_(z_i) S3(t_i,z_(i+1),z_i))`.
Each `H_i'` is nonzero by distinct roots. For `S3(x,y,z)=A z²+B z+C`, its discriminant identity is
`B²-4AC=16f(x)f(y)`.
At any finite root, `(partial_z S3)^2=(2Az+B)^2=B²-4AC=16f(x)f(y) != 0`, **even if A=0 and S3 becomes linear**. Thus that minor never vanishes. Finite slot roots and degree-at-most-two suffix equations make the chart zero-dimensional. Its reduced zero-dimensional coordinate ring is a finite product of fields over the algebraic closure; imposing the seventh/public-target S3 equation selects a subset and preserves reducedness. QED.

**Complexity warning:** Before imposing the public target equation, the eight t choices and six suffix equations give at most `8^8 * 2^6 = 2^30` geometric chart points (some branches degenerate to linear or have no finite root). Do not expand that quotient, and do not interpret full rank as small F4 cost.

## Frozen original-Rust check
Reuse the pinned original `Arithmetic`, `PRIME31`, `check_curve`. Sample 128 signed planted 8-slot relations in each of the same four original windows (total 512), deterministic SplitMix64 seed `0x4a41434f4249414e`. Precompute `[2^i]P_j` using the original Rust scalar routine. For each ordinary complete suffix chain:
- independently evaluate each `H_i'(t_i)` as seven nonzero root differences (8 per sample);
- evaluate the six `partial_z S3` pivots for i=1..6 and require every one nonzero (14 pivots per sample);
- check the quadratic discriminant equality `(partial_z S3)^2 == 16*f(t_i)*f(z_(i+1))` for all seven F3 equations (i=0..6);
- assert `S3=0` for all seven equations and source/target x match.
- Include a focused Rust test, entire-cohort release JSON and strict CI assertions on the pinned original library. Do not use hidden signs in any public extractor, no generic table in this structural test.
- Any infinity suffix, zero pivot, mismatched identity, missing original-source test, or CI failure falsifies/pends the reported finite-chart gate.

**Expected on the preflight cohort, not a presumed CI result:** 512 ordinary, 7168 nonzero Jacobian pivots, 3584 F3/discriminant identities. All source-specific measurements must be treated as pending until a corresponding workflow run succeeds. Signed PCC separate.

## Next decision
Pass this nonsingularity lemma and the 14-variable selector/S3 chart to #4692's existing F4 owner. Compare bounded public-target extraction to the earlier 21-variable quartic and 14-variable degree-10 variants on exactly the frozen 46-positive/18-negative B8 corpus. Record solver-degree, memory, certified UNSAT versus timeout, and full failed-query cost, then integrate only if it beats 1/64/1024-target same-runner index-calculus, BSGS, and optimized-rho references.
