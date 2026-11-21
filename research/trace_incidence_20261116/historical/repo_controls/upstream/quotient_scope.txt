# EXP-R31-I — exact target-dependent S3 quotient/GCD extraction on ordinary prime31

**Status (2026-11-15):** development / [pcc:unverified]. Primary implementation: [`examples/prime31_s3_quotient_gcd.rs`](examples/prime31_s3_quotient_gcd.rs). This is **algebraic index-calculus oracle engineering**, not an established novel asymptotic ECDLP algorithm or cheaper complete solver. This note is source-bound to the existing #4188 prime31 branch; no comparison row is promoted.

## Literature, prior leads and choice of experiment

Read the repo [native baseline](README.md), [two-sum support gate](TWO_SUM_YIELD_GATE.md), [S3 discriminant / generic-equivalence correction](S3_DISCRIMINANT_CLASSIFICATION_GATE_20261115.md), [paired-S3 certified quotient #4228](https://github.com/isogeny-labs/crypto/pull/4228), [dense-versus-tower #4275](https://github.com/isogeny-labs/crypto/pull/4275), [whole-call gamma/delta gate #4077](https://github.com/isogeny-labs/crypto/pull/4077), [chained transfers #3877](https://github.com/isogeny-labs/crypto/pull/3877), [auxiliary-map Rust IC, cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536) and [autoresearcher lead atlas #3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305). Also checked the primary [Petit–Kosters–Messeng PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1) large-degree rational-map formulation and Diem's [prime/extension-field paper](https://www.math.uni-leipzig.de/~diem/preprints/ind-cal-ell-curves.pdf). **Classical Semaev, resultant, quotient-remainder and GCD elimination are prior art.** This is a small exact specialisation and experimental implementation, not a novelty claim.

**KB access:** The required `kb_status/search_knowledge` tools are not exposed in this runtime and the `scripts/research-kb` repository wrapper is not materialized; bounded primary GitHub notes, open PRs and literature were used instead. An exhaustive searched-leads assertion would be unsupported.

## The exact target-dependent construction

Same full-prime-order ordinary curve throughout:

- `p=2147483647, a=847514846, b=2087444724`
- `n=2147446831, G=(625037715,2100627251)`; cofactor one
- Canonical depth-six Dickson fibre has `B=31` curve points. Depth eight has `B=135` for additional independent-language test; *actual liftable cardinality*, not fibre degree.
- Input: public `R=(t,y_R)`, **no target scalar**, and the factor-base abscissa polynomial `H(X)=prod_i(X-x_i)`. The algebraic stage never reads point ordinates or computes known-point EC sums.

Put the standard short-Weierstrass Semaev polynomial in the form

```
S3(X,Y,t) = A(X)Y^2 + B(X)Y + C(X)
A(X)=(X-t)^2
B(X)=-2[(X+t)(Xt+a)+2b]
C(X)=(Xt-a)^2-4b(X+t).
```

When `t` is not a root of `H`, `A(X)` is invertible in the *finite product of fields* `T=Fp[X]/(H)`. Normalize `q(Y)=Y²+beta Y+gamma` with `beta=B/A`, `gamma=C/A` in `T`. Reduce the **full** factor-base polynomial `H(Y)` modulo `q(Y)` by Horner recurrence:

```
(r0,r1) <- (0,0)
for c in reverse(coefficients(H)):
    (r0,r1) <- (c-gamma*r1, r0-beta*r1) in T.
```

Thus `H(Y) mod q(Y) = r0+r1 Y`. Their exact resultant with the monic quadratic is

```
D(X)=r0²-beta*r0*r1+gamma*r1² mod H(X)
J(X)=gcd(H(X),D(X)).
```

**Proof of completeness for the abscissa oracle:** for every `x_i` with `A(x_i) != 0`, `D(x_i)=0` if and only if `gcd_Y(H(Y),S3(x_i,Y,t))` has positive degree. Since `H` is a product of distinct linear factors in `Fp[Y]`, every common root is some actual base abscissa `x_j`. For each root of `J`, the implementation computes that one-variable GCD and selects its linear roots among the already-known `x_j`. It performs no `(i,j)` point-pair group join. Only *after* algebraic candidate extraction are all signed ordinates lifted and checked against the **full** public point `R`, rejecting matches to `-R` and erroneous Semaev lift choices.

**Exceptional `t=x_i`:** remove `X-t` exactly from `H` before inverting `A`; process the removed root separately using `gcd_Y(H(Y),S3(t,Y,t))`. Zero- or linear-degree S3 specialisations are handled by the same polynomial GCD. The identity target needs a separate inverse-pair rule; it is not an affine S3 input.

**Resource interpretation:** with naive quadratic polynomial arithmetic, modular multiplication in `Fp[X]/H` is O(B²) base-field operations and the Horner stage uses O(B) such operations (O(B³)); other Euclidean steps add cost. This is not a sub-generic complexity bound or a breakthrough at large B. The symbolic representation's ability to test many pairs at once is *not* a proof of subgeneric cost: exact two-summand support remains approximately B²/n, and all setup/gcd/inversion/root-selection costs must be charged.

## Independent producer-local exact arithmetic

I implemented the same mathematical gate separately with Python integer field arithmetic and a distinct elliptic-curve group law. The source Rust is the primary checked-in implementation; this is a **producer-local** cross-language calculation, not a signed PCC certificate.

The Python check established the following on the inherited prime31 curve:

| Cell | Exact algebraic outcome | Post-hoc exhaustive signed EC comparison |
|---|---|---|
| B31, `R=P3+P9` | degree-2 `J`, ordered candidates (3,9),(9,3); two signed rows | identical |
| B31, `R=2P4` | degree-1 `J`, candidate (4,4) | identical |
| B31, `R=P0-P7` | degree-2 `J`, candidates (0,7),(7,0) | identical |
| B31, `R=[1,17,137,20261115]G` (four separate public controls) | four degree-0 `J`, zero signed rows | identical |
| B31, `R=P0,P1,P2` | three `t`-in-base exceptional paths, no signed rows | identical |
| B135, `R=P0+P14` | degree-2 `J`, candidates (0,14),(14,0) | same signed EC relation |

The Python study took ~0.34s to run the B31 algebraic and *all-pair EC verification* 10-target panel and ~1.66s for the single B135 algebraic producer fixture on its local interpreter. **Do not use these as Rust or cold ECDLP timings.** They were not paired, calibrated, ecbench-admitted or independent PCC signed. The checked-in Rust example includes a `#[test]` comparing all signed relations for depth-2/depth-6 positive, public and exceptional cases to a test-only generic addition oracle. The generic control cannot feed the algebraic solver.

## Reproduction and explicit gates

```sh
cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_quotient_gcd
cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_quotient_gcd
```

The focused [development workflow](../../../.github/workflows/prime31-algebraic-ic.yml) calls both after the existing signed-S3 discrimination test. Rust compilation, actual Rust output and current-head workflow state must be checked before describing the code as executed. This runtime does not have `cargo/rustc`, so the Python arithmetic is the only locally executed numerical evidence.

## Next falsifiable experiments

**I1 (do next):** On identical depth-6 and depth-8 prime31 planted, known-negative and *fresh natural masked public targets*, compare this exact m2 GCD oracle with the existing `decompose` (tower F4) and `algebraic_decompose_dense` (#4275) using **actual Rust counters and end-to-end time including setup**, distinguishing all completed misses, ambiguous/exceptional outcomes, nonunit failures and extraction, no group join. Stop if GCD loses to existing dense for both positive and negative cells. A microbenchmark does not establish cold ECDLP performance.

**I2:** If I1 improves genuinely target-dependent oracle cost, combine this elimination as a *building block* for the balanced m8 central S4 quotient, against #4228's certified subresultant route. Preregister unit/zero-divisor and repeated-root test matrices, degree, memory, natural witnesses and a comparator outside the repo. Do not form or scan the 17,980²×930 generic sumset; the last S3-discriminant audit showed how easily that happens.

**I3:** Only after real natural m8 positive extraction, collect projectively unique useful independent rows, solve 12 unseen public prime31 logs (four holdouts) and compare complete cold cost `S(w=0,0.1,1)` with optimized same-host Rust rho/BSGS plus fastest external method. Include *every* failed/censored query, base production, amortization policy, matrix, root extraction, final scalar verification. A full end-to-end advantage requires independently replayed consistent numbers.

The result implemented here is a **measurable algebraic subproblem** on a non-weak prime31 curve, not yet a complete or cheaper ECDLP solver.

## Source-bound Rust execution and stage measurements (2026-11-15)

The full narrow development suite passed at source/workflow commit `afdc9e50699f2dad882dbe354238741528103910`: [Actions run 43741710811](https://github.com/isogeny-labs/crypto/actions/runs/43741710811), job `132896091678`. The new exact algebraic-versus-generic Rust parity test passed `1/1`, the executable completed, and the development job's narrow Clippy and existing modules passed. The full repository root release-lib suite was not run for this commit. The exact output and scope are frozen in [`S3_QUOTIENT_GCD_CI_EVIDENCE_20261115.json`](S3_QUOTIENT_GCD_CI_EVIDENCE_20261115.json). **All measurements are producer-only and [pcc:unverified].**

| Fixed factor base on same prime31 curve | Public target type | Exact GCD stage | Algebraically extracted signed relations | Old packed Rust F4 control |
|---|---|---:|---:|---|
| B=31, Dickson depth 6 | planted positive `P3+P9` | 0.485900 ms | 1 | 3.079562 ms, **undetermined**, degree 4, 252732 partial muladds |
| B=31 | public `[17]G`, no decomposition | 0.446576 ms | 0 | 2.425976 ms, **undetermined**, 252732 partial muladds |
| B=135, Dickson depth 8 | planted positive `P0+P14` | 29.476025 ms | 1 | **not run** |
| B=135 | public `[17]G`, no decomposition | 28.549258 ms | 0 | **not run** |

These are **one sample per cell**, not statistically calibrated benchmarks. B=31 fixed-base construction was excluded from both stage intervals, and B=135 base construction was also excluded; the GCD interval includes subsequent signed EC candidate validation. The old F4 has a lower stopping resource cap and did **not** finish, so dividing times or calling it an end-to-end speedup would be invalid. B135 scaling from roughly 0.5 to 29 ms also warns against treating this exact two-summand algebraic method as the final cheap solver. None of these rows is a complete scalar recovery, cold cost, useful-rank rate or rho comparison.

The newly checked [Amadori–Pintore–Sala, *On the discrete logarithm problem for prime-field elliptic curves* (2018)](https://doi.org/10.1016/j.ffa.2018.01.009), alongside Petit–Kosters–Messeng (2016) and the repository's #4275/#4228 results, further limits the novelty of applying small-degree Semaev elimination. Its published approach reduces the number of factor-base relations required in a prime-field IC algorithm. We have **not** shown that our special resultant/GCD implementation improves or subsumes their method. A direct reproduction of their complete-cost model and a paired end-to-end comparison is added as a literature-control requirement in I3.

**Immediate execution gate:** use this working S3 quotient as an exact reusable building block in a target-dependent, genuinely algebraic **m≥4/m8 S4 system**, obtain fresh natural-positive relations not constructed from the base points, deduplicate actual signed linear rows, finish held-out prime31 scalar recovery, and only then charge cold setup plus every miss and compare with optimized rho and strongest external F4. Do not substitute the old known-point triple-root tables or regard the capped F4 loss as an overall performance ranking.
