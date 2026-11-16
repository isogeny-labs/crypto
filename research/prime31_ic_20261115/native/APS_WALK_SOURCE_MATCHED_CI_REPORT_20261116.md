# EXP-R31-APS-WALK — completed Rust 24-target same-run development result (2026-11-16)

**Standing: [pcc:unverified].** This is one source-specific *GitHub Actions development runner* result, not signed/isolated PCC, mature external index-calculus frontier superiority, a new non-generic asymptotic algorithm or a prime-curve cryptanalytic security break.

## Frozen protocol, source and independent scope

- Frozen before implementation/timing: [APS_WALK_SOURCE_PROTOCOL_20261116.md](APS_WALK_SOURCE_PROTOCOL_20261116.md).
- Exact source head exercised: `331a55123fcb06a19655d24c7c0d2327afafd7fa`, [successful workflow #43798366532](https://github.com/isogeny-labs/crypto/actions/runs/43798366532), [Rust job 133072340611](https://github.com/isogeny-labs/crypto/actions/runs/43798366532/job/133072340611).
- [Full machine-readable case-by-case Rust stdout, source and job identifiers](APS_WALK_SOURCE_MATCHED_RUST_CI_20261116.json). That file was committed after the original execution; the code/results are tied to the above pre-report head. Rust compiler is 1.100.0, release mode, reused `prime31_algebraic_ic` and original `prime31_aps_one_relation.rs` algebraic S3 oracle.
- Fixed ordinary prime-order **p=2^31−1** curve: `E:y²=x³+847514846x+2087444724`, `#E=n=2147446831` prime, cofactor 1, `G=(625037715,2100627251)`.
- Twenty-four *fresh* public `Q=[k]G` with k known only by the caller after `execute_sized_mode` returns; all arms use same Q, three disjoint T=1536 lists, up to three rounds, independent complete source setup, zero hidden precomputation. Arm order alternates independent→walk→repeat versus walk→repeat→independent. Walk repeat is explicit source-deterministic A/A; baseline arm is not separately repeated.
- The unit test separately verifies every `(a,b)`-labelled point from 3x128 walk points by `[a]G+[b]Q`, uniqueness, and on-curve membership. Rust library 40 tests, S3 chain F4, quotient bridge, original APS modes, and added walk example test passed in the same focused workflow.

## Measured full cold Rust wall (all setup, failed rounds, S3 and sign replay)

| Same 24 fresh public Q | Correct unknown logs | Total cold | Mean per Q | Label setup | Algebraic pair S3 trials | Source EC additions |
|---|---:|---:|---:|---:|---:|---:|
| Original independent-scalar public labels (T1536) | 24/24 | **6.028587136 s** | 251.191 ms | 3.715593095 s | 7,868,928 | 10,187,232 |
| **New 64-step public additive walk (T1536)** | **24/24** | **2.796654928 s** | **116.527 ms** | **0.098190545 s** | 9,217,536 | 259,152 |
| New walk, repeated A/A (same Q and seed) | 24/24 | 2.801513946 s | 116.730 ms | 0.098553843 s | 9,217,536 | 259,152 |

**Development observation: full cold original/walk ratio = 2.15564×**, full cold original/walk-repeat = 2.15190×. Walk repeat differs by **+0.1737%** in aggregate (same deterministic sources). Walk won **22/24** per-target comparisons. All **72/72** target logarithms verified; all arms completed in round 1; 0 recorded duplicate abscissa and 0 identity rejection in those runs.

**Interpretation:** source setup reduced **37.84×** (3.71559s → 0.09819s) and counted source EC additions dropped **39.31×**. This paid for an increase of **17.14%** in S3 pair trials (7.869M → 9.218M). Algebraic work was *not* accelerated; the gain comes from generating correct public labels at dramatically lower cost. The walk does not condition on a known scalar and the S3 polynomial root machinery is unchanged. Correlated walks have a different statistical distribution from independent labels; do not equate the result to a same-candidate, same-pairs F4 speedup.

The 24-Q experiment is a paired full-cost development A/B/B on one unisolated runner. It demonstrates a substantial scoped engineering improvement relative to **our own previous complete Rust APS-style index-calculus control** on this curve, not an independently verified performance theorem or mature external IC frontier win. A/A agreement is diagnostic, not a confidence interval, and 24/24 successful solves do not establish universal completeness under every finite trial cap. The algorithm remains **published APS one-relation methodology implemented by generic pair-MITM-equivalent S3 support enumeration**. There is **no non-generic cryptanalytic complexity claim**.

## Prior leads checked / required frontier comparisons

- **Published APS 2018** *On the discrete logarithm problem for prime-field elliptic curves*, https://doi.org/10.1016/j.ffa.2018.01.009: one useful labelled relation rather than conventional factor-base linear algebra. This run improves a source-generation component of an APS-style implementation, not the published algebraic algorithm itself.
- **PKM 2016** *Algebraic Approaches for ECDLP over Prime Fields*, https://doi.org/10.1007/978-3-662-49387-8_1: compositional rational-map bases and Semaev-system solving remain the non-generic prime-field IC comparison, not BSGS.
- [Merged #4383](https://github.com/isogeny-labs/crypto/pull/4383): fresh 24-Q control rejected a T1024 exploratory speed claim; T1536 retained. This experiment uses the **same T1536**. [Merged #4374](https://github.com/isogeny-labs/crypto/pull/4374): published-APS-inspired complete 12-Q Rust solver. [Merged #4373](https://github.com/isogeny-labs/crypto/pull/4373): actual prime31 balanced m8 F4 censored, no algebraic natural roots.
- [Merged #4228](https://github.com/isogeny-labs/crypto/pull/4228) / [open #4349](https://github.com/isogeny-labs/crypto/pull/4349): certified original-ideal quotients and compact selected-root certificates; still no successful full B31 target-conditioned S5/K rational-root oracle. [#4188](https://github.com/isogeny-labs/crypto/pull/4188) m4 factorized S4 and finite base support remains historical, not a complete natural m8 solver.
- Prior [S3 discriminant classification](S3_DISCRIMINANT_CLASSIFICATION_GATE_20261115.md) shows ordinate-aware S3 roots are just ordinary signed elliptic group sums. Do not claim further generic coordinate identities as new non-generic IC.
- KB unavailable in the GitHub-only tool runtime, repo logs and primary papers inspected. No exhaustive global priority claim.

## Next Rust tests and falsifiers

1. **Independent replay and calibration:** replicate the 24-Q fully charged run on an independent validator/source-binary identity, randomized arm order and *baseline* A/A, under isolated L2 admission only when allowed by budget. Retain negative/censored runs. Do not upload a leaderboard speedup without validated PCC record and matched reference.
2. **Robustness and scaling:** fresh independent Q, controlled table sizes 16/32/64/128, T1024/1536/2048, explicit per-Q relation success, pair counts, source-correlation diagnostics and exceptional identity branches. Every target/round source table must be paid. Avoid tuning to the same 24-Q panel.
3. **IC frontier:** compare *completed* whole cold APS-compatible Rust cost with the fastest mature APS/PKM implementations under matched constraints; separately compare actual algebraic oracle operations against native dense/tower F4 and certified quotient on equal polynomial ideals. BSGS/rho remain final generic security controls, **not** the primary index-calculus speed denominator.
4. **True non-generic research:** attack target-conditioned K961/S5 eight-summand rational-root extraction without enumerating full pair support, fully certify original rational-root equivalence and signed y-lifts, then measure independent useful relations/rank and 12 held-out cold prime31 logarithms. An improved generic walk source by itself cannot satisfy this goal.

No paid EKS/CI validation promotion or secret changes. Distinguish this same-run full pipeline engineering result from genuinely new non-generic algebraic ECDLP progress.
