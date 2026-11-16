# Prime31 APS Mersenne field gate — completed Rust development A/B (2026-11-16)

**Standing: [pcc:unverified] development evidence.** Exact checked source `998c4b7a6e5787da993d55626e088563586f7be2` (Rust source changed at `c815d27a17616bbd2026ce5f1f2e27e371d603d2`). [GitHub Actions run 43804327344](https://github.com/isogeny-labs/crypto/actions/runs/43804327344), job `133091508151`, succeeded; original source-bound output, all 24 Q, 72 cold-arm transcripts and timing fields retained in [APS_MERSENNE_FP_MATCHED_RUST_CI_20261116.json](APS_MERSENNE_FP_MATCHED_RUST_CI_20261116.json). Preregistered [APS_MERSENNE_FP_PROTOCOL_20261116.md](APS_MERSENNE_FP_PROTOCOL_20261116.md) before source changes.

## Actual completed results

Same ordinary full-prime-order curve `p=2147483647`, `E:y²=x³+847514846x+2087444724`, `n=2147446831` prime, cofactor 1. All arms use existing **Rust** `prime31_algebraic_ic::Arithmetic`, identical public Q and three T1536 walk-labelled lists per target, identical signed S3 one-relation algebraic oracle and complete `[k]G=Q` verification. Every fresh target's caller-known k is hidden from the solver.

| Fully paid cold arm, 24 new public Q | Verified logs | Total wall | Mean per Q | S3 pairs |
| --- | ---: | ---: | ---: | ---: |
| Existing `u128` modular Fp multiplication | 24/24 | **2036.887 ms** | 84.870 ms | 6,744,576 |
| Mersenne fold Fp multiplication | **24/24** | **1156.075 ms** | **48.170 ms** | 6,744,576 |
| Mersenne repeat (A/A) | 24/24 | 1158.010 ms | 48.250 ms | 6,744,576 |

**Measured same-host fully cold original / Mersenne ratio = 1.761898×.** Fast path won **24/24 paired public Q**. Median per-Q original/fast = **1.727716×**. Fast repeat/fast aggregate = **1.001674** (about 0.167% higher). The arm order alternated old/fast/fast and fast/fast/old. All 72 runs returned full correct scalars in their first bounded attempt, and the Rust experiment checked identical scalar results, attempts and visited S3 pair counts on every Q. No known logarithm enters `execute_sized_mode_mul`.

**Where it came from:** Algebraic S3 pair root work (including candidate verification) dropped **1950.977 → 1070.372 ms**, a **1.82275×** stage improvement. Public source setup was effectively unchanged: original **77.134 ms**, fast **77.008 ms**, repeat **76.901 ms**. The underlying S3 query count and signed-search semantics did **not** change. The only experimental change is exact prime-field multiplication/square-root arithmetic. The Rust tests compare 100,000 deterministic modular products plus boundary products and S3 coefficients/evaluation, and compare old/new complete search status, scalar and query count. The focused release CI and all retained steps passed. No optimized-rho or mature external IC solver timing was substituted.

## Why this is valid—and why it is not a cryptanalytic breakthrough

For canonical field elements, `x*y < 2^62`; since `2^31≡1 mod (2^31-1)`, two folds
`z=(x*y & p)+(x*y >> 31); z=(z&p)+(z>>31)` with one conditional subtraction produce the same canonical residue as `(x as u128*y as u128)%p`. This replaces a general u128 modulus at millions of algebraic S3 operations, with exact Rust old-vs-new equivalence checks. No different factor base, weak group, precomputed target scalar or generic join fallback was introduced.

The implemented algorithm is an **engineering improvement to the completed published APS-style one-relation prime-field index-calculus program**. Its explicit S3 pair-root search is mathematically **generic meet-in-the-middle equivalent**; it is **not** novel non-generic complexity, proof of an asymptotic improvement, or an external frontier win. A same-run faster internal APS implementation is useful but insufficient to claim ECDLP security impact. Publication prior art: [Amadori–Pintore–Sala 2018](https://doi.org/10.1016/j.ffa.2018.01.009); [Petit–Kosters–Messeng 2016](https://doi.org/10.1007/978-3-662-49387-8_1).

## Previously tested and current leads

- **#4374 APS original (merged):** 12/12 complete original Rust one-relation targets; math generic MITM equivalent. Its original source setup required two independent scalar multiplies per labelled point.
- **#4383 APS size experiment (merged):** exploratory T1024 appeared favorable; 24 new Q reversed this result, making T1536 the correct frozen control.
- **#4391 additive walk:** separate same-run 24-Q source-setup improvement 2.15564× full cold over independent-source APS. This new Mersenne A/B is **on the walk implementation**, on a different fresh Q panel; do not multiply ratios as a measured cross-panel claim.
- **#4373 m8 chained S3 (merged) / #4188 m4:** F4 matrix cap censored all m8 natural tests, m4 B31 natural-target coverage insufficient. Rearranging S3/S4 alone did not produce new natural m8 relations.
- **#4228 certified quotient (merged) / #4349 actual-root selected certificates (open):** no 31-bit target-conditioned K/S5 completed root extractor yet; earlier compact certificate speed results concern smaller graph systems and were not external-frontier wins.
- **#4077 whole-call gate, #4311 labelled selectors, cryptanalysis #1536 auxiliary field IC and crypto-autoresearcher #3305 lead atlas:** searched leads and prior negative/incomplete evidence retained; do not repurpose generic root catalogues as algebraic discovery. Current research-KB unavailable in tool-only runtime; GitHub and original publications used as bounded fallback.

## Next decisively non-generic experiment

Build an **original-rational-root-equivalent target-conditioned K/S5 selected-root solver** on small B4/B8 controls before full B31. The degree-961 squarefree completely split K pair-coordinate quotient has 961 rational roots, but the **full** K(U1)···K(U4) quotient would have `961^4=852,891,037,441` basis positions. No Cartesian expansion or EC group-pair joins may propose algebraic roots. The candidate must use certified target-conditioned resultant/subresultant constraints, exhibit rational roots of the **same** m8 ideal on frozen natural Q, reconstruct signed point identities (including infinity/repeated-x exceptional cases), and track independent useful relation rank. Then compare **completed** algebraic oracle costs against PKM/APS, dense/tower F4, certified quotient and mature external CAS before claiming frontier progress; optimized rho remains a separate end-to-end security comparator.

## Limitations and reproducibility

The measured result is one unisolated same-host GitHub Actions release Rust run with 24 fresh distinct public Q and alternating-order controls, not independent signed PCC, isolated L2/EKS, multi-prime curve validation, confidence-confirmed scaling or mature external solver ranking. It does establish fully charged complete **internal** Rust APS-path timing and correctness for these fixed prime31 cases. No paid compute, infrastructure change, history rewrite or leaderboard promotion.

Source command:
```sh
cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation
cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation -- --mersenne-confirm
```


## Follow-on optimized one-target Rust CLI exact-head evidence

A later source commit exposed `--walk-mersenne-one` while keeping the original modes unchanged. The **full new CLI** passed exact-source [GitHub Actions run 43804915562](https://github.com/isogeny-labs/crypto/actions/runs/43804915562), job 133093351405, tested head `710aeb31fda4933a43f95a1500df2f098f043910`, after the paired experiment above. The existing Rust example release unit tests and all frozen prior methods also passed, plus the new single-call execution. On the published public Q=(1477799919,824532935), the optimized walk/Mersenne CLI correctly recovered the unknown log **1038579171**, replayed the complete group identity, and reported **23.058090 ms** in-process fully cold wall, **199,680** S3 quadratic trials, `field_multiplier=mersenne_fold`. The known secret is not passed to the solver. [Exact-source one-target JSON](APS_MERSENNE_ONE_TARGET_CI_20261116.json).

This 23.06ms sample and the earlier 24-Q matched experiment are separate measurement populations. Neither is a fair *new* same-run comparison to optimized negation rho, BSGS or mature external PKM/APS. The source is Rust-only, and the original 24-Q matching report remains immutable at its earlier SHA.
