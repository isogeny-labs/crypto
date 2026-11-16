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
