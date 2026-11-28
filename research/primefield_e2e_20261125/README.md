# Prime-field index calculus

End-to-end runtime accounting

**The integration connects useful relation yield to complete phase costs.** The native model turns explicit assumptions about solver work, relation coverage, rank retention, preprocessing and linear algebra into a conditional work exponent. A separate native adapter recomputes archived benchmark summaries while retaining incomplete and unpriced outcomes.

This report answers the request to wire the existing work together and expose its end-to-end asymptotic complexity. It documents an executable accounting integration and a conditional mathematical model. The archived records do not yet supply a complete cold single-target runtime series from which an ECDLP exponent can be established. [pcc:unverified]

![Figure 1. The collection budget combines work per attempt, useful yield and required rank. The cold total also includes factor-base construction, other setup, rank maintenance, final linear algebra, recovery and verification. These are accounting dependencies, not measured timings.](pipeline.svg)

Figure 1. The collection budget combines work per attempt, useful yield and required rank. The cold total also includes factor-base construction, other setup, rank maintenance, final linear algebra, recovery and verification. These are accounting dependencies, not measured timings.

| Requested connection | Evidence delivered |
| --- | --- |
| Solver → useful relations | Explicit probability and retention inputs; solver operations remain a separate unit from calibrated GAE. |
| Relations → independent rank | Recorded ranks and full-rank flags; conditional rank-retention loss in the model. |
| Rank → total work | Cold, reusable and target-dependent phase ledger; unknown required costs cannot become zero. |
| Total work → asymptotics | Piecewise-affine exponent optimizer and finite arity scenarios; archived fits remain descriptive. |

Status: branch-local producer analysis, not signed independent performance qualification. No new cryptographic benchmark or solver run is represented by this report. Historical inputs are immutable and are referenced by source commit, path and content hash.

## 1. Count useful relations before pricing them

Let n be the subgroup order, B the actual number of admissible distinct factor-base points and m a fixed relation arity. The number of matrix columns is a separate quantity: sign folding can give two signed points per column, and a target column may be added. A map's polynomial degree, such as 2^(t), is not automatically B.

### Finite coverage and collisions

For a uniform residual and a fixed admissible tuple convention, exact solvability is the size of the corresponding sumset divided by n. Tuple counting bounds that coverage; it does not establish uniformity, independence or a collision-free sum map.

p_(solvable) = |S_(m)(F)| / n ≤ min(1, N_(tuples)/n)
N_(tuples) = C(B,m) for distinct unordered points;
N_(tuples) = C(B+m−1,m) when repetition is admissible.

The finite native model keeps the factorial and binomial terms in logarithmic form. When exact or measured coverage is absent, its Poisson occupancy approximation 1−exp(−N_(tuples)/n) is labeled heuristic. Structured factor bases, signs, repeated elements and nonuniform residual walks can change the true coverage.

### Solvability is only the first filter

p_(increment) = p_(solvable) × P(valid relation found | solvable) × P(rank increment | valid relation)
T_(collection) ≈ R_(needed) C_(trial) / p_(increment).

The last expression assumes a stationary per-attempt model and at most one independent rank increment per attempt. Rank-dependent collection uses a sum of conditional budgets over rank states. Multi-output, adaptive or graph collectors need a model that matches their actual output process. Repeated relations do not automatically increase rank.

C_(trial) is the expected work of all attempts, including unsuccessful searches, refutations, capped calls, point lifting, validation and bookkeeping. Successful-call timings alone underprice collection. When available, all attempted work divided by the actual useful rank gained is the direct finite diagnostic.

Source basis: PKM 2016, §3.2 [1]; finite model implementation [7]. The familiar B^(m)/(m!n) intensity is a sparse, fixed-arity approximation. It is not an unconditional theorem about every chosen factor base.

## 2. Sum the cold costs in one work unit

T_(cold) = T_(base) + T_(preprocess) + T_(collection) + T_(rank) + T_(linear algebra) + T_(recovery) + T_(verification).

Cold means one previously unseen target with all required setup charged to that target. A complete phase ledger states its work unit and required phases. An unknown phase leaves the complete total unknown; it is not assigned zero. Nested solver counters are not added again when the encompassing decomposition phase already includes them.

| Quantity | Meaning and handling |
| --- | --- |
| Native operations | Keep operation kinds separate: field multiplications, group additions, row operations and instructions are not interchangeable. |
| Recorded GAE | A calibrated group-addition-equivalent subtotal. A zero-priced positive-work phase or an uncharged counter prevents complete-cost interpretation. |
| Wall time | Retain the exact timing interval, process/row distinction, setup boundary, host and isolation metadata. Do not add overlapping intervals. |
| Memory | Separate live storage from peak solver allocation. Missing coverage is unknown; a reported zero is not evidence of zero memory use. |
| Failure/censoring | Retain unsuccessful, exhausted, capped and malformed outcomes. A completed-only fit cannot establish complete expected runtime. |

### Reusable work and online work

Only phases explicitly reusable across targets may be amortized. A relation phase tied to the target remains online. If reusable setup scales as n^(s), online work as n^(o) and there are K=n^(κ) targets, then:

e_(cold) = max(s,o)
e_(K targets) = max(s,κ+o)
e_(average per target) = max(s−κ,o).

These are different accounting questions. A warm online gain or a lower average over many targets does not replace the cold single-target result. The native finite aggregator uses stable log-sums rather than keeping only the dominant phase; the exponent model uses the maximum because it describes asymptotic powers.

The historical field named total_gae is retained with its producer meaning. In this integration it is shown as recorded GAE whenever unpriced work or an incomplete qualification record prevents treating it as complete cold cost.

## 3. Optimize the conditional exponent

Set B=n^(β), C_(trial)=n^(γ)B^(δ) up to polylogarithmic factors, and let useful retention incur the loss n^(−η_n)B^(−η_B). With Θ(B) required increments, write a=γ+η_(n) and d=1+δ+η_(B). Fixed arity gives:

e_(collection)(β) = a+dβ+max(0,1−mβ)
= max(a+dβ, a+1+(d−m)β).
e(β) = max(e_(collection)(β), ell·β, all other phase exponents).

ell=2 describes an assumed sparse solve with O(mB) nonzeros and O(B) iterations at fixed m. Dense elimination has a different exponent, and dense incremental rank auditing can dominate even when the final solver is sparse. The explicit base-materialization cost, preprocessing, recovery and affine memory constraints also belong in the envelope.

![Figure 2. Native optimizer outputs for declared fixed-arity scenarios. Lines connect separate conditional models; they are not measured scaling curves. The dashed line is the generic square-root exponent. No numerical uncertainty interval applies to these deterministic model evaluations. [pcc:unverified]](conditional-exponents.svg)

Figure 2. Native optimizer outputs for declared fixed-arity scenarios. Lines connect separate conditional models; they are not measured scaling curves. The dashed line is the generic square-root exponent. No numerical uncertainty interval applies to these deterministic model evaluations. [pcc:unverified]

Each phase is affine in β after the collection term is expanded into two lines. The maximum is convex and piecewise affine. On the feasible closed interval, an optimum occurs at an endpoint or at a pairwise line intersection (flat minima include such points). The implementation enumerates these candidates, avoiding a grid-resolution assumption.

This is a model optimization argument. It supplies no missing solver-cost bound, coverage law, rank-retention law or independent performance evidence. Its assumptions must be connected to a concrete algorithm before its exponent becomes an algorithmic bound.

## 4. What the model says, conditionally

| Scenario | m | β at optimum | Exponent e |
| --- | --- | --- | --- |
| unit trial, sparse LA | 3 | 0.250000 | 0.500000 |
| unit trial, sparse LA | 4 | 0.200000 | 0.400000 |
| linear B trial, sparse LA | 2 | 0.000000 | 1.000000 |
| linear B trial, sparse LA | 3 | 0.333333 | 0.666667 |
| linear B trial, sparse LA | 5 | 0.200000 | 0.400000 |
| quadratic B trial, sparse LA | 7 | 0.142857 | 0.428571 |
| linear B trial, dense LA | 6 | 0.142857 | 0.428571 |
| m5 linear trial; n^0.1 retention loss | 5 | 0.200000 | 0.500000 |
| m3 linear trial; B^3 setup | 3 | 0.250000 | 0.750000 |
| m5 linear trial; memory beta<=0.15 | 5 | 0.150000 | 0.550000 |

The table contains native evaluations of explicit assumptions. It does not assert that an implemented decomposition oracle attains the listed trial costs. [pcc:unverified]

### The current two-summand architecture

For m=2 with work at least linear in B per attempt, no favorable rank loss and the standard full-base relation goal, collection costs at least order n in this model. In the sparse-coverage regime, Θ(B) increments require Θ(n/B) attempts, and multiplying by Ω(B) work per attempt gives Ω(n). Saturation does not lower the resulting exponent. This conclusion is scoped to that architecture and cost assumption.

### A useful threshold test

With only the collection envelope and ell·β active, m>d and nonnegative coefficients, the conditional optimum can be written as:

β* = min(1/m, (a+1)/(m−d+ell))
e* = max(a+d/m, ell(a+1)/(m−d+ell)).

A strict exponent below 1/2 requires both terms to be below 1/2. At a=0 this requires m>max(2d,d+ell). A fixed polynomial trial-cost model thus states what would have to be demonstrated about a higher-arity oracle; increasing m alone does not demonstrate it.

### Growing arity is a separate asymptotic analysis

If m grows with n, retain log_(n)(m!), all m-dependent solver costs, matrix density, memory and uniformity bounds. Substituting m→∞ into a fixed-m formula silently drops terms and cannot establish subexponential runtime. The finite model exposes these inputs rather than asserting that they vanish.

## 5. Reconcile the archived measurements

The archive is pinned to [source commit aab505dd2d24](https://github.com/isogeny-labs/crypto/tree/aab505dd2d24cd5c98f9be1a2bd2243793a2001b/research/pkm_adaptive_sweep_20261123) in PR #5067. The native adapter recomputes the selected records without changing those inputs. Original selected-screen results, later full-rank panels and alternative engines remain separate observation groups.

![Figure 3. Native arithmetic ratios from the archived records identified in the source manifest. Solver operations and recorded GAE have distinct denominators and calibration. These historical arithmetic ratios are not new benchmark measurements or complete-cost speedup claims. Sources: degree32.jsonl, degree32_replication_extend.jsonl and e2e_full_rank_13bit_20261185.jsonl. [pcc:unverified]](archived-ratios.svg)

Figure 3. Native arithmetic ratios from the archived records identified in the source manifest. Solver operations and recorded GAE have distinct denominators and calibration. These historical arithmetic ratios are not new benchmark measurements or complete-cost speedup claims. Sources: degree32.jsonl, degree32_replication_extend.jsonl and e2e_full_rank_13bit_20261185.jsonl. [pcc:unverified]

| Recorded series | Rows / outcome | Accounting status |
| --- | --- | --- |
| Timed archive rows | 768 rows; 0 exhausted | Failures retained; preflights recorded separately |
| Producer recovery flags | 768 true | Producer assertions; no independent recovery replay here |
| Full-rank flags | 672 reported | Separate from scalar recovery; historical stopping rules differ |
| Complete-cost qualification | 0 qualified | Complete cold totals remain unknown |

The uploaded summary combined a raw-operation comparison with a calibrated-cost ratio. Both may be correctly computed and still differ substantially: the operations have different measured costs, and the phase totals cover different work. Report the numerator, denominator, units, panel and inclusion rule with every ratio.

Later full-rank rows improve the documented linear-algebra stopping condition. A recorded recovery value, a producer verified flag and a recorded full-rank flag are distinct evidence fields; none substitutes for signed independent replay. A source file hash binds the input bytes, not the executable that originally produced them. [pcc:unverified]

The incomplete replication file contains 384 preflight rows, 384 marked exhausted, and 24 failed envelopes. It has no timed envelope. Those outcomes remain in the native source summary and are excluded from the timed ratios by their recorded stage, not because they failed. [pcc:unverified]

No confidence interval is invented for an arithmetic aggregate. Any descriptive fit supplied by the native analyzer retains its actual group and input range. The tiny, fixed-parameter panels do not meet the repository's prospective exponent-evidence criteria.

## 6. Corrections to the uploaded summary

| Original implication | Required correction |
| --- | --- |
| Graph cycles give a universal obstruction | The cost inequality is conditional on the residual distribution, edge-extraction cost and useful-cycle stopping law. A graph cycle need not be a row dependency or have a nonzero target projection. |
| Unique factorization makes decomposition efficient | Uniqueness does not imply an efficient factorization algorithm. State the actual smoothness/factoring procedure and its cost. |
| O(mb) encoding size destroys subexponentiality | The cited 2026 Theorem 4 explicitly uses O(mb) edges. Its cost bound retains both b/m and mb/sqrt(log b); the admissible growing-arity choice keeps their sum sublinear in b. |
| The encoding forces Jacobian coordinates | The cited construction uses affine coordinates, exceptional-case selectors and auxiliary inverse/slope witnesses. A separate width objection needs the actual encoding assumptions. |
| SWU totality implies uniform or injective indexing | RFC 9380 distinguishes encoding properties. A total curve map does not preserve a chosen index population injectively or uniformly. Count distinct points and collisions. |
| Ordering all slots always supplies an m! quotient | Permutation symmetry requires compatible slot domains and constraints. Distinct slot factor bases and repeated entries change the symmetry/stabilizer count. |
| A chain implies treewidth1 for the whole solve | The graph definition matters. A chain component does not establish a whole-system elimination bound; cross-slot constraints and field arithmetic must be included. |

2026 conditional bound [2]: 2^(O(b/m + mb/√(log b)))
m = ceil((log b)^(1/4)) gives 2^(O(b/(log b)^(1/4))).

The pinned graph source contains finite signed-incidence examples and recovery conditional on a nonzero target coefficient [5]. It does not establish an unconditional natural-stream stopping distribution. The 2026 reduction is conditional on its tensor-contraction premise; its asymptotic expression is not a concrete runtime estimate for deployed curves.

For example, a pair-sum directory model charges L²+nB/L², while an idealized oracle model with Θ(L) required accepted edges charges L+n/L. Their square-root bounds depend on those specified setup, occupancy and stopping assumptions; neither expression is a universal statement about every graph-based method.

These corrections preserve the published experiments and narrow interpretations to their supported hypotheses. They do not reopen or relaunch unrelated stopped research tasks.

## 7. Reproduce and extend the accounting

The native CLI reads the source manifest, checks the pinned input bytes, preserves outcome counts with links to the immutable row-level records, and emits the accounting and model outputs consumed by this renderer. Use the command documented with the CLI to regenerate those outputs, then render:

```sh
python3 research/primefield_e2e_20261125/render_report.py
```

The renderer only formats native values and plots coordinates; it does not compute ratios, regressions, relation budgets or exponents. Unit tests exercise the model's analytic cases, missing-phase rejection, numerical edge cases and malformed archived inputs. Consult the PR's current test receipt for commands and actual pass/fail scope; a small standalone build does not establish qualification of the whole repository.

### What is needed for a measured exponent

A prospective series needs complete cold costs, comparable parameter policies B(n) and m(n), measured relation/rank yield, setup and memory coverage, retained failures, source-bound execution and independent replay. The repository's T2 criterion requires at least six sizes spanning at least 24 bits with a held-out size. A T3 subexponential claim needs a mathematical argument with explicit assumptions; measurements alone cannot supply it [6].

For a raw fit log_(2)T against log_(2)n, a square-root comparison uses slope 1/2. For log_(2)(T/T_(rho)) the corresponding threshold is zero when the reference scales as n^(1/2). The uploaded summary's normalized threshold of 0.50 therefore mixed the two axes.

### Sources and evidence custody

[1] Petit, Kosters and Messeng (2016), [Algebraic approaches for the elliptic curve discrete logarithm problem over prime fields](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf), especially §3.2. It separates preprocessing, relation collection and linear algebra.

[2] Akshima et al. (2026), [A Subexponential Time Algorithm for Elliptic Curve Discrete Logarithm](https://github.com/bochum-crypto/subexponential-ecdl/blob/1ee44e562b1b9a50ea339f3b8f81112b5a24b96f/subexponential-ecdl.pdf), v0.2, 29 October 2026, Theorem 4 and Lemmas 11–13. Conditional source; no independent proof claim is made here.

[3] [RFC 9380: Hashing to Elliptic Curves](https://www.rfc-editor.org/rfc/rfc9380.html), encoding/map properties and simplified SWU definitions.

[4] [crypto archived PKM study at aab505dd2d24](https://github.com/isogeny-labs/crypto/tree/aab505dd2d24cd5c98f9be1a2bd2243793a2001b/research/pkm_adaptive_sweep_20261123); the manifest records exact paths/hashes. Original JSONL files and their producer reports remain canonical historical evidence.

[5] [Cycle-closure report](https://github.com/isogeny-labs/crypto/blob/26233d8bd28031bf64e959c7234815bf1a38958b/research/ic_cycle_closure_20261125/README.md) and papers/ic-negative-results/lean/IcNegative/CycleClosure.lean at main 26233d8bd280. Source-scope review only in this integration; no new Lean build is represented.

[6] [Repository success criteria](https://github.com/isogeny-labs/crypto/blob/26233d8bd28031bf64e959c7234815bf1a38958b/docs/ic/SUCCESS_CRITERIA.md) and its AGENTS.md. [7] This PR's complexity_model.rs, complexity_accounting.rs and tools/primefield-e2e; immutable input manifest and native outputs beside this report.

Sources checked (KB): KB unavailable; the wrapper returned exit127 because no research-kb executable/catalog was installed. Parent-reviewed primary papers, the archived source records and current graph correction were the fallback. No search-absence or novelty conclusion follows.

Canonical graphs and benchmark copies: no new eligible measured speedup or leaderboard entry is produced. Existing historical graphs in PR #5067 and canonical scoreboards remain unchanged. This report adds accounting/model visuals, not a new performance run. [pcc:unverified]

## Appendix. Native outputs and run commands

| Archived solver series | Raw slope | Slope vs √n | Input range |
| --- | --- | --- | --- |
| Interval / F4 | 0.9499 | 0.4499 | 7 orders / 3.713 bits |
| Vélu / F4 | 1.0009 | 0.5009 | 7 orders / 3.713 bits |

These OLS slopes describe the raw solver-operation file means in the recorded full-rank F4 panels. They are not calibrated total-runtime slopes. The fits combine different curves and repeated seed files at nominally fixed t; their short input range and incomplete cost coverage prevent an asymptotic inference. [pcc:unverified]

| Cost/provenance gap | Implication |
| --- | --- |
| Positive uncharged native counters | Auxiliary work remains outside the recorded GAE subtotal. |
| Positive setup wall time with zero GAE | The phase is present but not fully priced in that work unit. |
| Missing calibration for positive work | A zero/null price cannot establish zero cost. |
| 768 timed rows missing producer git commit | Input commit pinning does not restore the original executable's source binding. |

Native analysis used Rust 1.100.0. From the repository root, fetch the pinned archive commit if it is absent, build the native tool and regenerate the outputs. The example pipeline input deliberately leaves recovery/verification unknown, so its complete budget must remain null even though its conditional exponent can be evaluated.

For that illustrative input, the native result gives e*=0.400000 and log_(2) relation work=46.912911, retaining log_(2)(m!)=6.906891. The finite probability model is the Poisson occupancy heuristic. The unit is explicitly hypothetical, and complete_budget remains null because recovery_verification is unknown. This demonstrates the connection without presenting it as a measurement. [pcc:unverified]

```sh
git fetch origin aab505dd2d24cd5c98f9be1a2bd2243793a2001b
cargo test --locked --release --manifest-path tools/primefield-e2e/Cargo.toml
cargo build --locked --release --manifest-path tools/primefield-e2e/Cargo.toml
e2e_study=research/primefield_e2e_20261125
e2e_bin=tools/primefield-e2e/target/release/primefield-e2e
"$e2e_bin" analyze . "$e2e_study/sources.json" "$e2e_study"
"$e2e_bin" models "$e2e_study"
"$e2e_bin" evaluate "$e2e_study/pipeline-input.json" "$e2e_study/pipeline-result.json"
python3 "$e2e_study/render_report.py"
```

Rendering dependencies: Python with ReportLab and PyMuPDF, Liberation Sans Regular/Bold fonts under /usr/share/fonts/truetype/liberation, and Poppler for page inspection. These dependencies lay out the report only; Rust performs the analysis.

