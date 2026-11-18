# Prime31 RR9: four signed leaves eliminate four chart variables

**Date:** 2026-11-18. **Lane:** [#4728](https://github.com/isogeny-labs/crypto/pull/4728). **Pre-registered protocol:** [RR9_FOUR_PREFIX_REDUCTION_PROTOCOL_20261118.md](RR9_FOUR_PREFIX_REDUCTION_PROTOCOL_20261118.md). **Native reused source:** [prime31_rr9_weighted_resultants.rs](../native/examples/prime31_rr9_weighted_resultants.rs). **Independent finite-difference checker:** [verify_prime31_rr9_four_prefix.py](../native/verify_prime31_rr9_four_prefix.py). **Standing:** [pcc:unverified].

## Algebraic reduction

On the **unchanged** ordinary cofactor-one curve
[
p=2147483647,quad E:y^2=x^3+847514846x+2087444724,quad n=2147446831,
]
use the eight *distinct-x* weighted slot supports (pm[w_j]P_i), (w=(1,3,5,7,11,13,17,19)), (i=0,ldots,30). No curve replacement or weak-curve specialization was made.

For public (R=(r,s)), the existing normalized RR9 chart is
[
F(X,Y)=(X-r)A(X)+(Y+s)V(X),quad
A(X)=a_0+a_1X+a_2X^2+a_3X^3,quad
V(X)=X^3+v_2X^2+v_1X+v_0 .
]

Fix **four signed leaves** (Q_j=(x_j,y_j)), for (j=0,1,2,3). If the four (x_j) are pairwise distinct and (x_j\neq r), then requiring (F(Q_j)=0) gives
[
A(x_j)=\lambda_j V(x_j),qquad
\lambda_j=-{y_j+s\over x_j-r}.
]
Writing (L_j(X)) for the degree-three Lagrange basis for those four abscissae yields the exact identity
[
A(X)=\sum_{j=0}^{3}L_j(X)\lambda_j X_j^3
+v_0\sum_jL_j(X)\lambda_j
+v_1\sum_jL_j(X)\lambda_j x_j
+v_2\sum_jL_j(X)\lambda_j x_j^2.
]
(Here (X_j^3) means the scalar (x_j^3).) Thus **four of the seven chart variables are eliminated by linear algebra**, leaving only (v_0,v_1,v_2). Substitute this affine expression into the unchanged degree-eight norm
[
M(X)=D_R(X)V(X)^2-(X-r)A(X)^2-2sA(X)V(X),quad
D_R(X)=X^2+rX+r^2+a.
]

The remaining four weighted slot resultants
(g_j(v)=\prod_{i=0}^{B-1} M(x([w_j]P_i);v)), (j=4,5,6,7),
are **four factored degree-≤62 equations in three variables** at B31.
Each individual norm evaluation is quadratic in (v). No claim is made that an F4/quotient solver can solve them quickly; the seven-to-three reduction is a classical partial-interpolation identity, not a novel cryptanalytic complexity theorem.

### Exceptional branch

If a chosen (x_j=r), the interpolation divides by zero and is **inapplicable**. If any two (x_j) coincide, the Vandermonde is singular. The native code refuses both cases rather than silently accepting a partial solution. For a complete public-target extractor, one must choose other pivot leaves or use the jet/saturation exceptional branch from [#4547](https://github.com/isogeny-labs/crypto/pull/4547).

### Local mathematical checks (not native CI)

A separately implemented Python BigInt preflight on the exact original curve and the same 16 deterministic RR9 planted fixtures produced:
- **16/16:** degree-three interpolation reconstructed each of the four original (a_k) residues exactly at the original (v_0,v_1,v_2).
- **16/16:** the remaining four norm-root equations vanished at the planted chart.
- **16/16:** the exact (4\times3) Jacobian at the planted chart had finite-field rank **3** (local nonsingularity of the reduced constraints; no global unique-solution conclusion).
- **16/16:** changing (v_0\mapsto v_0+1), while reinterpolating (A) so the **first four constraints still hold**, caused at least one of the **remaining four** weighted slot resultants to fail.

These are local producer-side checks **[pcc:unverified]**. Original arithmetic is independently recreated by the Python checker, while the new native Rust source reuses `prime31-algebraic-ic` arithmetic pinned at `1df714645aa683cf975bc76fcacb9842cb4780fd`. The added independent Python CI stage computes the three-variable Jacobian via *symmetric finite differences*, not the Rust analytic derivative; the previous full original-curve checker still replays each certificate.

**Native CI status at report creation:** submitted, not yet declared passed. Source-specific runs, compiler errors and independent validation will be recorded after observing their outcomes; a queued or in-progress workflow is not evidence of native success.

## Complexity and verdict on brute-forced prefix enumeration

The reduction is only useful if the four leaves can be selected efficiently from the public target without hidden scalar/chart access. Naively looping over all signed prefixes takes exactly ((2B)^4) combinations: B4 **4,096**, B8 **65,536**, B31 **14,776,336** *per target*. Distinct-x supports prevent duplicated weighted abscissae but do not improve this exponential search count. Some combinations can be skipped on (x_j=r); this does not provide a new asymptotic bound.

The prior [matched complete 1,024-target frontier in #4714](https://github.com/isogeny-labs/crypto/pull/4714) recorded a historical best shared BSGS time of approximately **144.049 ms**, not measured again here. Across 1,024 targets, this is about **140.67 microseconds/target**. A brute-force B31 four-prefix loop would require, on average, fewer than **0.00952 ns for each signed prefix** *including all algebraic solving* to fit that historical batch budget. This is an arithmetic budget, **not** a machine-calibrated lower bound and does not exclude a nonenumerative prefix solver.

This rules out interpreting the reduction by itself, or the prior supplied-x zeta/Walsh microkernel, as evidence of a competitive complete ECDLP algorithm. It does not rule out deeper syzygies or a polynomial-time public-prefix selector.

## Prior-work audit and specific next experiments

1. **Immediate B4/B8 falsifier:** Apply the reduced three-variable factored ideal to unknown *public* target residuals on the frozen 64-point B8 corpus (**46 positive, 18 negative** by independent grader). Compare a capped F4/saturated ideal to the prior **seven-variable** form, counting all 65,536 possible signed prefix branches if that is the selected strategy. Do not use generic 4+4 group-sum tables or seeded chart coefficients in the candidate.
2. **Before scale-up:** Derive a **nonenumerative prefix projector** or show through measured F4/memory bounds that the four-prefix hybrid is killed; alternatively let fewer than four algebraic prefix constraints be fixed and compare the resulting higher-dimensional but lower-branch-count ideal.
3. **If public extraction succeeds:** B31 rank-31 learning and 1,024 verified blinded ECDLP logs in the existing Rust pipeline. Same-runner, same-target cold and amortized measurements against mixed 4+4 index calculus, any-witness M6, cached/cold shared BSGS and optimized negation-map rho, with every failed F4 call, rootfinding, precomputation, verification and memory charge.

This lane was compared against existing [#4539 RR9 chart](https://github.com/isogeny-labs/crypto/pull/4539), [#4547 seven-known-leaf generic-equivalence obstruction](https://github.com/isogeny-labs/crypto/pull/4547), [#4692 unknown-chart F4](https://github.com/isogeny-labs/crypto/pull/4692), [#4720 algebraic signed pair](https://github.com/isogeny-labs/crypto/pull/4720), [#4717 low-power multiplicity](https://github.com/isogeny-labs/crypto/pull/4717), [#4728 supplied-x kernels](https://github.com/isogeny-labs/crypto/pull/4728), and [#4746 corrected PKM exponent](https://github.com/isogeny-labs/crypto/pull/4746). Repo code searches additionally checked `crypto-autoresearcher` and `cryptanalysis` for the same four-prefix interpolation mechanism. Published references: Petit–Kosters–Messeng, PKC 2016 (DOI 10.1007/978-3-662-49387-8_1), Amadori–Pintore–Sala, FFA 2018 (DOI 10.1016/j.ffa.2018.01.009). Shared KB MCP/wrapper is not accessible here; no exhaustive novelty claim.

**Research status:** mathematically explicit preprocessing and directly executable Rust/independent Python checks are checked in, but this test does **not** discover an unknown four-prefix, compute full unknown-chart solutions, or show a cheaper complete prime-field solver. Signed PCC validation remains pending.
