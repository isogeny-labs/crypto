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
