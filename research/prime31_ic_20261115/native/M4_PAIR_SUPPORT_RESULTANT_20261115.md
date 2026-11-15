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

**Completeness caveats:** this is only an eliminant/membership stage. The current code does not factor `D_t` to retrieve `u,v`, descend them to signed factor-base pairs, check the full `y_R` sign, accumulate independent rank or recover the scalar. The affine pair-support construction omits identity intermediates, which need a separately defined zero-pair/cancellation treatment for a truly complete four-summand oracle. It has no subgeneric runtime guarantee: `deg K=B^2`, and naive quotient Horner/remainder arithmetic grows at least cubically in that degree in the implemented approach.

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

The Rust implementation exposes `--m4-preview` (B31 support/squarefreeness and B4 positive/public pilot) and a self-contained `#[test]` that independently checks the candidate polynomial's roots against actual signed EC additions *only after construction*. Rust source CI is pending at authoring time; archive its actual run SHA and outputs before claiming that execution. This experiment runs in the existing narrow GitHub development workflow, not EKS/charged benchmark infrastructure.

## Explicit choices and next experiments

1. **J1 — extend the m4 eliminant to extraction on a B4/6 slice.** Factor `D_t` after checking the squarefree conditions, recover `u,v` by univariate `gcd(K(V),S3(u,V,t))`, then recover the original `(i,j)` factor pairs using `S3(x_i,x_j,u)=0` and validate **the entire signed public point**, including repeated indices and zero-pair exceptions. Compare exactly with a test-only four-point EC sum enumerator. No table/sum list may enter the algebraic discovery. Freeze 16 planted and 16 unrelated target encodings before tuning. Measure all misses.
2. **J2 — attack the cost explosion, not merely the representation.** At B4, B8, B16, B31 measure wall/CPU/RSS and base `Fp` multiplication counts for constructing K and computing `D_t`, with exact same inputs using native dense F4, tower F4 and #4228 certified quotient. Treat all caps as censored; no speedup ratio against an unfinished baseline. Prefer half-gcd/subresultant, balanced product trees and fast modular composition if the naive `B^2` eliminant grows too fast. A mature msolve/Singular comparison is mandatory before any fastest-solver claim.
3. **J3 — return to the real m8 challenge.** Use m4/pair support as a certified inner block only if its total natural-query charge is competitive. Feed it into the balanced m8 target-dependent S4 intersection; do **not** substitute a precomputed EC known-sum table, and do not scan the 17,980^2*930 projected triple product. Benchmark extracted natural positives and negatives, normalize signed/projective duplicates, measure *incremental independent rank* and all failed/censored calls.
4. **J4 — final acceptance.** On 12 held-out public prime31 points, recover unknown scalars end-to-end with complete cold setup, relation finding, LA, individual log, final replay and same-host optimized negation-map Rust rho/BSGS. Explicitly separate multi-target amortized results and signed independent PCC replay. If a faster algorithm emerges on this small prime31 curve, repeat over other ordinary prime-order curves and growing p to test whether the advantage is structural rather than accidental.

**Decision:** The low-degree finite m4 eliminant is a promising *correctness/engineering bridge* to #4228, not yet evidence of a cheaper solver. The immediate stop/go criterion is whether the same Rust build can factor and lift natural m4 positives at B8/B16 without generic pair-sum enumeration and at bounded total cost.