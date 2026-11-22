# Prime31 W8: complementary subset-zeta plus Walsh transform

**2026-11-18**, [PR #4728](https://github.com/isogeny-labs/crypto/pull/4728) · [frozen prospective protocol](W8_ZETA_WALSH_PROTOCOL_20261118.md) · [original-crate Rust source](../native/examples/prime31_w8_zeta_fourier.rs) · [independent checker](../native/verify_prime31_w8_zeta_fourier.py) · [source-specific CI workflow](../../../.github/workflows/prime31-w8-zeta-walsh.yml).

## Mathematical result and boundaries

This is an exact acceleration of an **already supplied-eight-x-coordinates** signed Riemann–Roch determinant evaluator. The input contains all eight weighted elliptic-curve points; only the sign mask is scanned. **It does not find the unknown eight factor-base indices or solve an unknown ECDLP**.

For target R=(r,s), x_i,y_i fixed, and any four-subset U of [8], write S=[8]\U, A_S=product_(i in S)(x_i-r), Y_T=product_(i in T)y_i, and Δ(U)=product_(i<j in U)(x_j-x_i). Define

    h_U=(-1)^(sum_(i in S)i) Δ(S)Δ(U) A_S (if |U|=4), otherwise 0.
    H_T=Σ_(U⊇T) h_U.
    c_T=Y_T s^(4-|T|) H_T (if |T|<=4), otherwise 0.

Then the signed target-reduced 8x8 RR9 determinant is exactly D(ε)=Σ_T c_T product_(i in T) ε_i. Proof: Laplace expand the first four columns as in the existing 70-minor protocol, expand each product_(j in U)(s+ε_j y_j) into subsets T⊆U, and exchange the 70- and 16-element sums. The inner sum over all U⊇T is precisely the upper Boolean subset-zeta transform. The 256 determinant values follow via 8-bit Walsh-Hadamard transform.

Both transforms require only additions/subtractions; neither requires field inversion or elliptic-curve group addition. The new algebraic factorization is an instance of **standard Boolean subset convolution**, not a new cryptanalytic hardness result.

## Deterministic component arithmetic accounting

In this **specific** Rust implementation, counting Fp multiplications only, each supplied-x, supplied-target 256-sign sweep needs:

| Stage | Multiplies |
|---|---:|
| 255 nonempty mask products A_mask and Y_mask | 510 |
| Vandermonde products for 70 four-subsets | 420 |
| Powers s^1 through s^4 | 4 |
| 70 complementary-minor h_U terms (two products each) | 140 |
| 163 coefficient c_T terms (two products each) | 326 |
| Upper subset-zeta (1024 modular sums), Walsh (1024 butterflies) | **0** |
| **Total per 256 signed decisions** | **1,400** |

The previous 70-minor Fourier implementation counts 5,600 Fp multiplications per target. **The exact algorithmic prediction is 4× fewer counted field multiplications** for this isolated supplied-x component, and 0 Fp inversions in both fast arms. For 32 identical test cases: 44,800 vs 179,200 expected multiplies. This is **not a prediction of 4× wall-time or a whole-solver improvement**; modular additions, branching, test-fixture EC arithmetic and independent checking are outside that specific multiplier count.

## Evidence and provenance

- Independent local Python BigInt feasibility pilot (not Rust): **2,048 exact determinant residues** for four planted/shifted original-curve target fixtures, all identical to independently evaluated 8x8 Gaussian residues, 2026-11-18. Local code was not committed, so this is exploratory preflight evidence [pcc:unverified].
- Native Rust source uses the pinned Prime31 original arithmetic crate `1df714645aa683cf975bc76fcacb9842cb4780fd`, unchanged `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`, cofactor one, 31-point canonical positive x factor base and weights (1,3,5,7,11,13,17,19).
- **Native source-specific GitHub Actions and independent full 8,192-case Python checks: PASSED on source-specific run #44391371316 (producer evidence only).** The Python checker independently reconstructs all original-curve points, rechecks the 8,192 direct Gaussian residues and group identities through the prior checker, then evaluates this result's complementary zeta formula independently (sum over superset 4-subsets, with direct sign-monomial evaluation rather than Rust Walsh).
- No signed independent PCC verdict or whole-solver cost advantage.

## Rejected or already-explored leads and next decisive gate

1. The prior 70-minor/256-sign kernel on [PR #4728](https://github.com/isogeny-labs/crypto/pull/4728) and old 8x8/9x9 determinant controls are **supplied-x** evaluations; this extension avoids the redundant 16-fold sign-monomial inner loops but does not remove index search. The older [RR9/F4 #4692](https://github.com/isogeny-labs/crypto/pull/4692) is the correct lane for blinded unknown-index extraction.
2. Prior [#4714](https://github.com/isogeny-labs/crypto/pull/4714) complete 1024-Q weighted-M6 Rust index calculus was generic 3+3 and previously ~21× slower than the matched shared BSGS; [#4722](https://github.com/isogeny-labs/crypto/pull/4722) bounds the current M6 relation support. Do not repeat weight tuning as if it removes enumeration.
3. [#4717](https://github.com/isogeny-labs/crypto/pull/4717) multiplicity strata, [#4720](https://github.com/isogeny-labs/crypto/pull/4720) algebraic two-point projection and the negative [crypto-autoresearcher SA-04/SA-18](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/experiments/semaev-alternatives-20261105/SUMMARY.md) determinant/norm experiments constrain novelty; [#4746](https://github.com/isogeny-labs/crypto/pull/4746) withdraws a separate favorable exponent.
4. Primary published prime-field methods [Petit–Kosters–Messeng 2016](https://doi.org/10.1007/978-3-662-49387-8_1) and [Amadori–Pintore–Sala 2018](https://doi.org/10.1016/j.ffa.2018.01.009) are the relevant algebraic prior art. No research-KB access in this runtime; primary repository and paper metadata were consulted instead.

**Next experiment, not yet accomplished:** Build the public-target unknown-slot-index selector that treats each x_i,y_i as an algebraic element of the B4/B8 slot-membership set, symbolically contracts the h_U terms in a polynomial constraint system, and recovers true witnesses with no planted chart, generic 3+3/4+4 sum tables or leaked x indices. Evaluate against the frozen B8 46-hit/18-miss natural panel with strict component cost and negative controls. If it survives, feed natural B31 relations into the existing Rust rank31/1024-Q full solve and account for all collection, linear algebra, target attempts, setup and verification against strongest mixed IC, shared BSGS and rho on one host. Do not claim a cheaper complete solver until these gates are passed.

## Native original-curve source-specific result (2026-11-18, 20:31 UTC)

[Exact-source GitHub Actions run #44391371316](https://github.com/isogeny-labs/crypto/actions/runs/44391371316) **PASS** at Rust source commit `5d82f7b39ab4e04d08eff94a7c86b11aebdd769b` (original arithmetic pinned at `1df714645aa683cf975bc76fcacb9842cb4780fd`). [Raw 32-case JSON artifact #14468601894](https://github.com/isogeny-labs/crypto/actions/runs/44391371316/artifacts/14468601894) has per-case counted field operations and alternate-order diagnostic nanoseconds.

| Whole 32-target, 8192-mask supplied-x panel | Direct 8x8 Gaussian | Prior Fourier | New zeta/Walsh |
|---|---:|---:|---:|
| Counted Fp multiplications | 6,159,408 | 179,200 | **44,800** |
| Counted Fp inversions | 65,520 | 0 | **0** |
| Exact determinant residue decisions | 8,192/8,192 | 8,192/8,192 | **8,192/8,192** |
| Public unknown x-index extractions | 0 | 0 | **0** |

Exactly **4.00× fewer Fp multiplications than the previous Fourier kernel** (or 137.49× fewer than direct Gaussian) on this isolated **given-x** task. This number is **not** a speedup of a complete ECDLP solver; total wall-time is separately recorded without a calibrated speedup claim. All original 40 Rust library tests and one new full Rust example test passed. The independent Python BigInt checker recomputed all 8192 direct 8x8 determinant residues and elliptic-curve group identities, all 16 planted sign positives and 16 shifted negatives, and separately evaluated the new complementary-zeta formula with direct monomial summation instead of the Rust Walsh implementation. It also recalculated the 32×1400=44800 instrumented multiplications and checked the raw record's per-target counters.

The first CI execution [#44390771119](https://github.com/isogeny-labs/crypto/actions/runs/44390771119) **failed** because the emitted Rust example contained `nz+zeros.len()` instead of `nz+=zeros.len()` when accumulating fixture positives. This was a producer bug in the result counter, **not** the determinant evaluator. It was corrected at `5d82f7b`, then exact-source CI passed, including all 16 positive controls. Failed runs are retained and disclosed.

**Evidence standing:** source-bound producer execution plus separately authored independent Python arithmetic replay [pcc:unverified]. No signed PCC verdict; no nonenumerative x-index selection; no new complete-solver advantage. The next experiment remains the *unknown-index algebraic selector*, not further given-x micro-optimization.
