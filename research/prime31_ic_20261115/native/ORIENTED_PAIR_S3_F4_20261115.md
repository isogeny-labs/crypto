# Prime31 fixed-S3 oriented chord–tangent F4 ablation (2026-11-15)

**Standing:** source-level construction and preregistered experiment; **[pcc:unverified]** until exact-source Rust execution. No complete ECDLP/IC relation, novelty theorem, whole-call advantage, calibration or PCC signature follows from this study.

## Why this lead was selected

The inherited ordinary cofactor-one instance is p=2147483647, E: y²=x³+847514846*x+2087444724, n=2147446831 prime, generator (625037715,2100627251). The inherited Dickson depth-six fibre has 64 preimages, of which 31 are valid canonical point abscissae.

Prior-source check:

- M8_NATURAL_F4_CI_OUTCOME_20261115.md / #4188 (now merged): all 16 public masks stopped on the **same fixed pair S3** at degree six, max dense reducer 26,618 > 15,000 cap; zero actual natural relations extracted. Removing unused tower blocks did not change that stop.
- PAIR_S3_ELIMINATION_POLYNOMIAL_20261115.md / #4188 (merged): exact rational pair-abscissa elimination to H(w) degree 961; a 961-branch specialization is not free and cannot itself establish a nongeneric speedup. The six-block S4 continuation also oversizes on its tested branches.
- #4228 (now merged), #4349 (still open): quotient/certified actual-root developments, but no completed **prime31** m8 target-conditioned wide-field measurement. Avoid duplicating or misrepresenting their scope.
- #4275 (merged): dense membership sometimes beat tower on small planted positives but not enumeration; F4 row counts are not complete solving costs.
- #4338 (open): alternative label-free selectors are proposals without a measured prime31 recovery.
- cryptanalysis #1536 (open): Rust auxiliary IC, no independently verified colder-than-rho prime31 recovery.
- crypto-autoresearcher #3305 (merged): semaev-alternatives prior-lead atlas; avoid counting degree reductions without natural useful rank.
- Published prior art: Petit–Kosters–Messeng, PKC 2016, https://doi.org/10.1007/978-3-662-49387-8_1 (composed rational-map prime-field factor bases); Amadori–Pintore–Sala, FFA 2018, https://doi.org/10.1016/j.ffa.2018.01.009 (one-relation IC frontier); Yokoyama–Yasuda–Takahashi–Kogure, JMC 2020, https://doi.org/10.1515/jmc-2019-0029 (conditional lower-bound caveats for naive Semaev IC). This is not a general novelty claim.

Research KB MCP, repo CLI wrapper, Conductor and authenticated EKS checkpoint submission are unavailable in the current tool context. No exhaustive KB search or paid measurement was attempted. GitHub-hosted tests are developer checks, not signed PCC measurements.

## An exact denominator-free oriented lift of S3

Let x=x(P), z=x(Q) be affine abscissae. Introduce y (the ordinate of the first summand), t (the chord/tangent slope), and w=x(P+Q). Define three equations over Fp:

    F1 = y^2 - (x^3+a*x+b)
    F2 = t^2 - (w+x+z)
    F3 = 2*y*t + (z-x)*w - (2*x^2+x*z+a).

These use total degrees 3, 2 and 2, versus the original S3's degree 4 in x,z,w. They involve **two more free variables**. They are not asserted to be cheaper merely from degree.

Set D=2*x²+x*z+a-(z-x)*w and f(x)=x³+a*x+b. Direct expansion gives the **polynomial identity**

    D² - 4*f(x)*(w+x+z) = S3(x,z,w),

with the repository's standard S3 normalization:

    S3=(x-z)²w²-2[(x+z)(xz+a)+2b]w+(xz-a)²-4b(x+z).

Forward: F1–F3 imply S3=0. Conversely, if x is a rational abscissa on E with a chosen nonzero y and S3(x,z,w)=0, let t=D/(2y). The identity plus y²=f(x) guarantees F2, and F3 holds by construction. With y2=y+t(z-x) the finite-field curve equation y2²=f(z) follows. The slope and w equations recover the point addition law. For z=x, F3 specializes to 2yt=3x²+a, automatically encoding doubling rather than a spurious free slope. The inverse-pair O case remains explicitly outside the affine w model; nonzero y holds for the selected odd-prime-order full curve.

Thus **on the actual 31-point liftable factor base** the two models have the same rational projected (x,z,w) solution set, up to the familiar point-at-infinity exception. Over the full 64-preimage Dickson fibre, however, x values can be *nonliftable*; the oriented presentation enforces liftability of x while the unfiltered S3 model need not. Comparing their raw F4 counts without the same factor-base restriction is **not an apples-to-apples algorithmic speedup**.

This is a standard consequence of Weierstrass addition and Semaev elimination, not a novel ECDLP complexity result. The potentially valuable *implementation hypothesis* is that F4 handles the additional low-degree variables better than the original reduced quartic/tower polynomial.

## Rust-only executable and exact checks

[examples/prime31_s3_oriented_pair.rs](examples/prime31_s3_oriented_pair.rs) links the existing project's Rust Arithmetic, Base and *actual* f4_fp_tower.rs. It does not clone a polynomial solver or use a generic sum table in polynomial discovery.

The semantic test runs 31×31×5 direct field evaluations of the resultant identity; extracts all 961 finite S3 roots among 496 unordered base pairs (465 distinct pairs with two roots and 31 diagonal doubling roots), constructs valid oriented y,t lifts and independently validates signed EC sums. EC operations here are **test-only validators**; they are not fed to any algebraic solver. Root counts are expected from the earlier certified S3 work, not newly discovered performance data.

The four development F4 arms are preregistered with a 12 solving-degree bound, 15k dense and 15k NNZ caps, and five-second per-call cooperative time budget, using the same prime p, tower representation, engine and full B31 base:

| Arm | Constraints | Comparison role |
| --- | --- | --- |
| S3 unrestricted | Original S3 plus two full 64-root Dickson towers | Reproduce fixed-pair oversize control |
| Oriented unrestricted | F1,F2,F3 plus the same towers | Structural work only; rational projections may differ |
| **S3 restricted** | S3 + monic \prod_(i=1..31)(x-x_i) and same polynomial in z | Correct 31-base comparison |
| **Oriented restricted** | F1,F2,F3 + identical 31-base membership polynomials in x,z | Correct 31-base comparison |

Report full input build, tower-normal-form input lengths, F4 wall time, reducer-matrix caps, field multiply-adds, degree reached, oversize/timeout, and no-root-extraction status. A censored F4 computation remains **undetermined**, never an algebraic no-relation result. F4 times are development-phase metrics, not complete PKM or APS IC costs.

Reproduction:

    cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_oriented_pair
    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_s3_oriented_pair -- --f4

The dedicated, ten-minute capped GitHub Actions development workflow compiles/tests/runs only Rust.

## Stop/go

**Go:** only if the exact-B31 oriented arm completes a meaningful portion of equivalent algebraic queries, or substantially lowers matrix/call costs **without** changing the original-ideal rational root set, and produces enough data to justify integrating it into the full target-dependent m8 S4+S3 system. A candidate m8 variant substitutes w=t²-x6-x7 into S4(u,v,w,xR), replacing the original free w with free y,t; the native tower F4 has a maximum of four ordinary free variables (u,v,y,t) for this variant. Its target polynomial can become higher-degree on substitution, so it must be measured rather than assumed favorable. Maintain explicit infinity and signed-lift controls.

**Stop:** if the oriented presentation still oversizes (or is worse), or the exact-base membership polynomial dominates both models. Retain the negative numbers and switch to certified quotient #4228/#4349, sparse implicit factor bases or other leads rather than increasing caps indiscriminately.

**Final gate:** full m8 natural-root extraction without generic pair support enumeration, independent signed relation rank, 12 independently verified unknown target logarithms, complete cold cost against PKM/APS IC frontier on the same p=2³¹−1 curve, and separately optimized negation-map rho as the ultimate ECDLP security metric. No requirement is already satisfied merely by a completed component.


## Exact-head first 15k-cap Rust development result (2026-11-16 UTC)

At source commit [134ba1a06ce0](https://github.com/isogeny-labs/crypto/commit/134ba1a06ce0ad74e3f58e116cc4c25363a033eb), [Actions run 43781126033](https://github.com/isogeny-labs/crypto/actions/runs/43781126033) job 133018316921 **passed**, including **2/2** new example tests. Its outputs are frozen verbatim as structured data in [ORIENTED_PAIR_F4_CI_15000_20261116.json](ORIENTED_PAIR_F4_CI_15000_20261116.json). All numbers below are single-run producer-development results **[pcc:unverified]**.

- The Rust semantic test verified **4,805** equality evaluations of the exact S3 identity, all **961** rational finite S3 roots among **496** unordered Dickson B31 point pairs, with **961** signed affine lifts/replays, **31** tangent-diagonal cases and **961** unique x-sums. These are mathematical correctness controls only.
- All four F4 calls **stopped oversize**; **zero** completed roots/refutations and no target-conditioned IC solve. Same host/input limits, F4 degree cap 12, 15k dense/NNZ, budget 5s.

| Exact B31 member equations? | Presentation | Degree reached at stop | Max reducer B' entries | Max columns | Elimination multiply-adds already done | Verdict |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| No (Dickson64) | S3 x-only | 6 | 26,618 | 429 | 30,943 | oversize / undetermined |
| No (Dickson64) | Oriented chord-tangent | 5 | 23,535 | 337 | 10,148 | oversize / undetermined |
| **Yes (B31)** | **S3 x-only** | **6** | **32,599** | **495** | **30,943** | **oversize / undetermined** |
| **Yes (B31)** | **Oriented chord-tangent** | **5** | **27,377** | **366** | **10,148** | **oversize / undetermined** |

For the **domain-comparable B31 rows**, the partial observed reducer-block entry count is 16.0% lower, the max column count is 26.1% lower, and the operations incurred **before different early stops** are ~3.05× fewer in the oriented arm. This is *not* an admitted 3.05× speedup or a completed solver cost: the oriented arm stopped at a lower degree, and neither algorithm completed. The exact cap threshold **27,377** versus **32,599** motivates a cap ladder that can discriminate whether the oriented approach reaches further with identical resource caps. The untreated Dickson64 rows are not valid same-projection comparisons.

## Preregistered next cap-ladder (before any result)

On **only the two equivalent B31-restricted ideals**, run both methods at paired dense/NNZ caps **30,000** and **60,000**, preserving the same pinned ordinary curve, point set, exact membership constraints, degree cap 12, per-call 5s cooperative limit and original F4 engine. Record every capped/timeout/basis-only result, wall time, matrix size, steps, field multiply-adds, and emitted complete solution counts (which remain unavailable without root extraction). Do not silently change resource limits between arms or assert a solver speedup from two censored stages. If both still hit oversize or cost balloons, close this presentation as a negative F4-engine lead rather than raising caps indefinitely.

This is a bounded development diagnostic on the ordinary p=2³¹−1 prime-order curve; no authenticated paid EKS benchmark, PCC signature, full m8 natural relation or PKM/APS completion is being claimed.


## Executed paired exact-B31 cap ladder — still no complete F4 output

At source [19efc09da705](https://github.com/isogeny-labs/crypto/commit/19efc09da705d6412cb8e3919f1f54dd192696ae), [Actions 43781444534](https://github.com/isogeny-labs/crypto/actions/runs/43781444534) job 133019318139 completed all Rust tests and both 15k and 30k/60k paired panels. Exact measured output: [ORIENTED_PAIR_F4_CAP_LADDER_CI_20261116.json](ORIENTED_PAIR_F4_CAP_LADDER_CI_20261116.json), producer-only [pcc:unverified].

| Exact-B31 cap | Original S3 reached degree | Original max B' | Oriented degree | Oriented max B' | Both complete? |
|---:|---:|---:|---:|---:|---|
| 30,000 | 6 | 32,599 | 5 | 37,664 | No; both oversize |
| 60,000 | 6 | **280,334** | 5 | **70,526** | No; both oversize |

The oriented model remains below the original model's first 60k-admitted large step, with a ~4.0× smaller *next rejected* B' block (70,526 versus 280,334) at the tested 60k cap. The corresponding partial multiply-adds were 246,250 versus 666,277. Crucially, **neither model completed**, and the degree and accepted-step histories differ. This is a structural *matrix-growth observation*, not a 4× reduction in complete relation-oracle time or an achieved lower ECDLP cost. Raising the fixed pair's cap alone is not the next action.

## A single target-conditioned m8 transfer diagnostic, preregistered before execution

A potentially useful property of the oriented lift is a **free-variable budget preserving substitution** into the *actual target-dependent* balanced m8 ideal. The original has three free partial-sum abscissae u,v,w. Replace the last fixed S3(x6,x7,w) by one ordinate y and one slope t, with

    y² = x6³+a*x6+b,
    2*y*t +(x7-x6)*t² = x6²+x6*x7+x7²+a,
    w = t² - x6 - x7.

The full target equation is then S4(u,v,t²-x6-x7,xR)=0, along with S4(x0,x1,x2,u)=S4(x3,x4,x5,v)=0. Thus there are **four** native F4 free variables u,v,y,t, within the actual MAX_FREE=4 engine limit, rather than adding variables to an unmodified w. The target S4 may grow to high degree/term count on substitution; this **must** be measured as a possible regression. The affine inverse-pair/identity exception is still separate.

The controlled m8 experiment is a **bounded development falsifier**, not a claim of full algebraic recovery:

1. Freeze the same 16 existing natural public masks from M8_NATURAL_F4_PANEL_20261115.json; the solver sees only full public coordinates to build the target polynomial, never the known mixed-walk coefficients or test-only split labels.
2. Build the *original* 8-tower+3-free four-polynomial ideal and the *oriented* 8-tower+4-free five-polynomial ideal using the same existing Rust S3/S4 resultant builder and F4 engine, with 12 degree, 15k dense/NNZ and five-second per-call budgets. Compare original normal-form term counts against [526,526,16] from the existing Rust source; assert semantic vanishing on a fixed planted 8-point control without using it to choose a solution.
3. Report every public target's F4 complete/oversize/timeout, partial matrix sizes, input construction time, steps, multiply-adds and root-extraction status. Do not call an incomplete basis a negative; no generic sum table or scalar input is allowed.
4. Stop this presentation if the full target-conditioned oriented system also stops uniformly with no completed roots. Further cap increases and any performance claims require a distinct admitted study with actual target roots, row rank and PKM/APS index-calculus comparisons.

The previous fixed-pair cap ladder has **not** established a successful oracle; this target-dependent experiment tests whether the two partial S4 constraints change that conclusion before abandoning the oriented mechanism.


## Executed 16-natural-target target-conditioned m8 comparison — orientation helps early matrices, not completion

The preregistered Rust source [ea455b457891](https://github.com/isogeny-labs/crypto/commit/ea455b4578913c42427753e7e8fd2d8d1ebbc9c0) ran successfully in [Actions 43782128635](https://github.com/isogeny-labs/crypto/actions/runs/43782128635), job 133021473364. It includes the actual existing ordinary prime31 \`Base\`, \`Arithmetic\`, and \`f4_fp_tower\` implementation; full exact-source JSON for all 16 natural public inputs is [ORIENTED_M8_NATURAL_F4_CI_20261116.json](ORIENTED_M8_NATURAL_F4_CI_20261116.json). The Rust positive fixture independently verifies both original four-polynomial and oriented five-polynomial inputs vanish on the same actual signed eight-point sum. No private mask coefficients or generic group-sum tables enter algebraic discovery. Producer/development **[pcc:unverified]**; not independent replicated PCC.

| Field from 16 naturally sampled public targets | Original balanced m8 S4/S3 | Target-conditioned oriented m8 |
| --- | ---: | ---: |
| Total free variables | 3 | 4 |
| Polynomials | 4 | 5 |
| Fixed polynomials after tower normal form | 526, 526, 16 terms | 526, 526, 4, 7 terms |
| Target S4 normal-form terms | 125 | **875** |
| Target construction median (observed single CI host) | 0.024 ms | **0.172 ms** |
| Degree reached | 6 | 6 |
| Max dense B' entries (each target) | 26,618 | **17,216** |
| Max columns (each target) | 429 | **291** |
| Partial multiply-adds (each target) | 30,943 | **12,141** |
| F4 wall median (observed single CI host) | 1.758 ms | 1.085 ms |
| Completed bases/root extractions | **0** | **0** |
| Censored/undetermined due to 15k dense cap | **16/16** | **16/16** |

This is a **real source-executed structural tradeoff**: the oriented m8 representation reduced the early rejected B' matrix ~35.3%, its first-stage multiply-adds ~60.8%, and its observed F4 wall clock. However, substitution into target S4 enlarged the symbolic target normal form **7×** (125 to 875 terms) and its construction time. Every natural target was *still oversize* at 15k and produced no roots. Therefore **the current implementation has no admissible per-successful-relation cost or useful-rank improvement**, no proof of a new non-generic algorithm and no complete prime31 ECDLP solver. The current F4 timings are incomplete/censored and must never be compared as a cold-solve speed ratio.

**Decision:** stop raising this orientation presentation's full m8 F4 caps blindly; both the fixed-pair 15k/30k/60k and target-conditioned 15k experiments censor every run. Freeze this as a negative completion result plus a precise resource-profile improvement. The next prioritized path is certified quotient/root extraction from merged #4228 and ongoing #4349, with wide prime31 compatibility and external same-ideal comparisons; independently explore the published APS 2018 one-relation formulation rather than merely increasing m8 F4 resource budgets. A meaningful continuation must supply *completed* natural target roots, replay signed relations and independent rank before claiming an index-calculus frontier cost benefit.
