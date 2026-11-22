# Prime31 any-witness M6: exact 1,024-log Rust control and matched A/B/B/A

2026-11-18 UTC · [PR #4714](https://github.com/isogeny-labs/crypto/pull/4714) · [source-specific passing GitHub CI](https://github.com/isogeny-labs/crypto/actions/runs/44316824448) · [full raw six-file evidence](https://github.com/isogeny-labs/crypto/actions/runs/44316824448/artifacts/14432881900) · [prospective protocol](PRIME31_M6_ANY_WITNESS_PROTOCOL_20261118.md)

**Result [pcc:unverified]:** Accepting any exact ordered six-slot witness, rather than requiring odd multiplicity, and stopping chunked Montgomery complements at the first match, made the existing **generic-equivalent** six-slot index-calculus-style complete Rust solver **1.440–1.441× faster** on the same original Prime31 1,024 public targets. Both arms recovered **1,024/1,024 logarithms** with **31/31 rank**. This does **not** establish a cheaper non-generic prime-field ECDLP solver or a win against optimized shared BSGS / the best existing mixed-4+4 IC.

## Original curve and source provenance

`p=2147483647=2^31-1, a=847514846, b=2087444724, n=2147446831, h=1, G=(625037715,2100627251)`.

- Retains pinned original Rust `Arithmetic`, `Point`, `Counts`, curve certificate and modular rank recovery from commit `1df714645aa683cf975bc76fcacb9842cb4780fd`. The two arms differ only in table-membership semantics and query inversion schedule.
- Identical B31 first-rational-x factor base, the same six public weights `[1,3,5,7,11,13]`, same `TRAIN_SEED` and `TARGET_SEED`, 512 training cap, 64 shifts and exactly 1,024 public targets. Caller-only scalars are used to check answers, never to decide relation existence.
- Both methods still generate the same ~29,791 ordered three-sum candidate words per half using generic EC arithmetic. There is no Semaev/RR9/F4 extractor in this new arm.

## Exact mathematical improvement in the oracle predicate

Let `w_R` count the ordered six-slot decompositions of a public residual `R`. The old parity predicate only returned a relation if `w_R` was odd. The new predicate returns the **first exact ordered witness** if `w_R >= 1`, retaining one word for each distinct three-sum half-point. For every group and every factor base, an odd count implies positive existence, but not conversely. Therefore every odd-positive target is an existence-positive target, with possibly more positives. This is a **rigorous inclusion of supports**, not a theorem of improved rank or complexity.

The new Rust query uses chunks of 256 left complements, one Montgomery batch inversion per nonexceptional chunk, and immediately returns the first exact match. Missing results require exhausting every chunk. An exhaustive B2 control checks all 64 ordered words, every distinct image and at least 64 outside-support negatives. The unchanged parity B2 differential control remains intact.

As a **nonrigorous occupancy heuristic only**, `lambda=31^6/n=0.413283` suggests `Pr(exists)≈1-e^-lambda=0.3385` and `Pr(odd)≈(1-e^(-2lambda))/2=0.2812` if ordered word images were independent uniform placements. Shared points and algebraic dependencies prevent invoking this as an exact predicted distribution.

## Actual same-runner fully charged results

The preregistered A/B/B/A executions were performed on one GitHub-hosted Ubuntu 24.04 runner after both examples were built. These are full program diagnostic times, including source checks, basis setup, natural training, modular solving, target queries, failed shifts and verification, excluding compilation. No reference solver was run in the new A/B/B/A pair.

| Measured arm | Execution 1 | Execution 2 |
|---|---:|---:|
| Parity M6 (A) | **6,155.578 ms** | **6,176.025 ms** |
| Any witness / 256 chunk (B) | **4,274.759 ms** | **4,286.142 ms** |
| Paired parity/any speedup | **1.43998×** | **1.44093×** |

| Counts from same run | Parity M6 | Any witness |
|---|---:|---:|
| Independent relation rank | **31/31** | **31/31** |
| Public training calls to full rank | 106 | **87** |
| Complete correct public logs | 1,024/1,024 | **1,024/1,024** |
| Total target attempts incl. failed shifts | 3,717 | **3,143** |
| Online EC additions | 110,834,780 | **77,239,419** |
| Online Fp multiplications | 670,994,789 | **487,624,549** |
| Online Fp inversions | **102,278** | 400,642 |
| Training EC additions | 3,162,785 | **2,040,000** |
| Training Fp multiplications | 19,264,019 | **12,959,397** |

The smaller 256-point batches increase Fp inversions approximately 3.92× in the target phase; the lower overall arithmetic is due to early matched-witness termination and better coverage. Changes in the found witness or training rank path are permitted, but every identity was separately verified.

## Verification, evidence and limitations

The exact-source workflow passed **40 original native Rust library tests**, **3 parity-example tests**, **4 new existence-example tests**, and both paired whole-program executions. A separate Python BigInt elliptic-curve implementation independently replayed the four whole transcripts, checking group-sum witness identities, modular relations, rank, basis logs and all 1,024 final recovered scalars in each arm. All completed.

- [Passing exact-source run #44316824448](https://github.com/isogeny-labs/crypto/actions/runs/44316824448), commit `c05ee1fb1289d92e9437aa5ca491ee90cba16b50`.
- [Raw A/B/B/A complete evidence ZIP](https://github.com/isogeny-labs/crypto/actions/runs/44316824448/artifacts/14432881900), artifact ID `14432881900`, SHA-256 `75e8a678bd90905fe80e4049084fcba2314466ae9381ba5d0a6e9b54b15492f9`, expiration 2026-12-18.
- First run [#44316529618](https://github.com/isogeny-labs/crypto/actions/runs/44316529618) failed Rust E0425 because the differential B2 parity test still called the renamed `query_batched`; restored the unchanged helper and reran successfully. The earlier failed run contains **no** performance data.
- The workflow's independent Python replay is producer-run arithmetic validation, **not** a separately signed PCC validation.
- Main/PR-wide artifact-store hydration and legacy CI issues are not part of this source-specific successful experiment.

## Lead audit / prioritized follow-up

1. **RR9/F4 nonenumerative extraction is the high-value work:** continue [#4692](https://github.com/isogeny-labs/crypto/pull/4692), including exact complete-fibre B4/B8 positives and negatives, bounded B31 natural relation yield and full-rank recovery on the *same* original Prime31 curve. The independent low-power strata [#4717](https://github.com/isogeny-labs/crypto/pull/4717) are a separate lead. A timeout is *undetermined*, never a negative membership proof.
2. **Full performance frontier:** redo this B arm on the original existing 1,024-Q shared corpus alongside optimized shared BSGS and native mixed 4+4 IC, all on one runner, and add optimized negation-map rho for cold single-target cost. This A/B only shows a relative M6 speedup and no BSGS comparison. Previous exact matched M6-vs-BSGS evidence from [#4714](https://github.com/isogeny-labs/crypto/pull/4714) found parity M6 **20.85–23.32× slower**; new wall times on a different runner must not be divided by the old BSGS times.
3. **Engineering ceiling only:** chunk-size sweep over 64/256/1024/4096 and early-exit positive-vs-negative costs; freeze new protocol and collect exact counts. No new attack claim if only a constant-factor change.
4. **Extraordinary theorem audit:** [#4698](https://github.com/isogeny-labs/crypto/pull/4698) claims prime-field `r^{o(1)}` from graded parity-network contraction, without end-to-end kernel verification or an executed ECDLP benchmark. It warrants independent proof/complexity checking, not promotion. Existing algebraic S4/S3 failures, coordinate re-ranking null [#4652](https://github.com/isogeny-labs/crypto/pull/4652), isogeny relation-rank preservation [#4671](https://github.com/isogeny-labs/crypto/pull/4671), and mask support obstruction [#4716](https://github.com/isogeny-labs/crypto/pull/4716) remain the relevant exclusions.

**Research classification:** a measured, reproducible improvement to an existing generic-equivalent index-calculus-style complete Rust solver. A genuinely nonenumerative prime-field ECDLP algorithm with a better total cost than frontier baselines has not been established.

Relevant published context: Semaev summation polynomials (2004), Gaudry (2009), Diem (2011), as catalogued in [Isogeny Labs summation polynomial survey](https://isogenylabs.io/cryptanalysis/summation-polynomials.html). The knowledge-base MCP server was not available in this runtime; the review used GitHub repositories, specific PRs and the linked survey, not an exhaustive literature corpus.
