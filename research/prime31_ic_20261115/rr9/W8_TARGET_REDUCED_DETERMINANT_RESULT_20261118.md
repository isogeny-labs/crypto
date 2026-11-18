# Prime31 signed W8 — exact 9x9 to 8x8 determinant elimination

**Date:** 2026-11-18 UTC. **Tracking:** [PR #4728](https://github.com/isogeny-labs/crypto/pull/4728). [Frozen protocol](W8_TARGET_REDUCED_DETERMINANT_PROTOCOL_20261118.md), [original-Rust example](../native/examples/prime31_w8_det8.rs), [independent Python validator](../native/verify_prime31_w8_det8.py), [source-specific workflow](../../../.github/workflows/prime31-w8-det8.yml).

**Scope:** exact smaller finite-field circuit for a **supplied** signed relation, potentially useful to the RR9 coefficient-elimination lane. **No public unknown signed points have been extracted, no complete ECDLP solve was run, and no complete-cost advantage is established.** Signed independent PCC verification pending.

## Mathematical result (exact equality, not merely the same zero set)

On the unchanged original ordinary prime-order cofactor-one curve `y²=x³+847514846x+2087444724 (mod 2147483647)`, let `R=(r,s)!=O` be a public residual and `Q_i=(x_i,y_i)` eight signed weighted slot choices. The target-conditioned eight-column matrix has row

`[(x_i-r), (x_i-r)x_i, (x_i-r)x_i², (x_i-r)x_i³, (y_i+s), (y_i+s)x_i, (y_i+s)x_i², (y_i+s)x_i³]`.

Its determinant equals the classical 9x9 Riemann--Roch evaluation determinant on `Q_1,...,Q_8,-R`, **as an element of the ground field**. Proof: the linear change of `L(9O)` columns from `[1,x,y,x²,xy,x³,x²y,x⁴,x³y]` to `[1,(x-r)A_0,...,(x-r)A_3,(y+s)V_0,...,(y+s)V_3]` has determinant **+1**. Its final row at `-R` is `[1,0,...,0]`. Expanding along that row with positive cofactor yields the 8x8 determinant, exactly. This is a classical change of basis, not a novel Riemann--Roch theorem.

For pairwise distinct non-infinite `Q_1,...,Q_8,-R`, the existing genus-one divisor argument then gives `det8=0 iff sum Q_i=R`. **Do not apply this biconditional to repeated points:** two identical signed rows, or a supplied point exactly equal to `-R`, force a raw false zero. The safe test explicitly rejects these cases; previously implemented Hermite jets are the correct repeated-point branch. `R=O` is not covered.

## Independent local BigInt preflight

Before committing the new Rust code, a separate Python implementation reconstructed the first 31 rational-x base points and all 248 weighted positive x-coordinates with public weights `(1,3,5,7,11,13,17,19)`; all 248 are distinct. It tested the 16 registered planted signed tuples with three target/input arms per fixture.

| Check on original curve | Local Python preflight |
|---|---:|
| Genuine supplied eight-slot sums at their actual R | 16/16 zero |
| Flip first sign but retain R | 16/16 nonzero |
| Retain signed points but replace R by R+G | 16/16 nonzero |
| Exact **det8 = det9** field residues across all cases | **48/48** |
| All 16 genuine-positive coefficient matrices have 7 pivots | 16/16 |

The independent finite-field Gaussian elimination preflight accumulated **8,432 vs 11,936 elimination-stage field multiplications** and **368 vs 416 field inversions** (8x8 vs 9x9), excluding matrix construction and multiplication work inside inversions. This is a **finite operation count from local Python**, not a native Rust timing or full solver advantage. The exact original-Rust example additionally instruments all modular multiplications including inversion exponentiation and emits per-case costs, so unlike the preflight it is designed to produce source-bound native costs.

**Source-bound validation:** the Rust 1.100.0 example, two focused tests, existing 40 library tests, independent Python replay of all 48 exact numeric determinant residues, and CI workflow have been committed to the PR branch. A GitHub Actions success and raw artifact must be linked **only after it is actually observed**; this report does not infer a PASS merely from creating the workflow.

## Why this could matter, and what remains hard

The eight matrix rows are *slot-labelled*. For B8 each has sixteen public signed alternatives, representable by four Boolean bits. Their multilinear interpolation has degree at most four in the four bits belonging to that row; consequently the determinant circuit is a polynomial of total degree at most 32 in 32 Boolean index/sign bits (with Boolean relations `b_i²-b_i=0`). This is a new **candidate formulation for this specific weighted-slot lane**, not an extractor yet. It removes the seven unknown RR9 chart coefficients and the eight 31-factor degree-62 resultants from the *input representation*, at the price of combinatorial Boolean variables and a determinant circuit. **No claim that the determinant's expanded polynomial is sparse or that F4/SAT can solve it efficiently.**

### Prior art / lead audit

- [#4466 original RR9 9x9 determinant](https://github.com/isogeny-labs/crypto/pull/4466) already establishes the classical determinant predicate, confluent Hermite jets and the duplicate-row obstruction. **Reuse and reduce; do not claim rediscovery of a new determinant method.**
- [#4539 signed seven-chart](https://github.com/isogeny-labs/crypto/pull/4539), [#4728 eight disjoint weighted resultants](https://github.com/isogeny-labs/crypto/pull/4728), [#4692 unfinished RR9 extractor](https://github.com/isogeny-labs/crypto/pull/4692), and [#4717 multiplicity strata](https://github.com/isogeny-labs/crypto/pull/4717) are the algebraic competitors; no duplicate independent solver implementation started.
- [#4728 exact generic B4/B8 grader](W8_B4_B8_SIGNED_ORACLE_RESULT_20261118.md) supplies the 64 unseen B8 targets (46 positive, 18 negative) and planted controls; **must stay outside** candidate algebraic extraction.
- [#4714 original complete Rust M6](https://github.com/isogeny-labs/crypto/pull/4714) learned rank31 and recovered 1,024/1,024 ECDLP values, but generic table enumeration lost to matched shared BSGS by roughly 21x. [#4722 support ceiling](https://github.com/isogeny-labs/crypto/pull/4722) limits weight-yield tuning; component reductions alone cannot establish the full-cost win.
- Relevant published prior art: [Amadori--Pintore--Sala, 2018](https://doi.org/10.1016/j.ffa.2018.01.009), and Semaev/summation-polynomial Gröbner methods. Repository research-KB MCP and executable wrapper are inaccessible in this runtime; bounded exact GitHub code search (including `RR9 determinant`, `chart elimination`, `8x8 interpolation determinant`) found the prior 9x9 method. This is not proof of novelty.

### Prioritized experiments

1. **Actual solver gate, B8:** implement a Boolean-selector/DAG evaluation plus SAT/F4 backend for the 8x8 determinant, using the existing Rust field arithmetic and F4 infrastructure; no 4+4 group tables. Require all eight planted witnesses and exact positive/negative decisions on the frozen 64 natural residuals, with separate handling of multiplicity and timeout. Test whether an explicit signed incidence system avoids the 32-bit determinant expansion.
2. **Competing algebraic gate:** benchmark weighted 7-chart factored resultants vs this determinant circuit vs low-degree slot-root auxiliary equations, on *identical* B4/B8 target sets and bounded CPU/RSS budgets. Log field operations, Gröbner matrices, failed/censored attempts, witness correctness and original-curve additions.
3. **Whole-solve gate:** only after B8 natural extraction, scale to B31; learn rank31 and recover 1,024 blinded unknown logs with the unchanged native Rust complete-solver pipeline, and report fully charged cold and amortized costs vs same-runner mixed IC, any-witness M6, optimized shared BSGS and rho. New component operations or positive planted charts do not fulfill this gate.

**No cheaper complete non-generic prime-field ECDLP solver is asserted by this result.**
