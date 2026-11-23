# Prime31: signed-pair algebraic projector, frozen prospective control

Date: 2026-11-18 UTC. Branch: `automation/prime31-algebraic-pair-projector-20261118`. PR #4720. Status: **registered before any new benchmark or sample outcome**.

## Scope and rationale

Test a narrow non-generic primitive needed by the public-residual RR9/F4 lane: on the unchanged source curve
(E:y^2=x^3+847514846x+2087444724) over (p=2^{31}-1=2147483647),
group prime order (n=2147446831), (G=(625037715,2100627251)), determine whether an **unknown public** (Q) admits a **signed two-point** expression
(Q=P_i+\epsilon P_j), (epsilon\in\{-1,+1\}), from the first-x B4/B8/B31 factor base, without enumerating any point-sum lookup table.

Reuse `prime31_algebraic_ic::{PRIME31,Arithmetic,check_curve}` from `research/prime31_ic_20261115/native/lib.rs`; use the same first rational x-lifts as the RR9 low-power strata control (#4717). This is a low-degree extractor component, **not** a six-slot ECDLP solver.

## Exact arithmetic identity and all exceptional cases

Let (Q=(u,v)), (P_i=(x_i,y_i)), (D_i=(u-x_i)^2), (N_i=(v+y_i)^2-(u+x_i)D_i).
For (u\ne x_i), the point (Q-P_i) is finite with abscissa (N_i/D_i).
Write (H_B(T)=\prod_{j<B}(T-x_j)=\sum_{k=0}^B h_k T^k).
The no-inversion homogeneous screening test is

[
F_i(Q)=\sum_{k=0}^B h_k N_i^k D_i^{B-k}=0.
]

Because (D_i\ne0), this is equivalent to (x(Q-P_i)\in\{x_j\}). It is an exact signed-pair membership condition (a root can correspond to either (+P_j) or (-P_j)). On a hit, **reconstruct and replay** the signed witness by an ordinary point subtraction; do not silently assign a sign or a logarithm.

For (u=x_i), the homogeneous expression alone is invalid: at (Q=-P_i), it is identically zero although (Q-P_i=-2P_i) need not be in the base. For (Q=P_i), (Q-P_i=O) has no finite x. Handle these two cases separately: skip (Q=P_i); for (Q=-P_i) use the doubling tangent
(D_t=4y_i^2\ne0), (N_t=(3x_i^2+a)^2-2x_iD_t), test (H_B(N_t/D_t)=0) by the same homogeneous expression. For (Q=O), accept (O=P_0-P_0). No base y is zero, since the group order is odd prime.

## Registered controls

1. Verify the original curve and B4/B8/B31 first-rational-x bases, rejecting off-curve points.
2. For every base size, construct planted public positives (Q=P_i\pm P_j) for (i,j<\min(B,8)) (all 2*min(B,8)^2 cases); include O, each of the first min(B,8) base points and their negatives. Compare against an independently implemented **table-free signed pair loop** for every target.
3. Draw 128 additional public targets from a fixed xorshift64 state seeded `0xDEC0DE1234ABCDEF`, mapping each nonzero residue to (Q=[k]G). Their scalars are test-fixture provenance only; the extractor reads just Q.
4. For each target require the projector and the group-law baseline to agree; for each returned witness verify (P_i\pm P_j=Q). For every nonexceptional i also verify the homogeneous screen against the direct group-law x-membership predicate; verify that the unguarded Q=-P_i expression would be a false positive where the doubled x is not in the base.
5. Record tested targets, actual positives, exceptional counts, field multiplications and root-screen evaluations, baseline EC additions/inversions, exact witness replay, and source revision. The test prints JSON. If GitHub Actions is used, capture raw stdout as an artifact.
6. No timing advantage is asserted without same-host A/B/B/A and an optimized batch-inverted group baseline. This screen must later be lifted to genuinely nonenumerative 6-slot RR9/F4 extraction, full-rank relations and 1,024 unknown logs; match #4714's shared BSGS and mixed IC, not only a deliberately slow baseline.

## Prior lead audit and novelty constraint

- [#4714](https://github.com/isogeny-labs/crypto/pull/4714): complete M6 any-witness solver, 1,024/1,024, 31/31 rank, still generic 3+3 enumeration; prior matched M6 loses 20.85–23.32x to shared BSGS.
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692): open draft RR9 shared-coefficient, nonenumerative extraction not yet verified.
- [#4717](https://github.com/isogeny-labs/crypto/pull/4717): supplied M support predicates; no unknown-M extraction.
- [#4701](https://github.com/isogeny-labs/crypto/pull/4701): public-point Frobenius cubic inverse and 1.22x stage diagnostic, not relation extraction.
- [#4698](https://github.com/isogeny-labs/crypto/pull/4698): claimed subexponential Boolean network approach is source-level and unbenchmarked; at p=2^31-1 the constants are not known to cross over.
- Amadori–Pintore–Sala (2018), *On the discrete logarithm problem for prime-field elliptic curves*, DOI 10.1016/j.ffa.2018.01.009, and [Bochum 2026](https://github.com/bochum-crypto/subexponential-ecdl). No novelty claim for the classical Semaev three-point identity or homogeneous-polynomial evaluation.

Research KB MCP is not present in this runtime; wrapper cannot execute without local checkout. GitHub source files, PRs and the cited primary source records are the limited fallback.
