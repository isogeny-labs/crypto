# Prime31 complete Rust ECDLP: canonical-y B31 versus first-rational-x B31

**Date:** 2026-11-18 UTC · **Status:** two source-specific original-Rust exact-group benchmark runs passed, including AB/BA ordering and an independent Python BigInt replay in the final run. **[pcc:unverified]** — no independently signed scientific PCC certification.

## Scoped finding

On the unchanged ordinary cofactor-one curve
`E: y² = x³ + 847514846x + 2087444724` over
`F_p`, `p=2147483647=2^31-1`,
`#E(F_p)=n=2147446831` (prime),
`G=(625037715,2100627251)`, the alternative **canonical-small-y** factor base makes the **complete 256-target** matched original-Rust generic-equivalent IC control **28–36% slower** than the already measured first-rational-x base.

This retires **this deterministic y-ranked selection**, not all possible y-coordinate polynomial decomposition strategies. The precise reason is observed rather than asserted asymptotically: this base yields only 232/256 unshifted relations instead of 244/256 and forces twice as many exhaustive negative target searches, 24 rather than 12. It is not a new cryptanalytic algorithm or a faster complete ECDLP solver relative to BSGS/rho.

## Frozen selection and source

Existing `research/prime31_ic_20261115/native/examples/prime31_s9_b31_half_sum_oracle.rs` (Rust, **not** a rewritten group law), with the immutable original arithmetic adapter at commit `1df714645aa683cf975bc76fcacb9842cb4780fd`, runs both arms:
- Control: first 31 finite rational abscissae `x` in ascending order, canonical square-root ordinate `min(y,p-y)`.
- Candidate: first **256** such rational-x lifts, sorted by `(canonical y,x)`; take first 31. The 31 selected points are verified on the original curve, and only **3/31** coincide with first-x31.
- Both use the same signed eight-leaf `4A+4A` generic point-sum enumeration, negation quotient, original Rust 512-lane Montgomery-batched complement queries, same public basis-source seed, rank-31 modular linear algebra, 16 separate public controls and **identical caller-blinded 256 targets**.
- Each success has a signed original-curve eight-point identity. Full pipeline charges factor-base preparation, `4A` table and quotient, failed source/target searches, modular rank, controls, final recovery, original-curve verification and any fallback. No generic BSGS fallback was used in either arm.
- [Preregistered protocol](B31_Y_RANKED_COMPLETE_PROTOCOL_20261118.md) was committed before the candidate source edit; no targets or y-selection were tuned post hoc.

## Measured end-to-end costs (all phases included)

| GitHub-hosted runner | Ordered arm | First-x31 (ms) | Canonical-y31 (ms) | Extra full-solver time for y |
| --- | --- | ---: | ---: | ---: |
| [CI 44230419721](https://github.com/isogeny-labs/crypto/actions/runs/44230419721) | x then y | 2194.191 | 2824.346 | +28.7% |
| CI 44230419721 | y then x | 2160.965 | 2771.233 | +28.2% |
| [CI 44230869392](https://github.com/isogeny-labs/crypto/actions/runs/44230869392) | x then y | 1327.643 | 1722.114 | +29.7% |
| CI 44230869392 | y then x | 1330.056 | 1809.574 | +36.1% |

The independent host runs differ substantially in absolute wall time. Paired *within-run* comparisons point consistently in the same direction; these are source-specific host measurements, not isolated-host confidence intervals, a scaling law or a matched BSGS comparison.

### Phase-level accounting, final source-specific CI

| Charged phase | x-first, AB (ms) | y-ranked, AB (ms) | x-first, BA (ms) | y-ranked, BA (ms) |
| --- | ---: | ---: | ---: | ---: |
| Curve/base/4A setup, quotient | 253.486 | 255.903 | 255.933 | 258.549 |
| Public source relations/rank/replay | 103.725 | 136.767 | 103.131 | 150.086 |
| 16 separate public-source controls | 69.470 | 52.193 | 68.691 | 54.376 |
| 256 blinded unknown logs, all misses/verification | 900.963 | 1277.251 | 902.302 | 1346.564 |
| **Total** | **1327.643** | **1722.114** | **1330.056** | **1809.574** |

Both arms in both orders: **31/31 rank, 256/256 unknown logs verified, 0 BSGS fallbacks**. In both complete-run repetitions the first-x base has 244/256 unshifted hits, 12 exhaustive misses, 268 target attempts, 309443 ±-folded `4A` representatives; the y-ranked base has 232/256 hits, 24 misses, 280 attempts, 309365 representatives. The signed table has nearly identical size, so smaller table size does not determine the improvement.

### Paired coverage uncertainty (post hoc; same corpus)

The full 256 public targets are matched *point-by-point*, so the proper binary first-shift contingency is:

| First shift hit? | y-ranked yes | y-ranked no |
| --- | ---: | ---: |
| x-first yes | 221 | 23 |
| x-first no | 11 | 1 |

There are 34 discordant points, with 23 favoring x-first and 11 favoring y-ranked. A two-sided exact conditional McNemar/binomial test on those **same 256 targets** gives `p = 2 Pr(Binomial(34,1/2) <= 11) = 0.05761267291381955`. This diagnostic is post hoc and not a registered significance test. The repeat GitHub runner only repeats the **same target corpus**, so it does not furnish a second independent sampling trial. Thus this work establishes a **complete-cost deficit for this specific fixed 256-target study**, not high-confidence population dominance of first-x over all y-ranked bases or target cohorts. Reopen only with a distinct predeclared held-out target corpus and scientific power calculation, not by cherry-picking a favorable seed.


## Independent exact validation and evidence

The source-specific workflow pins Rust `1.100.0`, overlays only the candidate example onto immutable original arithmetic, and runs pinned original library regressions plus both complete full-solver AB/BA tests.

The final run, [Actions #44230869392](https://github.com/isogeny-labs/crypto/actions/runs/44230869392), **passed all workflow steps**, including a newly committed, independently implemented pure Python BigInt original-curve arithmetic verifier:
`research/prime31_ic_20261115/native/verify_b31_y_ranked.py`.
It successfully and independently replayed:
- **124** factor-base scalar multiplication identities (`[ell_i]G=P_i`).
- **124** true signed modular relation rows.
- **1,024** recovered target scalar multiplication identities (`[k]G=Q`), including repeat AB/BA panels.
- **1,024** signed eight-point group-sum witnesses against complete public targets and public shifts.
- Both deterministic factor-base selections, full 256-Q target corpus equality, and AB/BA equality of all recovered logs and signed witnesses.

The initial CI [#44230419721](https://github.com/isogeny-labs/crypto/actions/runs/44230419721) also passed original Rust arithmetic and AB/BA complete tests, and a separately implemented JS BigInt replay of its four panels passed the same 1,024-log/witness and 124-base/row checks. These are independently coded checks run by the producer, **not** independent signed PCC validation.

- [Frozen summary JSON](B31_Y_RANKED_COMPLETE_RAW_SUMMARY_20261118.json) reports both runs and all checks.
- [Final Actions raw AB/BA transcript and Python replay artifact](https://github.com/isogeny-labs/crypto/actions/runs/44230869392/artifacts/14395146631), GitHub recorded SHA256 ZIP digest `ff4808f9f9a28b77031ab4db3e56c1f19b2e39c3df3449dbcc71235e24823863`. The ZIP digest is a provider declaration, not independently ZIP-rehashed here.
- [Initial raw artifact](https://github.com/isogeny-labs/crypto/actions/runs/44230419721/artifacts/14395350831). The repeated candidate source is unchanged; the final workflow adds the independent verification program.
- Source-specific [Actions workflow](../../../.github/workflows/prime31-b31-y-ranked-full-cost.yml), [session log](../../../docs/session-logs/2026-11-18-prime31-y-ranked.md) and task in `docs/ACTIVE.md`.

## Prior leads / duplicate-work audit

| Lead | Established scope and implication | Next status |
| --- | --- | --- |
| [#4623](https://github.com/isogeny-labs/crypto/pull/4623) 31-column complete original Rust solver | Correct 1024-log `4A+4A` generic-equivalent pipeline; amortized setup, rank and full unknown logs | **Reuse as harness**, do not reimplement |
| [#4618](https://github.com/isogeny-labs/crypto/pull/4618), [#4647](https://github.com/isogeny-labs/crypto/pull/4647) | Batched inverse and ± quotient improve the **same generic-equivalent** relation oracle | **Keep as controls**, not cryptanalytic breakthrough |
| [#4626](https://github.com/isogeny-labs/crypto/pull/4626), [#4629](https://github.com/isogeny-labs/crypto/pull/4629), [#4635](https://github.com/isogeny-labs/crypto/pull/4635) | Dickson/random/first-x coverage, smaller B27/B29 bases, better first-x31 full cost | **Stop merely shrinking or re-ranking generic B31 bases** absent predictive theory |
| [#4575](https://github.com/isogeny-labs/crypto/pull/4575), [#4581](https://github.com/isogeny-labs/crypto/pull/4581) | B8 generic MITM beats sign enumeration; seven-equation RR9 quotient splits into 128 signed charts | Exact negative/positive controls; do not repeat enumeration |
| [#4604](https://github.com/isogeny-labs/crypto/pull/4604) | Public-only sparse RR9 D9 negative-certificate/Wiedemann gate, **in progress** | **Coordinate**; no duplicate owner |
| [#4596](https://github.com/isogeny-labs/crypto/pull/4596) | Actual-prime31 1024Q batched pair-table vs matched cached BSGS, **benchmark pending in reviewed PR** | **Reuse** for complete-cost frontier |
| `native/examples/prime31_m8_balanced_f4_input.rs` | Existing Rust balanced S4/S3 `8 leaves + 3 free abscissae` F4 input and bounded resource gates; no demonstrated fast extraction | **Promote this as first non-generic next test**, not another input-bridge rewrite |
| `crypto-autoresearcher` [#3305](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3305), `cryptanalysis` [#1536](https://github.com/isogeny-labs/cryptanalysis/pull/1536) | Failed/conditional Semaev alternatives and auxiliary-vs-random factor-base comparison with accounting details | Preserve prior exclusions and independent matched controls |

A complementary prior-art warning is Yokoyama–Yasuda–Takahashi–Kogure, *Complexity bounds on Semaev’s naive index calculus method for ECDLP* (J. Math. Cryptol. 2020, DOI `10.1515/jmc-2019-0029`): its Gröbner-cost obstruction is under stated statistical assumptions, not an unconditional exclusion of every target-conditioned algebraic extractor. Do not infer easy F4 solving from low-degree input alone.

Published relevant established methods: Petit–Kosters–Messeng, *Algebraic Approaches for the Elliptic Curve Discrete Logarithm Problem over Prime Fields* (PKC 2016, DOI `10.1007/978-3-662-49387-8_1`); Amadori–Pintore–Sala, *On the Discrete Logarithm Problem for Prime-Field Elliptic Curves* (FFA 2018, DOI `10.1016/j.ffa.2018.01.009`). Structural relations and summation polynomials are prior art; no claim of novelty clearance. Knowledge-base MCP/CLI were not exposed in this runtime, so repo/primary-paper audit is bounded.

## Prioritized non-generic experiment queue

**P0 — actual public-target algebraic m8 extractor, not its input formulation.**
Reuse the original Rust `prime31_m8_balanced_f4_input.rs`, `f4_fp_tower.rs`, quotient/rootfinding and existing exact original-curve arithmetic. Freeze positive, off-support negative, repeated-coordinate and exceptional-identity public cases on the actual prime31 group. Run the genuine **target-conditioned** `S4(x0,x1,x2,u), S4(x3,x4,x5,v), S3(x6,x7,w), S4(u,v,w,xR)` polynomial system with explicit degree, sparse-matrix nnz, wall, RAM and timeout ceilings. **Do not** use known leaves/signs, `4A` index, or source unknown scalar in the candidate. Extract a complete `x` leaf tuple and every necessary `y` orientation from a public R; verify the full R=(x,y). Negative full-rank/unit certificates must replay; timeout is censored, not negative. Registration gate: 1 independently confirmed positive plus two public negatives without sign/group support enumeration, all costs counted. Fail closed on truncated solution branches.

**P1 — public-only RR9 full D9 certificate/extractor.**
Coordinate with [#4604](https://github.com/isogeny-labs/crypto/pull/4604). Existing seven-equation decomposition is a 128-way sign split; require a full-eight-equation sign-independent elimination/certificate rather than a hidden sign-mask table. Test B8 held-out controls before B31 all multiplicities. Register memory/operation caps and comparison against the exact source-specific B31 paired generic control.

**P2 — complete 1024 unknown-log prime31 benchmark.**
Only after a P0/P1 extractor yields natural full-rank public relations, pass its signed witnesses into the unchanged Rust 31-column rank/recovery pipeline. Freeze 1024 caller-blinded target public points and same-runner shared-precompute BSGS, optimized negation-map rho and existing best IC; include setup, all failed calls/censored cases, field ops, modular rank, target calls, final verification, peak RSS and charged fallback. Independently replay every log and witness. An operation-count advantage alone is not a measured end-to-end speed win.

**Stopping rules:** If exact public extraction cannot produce natural positives within preregistered bounded time/RAM, report that obstruction and do not widen caps post hoc without a new frozen protocol. If real complete cost exceeds the best matched generic/index-calculus frontier, do not promote to an algorithmic breakthrough. Current low-y ranking is retired as a branch of generic factor-base tuning.

**Conclusion:** The new complete Rust comparison and independent replay are fully recorded and reproducible. The y-ranked construction is a quantified negative control; it helps focus resources on **nonenumerative algebraic relation extraction**, the unsatisfied mathematical gate.
