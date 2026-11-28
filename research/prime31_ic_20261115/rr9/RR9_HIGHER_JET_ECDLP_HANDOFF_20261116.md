# Prime31 RR9 higher-jet algebra and complete-ECDLP handoff (2026-11-16)

## Scope and provenance

Curve: Fp with p=2147483647, E: y²=x³+847514846*x+2087444724, #E(Fp)=2147446831 (prime), cofactor 1. Existing Rust Arithmetic, Point, Base and PRIME31; no toy replacement curve, no Python ECDLP solver, no secret scalar in the determinant predicate. Research branch for open PR #4466: research/prime31-rr9-evaluation-gate-20261116. Separate WZ rho CI dependency/parse repair: PR #4468.

This extension is not a complete ECDLP solver. The RR9 predicate receives all eight factor-base points *already chosen*; finding them from unknown R is the unsolved, potentially non-generic step. A relation predicate is not interchangeable with a target-conditioned root extractor. This experiment has no valid cold speedup or non-generic claim by itself.

## New derivation: every affine diagonal of L(9O) on this particular curve

All rational affine points have y!=0: y=0 would imply P=-P and so 2P=O, impossible in an odd-prime-order group for nonidentity P. Thus t=x-x0 is a uniformizer at every rational affine P, including any duplicated leaf or -R when R!=O.

For q=(x0,y0), let x(t)=x0+t and y(t)=y0+y1*t+...+y8*t^8. The Weierstrass equation implies:

    c1 = 3*x0^2+a, c2=3*x0, c3=1, c_j=0 for j>=4
    y_j = (c_j - sum_{i=1}^{j-1} y_i*y_{j-i})/(2*y0) mod p

This exact recurrence determines the series coefficient y_j for each j<=8. Using truncated polynomial products, obtain coefficients of t^j in 1,x,y,x²,xy,x³,x²y,x⁴,x³y. For a leaf occurring m times, use the rows j=0,...,m-1 in the 9x9 evaluation matrix. The determinant vanishes if and only if a nonzero element of L(9O) vanishes on the length-nine divisor including all multiplicities; the genus-one divisor criterion yields sum of nine points=O. This follows from Riemann-Roch and is classical, not an independently novel attack.

The old first-order tangent jet is retained as a separate arithmetic reference. By construction, the recurrence's j=1 coefficient agrees with (3x²+a)/(2y). Rows are Taylor *coefficients*, not raw j-th derivatives; no factorial is needed (important for p>=5, including p=2147483647). Infinity is not covered; future code must use an O-local uniformizer rather than silently yielding zero. The argument also does not apply unmodified on odd-characteristic curves containing rational 2-torsion.

## Preregistered original-curve verification

Protocol: RR9_DETERMINANT_PROTOCOL_20261116.md, extension B, committed before code in commit 51956ec.

- Preserve distinct 64 positives and 64 negatives across both actual first31 and Dickson-depth-six B31 factor bases.
- Preserve first-order double-leaf true/false controls and the naive duplicate-row false-zero counterexample.
- Check first 16 first-x rational points: generated 0th/1st jets equal independent evaluation / tangent rows.
- For each factor base and multiplicity 3,4,8, use eight signed points (m copies of P0, 8-m copies of P1), and construct R=sum points and wrong R+G with existing Rust EC arithmetic **outside** RR9. Demand det(true)=0, det(wrong)!=0, both finite; 12 new binary decisions. Serialize leaves, residuals and both determinant values to the raw artifact.
- Refuse to claim checks until source-pinned Rust CI passes. Require original-curve scalar replays by an independent implementation and signed PCC for stronger scientific standing.

## Prior lead triage (do not duplicate)

| Lead/source | What actually exists | Action |
| --- | --- | --- |
| #4391, #4454 APS one-relation S3 | Complete Rust unknown-Q logs; optimized public-walk engineering, explicit S3 pair enumeration still generic MITM equivalent | Retain cost/control, do not claim non-generic novelty |
| 24-target composed APS and unfolded rho | 74.434ms APS vs 79.833ms unfolded rho32, producer; only 13/24 paired wins, bootstrap interval crosses parity | No established win; stronger matched tuned WZ and batched rho required |
| #4451 signed S9 zero-fibre | On Dickson31, exact D4 injectivity implies no noncancelling 8-leaf zero relation | Avoid futile target-independent searches, keep known negatives |
| #4466 RR9 distinct/first-jet | Classical supplied-leaf determinant; 128-case and two-base duplicate gate previously passed | Extend multiplicities as here; extraction remains missing |
| #4228/#4349 certified quotient/subresultants | Actual Rust root/certificate machinery, some source-level F4 improvements; no complete target-conditioned B31 extractor | Reuse, ensure original-ideal equivalence and finite signed fibres |
| #4373 anchored m6 / m8 F4 | Censored/negative no completed natural unknown-Q extractor | Do not repeat without changed quotient/elimination |
| #4410 Vieta coset and rational towers | Conditioned cost improvements in some field regimes, regressions in others; no complete prime31 win | Do not extrapolate partial costs |
| #4453 remainder graphs, cryptanalysis #1536 auxiliary IC | Partial relation leads; no complete strong-rho win on this curve | Revisit only with end-to-end one-log cold cost |
| #4460/#4462/#4468 exact WZ rho | Root baseline blocked by unrelated workspace/module and duplicated match-arm compilation defects; fix in transient pinned checkout with exact source assertions | Finish exact 24-target WZ arms before any frontier claim |
| crypto-autoresearcher #3309 Gram | Finite toy cases, no complete B31 solver, remaining missing observations | Deprioritize unchanged forms |

## New prioritized experiments with falsifiers

1. **Blinded B4 and B8 S9 extraction (scientific priority).** From only public R, factor-base polynomial H, and public EC parameters, derive a *zero-dimensional* original-ideal-equivalent quotient encoding signed leaves and all multiplicity branches. Use RR9 confluent rows; recover an actual factor polynomial f(T) and the orientations *without* enumerating the factor-base combinations or 4+4 table. On B4 and B8 compare the exact solution sets against independent exhaustive controls (including negative residuals). Record Groebner/FGLM ranks, largest degree and monomial count, memory, root certification, false solutions, capped outcomes, and full paid wall time. If enumeration slips in, classify it generic and reject a non-generic claim.
2. **B31 natural signed roots and Dickson negatives.** Carry the historical two nonrepresentable Q controls and natural positives. Distinguish R=aG+bQ in the target-conditioned relation loop from R=O; the zero-fibre theorem rules out only the latter. Do not silently call censored solver calls negatives. Increase support only if positive target probability measured rather than assumed.
3. **Complete rank32 index calculus.** For at least 96 freshly drawn hidden-scalar Q, collect independent signed relations, solve factor-base logs, verify every column and target [k]G=Q, and price cold setup, failures, rootfinding, linear algebra, verification, memory, and recovery probability. Compare against tuned library WZ negation rho, best batched rho, and matched external published APS/PKM/F4 implementations on exactly the same point inputs and host. Preregister a statistical gate that excludes equality and uses 95% paired intervals with A/A controls.
4. **Alternative direct algebraic lead.** Evaluate Hermite-jet-aware *coefficient elimination* of f=prod_i(T-x_i) to avoid carrying eight point variables. At B4/B8 retain solution-set equivalence; high-degree determinant reduction, loss of orientations or unexpected positive-dimensional fibres is an explicit stop condition. Do not assume the compact 9x9 determinant yields cheap elimination.

Result classification: **mathematical lemma and Rust predicate improvement, pending source-exact CI**; **0** new complete unknown-scalar logarithms, **0** new strong-rho timing ratios, and **no** demonstrated non-generic breakthrough. Preserve PCC status [pcc:unverified].

## Numerical stop/go budget from measured 2026-11-16 cold controls

The prior source-exact producer 24-Q panel reported 74.433895 ms total for composed APS and 79.833260 ms for unfolded rho32. Per target those are approximately 3.1014 ms and 3.3264 ms respectively, with paired uncertainty crossing parity. These are **not** full externally optimized generic-rho benchmarks.

An ordinary rank-32 factor-base IC approach with 31 unknown base logs and one target log needs **at least 32 independent rows** if it demands full-column-rank reconstruction; in that *specific accounting protocol*, a 3.3264 ms cold target budget leaves at most 103.95 microseconds per successful relation even assuming zero setup, zero failed queries, and free modular linear algebra. Beating the observed APS time instead gives at most 96.92 microseconds. If the natural-relation rate were 22/24 (historical Dickson31 pilot, high sampling uncertainty), then 32 successes would take roughly 35 attempts; the oracle+walk budget would be ~95 microseconds/attempt versus rho32 and ~89 microseconds/attempt versus APS. These are **necessary engineering feasibility checks under the stated 32-row full-rank design**, not generic lower bounds on all algorithms.

Require timed rootfinding plus saturation/verification at B4/B8 before promoting an S9 coefficient extraction. Multi-target amortization can use a separate ledger, but must never be compared to a cold single-log rho result without charging the shared cost.

## Source-exact CI verification for extension B

**PASS:** https://github.com/isogeny-labs/crypto/actions/runs/43987609586, job 133719697836, source workflow from commit eafe23370c33e9a3cd2629f0572b142d1c816bd9. Pinned Rust 1.100.0, 40/40 reused native library tests and 3/3 RR9 example tests. Both actual bases accepted 32/32 distinct positives and 32/32 distinct negatives. The workflow also asserted all 12 m={3,4,8} true/false higher-Taylor-jet decisions, plus double-point controls; no failure. All raw cases and emitted higher-jet controls in artifact https://github.com/isogeny-labs/crypto/actions/runs/43987609586/artifacts/14285653414 (zip SHA256 f3c78c940d3cbf93e884cd3d8a22401de88384f7e318dec6fd1b4b4058b778ce). Developer CI is [pcc:unverified], no independently signed PCC.

A subsequent prospectively frozen ninth-jet false-residual test adds the order-eight coefficient check on nine identical affine points. It is separate from the passing extension B run and may be claimed only if its own source-specific CI succeeds. Neither test includes unknown-scalar extraction or cold cost measurements.
