# Prime31 W8: all 256 signed RR9 predicates from one 70-minor tensor + Walsh transform

**November 18, 2026** · existing [PR #4728](https://github.com/isogeny-labs/crypto/pull/4728) · [prospective native protocol](W8_SIGN_FOURIER_PROTOCOL_20261118.md) · [original-crate Rust example](../native/examples/prime31_w8_sign_fourier.rs) · [independent BigInt checker](../native/verify_prime31_w8_sign_fourier.py) · [source-specific workflow](../../../.github/workflows/prime31-w8-sign-fourier.yml).

## Result and exact scope

A classical **Cauchy–Binet/Laplace expansion** turns the target-conditioned 8×8 elliptic-curve Riemann–Roch determinant into a sum of **70 complementary four-row Vandermonde products**. Its dependence on eight independent **signs** is multilinear of Fourier degree **at most four**: of the 256 possible sign-monomial coefficients, only `sum_{k=0}^4 binom(8,k) = 163` can be nonzero. The 70 four-subset terms expand into 1,120 fixed sign-monomial contributions, followed by an 8-stage, 256-entry Walsh-Hadamard transform (1,024 butterflies). There are **zero finite-field inversions and zero EC group additions inside the fast finite-field sign evaluator**.

The 8 input **x-coordinate indices are supplied**. This advances a deterministic algebraic *component*, not public-target index extraction, and cannot by itself beat 4+4 group enumeration. It is a standard multilinear expansion rather than a new cryptanalytic theorem. A valid relation output must still be checked against original elliptic-curve group addition and witness distinctness.

## Identity

Set `v(x)=(1,x,x²,x³)`, `a_i=x_i-r`, `b_i=ε_i y_i+s`, where `R=(r,s)` and `ε_i=±1`. The signed 8×8 determinant has rows `(a_i v(x_i), b_i v(x_i))`. For a four-subset S of indices 0,...,7 let `Δ(S)=∏_{i<j∈S}(x_j-x_i)`. Then

`D(ε)=Σ_{|S|=4} (-1)^{Σ_{i∈S}i} Δ(S)Δ(Sᶜ) [∏_{i∈S}(x_i-r)] [∏_{j∈Sᶜ}(ε_j y_j+s)]`.

This follows from Laplace expansion by the first four columns and the Vandermonde identity. Expanding the four `b` factors gives `D(ε)=Σ_{|T|≤4}c_T∏_{i∈T}ε_i`. Over any odd prime, Walsh evaluation of this coefficient vector recovers **each signed determinant residue exactly**, not merely its zero status. The mathematical cost is constant with respect to the number of sign choices because eight is fixed; it does **not** remove the exponentially many possible x-index tuples.

## Original-curve verification panel

Curve unchanged: `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`, cofactor one, canonical first-31 base, weights 1/3/5/7/11/13/17/19. All **248 positive weighted x-coordinates** are distinct. Sixteen planted x-tuples with their actual public target and sixteen identically indexed target-plus-G negative controls, each tested under all 256 sign choices.

An **independent local Python BigInt preflight** (not Rust or PCC) completed **8,192/8,192** exact residue comparisons of the new Fourier formula with direct 8×8 finite-field Gaussian determinants, and independently replayed **8,192/8,192** signed EC group identities. Of the 32 target panels, all 16 planted panels had **exactly one** valid signed mask and all 16 target-shifted panels had **zero**. This is a fixed positive-control/negative-control test; no unseen public target x-indices were recovered.

Source-bound Rust CI is a separate gate. Operation counts and whole-panel AB/BA diagnostic elapsed time should be quoted **only after** its run and independent Python replay pass. Small component timings cannot be reported as ECDLP-solving speedups.

## Previously searched avenues, to avoid duplication

- [RR9 weighted resultant lane #4728](https://github.com/isogeny-labs/crypto/pull/4728): supplied seven-chart certificates and the 8×8 determinant; current Fourier arm extends the **sign-sweep evaluator** without duplicating root/resultant or witness truth tables.
- [RR9/F4 #4692](https://github.com/isogeny-labs/crypto/pull/4692), [low-power strata #4717](https://github.com/isogeny-labs/crypto/pull/4717), [signed-pair projector #4720](https://github.com/isogeny-labs/crypto/pull/4720): reuse as public-only extraction infrastructure. No claim those lanes have solved 64 unseen B8 x-index tuples.
- [M6 complete 1,024-target Rust #4714](https://github.com/isogeny-labs/crypto/pull/4714), [M6 formal-collision bound #4722](https://github.com/isogeny-labs/crypto/pull/4722): complete but generic 3+3 control, previously losing to optimized shared BSGS by 20.85–23.32×. Keep same target/runner and include failed trials in future whole solves.
- [crypto-autoresearcher alternatives SA-04, SA-18](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/experiments/semaev-alternatives-20261105/SUMMARY.md): prior RR norm and determinant approaches failed registered gates; do not describe this as the first determinant attack.
- [cryptanalysis](https://github.com/isogeny-labs/cryptanalysis): existing generic/prime-field arithmetic and binary SAT benchmark are reuse or instrumentation references, not proof of a Prime31 non-generic advantage.
- [Amadori–Pintore–Sala 2018](https://doi.org/10.1016/j.ffa.2018.01.009), [Petit–Kosters–Messeng 2016](https://eprint.iacr.org/2016/475): established prime-field summation-polynomial index-calculus prior art. New [Bochum 2026 proposal](https://github.com/bochum-crypto/subexponential-ecdl) and [claimed end-to-end result #4698](https://github.com/isogeny-labs/crypto/pull/4698) require independent proof and complete benchmark audits. [#4746](https://github.com/isogeny-labs/crypto/pull/4746) retracted an earlier favorable 0.43 cost exponent in a separate PKM setting.

Research-KB is unavailable in this connector runtime; this is a primary-GitHub and externally published sources review, not a complete novelty survey.

## Prioritized next computational gates

1. **Unknown-index selector/F4 extractor:** Build a genuinely public-residual-only Boolean/prime-field constraint for eight B8 slot indices, using the signed Fourier coefficients **symbolically** where cheaper than dense RR9; read no 4+4 grader table and no planted chart. Score against the sealed B8 panel of 46 natural positive/18 natural negative targets, log all timeouts and false witnesses, and compare candidate algebraic work against the baseline 1.009s generic 72-query grader only as a *component* diagnostic.
2. **Explicit cost kill gate:** If the bounded B8 selector uses more field-ops/memory than the existing optimized generic mixed relation extractor at matched yielding rates, reject it before B31.
3. **Complete path:** For any surviving extraction arm, B31 must produce rank 31 and recover 1,024 blinded unknown Prime31 logarithms in the original Rust pipeline, with cold/online setup, failed trials, elimination, and certification. Compare same-host against strongest mixed IC, shared BSGS and rho in both single-target and amortized regimes. Signed independent PCC is separately pending.

**Evidence label:** original-curve BigInt preflight mathematically checked; native Rust source and CI committed; source-specific CI and independent signed PCC statuses are tracked separately. No cheaper complete solver claimed here.


## Fixed-x combinatorial scaling screen (exact accounting, no extrapolated wall times)

The committed current Fourier kernel executes **70 × (12 Vandermonde + 4 left-factor + 16 × 4 sign-monomial) = 5,600 finite-field multiplications** for each known 8-tuple of x-coordinates, followed by 1,024 Walsh butterflies, and no inversions. It is not efficient to iterate all x tuples as the extraction algorithm:

| Factor-base size | 8 unsigned x-index tuples | This implementation's raw sweep multiplications per target if fully enumerated |
|---|---:|---:|
| B4 | 65,536 | 367,001,600 |
| B8 | 16,777,216 | 93,952,409,600 |
| B31 | 852,891,037,441 | 4,776,189,809,669,600 |

For B4, the product is exactly **367,001,600** Fp multiplications. (The Walsh adds and signed-witness extraction are additional.) These are *counts for the specified naive enumeration*, **not unconditional lower bounds** for F4, tensor contraction, reuse across tuples, or every possible algebraic extractor. They demonstrate why the Fourier component cannot progress to B31 merely by looping over the 31^8 possible x choices. Public-target x-index recovery is the decisive missing algorithm.


## Native Rust measured result — source-bound CI PASS

At 2026-11-18T19:38Z, **[exact-source GitHub Actions #44381852137](https://github.com/isogeny-labs/crypto/actions/runs/44381852137)** passed using pinned Rust 1.100.0 and pinned existing original Prime31 Arithmetic at `1df714645aa683cf975bc76fcacb9842cb4780fd`, with only the new example overlaid.

- All **40/40** original Rust library regression tests passed; the new 8,192-predicate signed Fourier Rust test passed **1/1**.
- The stand-alone independent Python BigInt verifier recalculated **8,192/8,192** complete determinant residual / group-relation truth decisions, all 16 planted signed positives and all 16 shifted negative arms, their checksums and supplied-curve identities. The frozen cases have exactly one sign mask on each planted target, zero on the target-shifted negative.
- **Measured native counted Fp multiplications, whole 32-target panel, all 256 signs per target**: direct per-mask 8x8 Gaussian evaluation **6,159,408** versus Fourier **179,200**; **34.37170× fewer**. Native counted inversions: direct **65,520** versus Fourier **zero**. The counts exclude curve/fixture generation and original-curve correctness verification equally on both arms. This is **only** cost of the algebraic signed-**given-x** determinant component.
- Raw JSON evidence: **[artifact #14464093350](https://github.com/isogeny-labs/crypto/actions/runs/44381852137/artifacts/14464093350)**, ZIP digest `sha256:9f500f2dc788c5759da9ab6579b1dbcecff88914265f7ee23ab5f1b93718cdd7`. The first native CI attempt [#44381033092](https://github.com/isogeny-labs/crypto/actions/runs/44381033092) caught a Rust E0499 nested mutable borrow; the expression was evaluated in two statements and the next exact-source run **passed**. No arithmetic changes from the preflight.

**Evidence status:** source-specific producer CI + separately authored Python BigInt replay passed. No independent signed PCC, no measured unknown-index extractor, no complete ECDLP benchmark or claim of beating BSGS/rho. The 34.37× is not a solver speedup and must not be extrapolated across all `B^8` index tuples.
