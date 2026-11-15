# PRIME31 m=8: exact useful-rank budget and capped existing-Rust F4 pilot

**2026-11-15. [pcc:unverified].** This is an exact finite-group counting derivation plus checked-in Rust *source* integration, not an executed F4 benchmark, algebraic witness discovery, asymptotic complexity result, or cheaper full ECDLP solver. All existing primary and completed historical runs remain unmodified.

## Registered ordinary curve and baseline

- Field p = 2,147,483,647 = 2^31 - 1; E: y² = x³ + 847514846x + 2087444724.
- Full prime group n = 2,147,446,831, cofactor one; G=(625037715,2100627251), public Q=(1547953785,1667575595). This is *not* the smooth-order y²=x³+x positive-control curve.
- The native Rust `Base::build` Dickson depth-six fibre has 64 raw x candidates but **31** admissible nonzero-y curve points. The previous signed 4+4 restricted-split control used 15 left and 16 right base points, and producer-reported 32 independent rows from 124 natural public masks (26 positive masks), then point-verified the scalar. It incurred 3,827,503 counted EC additions including final checks, but was a generic hash-join, not a polynomial solver [pcc:unverified].
- Previous same-curve BSGS historical comparison was 60,524 mean EC additions; these are not matched new-run timings, and optimized rho and signed PCC comparison remain absent.

## Exact fresh mathematical result: uniform-mask raw relation yield

Let a factor base contain B distinct curve points with nonzero ordinates, and consider *unordered* choices of eight distinct indices, each with independent signs. Fix a prime-order group of order n and draw the public relation target R independently and uniformly from the group. Let X(R) count **signed eight-point templates mapping to R, with multiplicity**. Each template has exactly one target, so by linearity

    E_R[X(R)] = N_templates / n

**exactly**, with no random-sum or independence-of-signed-sums heuristic. The probability of at least one valid relation is at most min(1, E[X]), and collisions can only reduce hit probability, not E[X]. The count concerns valid group-point sums; x-only Semaev roots that do not lift are not admitted.

At the registered p31 / n31 / B=31 cell:

| Family | Signed templates | E[raw hits] per uniform mask | E[calls] lower bound for 32 independent rows |
|---|---:|---:|---:|
| Existing fixed disjoint 4+4 split (15 and 16 points) | C(15,4)·2^4·C(16,4)·2^4 = **635,980,800** | **0.2961567154** | **108.0509012** |
| All eight distinct canonical base indices | C(31,8)·2^8 = **2,019,513,600** | **0.9404254256** | **34.0271532** |

These are **not** estimated F4 throughput, measurements of uniqueness or model success fractions. The unrestricted count offers **3.175431711×** more raw signed templates than the fixed split, but does not imply the same improvement in independent rank or runtime. The fixed-split observed 124 calls to 32 rank rows is close to its 108.05 **expectation lower bound** in the ideal uniform-mask model. No count implies a cryptanalytic cost advantage.

To prove the call lower bound, let X_i be raw template matches on the i-th freshly independent uniform target, and let τ be an integrable stopping time when the relation rank first reaches 32. Each rank increment is at most X_i, hence 32≤∑_{i≤τ}X_i. Conditional on the previous history, E[X_i]=N_templates/n, so Tonelli/optional sampling gives

    E[τ] >= 32 n / N_templates.

Do not round the rational real-valued lower bound *up* to a purported integer bound on E[τ]. The formula is prospective and applies to independent **uniform** query masks, not a deterministic lower bound for every fixed pseudorandom transcript. It also gives no guarantee that an algebraic oracle can produce even one witnessed relation.

An independently authored Python `math.comb`/`fractions.Fraction` calculation on this runner recomputed the precise integer/fraction values above. The existing Rust example now checks all four cardinalities and emits the exact rational numerators and denominators; its exact-head Rust execution is not established.

## New actual-Rust-engine integration: tightly capped F4 calls

The existing [balanced S4/S3 bridge](examples/prime31_m8_balanced_f4_input.rs) now accepts an **optional** `--probe-f4` argument. With that argument only, it feeds the four real quotient-normal-form input polynomials into the repo's existing `src/cryptanalysis/f4_fp_tower.rs::f4_tower` engine, without copying the engine or reducing the problem to generic group enumeration.

- Arm A: the first *previously recorded* natural positive m8 tuple, with a known group witness for an algebraic consistency regression. It must not be mistaken for a newly discovered witness.
- Arm B: direct public Q as an *unclassified* natural target. It must not be labeled UNSAT without a completed refutation and independently checked proof.
- Both arms use maximum degree 12, cooperative wall-time budget 5 s per call, and 15,000-entry nonzero and reduced-dense-matrix ceilings. Save status, degree reached, pair censoring, matrix widths, Fp elimination multiply-adds, elapsed milliseconds and basis size.
- Every produced intermediate F4 basis polynomial is checked against the previously certified positive assignment in arm A. A refutation or nonzero residual there is a hard error.
- **Important limitation:** a positive non-refuted F4 basis is *not* an extracted relation. This example does not enumerate new finite-field roots, filter all 64 Dickson abscissae down to the 31 liftable curve points, orient signed y-values on fresh roots, build new rank rows, or solve the discrete logarithm using F4.

Reproduction (requires the repository Rust toolchain):

    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_balanced_f4_input -- --probe-f4

The focused `.github/workflows/prime31-algebraic-ic.yml` now runs the bounded probe after its existing bridge checks, with a separate 2-minute job-step timeout. This session's selected container has no cargo/rustc or authenticated EKS command backend, so **no actual F4 execution, Rust compilation, new CPU-time or GAE numbers were obtained**. Push-triggered workflow checks and independent PCC must be inspected separately; commits are not proof of CI success.

## Literature and prior-lead survey (do not duplicate)

| Source / prior area | Decision at this gate |
|---|---|
| Petit–Kosters–Messeng, PKC 2016, [full paper](https://www.iacr.org/archive/pkc2016/96140156/96140156.pdf), esp. §2.1 and §3 | Composite small-degree rational-map factor bases and summation polynomial solving are existing index calculus, not new algorithms; avoid novelty claims and charge Gröbner work |
| crypto [#4091](https://github.com/isogeny-labs/crypto/pull/4091) / [#4275](https://github.com/isogeny-labs/crypto/pull/4275) | Existing Rust PKM tower/dense F4 and algebraic decomposition, with dense winning against tower on m2/m3 planted cells but enumeration winning overall; no extrapolation to m8 prime31 |
| crypto [#4077](https://github.com/isogeny-labs/crypto/pull/4077) / [PKM rank-aware gate](../../norm_branch_rank_pilot_20261115/PKM_PARTIAL_OUTPUT_GATE.md) | Keep the full-call gamma<3+delta/2 necessary gate and natural positive/negative held-out panel; no fit from known-witness controls |
| crypto [#4259](https://github.com/isogeny-labs/crypto/pull/4259) / [#4228](https://github.com/isogeny-labs/crypto/pull/4228) | PKM subgroup factor base is prior art; S3/quotient reduction needs its own total-cost validation; no double count of certificate preprocessing |
| crypto [#4282](https://github.com/isogeny-labs/crypto/pull/4282) | Multi-target Rust IC retune at 2^32 is a separate *amortized* lane, not a cold prime31 win |
| crypto [#3877](https://github.com/isogeny-labs/crypto/pull/3877), cryptanalysis [#1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536) | Chained transfer and auxiliary-map approaches have distinct public scalar-blind completeness/cost gaps; not copied into the present direct m8 test |
| crypto-autoresearcher [#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305) | Lead atlas explicitly tracks unevidenced or scale-exhausted claims; absence of evidence isn't evidence of nonexistence |

The shared research-KB MCP, Conductor, EKS submission and independent PCC signer are unavailable in this execution environment; repository PRs, source and primary literature were the bounded fallback. This does *not* establish a comprehensive novelty search.

## Next falsifiable run plan

1. **Run/check** exact-head Rust compile/CI and the two capped native F4 arms. Treat timeouts, cap hits and unclassified outputs as censored, not negatives. Record compile errors and repair them before allocating cloud CPU.
2. **Extract new natural witnesses** only if the F4 basis is complete. Implement finite-field root extraction for all 48 tower and 3 free variables or choose a lower-dimensional quotient, checking all 8 points lie on E(Fp), exact sign equation, projective relation dedup, rank increment and no unknown-scalar oracle.
3. **Use the same 124 public masks** from the previous generic 4+4 control *plus fresh preregistered held-out masks*, both positive/negative, and run dense/tower/split F4 head-to-head with their strongest external solver counterpart on identical polynomial inputs.
4. **Complete the pipeline:** each scalar must satisfy Q=[d]G; account for full cold factor-base construction, all failed calls, symbolic preprocessing, F4, root extraction, sparse LA, memory and verification on the ordinary p31 prime-order curve. Compare with matched optimized negation-map rho and the existing Rust cold IC baseline.
5. **Growth law:** repeat at two larger held-out factor-base sizes and at least one other ordinary prime-order curve; estimate cost gamma, independent useful-rank delta, and total end-to-end S with independent PCC/replay before any promotion or theorem-level claim.

**Decision:** the exact uniform-mask occupancy accounting shows which yield gains are mathematically available and removes raw-collision optimism, while the new CLI makes an actual bounded Rust-F4 experiment executable by CI. A cheaper complete non-generic prime-field ECDLP solver is **not** demonstrated by this step. [pcc:unverified]
