# EXP-R31-J — four-summand coordinate-only pair-support resultant gate

**Date:** 2026-11-15. **Standing:** producer-local derivation/checks, [pcc:unverified]. **Scope:** a new *implementation experiment* for ordinary prime-field index calculus; no novel asymptotic complexity or complete prime31 ECDLP performance claim. Continue [#4188](https://github.com/isogeny-labs/crypto/pull/4188), reuse its Rust field/curve/S3 elimination primitives in `native/examples/prime31_s3_quotient_gcd.rs`. This is not a replacement for #4228's certified quotient engine.

## Literature and already-searched axes

1. [Petit–Kosters–Messeng, PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1) already formulates prime-field ECDLP through algebraic rational-map/factor-base methods; this study is **not** an invention of prime-field summation polynomial IC.
2. [Amadori–Pintore–Sala, Finite Fields Appl. 51 (2018)](https://doi.org/10.1016/j.ffa.2018.01.009) develops a one-factor-base-relation variant particularly relevant to prime fields: end-to-end protocol comparisons must include it, not just rho.
3. Read existing [two-sum exact support](TWO_SUM_YIELD_GATE.md), [S3 quotient/GCD](S3_QUOTIENT_GCD_ORACLE_20261115.md), [S3 discriminant generic-equivalence correction](S3_DISCRIMINANT_CLASSIFICATION_GATE_20261115.md), and [m8 balance / F4 oversize](BALANCED_S4_M8_INPUT.md). #4275 showed dense factor-base membership can beat the tower on small oracle cells but still lost to enumeration. #4228 is an independent certified quotient/subresultant route. #4077 gave a whole-call m8 cost gate. #3877 chained transfers, cryptanalysis #1536 Rust auxiliary IC, and crypto-autoresearcher #3305 / #3301 lead maps are adjacent but not a completed cheaper prime31 cold IC result.
4. **Research KB receipt:** the `kb_status/search_knowledge` MCP and repo CLI were not available in this connector-only runtime. GitHub's current primary PR/source files plus publisher records were used as the bounded fallback. No exhaustive novelty review is claimed.

## Coordinate-only m=4 algebraic construction

Fix the same ordinary cofactor-one curve, not a deliberately weak group:

```
p = 2147483647, a = 847514846, b = 2087444724
n = 2147446831, G = (625037715, 2100627251)
```

For the registered canonical factor base with distinct abscissae `x_i`, the standard Semaev polynomial is

```
S3(x,y,u) = (x-y)^2 u^2
           - 2[(x+y)(xy+a)+2b]u
           + (xy-a)^2 - 4b(x+y).
```

Define the pair support polynomial, before **any target** is supplied:

```
K(U) = monic( product_{0 <= i <= j < B} S3(x_i,x_j,U) ).
```

For each pair of distinct factor-base points, the quadratic roots are exactly `x(P_i+P_j)` and `x(P_i-P_j)`, with the sign of the point itself forgotten. For `i=j` the polynomial drops to degree one and encodes `x(2P_i)`; the inverse-pair identity has no affine x-coordinate. Therefore the raw degree is `2*C(B,2)+B=B^2`. Repeated roots may occur from collisions, so this is a **multiset/polynomial degree** and not a universal count of distinct sum abscissae. The implementation builds every factor from `(x_i,x_j,a,b)`, never their ordinates or EC sums. It does enumerate *coordinate pairs* and performs naive polynomial multiplication: no subgeneric work bound follows.

For a public affine point `R=(t,y_R)`, a four-summand decomposition with *both intermediate pairs nonidentity* implies two finite roots `u,v` of `K` with `S3(u,v,t)=0`. Test this with the same quotient-ring norm used by the m=2 Rust oracle:

```
q(V) = S3(U,V,t) / (U-t)^2 = V^2 + beta(U)V + gamma(U)
H(V) mod q(V) = r0(U) + r1(U)V           (H = K)
N_t(U) = r0^2 - beta*r0*r1 + gamma*r1^2   (mod K(U))
D_t(U) = gcd(K(U), N_t(U)).
```

The exact membership equivalence for the *finite-pair polynomial system* follows from the resultant identity over a split semisimple algebra (and remains valid as a common-root test even if K has repeats). On the branch `U=t`, the leading coefficient vanishes; divide out the `U-t` factor **before** inverting, check `gcd(K(V),S3(t,V,t))` directly, and restore `U-t` only when that common-root test succeeds. The code does so. It fails closed on a zero/nonmonic support input.

**Completeness caveats (updated after executed Rust extraction):** the current bounded prototype **does** factor `D_t` over `Fp`, recover `u,v`, descend them to original x-pairs and check signed four-point identities against the complete public target point *after* polynomial discovery on B4; see the source-bound execution below. It does **not** establish complete coverage of identity intermediates/canceling pairs, natural-positive useful rank or final prime31 scalar recovery. No ordinary-curve B31 target-dependent m4 stage has completed. It has no subgeneric runtime guarantee: `deg K=B²`, and naive quotient Horner/remainder arithmetic grows at least cubically in that degree.

## Independently executed producer-local arithmetic

A separate Python integer-field implementation reconstructed the existing Dickson depth-6 base (`B=31`), implemented polynomial arithmetic and elliptic-curve group law independently, and recomputed the following. The Rust run is **not** inferred from these Python results.

| Test on the exact prime31 curve | Producer-local result |
|---|---:|
| Restrict canonical B31 to its first four abscissae; `deg K` | 16 |
| Post-hoc distinct pair-sum x values (signed combinations) | 16 |
| `gcd(K,K')` degree for B4 | 0 |
| Four distinct point planted positive `R=P0+P1+P2+P3` | `(1418256770, 748935662)` |
| Positive target `deg D_t` | 6 |
| Uniform-looking **fixed public control** `R=[17]G` | `(269688432, 1935681762)` |
| Fixed public control `deg D_t` | 0 |
| Full Dickson depth-6 base `B=31`; `deg K` | 961 |
| Full B31 `gcd(K,K')` degree | 0 |

These counts mean only that the algebraic **m=4 membership** correctly detects a constructed positive and excludes one unrelated public input on the B4 slice. A public scalar used to generate the test point is never an oracle input. Positive eliminant degree 6 is not six independent relations; multiplicities/orbit symmetries still require normalization.

The Rust implementation exposes `--m4-preview` (B31 support/squarefreeness and B4 positive/public **factor-and-lift** pilot), `--m4-scale` (bounded B4/B8/B16 source-matched positive/public phase panel), and a self-contained `#[test]` that independently checks the candidate polynomial roots and full signed point recovery, using actual EC additions *only after* polynomial discovery. The first source-bound GitHub development execution passed, as frozen below; the broader scaling run's result must be sourced from its own exact workflow SHA. This experiment uses the existing narrow GitHub development workflow, not EKS/charged benchmark infrastructure.

## Explicit choices and next experiments

1. **J1 — achieved on B4: factor and lift a full signed m4 relation.** Bounded deterministic split, `gcd(K(V),S3(u,V,t))`, abscissa-pair recovery and final signed public-point replay are implemented. Six ordered algebraic solutions reduce to **one** canonical signed row for the planted B4 target; `[17]G` produces none. Follow up with repeated-index and identity/cancellation completeness controls, explicit undetermined handling, 16 planted and 16 unrelated frozen public encodings, and a higher-B exact-output comparison. No sum table may feed discovery.
2. **J2 — attack the cost explosion, not merely the representation.** At B4, B8, B16, B31 measure wall/CPU/RSS and base `Fp` multiplication counts for constructing K and computing `D_t`, with exact same inputs using native dense F4, tower F4 and #4228 certified quotient. Treat all caps as censored; no speedup ratio against an unfinished baseline. Prefer half-gcd/subresultant, balanced product trees and fast modular composition if the naive `B^2` eliminant grows too fast. A mature msolve/Singular comparison is mandatory before any fastest-solver claim.
3. **J3 — return to the real m8 challenge.** Use m4/pair support as a certified inner block only if its total natural-query charge is competitive. Feed it into the balanced m8 target-dependent S4 intersection; do **not** substitute a precomputed EC known-sum table, and do not scan the 17,980^2*930 projected triple product. Benchmark extracted natural positives and negatives, normalize signed/projective duplicates, measure *incremental independent rank* and all failed/censored calls.
4. **J4 — final acceptance.** On 12 held-out public prime31 points, recover unknown scalars end-to-end with complete cold setup, relation finding, LA, individual log, final replay and same-host optimized negation-map Rust rho/BSGS. Explicitly separate multi-target amortized results and signed independent PCC replay. If a faster algorithm emerges on this small prime31 curve, repeat over other ordinary prime-order curves and growing p to test whether the advantage is structural rather than accidental.

**Decision:** An actual signed four-summand algebraic relation extractor now runs at B4 on the required ordinary prime31 curve, not just a nonzero eliminant. It is a *correctness/engineering bridge* to #4228, not yet evidence of a cheaper full solver. The next stop/go criterion is whether B8/B16 source-matched costs and natural-positive extraction remain competitive without generic group-sum preprocessing.

## Source-bound executed Rust development evidence (supersedes original source-only wording)

At source commit [`4ecd0327b43c`](https://github.com/isogeny-labs/crypto/commit/4ecd0327b43c7e036fb422b9f9a523e74cd0d5b6), actual [GitHub Actions development run #43746776862](https://github.com/isogeny-labs/crypto/actions/runs/43746776862), job 132911707466, PASSED: 40/40 focused reused-engine library tests, 2/2 focused S3/m4 example tests, and the `--m4-preview` executable. An owned-factor-candidate Rust cleanup at source commit `c23fae9` also passed [CI #43746826972](https://github.com/isogeny-labs/crypto/actions/runs/43746826972). All claims remain **producer-only [pcc:unverified]**, not independent scientific replication.

| Exact prime31 Rust component, identical source-host run | Observed |
|---|---:|
| Construct B31 coordinate-only K, degree 961 and squarefree | 1.543934 ms |
| Compute B4 planted-positive target eliminant, degree 6 | 0.056254 ms |
| Factor, recover and check six ordered algebraic (U,V) candidates | 6 ordered candidate tuples |
| Fully verified signed four-summand relations for planted R | **1 canonical row** |
| Post-extraction EC verifier work (not part of polynomial discovery) | 384 group additions |
| Compute B4 independent public [17]G eliminant, degree 0 | 0.056403 ms |
| Public [17]G four-summand signed rows | 0 |

The B4 quotient stage times **exclude** factorization, candidate reconstruction, final group validation, K construction and cold base construction. The B31 K construction interval **does not** include a B31 m4 target membership query. These single-run stage timings cannot be compared with cold rho. Full frozen machine-readable source and output: [`M4_PAIR_SUPPORT_CI_EVIDENCE_20261115.json`](M4_PAIR_SUPPORT_CI_EVIDENCE_20261115.json).

Next paired predeclared development probe: `--m4-scale` on B4/B8/B16 prefixes of this *same* B31 Dickson factor base, using identical full public positive and [17]G targets, with separately measured support build, target-dependent norm/GCD, algebraic root splitting, signed validation and completed/censored outcomes. Its commit/workflow is distinct; its actual results must be inspected and recorded before interpretation.

## Source-bound B4/B8/B16 scaling: a negative cost signal

The preregistered `--m4-scale` experiment [GitHub Actions 43747237466](https://github.com/isogeny-labs/crypto/actions/runs/43747237466) at source/workflow commit `ebb2c345ae0c00d5ea69e1e433540f4466217305` PASSED in the development workflow. Same existing ordinary prime31 curve and B31 Dickson fibre, sliced deterministically to B4, B8, B16. Planted-positive `P0+P1+P2+P3` has one canonical signed four-point relation for every slice; the fixed unrelated public `[17]G` has zero. Frozen per-target output: [`M4_PAIR_SUPPORT_SCALE_CI_20261115.json`](M4_PAIR_SUPPORT_SCALE_CI_20261115.json). Producer-only development evidence, **[pcc:unverified]**.

| B | degree K | K build, ms | target quotient norm/GCD, planted ms | target quotient norm/GCD, public [17]G ms | canonical positive rows |
|---:|---:|---:|---:|---:|---:|
| 4 | 16 | 0.012 | 0.141 | 0.142 | 1 |
| 8 | 64 | 0.053 | 3.553 | 3.195 | 1 |
| 16 | 256 | 0.234 | 191.769 | 192.243 | 1 |

**Material negative scaling:** doubling B8→B16 makes the quotient norm/GCD about 54× slower despite its still sparse positive output. The naive high-degree polynomial representation is a substantial cost blocker; do not project small-slice success to B31 or claim cold-rho advantage. Not shown: a natural-positive random-target success rate, independent relation-rank growth, ordinary B31 target-dependent quotient, complete scalar recovery, or any paired external CAS/rho benchmark.

**Source-head accounting correction:** in the `ebb2c345` probe, `m4_extract_pairs` called `m4_first_eliminant` a *second* time, so the reported `algebraic_root_extraction_ns` also includes a redundant full quotient norm/GCD. Those two ns fields were not disjoint phases and **must not be added** to quote a baseline one-pass pipeline. Source commit `f1944c706e27d4662047f7e8deab17f1831e976d` replaces that duplicate call with `m4_extract_from_eliminant` and adds a wrapper/parity test; the corrected executable needs its own successful CI and frozen output before a revised performance conclusion. This eliminates redundant computational work, not the `B²` degree and steep observed one-pass norm scaling. Keep the original measurements rather than overwriting them.


## Successful no-duplicate-norm Rust execution

At `f1944c706e27d4662047f7e8deab17f1831e976d`, [development CI #43747735303](https://github.com/isogeny-labs/crypto/actions/runs/43747735303) **passed** the same focused suite, including the Rust wrapper-vs-reused-eliminant test and frozen B4/B8/B16 executable. See [`M4_NO_DUPLICATE_NORM_CI_20261115.json`](M4_NO_DUPLICATE_NORM_CI_20261115.json).

| B | degree K | target norm/GCD positive | algebraic root/pair extraction positive | signed verification positive | target norm/GCD [17]G | signed rows positive / [17]G |
|---:|---:|---:|---:|---:|---:|---:|
| 4 | 16 | 0.057 ms | 0.251 ms | 0.051 ms | 0.055 ms | 1 / 0 |
| 8 | 64 | 1.783 ms | 0.300 ms | 0.050 ms | 1.785 ms | 1 / 0 |
| 16 | 256 | 106.916 ms | 0.572 ms | 0.052 ms | 106.555 ms | 1 / 0 |

The extraction phase now **does not repeat** the already charged norm/GCD. Those three reported phases are disjoint inside one target invocation, but exclude cold base construction and charge pair-support K construction separately (B16 0.136ms); candidate validation includes 384 EC additions in the planted-positive case. These are single-run GitHub Actions timings, not a paired statistical benchmark, independent PCC replay, cold whole-ECDLP solve, or comparison with rho. B8→B16 still makes the expensive norm about 60× slower, so removing the duplicate norm is a useful exact source-level reduction but does not remove the central asymptotic bottleneck. A small-B positive fixture still supplies only one canonical signed row, not an independent-rank recovery.

**Next blocker to attack:** replace degree-`B²` naive quotient Horner/remainders with fast polynomial arithmetic or the #4228 certified quotient route. Preregister correct outputs, natural-positive query yields and complete cost on B4/B8/B16/B31 before changing the algorithm; stop if the faster implementation is not cheaper than dense F4 and optimized matched rho at the relevant useful-rank yield. Handle intermediate identities and repeated indices before calling any m4 oracle complete.
