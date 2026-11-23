# Native Prime31 Rust F4 and monic S3 gate — 2026-11-23 UTC

## New evidence and exact provenance
Read the **completed original-Rust** [GitHub Actions run #45086381347](https://github.com/isogeny-labs/crypto/actions/runs/45086381347), job [137405842381](https://github.com/isogeny-labs/crypto/actions/runs/45086381347/job/137405842381) on commit `b8a594ea514ff44cadb5342986c68b1eac6e22ed`, Rust 1.100.0, unchanged ordinary prime-order `E/F_(2^31-1): y²=x³+847514846x+2087444724`, `n=2147446831`. **Job conclusion SUCCESS**, two skew-pair tests pass, two moment tests pass, two non-table monic-pair tests pass, every requested F4 stage completed without reported timeouts. Independently inspected GitHub's actual downloaded log rather than treating an enqueued run as a pass. Earlier [run #45082848679](https://github.com/isogeny-labs/crypto/actions/runs/45082848679), job 137394665846, also reports success; different source-stage coverage, so do not pool wall times without control.

**Scope:** All `prime31_trace_f4_pilot` cases are **planted positive S3 relations**, not blind unknown-index target extraction. Reported `degree` is the engine's productive/solving-degree counter, not a certified degree of regularity or a successful full root isolation. Matched `reached` and field-operation counts are source-specific observations. No end-to-end ECDLP speedup, no asymptotic improvement, and no independent PCC sign-off.

## m=2 same-engine original-Rust F4 (one genuine planted S3 target, `--sweep`)
`B` is rational x-coordinate base size; each row below is an exact engine counter and a single same-runner wall-time observation. `ops` = reported field operations, `rows x cols` are **separately recorded maxima**.

| B | Ordered F4 ms / ops / rows x cols | Symmetric degree-B G F4 ms / ops / rows x cols | G/ordered F4 wall |
|---:|---|---|---:|
| 3 | 0.324 / 511 / 13 x 15 | 0.032 / 359 / 10 x 11 | 0.099 |
| 4 | 0.072 / 1,238 / 17 x 21 | 0.043 / 783 / 11 x 15 | 0.597 |
| 6 | 0.129 / 3,722 / 21 x 33 | 0.067 / 1,845 / 17 x 28 | 0.519 |
| 8 | 0.188 / 7,388 / 25 x 45 | 0.100 / 3,363 / 30 x 45 | 0.532 |
| 12 | 0.352 / 18,344 / 33 x 69 | 0.193 / 7,767 / 68 x 91 | 0.548 |
| 16 | 0.541 / 34,132 / 41 x 93 | 0.270 / 13,995 / 122 x 153 | 0.499 |
| 24 | 1.113 / 80,204 / 57 x 141 | 0.775 / 31,923 / 278 x 325 | 0.696 |
| **32** | **1.881 / 145,604 / 73 x 189** | **1.023 / 57,147 / 498 x 561** | **0.544** |

At B=32 the transformed F4 stage has 2.55x fewer field operations and 1.84x lower observed wall time, but **20.25x larger product of reported maximum rows and maximum columns**. The product is a *size-risk proxy*, NOT an actually allocated peak matrix: maxima can occur in different rounds. Before any B=677 inference, measure real allocations/peak RSS and repeated, randomized held-out instances. Input simplification did **not** translate into uniformly smaller matrices.

## Quadratic Newton circuit negative gate
Under the same m=2 planted S3 at B=4, original Rust `--circuit` measured **0.166 ms / 4,482 field ops / 67 rows x 74 columns**, versus degree-B G **0.038 ms / 783 field ops / 11 rows x 15 columns**. This small-cell lift is **4.37x slower by wall**, **5.72x more field operations**; B=3 similarly 0.086 ms versus 0.035 ms. Do not promote the extra-variable quadratic formulation based solely on total polynomial degree 2. It remains a distinct possible route if sparse/non-F4 circuit propagation or scaling reverses the negative result.

## m=4 real S3 chain, B=4, six variables
G+2G+3G+4G=10G planted, signed x checks included in stage builder:
- Ordered: **8.065 ms**, **1,036,961** field ops, **686 x 742** maxima, productive-degree counter 5.
- Symmetric raw H: **6.550 ms**, **2,182,508** field ops, **294 x 326** maxima, counter 4.
- Symmetric reduced G: **6.557 ms**, **2,182,404** field ops, **294 x 326** maxima, counter 4.

Symmetric representation has a smaller row/column envelope but **2.10x *more* field operations** here, and reduced G has no observed wall improvement over symmetric H. This contradicts a blanket claim that degree-B reduction necessarily accelerates the important four-leaf S3 stage. All variants remain tiny planted controls, not a solver.

## Non-table public x-only pair oracle
The new original Rust `prime31_monic_s3_pair_oracle.rs` uses monic quadratic reduction, a univariate eliminant/gcd and Fp factorization (no generic pair-table input). The full source-linked CI run passed the tests, including exact exhaustive **post-solve** pair checks. For B=32 and four test target x-values, `online_ms` was **0.114 / 0.107 / 0.099 / 0.110 ms** with one or zero accepted unordered pairs, and balanced-tree vs Newton polynomial coefficients agreed. The standalone wall numbers **do not** include complete signed point lifting, authentic independent relation-rank recovery or cold many-target costs. A zero gcd root is not yet a certified ECDLP nonrelation until exceptional/group cases are covered.

## Statistical/structural triage against p=2^31-1
If every trial target is uniformly distributed over the prime-order group, an m=2 factor base of B distinct x-coordinates has at most `4*binom(B+1,2)` signed pair images (an intentionally loose bound), hence a uniform-success probability at most `4*binom(B+1,2)/n`. At B=677 this equals **918,012 / 2,147,446,831 = 0.00042748998** (expected at least ~2,339 independent uniform trials per hit). Collecting B independent pair relations would therefore require about **1.58 million trial targets at the optimistic upper success rate**, assuming one usable independent relation per hit. This is **a random-query model lower bound**, not an unconditional algebraic complexity lower bound. It does not apply to algorithms generating adaptively biased targets, individual target quotient stopping or other-arity relation families. It explains why a fast B32 pair-root microbenchmark must not be extrapolated to a rho-beating complete index calculus.

## Prior-art/overlap and next-pass tests
- Existing stronger pair-incumbent `research/radical_pair_image_20260921/RESULTS.md` already established small-prime DRL solver-stage reductions; symmetrization itself appears in Faugère–Huot–Joux–Renault–Vitse 2014. This work does not claim new symmetrization.
- #4924 owns the Newton/monic m2 oracle; #4920 chained S3, #4922 shared elimination, #4901 authentic RHS full generic control; #4692/#4812/#4822 own public-only RR9/F3 extraction. Do not reimplement those concurrently.
- **P0**: Replace planted S3 with a balanced, pre-registered public unknown-index 46-positive/18-negative corpus; run genuine signed verification, measured failures and exact target-specific rank on authentic RHS.
- **P1**: Compare native F4 degree-B G, raw H, ordered, existing radical-DRL, monic pair oracle, and circuit using *peak RSS*, exact engine field counts, and repeated timing. Need B scaling and negative targets on the same ordinary curve.
- **P2**: Qualify only a complete same-host Rust 1/32/128/1024 cold/batch solver against strongest project index calculus, optimized BSGS and negation-map rho, with setup, memory, unsuccessful work, exceptions and linear algebra included.

**Decision: native original-Rust mathematical/F4 stage verified. Faster complete non-generic Prime31 ECDLP solver NOT established.**
