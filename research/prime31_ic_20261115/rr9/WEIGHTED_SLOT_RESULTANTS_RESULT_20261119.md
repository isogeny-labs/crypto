# Prime31 W8: exact slot-root resultants, radical membership ideal, and 16,777,216-root warning

**Date:** November 19, 2026. Existing original-Rust research lane [PR #4812](https://github.com/isogeny-labs/crypto/pull/4812), with public-only F4/remainder extractor work reserved to [#4692](https://github.com/isogeny-labs/crypto/pull/4692). Frozen protocol: [WEIGHTED_SLOT_RESULTANT_PROTOCOL_20261119.md](WEIGHTED_SLOT_RESULTANT_PROTOCOL_20261119.md). [pcc:unverified]

## New theorem — exact transformed slot membership, without enumeration in its equations

Let E/Fp be an ordinary elliptic curve, and let W be eight ordinary rational factor-base points. Define eight *public* transformed abscissa sets
`T_i = { x([2^i] P): P in W }`, i=0,...,7.

Assume each T_i has eight distinct Fp elements and T_i are pairwise disjoint. For the unknown **monic degree-eight polynomial**
`M(X) = X^8 + c7 X^7 + ... + c0`, define for each slot

`R_i(c0,...,c7) = Res_X(H_i(X),M(X)) = product_(t in T_i) M(t)`,
where `H_i(X) = product_(t in T_i)(X-t)` is known from the original curve and public base.

**Exact iff.** The eight coefficient equations `R_i=0` have a solution precisely when `M` is the product of one linear factor `(X-t_i)` from each T_i. Indeed, the product-of-values equation says a root of M belongs to T_i. Pairwise disjoint sets force eight different roots; monic degree eight forces M to equal their product. Conversely that product makes all eight resultants zero. This is a completely exact **unsigned transformed-x membership** encoding; no generic group-sum lookup or discriminant saturation is required to state or verify it.

**Exact geometry/complexity qualification.** There are exactly `8^8 = 16,777,216` different choices of `(t_0,...,t_7)`, and each yields a different monic M. At any such solution, differentiating R_i with respect to c_k gives

`dR_i/dc_k = [product_(t in T_i, t != t_i) M(t)] * t_i^k` (k=0,...,7).

Its 8×8 Jacobian determinant is
`(product_i product_(t in T_i, t != t_i) M(t)) * (product_(i<j)(t_j-t_i))`.
Every factor is nonzero. Thus **all 16,777,216 geometric solutions are Fp-rational and simple**; the eight-resultant ideal is a reduced zero-dimensional complete intersection (radical, geometrically split). This is a *double-edged result*: algebraic correctness becomes easy to certify, but a full standalone membership quotient has dimension **16.8 million**. A single 32-bit field vector indexed by that quotient takes 64 MiB; the scalar Krylov/standard-monomial approach should **not** first compute all those solutions before coupling public Q. This is not a proof of a general runtime lower bound on all quotient or target-coupled methods.

**Scope boundary:** R_i=0 chooses only **x** coordinates. It does not encode the 2^8 sign choices nor the group-sum equation `Q = sum_i [2^i] S_i`. Therefore this eight-coefficient gate is NOT an eight-variable complete ECDLP extractor; it must be coupled to exact target-conditioned RR9 quotient/remainder or equivalent summation constraints *before* elimination. The earlier 14-variable Horner chart includes additional target conditions and cannot be timing-compared with bare eight-resultant membership.

## Original ordinary Prime31 experiment, independently computed before source CI

Original, unchanged parameters (from original Rust source):
`p=2147483647`, `a=847514846`, `b=2087444724`, prime group order `n=2147446831`, generator `(625037715,2100627251)`, cofactor 1. First 31 increasing admissible ordinary abscissas:
`[0,1,2,3,5,6,8,9,10,12,14,16,18,19,22,26,27,33,35,37,38,41,42,46,47,50,53,56,59,60,66]`.

Four windows 0..7, 8..15, 16..23, 23..30 each have **64 distinct transformed x values** among the eight slot sets — zero cross-slot intersections. This is empirically verified for these exact original-curve windows only, not asserted for arbitrary curves or larger bases.

The frozen SplitMix64 seed `0x534c4f545f524553` produced 128 planted signed W8 words/window (repeated leaf indices permitted), 512 in total. A separate Python bigint-affine implementation computed public W8 weighted Q, the unsigned transformed roots, M and all eight resultants. Then it replaced the chosen slot-0 root with a *second* slot-1 root to form matched monic degree-eight M'; exactly one resultant must fail.

| Independent Python control | Result |
|---|---:|
| Windows with globally disjoint eight-slot supports | 4/4, each 64/64 distinct |
| Planted positive polynomials M | 512/512 |
| Zero resultants on positives | 4,096/4,096 |
| Matched negatives M' failing exactly slot 0 | 512/512 |
| Nonzero 8×8 Jacobian determinants | 512/512 |
| Geometric membership quotient dimension | 16,777,216 |
| Public-only unknown-index extractions | **0** |
| New complete public ECDLP solutions | **0** |

First-case Jacobian determinants (window order): `293475591, 155765304, 235689639, 1686373975` mod p. These are reproducibility checks, **not ECDLP success claims**. The source-specific Rust workflow was updated with original-crate 40-test baseline, focused new example, full 512 positive/negative JSON gate, Python entire-cohort independent replay and SHA256 receipts. **Native Rust CI result must be read separately**; local Python preflight is not substituted for a successful original Rust run. Signed PCC remains unverified.

- Original-Rust example: [prime31_weighted_slot_resultants.rs](../native/examples/prime31_weighted_slot_resultants.rs); reuses `Arithmetic` and `PRIME31` from pinned original crate `1df714645aa683cf975bc76fcacb9842cb4780fd`.
- Entire-cohort independent validator: [verify_prime31_weighted_slot_resultants.py](../native/verify_prime31_weighted_slot_resultants.py).
- Source-specific workflow: [.github/workflows/prime31-horner-xy-chain.yml](https://github.com/isogeny-labs/crypto/blob/automation/prime31-horner-xy-chain-20261119/.github/workflows/prime31-horner-xy-chain.yml).

## Prior lead audit and cross-repository boundaries

1. [#4816](https://github.com/isogeny-labs/crypto/pull/4816) solves a fixed unweighted eight-point squarefree language `M=H_B` with only 256 signed sums. Its negative full-cost result **does not transfer** to weighted W8; this exact new slot-root gate replaces that inappropriate constant M in the unsigned membership component only.
2. [#4812](https://github.com/isogeny-labs/crypto/pull/4812) has validated 21-variable quartic and 14-variable degree-10 Horner/F3 candidate charts, but no public-only unknown-index extraction. Their target-conditioned sign-sum constraints remain essential.
3. [#4692](https://github.com/isogeny-labs/crypto/pull/4692), [#4822](https://github.com/isogeny-labs/crypto/pull/4822), and [#4816](https://github.com/isogeny-labs/crypto/pull/4816) own RR9/F4, sparse Wiedemann and quotient extraction. Avoid creating another F4 engine or copying the fixed unweighted quotient into weighted W8.
4. [#4807](https://github.com/isogeny-labs/crypto/pull/4807)/[#4808](https://github.com/isogeny-labs/crypto/pull/4808) improved **generic-table relation grading** with power-of-two weights, but not public algebraic extraction; [#4728](https://github.com/isogeny-labs/crypto/pull/4728) is the existing rank-31, 1,024-Q complete original-Rust generic-grader baseline.
5. The strongest historical complete shared-BSGS control [#4532](https://github.com/isogeny-labs/crypto/pull/4532) is **170.113 ms for 1,024 targets**, from a previous runner execution. Given 1,312 attempts in #4728, <129.66 microseconds per attempt is only an optimistic necessary extractor budget, ignoring all other stages; same-runner comparison is still mandatory.
6. Prime-field algebraic/index-calculus prior art: [Petit–Kosters–Messeng 2016](https://doi.org/10.1007/978-3-662-49387-8_1), [Amadori–Pintore–Sala 2018](https://doi.org/10.1016/j.ffa.2018.01.009), [Kudo–Yokota–Takahashi–Yasuda 2018](https://doi.org/10.1007/978-3-030-00434-7_19). These establish summation-polynomial, hybrid and symmetry traditions; do **not** claim novelty priority from an incomplete search. Cross-repository `crypto-autoresearcher` and `cryptanalysis` checks remain bounded by accessible GitHub sources; shared research-KB MCP was not exposed in this runtime.

## Highest-value falsifier: couple Q before the large quotient

Use existing #4692 F4/remainder engine to add the eight public slot-resultant equations **together with** the exact RR9 public-Q constraints. Freeze monomial order and degree caps; compare against the 14-var Horner chart and 21-var quartic chart on identical blinded 46-positive/18-negative B8 panel. For positives recover actual signed slot indices from public Q alone, verify with original Rust ECC; for negatives distinguish a certified UNSAT from a timeout. Record generic sum table calls = 0, elimination degree, matrix nonzeros, peak RSS, time per attempted residual, failed searches and setup amortization. A standalone R_i-only quotient of 16.8 million roots is **not** an acceptable shortcut.

Only if public-only B8 extraction passes should it replace the graded oracle inside rank31/1024 complete Rust IC, with 1/64/1024 cold/cached curves and same-runner BSGS/rho/index-calculus frontier. At the moment, the new result is an exact membership reduction plus a dimension warning, not a measured cheaper complete ECDLP solver.

## Exact-source original-Rust and independent validation closeout — November 19, 21:35 UTC

**[GitHub Actions run #44619747239](https://github.com/isogeny-labs/crypto/actions/runs/44619747239) finished SUCCESS**, verified by fetching the completed job and its exact log. The registered candidate workflow change was commit `688a3fca29b19c8a5cda1c1b10be6aa10eedda90`; pinned original Rust arithmetic `1df714645aa683cf975bc76fcacb9842cb4780fd` on Rust 1.100.0, ubuntu-24.04.

- Baseline original native library **40/40**, preexisting x/y **2/2**, x-only **4/4**, weighted/fixed discriminator **1/1**, and new slot-resultant focused Rust **1/1** tests passed.
- New original-crate release producer passed the strict JSON gate: four 64/64-disjoint transformed windows; 512 positive / 512 matched negative polynomials; 4,096 zero positive resultants; 512 single-slot negative rejections; **512/512 nonzero Jacobians**; exactly `16,777,216` as the symbolic quotient dimension.
- A separately authored Python BigInt affine implementation replayed **all 512** new source cases, independently regenerated the first-case weighted targets, full coefficients, negative resultants and determinants, checked original generator order, and matched the original-crate output. `pcc_signed=false`; not an independently signed validation.
- [Raw run artifact #14576493274](https://github.com/isogeny-labs/crypto/actions/runs/44619747239/artifacts/14576493274). Original Rust new JSON SHA256: `690f24656bf2a7587ba10d8dcb23ad017a43c5737216db233cad505e94242fa4`; independent verifier output SHA256: `c789ff538fd080188e6fe7b0b42f2f3d64d5f999a131f81636f67ae22644f7cd`; original Rust source SHA256: `1aec75ac9b289b939997bc51ffebf4dc5a4f753fa0bd18e1116ab61c5dc6e0ad`.

**Disposition:** The exact unsigned W8 resultant membership characterization and its radical/16.8M solution-space warning are mathematically established and original-source checked on the registered Prime31 windows. No public-only target-conditioned RR9 extraction, new 1,024-log solve, or lower complete solving cost has been claimed.
