# Actual prime31 B31 complete 256-log solver: first-x versus Dickson, three exact-source Rust panels (2026-11-17/18)

**Result [pcc:unverified]:** Switching the unchanged **generic-equivalent** signed 4A+4A Rust index-calculus-style solver from Dickson31 to the first 31 rational x-coordinate base points lowers *complete charged 256-target* runtime by **19.6–20.8% (1.244–1.263× speed ratio)** over three source-specific same-curve executions. This is an **engineering factor-base choice improvement** to a correct complete solver, **not** a genuinely non-generic algorithm, not a proven asymptotic advantage and not a matched victory over BSGS/rho. Candidate PR [#4635](https://github.com/isogeny-labs/crypto/pull/4635), stacked on unmerged #4629.

## Exact curve and solver contract

Unchanged original ordinary cofactor-one curve over `p=2147483647=2^31−1`: `y²=x³+847514846x+2087444724`; prime subgroup order `n=2147446831`, generator `G=(625037715,2100627251)`. Neither curve, order nor 256 identical blinded public Q were changed to favor a base. Both arms retain B=31, eight signed leaves, the same half-sum 4A hash map (618,885 distinct group points; 31 signed base x-coordinates), 512-wide Montgomery batched complements, public-known-scalar **offline** source relations and original modular rank-31 linear algebra. Source scalars are allowed only in offline relation-row RHS; **all 256 target scalars are withheld from the solver**. Every returned target logarithm is checked by `[k]G=Q` and caller-only known answer. Nine shifts are allowed; any missing hit would incur charged full-subgroup BSGS fallback (none needed here).

The original-prime31 `Rust 1.100.0` crate and curve arithmetic were source-pinned to `1df714645aa683cf975bc76fcacb9842cb4780fd`; only the existing [native Rust example](examples/prime31_s9_b31_half_sum_oracle.rs) was overlaid for the new test, not a new solver or reimplemented arithmetic. Frozen [prospective AB protocol](B31_DICKSON_FIRSTX_FULL_COST_PROTOCOL_20261117.md) was checked in before source edits. After the first AB result, separately [preregistered a posthoc BA order-bias replay](B31_BASECHOICE_REVERSE_REPLAY_PROTOCOL_20261117.md), with **no retuning** of input corpus, base size or algorithm.

## Exact-source complete costs

All wall time is **milliseconds**, including curve/base/4A setup, authentic relation collection and rank solve, 16 additional known-source relations, all target retries and failures, target-log verification, and any fallback. The input-target generation harness is excluded equally from both arms.

| Run / panel order | Dickson31 full 256 logs | First-x31 full 256 logs | First-x full-time reduction | Ratio Dickson / first-x |
|---|---:|---:|---:|---:|
| Initial prospective AB [CI #44215701063](https://github.com/isogeny-labs/crypto/actions/runs/44215701063) | 3,642.075 | **2,884.425** | 20.80% | **1.263×** |
| Repeat AB [CI #44216218925](https://github.com/isogeny-labs/crypto/actions/runs/44216218925) | 3,401.400 | **2,734.304** | 19.61% | **1.244×** |
| Repeat BA, reversed order [same CI #44216218925](https://github.com/isogeny-labs/crypto/actions/runs/44216218925) | 3,413.456 | **2,729.344** | 20.04% | **1.251×** |

A/B and B/A both show the same ordering. The two repeat executions share a hosted runner; initial and repeat jobs are separately launched. This is a source-specific sequential diagnostic, **not an independently host-calibrated performance comparison or a confidence interval**. Fixed corpus selection and possible runner variability limit generalization.

| First exact-source AB run component | Dickson31 | First-x31 |
|---|---:|---:|
| Curve/base/four-index setup | 373.867 ms | 370.978 ms |
| Source rank-31 relations, modular solve and base-log verification | 233.103 ms | 247.170 ms |
| 16 extra source relation controls | 192.446 ms | 165.346 ms |
| 256 blinded target logarithms and full checks | 2,842.661 ms | **2,100.931 ms** |
| First-shift target hits | 237/256 | **244/256** |
| Target membership attempts including failed shifts | 276 | **268** |
| Exhaustive negative target searches | 20 | **12** |
| Batched complement lanes processed (all phases) | 50,329,065 | **39,061,441** |
| Batched complement modular multiplications (all phases, separately metered) | 307,968,990 | **239,020,745** |
| Batched inversion windows (all phases) | 98,304 | **76,295** |
| Full factor-base log rank | 31/31 | 31/31 |
| Blinded unknown logarithms recovered and checked | **256/256** | **256/256** |
| Charged generic fallback calls | 0 | 0 |

Both arms needed exactly 31 public known-source relations to reach full rank, 48 known-source membership attempts including controls, and only one source-side exhaustive negative. The 16 extra relation controls passed for each arm. Distinct 4A group image count, hash-map capacity and factor-base rank were identical. The time improvement is mainly in target lookup: the first-x base reduced exhaustive misses and required fewer scanned complement windows. In the AB/BA replay, **all 31 base logs, all 31 modular rows, all 256 public Q and recovered k per base, all 256 signed eight-leaf witnesses, first-shift counts and failure flags were bit-for-bit identical between two orderings**. The original source exact tests passed 40/40 library + 2/2 focused AB/BA tests in the repeat workflow.

## Arithmetic replay and evidence custody

- [Initial 512-target raw Rust JSON plus reported Actions artifact digest](B31_DICKSON_FIRSTX_FULL_COST_RAW_20261117.json) — includes all rank rows, factor-base points/logs, 256 targets per base, signed witness, failed-attempt counts and phase/counter timings. Frozen exact source [CI #44215701063](https://github.com/isogeny-labs/crypto/actions/runs/44215701063) PASS; archive artifact [#14388691272](https://github.com/isogeny-labs/crypto/actions/runs/44215701063/artifacts/14388691272), GitHub-reported ZIP SHA256 `c8cc3fa89daf07621c2c5876a47a6f646e89e58e743eaef823be12f015c4c888`.
- [Both subsequent complete AB and BA original Rust JSON transcripts](B31_DICKSON_FIRSTX_ABBA_RAW_20261117.json) — frozen exact-source [CI #44216218925](https://github.com/isogeny-labs/crypto/actions/runs/44216218925), original lib 40/40 and focused runs 2/2 PASS; [archive artifact #14388960681](https://github.com/isogeny-labs/crypto/actions/runs/44216218925/artifacts/14388960681), GitHub-reported ZIP SHA256 `8ce84c95288a6778413c28d7bea7bd81e134b512575313ae8a1f246c420b59ce`. Reported digests are **not independently downloaded/rehashed**.
- A separately written producer-side BigInt short-Weierstrass implementation (not a second solver and not committed Rust code) checked **62 factor-base log scalar products**, **62 modular relation-row equalities**, **512 original-curve target scalar products**, **512 signed-eight-witness group identities**, exact generator order, 256 common Q and on-curve coordinates in the initial trial. Repeat AB/BA raw records are bit-for-bit identical on all arithmetic/witness fields, and so inherit those replay checks. This is a **producer arithmetic cross-check**, not independent signed PCC.
- The source-specific CI [workflow](../../../.github/workflows/prime31-b31-basechoice-full-cost.yml) uses Rust 1.100.0, pinned original arithmetic and sparse candidate overlay; no paid infrastructure or customer secrets.

## Interpretation / no false cryptanalytic promotion

The improvement is valid for **this fixed 256-public-Q transcript and generic-equivalent factor-base variant**. Generic meet-in-the-middle signed group-sum enumeration remains the relation extractor, so a factor-base improvement does not establish a non-generic ECDLP attack. Different unseen Q or another order of target cases may change the runtime. The old shared-advice BSGS reference [#4532](https://github.com/isogeny-labs/crypto/pull/4532) reported 170.113 ms for 1,024 targets, while Dickson31 complete control [#4623](https://github.com/isogeny-labs/crypto/pull/4623) used 7.658s for 1,024; **those are different jobs/configurations** and not directly comparable to this 256-Q run. The matched 1,024-Q benchmark owner [#4596](https://github.com/isogeny-labs/crypto/pull/4596) is distinct and should not be duplicated. No independent signed PCC or leaderboard promotion.

## Prior-lead audit — already searched, avoid duplication

- [#4629](https://github.com/isogeny-labs/crypto/pull/4629): on same public Q, B27 and B29 Dickson prefixes slower than B31 because negative searches dominate; not worth a shrink-only optimization.
- [#4626](https://github.com/isogeny-labs/crypto/pull/4626): preregistered 256-Q coverage control Dickson 237, first-x 244, two random B31 arms 237 each. Current experiment supplies **missing complete rank→log cost** for first-x; it does not prove first-x is universally best.
- [#4623](https://github.com/isogeny-labs/crypto/pull/4623), [#4618](https://github.com/isogeny-labs/crypto/pull/4618), [#4615](https://github.com/isogeny-labs/crypto/pull/4615): complete Dickson benchmark, 512-lane batch-inverted complement and adaptive public shifts reused here.
- [#4550](https://github.com/isogeny-labs/crypto/pull/4550), [#4575](https://github.com/isogeny-labs/crypto/pull/4575), [#4581](https://github.com/isogeny-labs/crypto/pull/4581), [#4604](https://github.com/isogeny-labs/crypto/pull/4604): exact RR9 D9 separator, B8 generic 4+4 no-go, seven-equation quotient split and ongoing sparse D9 certificate owner. The new experiment has not claimed to solve their algebraic issues.
- [autoresearcher #3301/#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305), [cryptanalysis #1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536), prime-field *Petit–Kosters–Messeng* (PKC 2016, DOI 10.1007/978-3-662-49387-8_1), *Amadori–Pintore–Sala* (FFA 2018, DOI 10.1016/j.ffa.2018.01.009) and *Kudo–Yokota–Takahashi–Yasuda* (2018, hybrid Gröbner improvements and limitations). Their published ideas are prior art; current hash-table engineering is not a novel IC exponent. Live research-KB MCP/CLI was unavailable, so this is a bounded prior-art check, not novelty clearance.

## Prioritized experiments with hard success/failure criteria

1. **Primary, truly non-generic:** on actual B31, implement a public-residual-only *sign-independent* RR9/S9 coefficient/roots/orientation extractor with existing Rust F4/quotient/rootfinding. The solver must never construct a signed `FourSet`, enumerate support words or receive the known scalar, and must pass B8 positive and the two published negative D9 cert controls **before** B31 natural targets, repeated x coordinates, exceptional y=0/point at infinity. Owner [#4604](https://github.com/isogeny-labs/crypto/pull/4604) handles D9 certificates; coordinate rather than duplicate.
2. **Alternate algebraic extraction:** K961/S5 target-conditioned x-only elimination with exact finite-field factoring and signed witness reconstruction. Require operation-level bounded cost below 4A group enumeration on original prime31 before integrating. Investigate hybrid x/y coordinate representation only if replayable public-only root extraction differs from prior [Y-coordinate support gate](Y_COORDINATE_SUPPORT_GATE_20261115.json) and prior Vieta attempts.
3. **Full-solver promotion:** once a *non-generic* candidate has verified authentic relations, insert into the same Rust 31-column rank pipeline; solve 1,024 externally blinded Q with complete setup, misses, controls, fallback, matrix and individual-log accounting. Compare on same CPU runner with optimized equal-precomputation BSGS, negation-map rho and mature published prime-field index-calculus implementation. Require independent signed PCC before announcing an advantage. Keep first-x as a better generic-equivalent correctness control, not a claimed new attack.
4. **Statistical holdout:** repeat complete first-x versus Dickson at 1,024 new seeded Q and reversed order; record 4A probe counts and peak RSS. This qualifies the factor-base choice on new data, but by itself does not satisfy the non-generic goal.

**Decision:** first-x31 is the faster complete **generic-equivalent control** for these three measured runs and this fixed corpus. Focus primary research on eliminating the group-sum table, not tuning it indefinitely. [pcc:unverified].
