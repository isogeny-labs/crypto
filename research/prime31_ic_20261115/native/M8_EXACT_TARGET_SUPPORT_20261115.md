# Prime31 signed m8 exact reachability census (2026-11-15)

**Standing:** producer-local exact arithmetic, [pcc:unverified]. **No algebraic F4 witness extraction, complete-cost improvement, cryptographic exponent or independently signed result is claimed.**

## Exact curve and previous work

Use the existing Rust-native [prime31 adapter](README.md), not the deliberately smooth-order curve:

- Field p = 2,147,483,647 = 2^31-1; curve y²=x³+847514846x+2087444724.
- Certified full group order n = 2,147,446,831 (prime), cofactor 1; generator G=(625037715,2100627251).
- The deterministic depth-6 p+1 Dickson fibre has terminal 933677148 and **31 distinct liftable abscissae**, one canonical signed point per abscissa.
- Historical [M8_SPLIT_CONTROL](M8_SPLIT_CONTROL.md) found 32 raw independent rows in 124 public masked queries with a **generic EC group hash join**, not polynomial-system extraction; the two prior positive masks and witnesses reused below are *verification controls*.

The 4+4 split fixes disjoint index halves, 15 and 16. Each half picks 4 distinct abscissae and all 2^4 y signs. The exact template counts are

    A = C(15,4)*16 = 21,840
    B = C(16,4)*16 = 29,120
    T = A*B = 635,980,800.

Let Aset and Bset be their respective EC point-sum multisets. We independently enumerated every one of the T cross-sums in an optimized **non-algebraic producer-local C++ diagnostic**. An exact 2^32-bit set encodes an affine point as (2*x + parity(y)), with key 2*p reserved for infinity. This encoding is injective on E(Fp), so distinct occupied bits count distinct *complete target points*, not just abscissae.

The diagnostic batched all 29,120 inverses of each left point via Montgomery's trick (one inversion per nonidentity left), evaluated the signed group sums, and OR-ed a 512-MiB bitmap. It was performed in sequential intervals of 10,000 and 11,840 left templates, preserving the previous bitmap. The enumerator recorded zero special identity/doubling cases. Its output is not a benchmark of the PKM algebraic oracle. The deterministic local Rust-source reference checks the same point encoding, same base and every prefix/full count.

## New measurement

| Quantity | Exact value |
|---|---:|
| Raw signed split templates, with repetitions | 635,980,800 |
| **Distinct reachable full public targets** | **552,008,270** |
| Unreachable public group points | 1,595,438,561 |
| Exact uniformly random target hit probability | **552008270 / 2147446831 = 25.70532886%** |
| Mean independent uniform masks to first hit | **2147446831 / 552008270 = 3.89024395** |
| Expected raw witness multiplicity per uniform query | **635980800 / 2147446831 = 0.29615672** |
| Mean raw templates per *positive* query | **635980800 / 552008270 = 1.15212187** |
| Raw template multiplicity beyond unique support | **83,972,530** |

The last line is *not* the count of dependent index-calculus rows: several signed templates for the same point can give distinct linear equations. A query outside the support, however, **provably cannot** produce a relation under this *particular restricted* 4+4 factor-base and template regime, regardless of F4 engineering. This is not a universal impossibility result for Semaev m8, other partitions, other factor bases, or large-prime variants.

The uniform hit probability is **exact** under sampling R uniformly in the prime-order group, e.g. R=[a]G+[b]Q when a is uniform modulo n, independently of b. It is not a guarantee for any fixed deterministic target. If an oracle returns at most one useful row per hit, its expected number of independent masks for 32 rows is bounded below by 32*n/D ≈ **124.49**. The historical generic oracle returned 32 rows from 26 positive masks in 124 queries because a positive mask can return *multiple* distinct equations; that observation does not contradict the bound's one-row-per-hit premise.

For scale: independent random occupancy with T draws predicts a support fraction approximately 1-exp(-T/n) = **25.6329%**, only **0.0724 percentage points** below the measured **25.7053%**. This is a descriptive random-occupancy heuristic, not a statistical model or evidence that relation rows are independent.

## Cross-checks and Rust reuse

- The first 100 left templates, 2,912,000 cross-products, occupy **2,911,034** distinct group points (fast development fixture).
- On the final 512-MiB bitset, a separately authored Python EC implementation recomputed both previously recorded natural-positive 8-summand relations for the same public mask and verified that their full target points are marked. Both checked.
- In a seeded development-only sample of **500** independently selected public points R=[a]G+[b]Q, **132** were marked (26.4%); this is a sanity check, not an estimate replacing the exact count.
- New [Rust example](examples/prime31_m8_exact_support.rs) reuses **the existing Arithmetic, Base and PRIME31 fixture** for curve certification and four-point-sum construction; it locally uses batched Mersenne-prime affine cross-sums for speed, with deterministic sparse replay through inherited Arithmetic::add.
- Default invocation checks the first 100 left templates and asserts 2,911,034; --full checks all 21,840 against 552,008,270. The [focused Rust development workflow](../../../.github/workflows/prime31-algebraic-ic.yml) runs the prefix test. **The Rust example has been checked in, but was not compiled or run locally** because cargo/rustc and authenticated EKS/checkpoint admission were unavailable. Do not upgrade these producer-local counts to Rust/PCC verification until CI and independent replay report success.

Run from repository root:

    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_exact_support
    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_exact_support -- --full

The 512-MiB bitmap is an audit-only cost: it would be inappropriate to insert this enumerator or bitset into the claimed *non-generic* relation oracle. All diagnostic memory, elapsed time and field operations are therefore separate from a complete PKM cold cost.

## Lead decision and next discriminating experiments

1. **Use this as an oracle ceiling, not a solver.** On the identical public masks, run the *actual* existing Rust S4/S3 F4 algebraic oracle. Classify every sampled mask as algebraically extracted, proven unsatisfiable, stopped/censored, or unsupported due to singularity. Match against the exact group-theoretic membership for independently frozen masks. Never substitute the generic 4+4 table for failed F4 extraction.
2. **Quotient before expanding to larger B.** The [ordered-root symmetry audit](M8_ORDERED_ROOT_SYMMETRY_GATE.md) proves at least 80,640 labeled roots and 560 manifest block orbits on a single positive target. Implement a correct block-symmetric/saturated polynomial description, including actual y-lifts, repeated-x exclusions, and signs. Compare full F4 costs with the ordered system and demand *incremental independent rank*, not raw root counts.
3. **Revisit factor-base selection by exact useful support per algebraic cost.** Compare Dickson depths 5/6/7, alternate fixed 4+4 partitions, and *pre-registered* odd-degree/other fibre presentations on the same ordinary prime31 curve. Account for base preparation, every polynomial failure, extraction, sparse elimination and final scalar checking. Do not mistake increased generic combinatorial support for higher algebraic extraction yield.
4. **Whole-call cold cost only.** Complete public Q recovery, holdout targets, matched optimized rho/BSGS, both factor-base and linear-algebra cost, memory caps, PCC independent replay and scaling beyond a single 31-bit instance. The previous PKM gamma/delta cost gate in #4077 remains the advancement criterion; a stage speedup is not an ECDLP win.

### Prior searched / novelty scope

- [Petit–Kosters–Messeng, PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1) already established composed rational-map prime-field algebraic index calculus, Semaev relation systems, and exploratory complexities; there is no novelty claim for the basic method.
- [Diem, Compositio Math. 2011](https://doi.org/10.1112/S0010437X10005075) supplies the wider extension-field and selected-prime-field sequence context; it does not assert a practical generic-prime-field ECDLP break.
- Existing merged [#4077](https://github.com/isogeny-labs/crypto/pull/4077) PKM whole-call gate, [#4275](https://github.com/isogeny-labs/crypto/pull/4275) dense membership, [#4282](https://github.com/isogeny-labs/crypto/pull/4282) multi-target IC; open [#4228](https://github.com/isogeny-labs/crypto/pull/4228) S3 quotient, [#3877](https://github.com/isogeny-labs/crypto/pull/3877) transfers, [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536) auxiliary IC, [autoresearcher #3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305) lead atlas.
- Research-KB MCP and the repository's research-KB CLI were unavailable in this runtime, so this is a bounded primary-source review, **not an exhaustive novelty search**.

**Decision:** prioritize natural algebraic extraction and symmetry quotient. The exact group-theoretic hit ceiling, about 25.7% for this registered split, bounds the next experiment but does not establish a cheaper complete prime-field index-calculus solver.
