# Source-bound native Rust F4 checkpoint — 2026-11-22 UTC

**Evidence:** [GitHub Actions run 45082848679](https://github.com/isogeny-labs/crypto/actions/runs/45082848679), completed original-rust-math job `137394665846` **success**, checkout commit `b8a594ea514ff44cadb5342986c68b1eac6e22ed`. Original `cargo test --locked --release --example prime31_trace_skew_pair` and `prime31_trace_moment`: **2/2 + 2/2 tests pass**. The executable pair/moment checks and four same-engine F4 modes all exited 0. This is an original-Rust **producer-run** CI result, not independently replayed PCC, full-root enumeration, or DLP-speed certification.

Curve held fixed: `p=2147483647`, `y²=x³+847514846*x+2087444724` (existing ordinary cofactor-one project fixture). Factor-base `B` denotes the number of selected rational x-abscissae, not the coefficient of the curve. Each cell is a **planted** positive identity; no random public unknown-index relation or extracted DLP is counted.

## Native F4 output: genuine two-point S3 (same target x(3G))

These are **F4 field operations** from the original engine, not independent field-operation accounting of full cold time. All cells report `timeout=false`, `inconsistent=false`, `pairs_above_bound=0`.

| B | Ordered S3+h(x)+h(y) | Pair-symmetric raw H+S3 | Degree-B G+S3 | Quadratic Newton circuit+S3 |
| ---: | ---: | ---: | ---: | ---: |
| 3 | 511 ops; 0.341 ms; max 15 cols | 483; 0.051 ms; 14 cols | **359**; 0.032 ms; 11 cols | 807; 0.110 ms; 35 cols |
| 4 | 1,238 ops; 0.082 ms; 21 cols | 864; 0.062 ms; 19 cols | **783**; 0.048 ms; 15 cols | 4,482; 0.177 ms; 74 cols |

The reduced G representation uses **29.75% and 36.75% fewer F4 field operations** than ordered at B=3 and B=4 respectively. It also beats the unconditioned H representation on this narrow field-operation metric. The circuit uses **2.25× and 5.72× more** field operations than G; simply lowering input degree to two is not a solving-work improvement. The sub-millisecond timings are non-stationary single samples, **not statistically significant**.

## Four-point chain, genuine G+2G+3G+4G=10G, B=4

Each arm contains three actual S3 equations, plus four membership equations, six variables and the same public planted group relation:

| Arm | F4 field operations | F4 ms | F4 solving degree | Peak cols/rows |
| --- | ---: | ---: | ---: | --- |
| Ordered x1,x2,x3,x4,u,v | **1,036,961** | 6.940 | 5 | 742/686 |
| Two pair-symmetric raw H systems | 2,182,508 | 5.259 | 4 | 326/294 |
| Two pair-symmetric degree-B G systems | 2,182,404 | 5.036 | 4 | 326/294 |

**Important adverse signal:** the ostensibly smaller *symmetrized* m4 chain incurs **2.105× the F4 field operations** of the ordered chain. A lower solving degree, fewer columns, or one runner's lower wall time does not reverse that fact. This source-specific six-variable chain does not yet favor the new G equations in field-work or establish a complete relation solve.

## Four-coordinate membership-only, not a relation oracle

For B=3 and B=4, the *ordered* four separated univariate membership polynomials need **0 F4 field operations** in this implementation (they are already a Gröbner basis). By contrast, raw H takes 2,985 / 16,104 ops and G takes 2,226 / 13,164 ops. G improves upon H but is not better than the correct ordered control when **no Semaev coupling is present**. Therefore membership-only degree reduction is not a meaningful stand-alone cost claim.

## Older competing quotient/GCD oracle already executed on Prime31

The prior [November 15 original-Rust S3 quotient/GCD](https://github.com/isogeny-labs/crypto/blob/main/research/prime31_ic_20261115/native/S3_QUOTIENT_GCD_ORACLE_20261115.md) was checked against genuine *public* x-targets and signed group lifts, unlike the present planted F4-only positive controls. Its [native CI #43741710811](https://github.com/isogeny-labs/crypto/actions/runs/43741710811) includes B31 ~0.45–0.49 ms and B135 ~28–29 ms single-sample quotient/GCD stages; setup is excluded. Neither a matched factor base nor identical sign checks are present in the new microbenchmarks. **No speed ratio is admissible between these dissimilar cells.** The newly proposed invariant monic-t quotient must compare with this prior working oracle as a stronger stage control before any claim of originality or advantage.

## Work still uncharged / validation gaps

1. The printed `setup_ms` times only the **Mpoly→F4 Poly conversion** inside `run_arm`. It excludes generating h, all Newton polynomial expansions, the alternative term-system construction, global preprocessing, memory, solution extraction and EC sign checks. Therefore even these component ms figures are not complete setup-plus-solve costs.
2. The F4 instrument prints matrix counts and termination diagnostics but **does not extract or independently validate all solutions from an F4 basis**. The input witness is checked; this is not a certified output relation oracle. Every failure/time budget must be treated as censored, not as a solved root.
3. The earliest B=3/4 m2 inputs **intentionally contain G and 2G** in W and their target is **x(3G)**. This positive control is selected to succeed; it does not estimate random-target yield or miss rate.
4. This new presentation has **not** faced the existing September 2026 radical pair DRL incumbent `research/radical_pair_image_20260921/RESULTS.md` on a common problem/engine. Symmetrization is prior art (Faugère et al., EUROCRYPT 2014).
5. No cold authentic rank, complete public unknown-log recovery, best-frontier index calculus or optimized negation-rho comparison is part of this gate. Nothing here demonstrates an improved asymptotic exponent.

## Next executable test

The same branch now has `--sweep` to run matched m2 ordered/H/G F4 at **B=3,4,6,8,12,16,24,32** on the same ordinary Prime31 fixture, with a hard 15-second F4 budget for every arm. The workflow records this **fixed-p, planted-target component scaling**, not a growing-field asymptotic law. Keep the quadratic circuit restricted to B<=4 pending feasibility, and do not promote single-machine millisecond noise. After this, use multiple public independent random Q and full signed-root extraction; compare to the September radical DRL control and then charge original Rust cold relation-to-log costs. `[pcc:unverified]`
