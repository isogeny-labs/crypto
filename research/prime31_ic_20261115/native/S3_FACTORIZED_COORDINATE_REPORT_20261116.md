# Prime31 APS S3 factorized-coordinate root kernel — development report, 2026-11-16

**Execution standing:** [pcc:unverified]. A Rust source implementation and frozen experiment protocol have been committed; the first source-specific CI exposed a test-only coefficient-index error (corrected in `39e8877`), and the repaired measured campaign is still subject to exact-head CI execution and subsequent evidence capture. **No timing or speed claim is made in this report before examining the complete actual stdout.**

## Mathematical derivation and classification

On the existing ordinary prime-order curve over p=2147483647, y²=f(x)=x³+847514846x+2087444724, for two distinct rational abscissae x,z and any finite square roots u²=f(x), v²=f(z), define

    d = (x-z)^2 != 0
    r_+ = (u+v)^2 / d - x-z
    r_- = (u-v)^2 / d - x-z.

Then, with the original source's S3 coefficient convention, **S3(x,z,T)=d(T-r_+)(T-r_-)**. This is the classical elliptic chord identity, not a new theorem. Changing the signs of u and/or v merely swaps the unordered roots. The source's global abscissa deduplication ensures x≠z across all lists. The odd prime-order rational group cannot contain nonzero rational 2-torsion, so each rational input point has nonzero ordinate; however repeated x would require its own tangent/identity branch and is explicitly rejected.

The previous x-fibre implementation computed a,b,c of S3 for each pair, a square-root product of discriminants and two quadratic roots; it also spent two complete S3 polynomial evaluations for every pair to check them. The new factorized kernel calculates two coordinate roots directly using x-fibre square roots obtained from **public x only**, a batch inversion of d per first-list row, and cheap field squares. The identical existing labelled-public-source generator, third-partition lookup, signed group identity verifier, nonzero target coefficient check and [k]G=Q replay are unchanged. The test independently evaluates 2304 pairs against both the original S3 polynomial and its separate quadratic formula and compares one complete baseline/candidate signed recovery.

**Classification:** This is an engineering acceleration of the published APS-style one-relation method. The current algorithm enumerates finite first/second-list pairs, which is equivalent in support and exponent to generic meet-in-the-middle; per repository AGENTS.md it belongs to the **generic-decomposition track**, even though it computes roots of S3 over Fp. Prior `src/cryptanalysis/prime_fast.rs` already uses group-theoretic point-sum root shortcuts. This report does not claim new non-generic index calculus, a sub-rho exponent, or a security break.

## Frozen matched experiment (before source changes)

[Protocol](S3_FACTORIZED_COORDINATE_PROTOCOL_20261116.md) was committed at `da4ef36` before the implementation. The amended example source commits include `9e8e1c2` and test correction `39e8877`.

- Ordinary prime31 n=2147446831, cofactor 1, fixed G and [384,384,16384] lists, max six full rounds. Twenty-four fresh public Q from deterministic seed `0x463343484f524430`.
- Three independent, all-cost-paid fully cold solver arms: old S3-coefficient/x-fibre/Jacobian, new factorized root/Jacobian, and factorized A/A repeat. Caller-side known k never passed to the solver. Rotate execution order 3 ways.
- All 72 scalar answers and exact signed relation transcripts (including misses, source collisions, pair counts, rejected identities and selected signs) must agree. Workload, public list ordering and code generation remain fixed.
- Developer speed screen: all 24/24 each arm complete, candidate ≤0.90× matched baseline cost, 18/24 or more individual Q wins, candidate A/A drift ≤5%. A positive screen **requires** a separate fresh confirmation and independent PCC before a default change; slower or incomplete outcomes are preserved.
- CI step: `.github/workflows/prime31-s4-split-20261116.yml`, `--factorized-s3-confirm` and Rust 1.100 release tests. No EKS spend.

First observed GitHub Actions run [43869362412](https://github.com/isogeny-labs/crypto/actions/runs/43869362412) passed the older native F4/B8 gates and 11 of 12 S3 tests, but its new unit assertion accidentally checked the S3 **constant** `c` (`f.2`) against the quadratic leading coefficient `(x-z)^2` instead of `f.0`. That test-only error blocked the benchmark; the failure is retained and correction committed in `39e8877`. **Not a measured candidate failure or evidence of identity falsification.**

## Prior-lead audit and next experiment queue

| Lead | Evidence/standing | Priority and falsifiable next gate |
| --- | --- | --- |
| Published APS one-relation finite S3 pair search | Complete historical Rust known-answer solves, engineering gains in #4391; explicitly generic MITM-equivalent | **Control**: complete equal-output and exact-source costing against mature external APS/PKM, then optimized rho separately |
| Factorized-coordinate S3 root kernel (this PR) | New source and preregistered matched experiment; **no result until repaired exact-source CI** | **Engineering**: equality + 24×3 full cold measurements; if win survives, 24 fresh Q and independent PCC |
| Target-conditioned symmetric S9/Vieta eight-leaf original-ideal extraction | #4373 direct F4 capped/undetermined; symmetric B8 root correctness but no natural root oracle, #4228/#4349 certificate infrastructure | **Highest scientific priority**: small-base B4/B8 natural positive/negative/infinity with original-ideal certificate, no list enumeration, then B31 full unknown Q solves |
| Sparse action of S9 in symmetric power of rational factor-base algebra | Proposed only, not established. With B rational base abscissae, symmetric degree-eight fibre has dimension binomial(B+7,8): **165 (B4), 6435 (B8), 48,903,492 (B31)** before S9 | **High-risk scientific**: construct sparse linear action or quotient certificate whose construction avoids diagonalizing/enumerating all multisets. Measure exact target-conditioned root outputs vs brute validation on B4/B8; abort if the action requires materializing the full support |
| Fixed two-anchor six-summand F4 | Prior measured coverage upper bound 3.7407%; 0/16 natural positives, incomplete F4 even on constructed fixture | **Stop** for natural full-solver route unless coverage or actual certified F4 completion mechanism changes |
| More post-Jacobian partition sweeps | 32 Q × five arms × two complete runs; incumbent [384,384,16384] wins in both, all 320 logs verified | **Stop** routine parameter cycling without a new mechanism |
| Y-coordinate or generator-tailored factor-base experiments | Existing proposals, no verified rank/degree advantage established for ordinary cofactor-one prime31 | **Lower**: only promote if equal-root-set and useful-rank tests show a structural, non-generic improvement; avoid replacing a correct solver with a restricted instance |

### Required non-generic benchmark admission

For a proposed S9 algebraic extractor, explicitly build the original ideal on E and the rational factor-base constraints; certify each returned solution in the original ideal, handle zero denominators/infinity, measure both positive and negative natural targets, and count *independent useful signed relations* rather than ordered decomposition multiplicities. Then recover complete, previously unknown prime31 scalars with all setup, failed calls, certificate checks, linear algebra and target replay charged. Compare complete equal-output oracle calls with PKM/APS, native F4/certified quotient, and mature external msolve/Magma F4 when accessible, and use matched optimized rho for security context. A cut-off F4 run is **undetermined**, not slower completed solution.

References: [APS 2018](https://doi.org/10.1016/j.ffa.2018.01.009), [PKM 2016](https://doi.org/10.1007/978-3-662-49387-8_1), [FPPR symmetry work](https://doi.org/10.1186/s40736-015-0012-6), [PR #4391](https://github.com/isogeny-labs/crypto/pull/4391), [PR #4373](https://github.com/isogeny-labs/crypto/pull/4373), [PR #4349](https://github.com/isogeny-labs/crypto/pull/4349). Research-KB access is unavailable to this GitHub connector runtime; code search and primary publication pages are a bounded fallback, not an exhaustive absence audit.


## Executed exact-source development benchmark — PASS, 2026-11-16 12:48 UTC [pcc:unverified]

**[GitHub Actions run 43869660129](https://github.com/isogeny-labs/crypto/actions/runs/43869660129), job 133307150870, completed SUCCESS** on the repaired Rust source commit `39e8877b89fa5697ccc30909655dea1222b7beb8` (the code change from prior failed run was only the S3 coefficient-index unit test assertion). Rust 1.100 release focused library, prior Rust S3/F4 correctness controls, **12/12 example unit tests**, the new factorized-root test, and the actual `--factorized-s3-confirm` completed. The full original source-printed JSON, with GitHub timestamp prefixes only removed, is frozen in [S3_FACTORIZED_COORDINATE_MATCHED_CI_20261116.json](S3_FACTORIZED_COORDINATE_MATCHED_CI_20261116.json); all 24 caller-generated public Q and all 72 complete arm transcripts, including the single failed/continued initial round, are retained.

| Full cold single-target Rust, sum of 24 different Q | Old S3 coefficients + x-fibre | New chord-factorized S3 | New A/A repeat |
| --- | ---: | ---: | ---: |
| Verified complete logs | 24/24 | **24/24** | **24/24** |
| Fully cold wall | 133.238445 ms | **111.633604 ms** | 110.292700 ms |
| Public Jacobian-labelled source cost | 54.272100 ms | 54.309344 ms | 53.648462 ms |
| S3 relation-search stage incl. verification | 71.150922 ms | **49.550447 ms** | 48.902912 ms |
| Actually visited S3 pair roots/quadratics | 684,672 | 684,672 | 684,672 |
| Total attempted labelled rounds | 25 | 25 | 25 |

Complete new/old cold-cost ratio **0.8378483**; **16.215% less wall** or **1.1935x faster** on this same-host development panel, with **24/24 paired target wins**. The candidate A/A aggregate drift is **1.2012%**. All signed relation indices, signs, alpha/beta, recovered k, round count, collisions, square roots, batch-inversion counts and actually visited pairs were exactly equal across the three arms; no target known-answer leaks into the solver. The preregistered exploratory gate **PASSES**. A descriptive 20,000-resample paired nonparametric bootstrap of the 24 cost ratios-of-sums produced approx **0.802–0.880** percentile endpoints (not externally calibrated and not an independent-PCC interval).

The setup cost is essentially unchanged, isolating the saving to the S3 arithmetic stage (71.151 → 49.550 ms). This engineering measurement does not change the algorithm's **generic-decomposition** classification: its finite pair support is still enumerated; there is no non-generic exponent claim, mature external APS/PKM/CAS matched price or optimized rho victory.

**Decision:** keep this as a measured source-specific *candidate* for the existing complete Rust APS-style solver. Do **not** silently change the incumbent/default or promote to a leaderboard on one panel. Per frozen protocol, execute a separate prospective 24-Q public target set with the same three arms and acceptance gates, then seek independent source/PCC replay before ratcheting. Do not rewrite the initial failed-test record; it was a test-only coefficient-index bug, corrected before this passing run.


## Cross-repository lead-atlas audit: isogeny-labs/crypto-autoresearcher

After the first matched run, located the **actual** shared sibling `isogeny-labs/crypto-autoresearcher` (a prior bounded search of `aburan28/crypto-autoresearcher` had returned no files, which was a repository-identity miss). Read the [full Semaev-alternatives campaign summary](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/experiments/semaev-alternatives-20261105/SUMMARY.md) and [ten errata](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/experiments/semaev-alternatives-20261105/ERRATA.md). This materially changes the **scientific** next-gate ranking:

- **Do not restart basic Vieta-conic sieving.** SA-02 KILL: empirical sieve exponent 1.04, and the conic condition reduces to a genus-one square constraint. Our proposal is a *different original-ideal-certified target-conditioned symmetric S9 quotient*, not another conic or tuple enumeration.
- **Do not restart unstructured lattice/PKM-tower F4 tuning.** SA-01c found the same m=3 determinant reach 1/24 for four presentations; SA-14b measured t=10 F4 degree 6 on the natural and matched null arms and rejected the sought tower advantage. SA-15b's 100 matched planted m4 instances were solved by generic pair table, 0 by non-generic solvers at the cap. Any new algebraic selected-root mechanism must demonstrate an actual complete natural result, not a different presentation of the same censored system.
- **Pay total costs, not relation labels.** SA-21 directly measured 20–40-bit full solver growth, with IC-pair `alpha≈0.65` and rho `alpha≈0.52` on its own curve family; SA-22's higher-bit projections depend on model assumptions. These are **different study workloads**, not matched run times for our prime31, but they warn that a faster generic S3 kernel is unlikely to change the exponent.
- **Covers over the same prime field are not a shortcut.** SA-24c showed genus-2 cover Jacobian index calculus approximately linear in p with 1–4% graph linear-algebra cost, a factor ≈p^1/2 behind generic E rho; earlier projected 4/3 exponent was corrected by ERRATA 10. Need an actual smaller-field transfer, not an unpriced cover.
- **Y coordinates have a narrow surviving mechanism, not a general speed claim.** SA-19/SA-20 found lower twisted-Edwards lattice degree budget (9→8) yet no provable improvement to the Coppersmith reach ceiling. Only test y-factor bases on our curve with an equal-domain original-root and useful-rank certificate, not an assumed generic ECDLP advantage.

**Revised scientific priority:** B4/B8 exact original-ideal-target-conditioned S9 selected-root extraction remains a *high-risk, falsifiable* lead, with an additional explicit control showing that all candidate roots were not generated by generic multiset/group enumeration. Abort quickly if construction reduces to SA-02/SA-15b support scanning. The shared KB itself remains unavailable; this is a direct source-backed sibling audit, not an exhaustive novelty clearance.


## Independent, prospectively seeded 24-Q confirmation — PASS, 2026-11-16 [pcc:unverified]

The independent seed/panel was **preregistered before source and execution** in [S3_FACTORIZED_FRESH_PROTOCOL_20261116.md](S3_FACTORIZED_FRESH_PROTOCOL_20261116.md), commit `95cd08ca7fb145e3314bf001d9651607581010c9`. Actual source-specific execution: [GitHub Actions run **43870507909**](https://github.com/isogeny-labs/crypto/actions/runs/43870507909), job `133310108831`, **PASS** with Rust 1.100 locked release and all focused pre-existing steps. The fresh executable source is `3730a51e0a1aebf1ee164b64ab2e4f82ab653a62`, with Actions wiring at `0ce7729c0d78e60a9299481cc29a4a8eb2dd4f4b`. Full original program stdout (only GitHub timestamps/intermittent U+FEFF log markers removed) is committed in [S3_FACTORIZED_FRESH_CI_20261116.json](S3_FACTORIZED_FRESH_CI_20261116.json).

The **new 24 public Q** come from seed `0x53335f434f4e4631`; the caller checked **zero scalar overlap** with the prior 24. They were not reused from the exploratory panel. All groups use the same ordinary prime-order curve `p=2147483647,n=2147446831` and identical signed S3 relation search semantics.

| 24 independent fresh Q, fully cold Rust | Old S3 polynomial/x-fibre incumbent | Factorized x-fibre chord S3 | Candidate A/A repeat |
| --- | ---: | ---: | ---: |
| Complete, correct unknown-log recoveries | 24/24 | **24/24** | **24/24** |
| Full cold wall, sum across 24 Q | 97.845927 ms | **80.391298 ms** | 79.612401 ms |
| Paid public source setup | 37.665217 ms | 37.922206 ms | 37.424289 ms |
| S3 algebraic search plus signed verification | 53.750756 ms | **36.016916 ms** | 35.764454 ms |
| Actual S3 pair quadratics visited | 840,576 | 840,576 | 840,576 |
| Total attempted complete labelled rounds | 24 | 24 | 24 |

**Measured fresh-confirmation cold-cost ratio B/A = 0.8216111**, i.e. **17.839% lower cold wall or 1.2171× faster** than the matched incumbent. Candidate beat the incumbent on **24/24** independently generated public Q. The two candidate repeat arms' aggregate drift was **0.9689%**, within the frozen 5% A/A cap. The preregistered fresh gate **PASSES** in every field. Exact signed relation indices, sign assignments, alpha/beta, target scalars, rejected identity/duplicate counts, batch-inversion counts, x-fibre square-root counts and pair counts matched all arms. Each scalar was independently replayed to the original public Q. A descriptive 20,000 paired-resample percentile interval for the fresh ratio-of-sums is about **[0.794,0.853]**; it is *not* a signed or cross-host scientific CI.

### Combined evidence, interpretation and next admission

**Two prospectively distinct 24-Q target panels** completed **144/144** full solver calls across incumbent/new/new-repeat with correct recovered discrete logs, source-exact equality of public relation transcripts, and complete charged cold costs. New/old ratios were **0.83785** on the first panel and **0.82161** on the fresh confirmation, with candidate faster on **48/48 paired targets**. This is a substantive measured internal Rust-engineering improvement to the *complete* APS-derived prime31 solver, not merely a faster kernel microbenchmark.

**The method remains generic-decomposition/MITM equivalent:** factorizing S3's roots is classical elliptic chord-coordinate arithmetic; enumerating the first-two finite labelled lists does not create a new algebraic oracle or superior ECDLP exponent. The repository's own two-track policy prohibits promoting this to the non-generic algebraic-track baseline. Development CI standing is **[pcc:unverified]**. No independent PCC signer, external same-host mature APS/PKM/msolve complete solver control, optimized rho/security benchmark, or multiple-curve scaling was executed in this run. Do not assert that it beats those implementations, nor change the official leaderboard or default without their gates.

**Immediate actionable next checks:** (i) independent immutable-source Rust replay/cross-host PCC of the 48 Q and a third truly held-out panel if required; (ii) fully cold matched external APS/PKM and optimized generic control on the exact prime31 curve; (iii) original-ideal-certified target-conditioned S9 selected-root B4/B8 extraction, explicitly excluding SA-02 Vieta-conic scans and SA-15b failed generic-support reformulations, with total root+certification work charged. A censored F4 remains undetermined, not a completed comparison.
