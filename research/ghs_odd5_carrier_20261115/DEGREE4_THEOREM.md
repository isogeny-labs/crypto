# Exact degree-four cover exclusion for the trace +26 witness

15 November 2026. This is a paper-level theorem with two independently implemented finite checks. Its arithmetic has not received a signed PCC verdict, and its geometric argument has not been formalized in Lean. [pcc:unverified]

## The exact pair and bound

Let `K=F3[a]/(a^5+2a+1)`. Define

`E/K: v²=u³+2u²+a`, with `#E(K)=218=2·109`, and

`Y/F3: y²=x¹¹+x⁹+2x⁸+x⁷+2x⁵+x³+x²+x+1`.

The independently checked Weil numerator of `Y` is `1−26T⁵+243T¹⁰`, so `J(Y)_K` is isogenous to `E^5`. There is **no degree-four morphism `Y_K→E` defined over `K`**. The previous geometric certificate excludes degrees one through three even over the algebraic closure. [Howe's Theorem 4.17 and Remark 4.18](https://arxiv.org/pdf/2110.04221) guarantee some `K`-morphism of degree at most 20, since `26²−4·243=−296` is a fundamental discriminant and `End(E)=Z[π]`. Therefore the minimum degree of a `K`-map `Y_K→E` is rigorously in **[5,20]**. Any map in that interval has injective pullback on the order-109 source subgroup by `φ_*φ^*=[deg φ]`; its target is `J(Y)(K)`, not yet the intended `J(Y)(F3)`. [pcc:unverified]

## Why the finite denominator list is complete

Translate any hypothetical degree-four map so the rational Weierstrass point at infinity goes to `O`. The hyperelliptic involution then corresponds to elliptic negation. Hence `u=x_E∘φ=A(x)/B(x)`, where `A,B∈K[x]` are coprime and `deg(A/B)=4`. Because infinity maps to `O`, `deg A=4` and `deg B=4−e∞`, where `e∞` is the ramification index at infinity. A local parameter changes sign under the two involutions, so `e∞` is odd and is either one or three. At a finite branch root of `f`, a pole has odd multiplicity in `B`; at a nonbranch root, it has even multiplicity. The factorization over `K` is `(x+1)Q1···Q5` with five irreducible quadratics. Thus, up to a nonzero scalar, `B` is exactly one of `(x+1)Qi`, `(x+1)(x−c)²` with `c∈K\{−1}`, `(x+1)³`, or `x+1`. There are `5+242+1+1=249` denominators. [pcc:unverified]

For each quadratic branch pair `Qj` absent from `B`, its two Weierstrass points must map to finite elliptic 2-torsion. The cubic `U³+2U²+a` has one `K`-rational root and a conjugate quadratic pair, giving exactly three Galois-compatible values for that source pair. Imposing `A≡B·ej (mod Qj)` gives eight or ten linear coefficient conditions. There are `5·3⁴+244·3⁵=59,697` denominator/assignment cases. Every degree-four map must yield a degree-four polynomial `A` satisfying one of those systems. [pcc:unverified]

## Exact certificate and independent replay

The frozen producer `degree4_all.sage` (SHA-256 `24344a31f67e7d98fc25d42820046bd86b0add3f7023a9f8d77058d0d210acd0`) reconstructs `A` by CRT and returns zero degree-four candidates in every denominator shape. The independent `degree4_linear_replay.sage` (SHA-256 `beea4fc7a10aa1fb04031ead87edf82eb90c09cc5c6451567e0a8ea02c9e94ed`) constructs the coefficient matrix directly, tests its left-kernel equations and solves all consistent systems without calling CRT or reading the producer receipt. Its 59,697-case replay found 249 systems consistent with a numerator of degree at most four, exactly one per denominator; every resulting numerator has degree below four. Its degree-four count is zero in all four shapes. The retained JSON receipts are `degree4_all.json` and `degree4_linear_replay.json`. The earlier five-denominator `degree4_screen.json` and its failed output-only first attempt remain documented in `DEGREE4_AMENDMENT.md`. [pcc:unverified]

| Denominator shape | Assignments | Linear systems consistent with degree ≤4 | Degree-four numerators |
|---|---:|---:|---:|
| `(x+1)Qi` | 405 | 5 | 0 |
| `(x+1)(x−c)²` | 58,806 | 242 | 0 |
| `(x+1)³` | 243 | 1 | 0 |
| `x+1` | 243 | 1 | 0 |

The zero numerator count is a necessary-condition exclusion; the full elliptic equation need not be tested when no degree-four numerator survives. The result is specific to this exact source curve, target elliptic curve and base field. It does not classify other genus-five carriers or other isogenous elliptic targets, and it does not furnish a map in degrees 5–20 or a measured GHS attack. Searches of the repository KB (revision `9141e386254340a9bdba3036f2bb706a`, last refreshed 14 November), the existing odd5 protocols, and [Howe's published theorem](https://arxiv.org/pdf/2110.04221) found no exact `+26` degree-four result; that bounded search does not establish global priority. [pcc:unverified]
