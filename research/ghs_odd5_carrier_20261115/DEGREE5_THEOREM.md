# Exact degree-five cover exclusion for the trace +26 witness

15 November 2026. This note sharpens the exact-pair bound in `DEGREE4_THEOREM.md`. The earlier `[5,20]` interval is superseded by **[6,20] for this exact pair over K**. Finite records and the geometric reduction remain [pcc:unverified] until an independent PCC theorem verdict is signed. This is a cover-degree result, not a measured GHS vulnerability.

## Exact pair and theorem

Let `K=F3[a]/(a^5+2a+1)`, and put

- `E/K: v²=u³+2u²+a`, with `#E(K)=218=2·109`;
- `Y/F3: y²=x¹¹+x⁹+2x⁸+x⁷+2x⁵+x³+x²+x+1`, with `L_Y(T)=1−26T⁵+243T¹⁰`.

There is **no K-defined degree-five morphism `Y_K→E`**. The earlier exact-pair certificate excludes degree four, and the geometric certificate excludes degrees one through three even over the algebraic closure. [Howe, Theorem 4.17 and Remark 4.18](https://arxiv.org/pdf/2110.04221) guarantee a K-morphism of degree at most twenty because `26²−4·243=−296` is a fundamental discriminant and `End(E)=Z[π]`. Thus the minimum degree of a K-morphism from this `Y_K` to this `E` lies in **[6,20]**. [pcc:unverified]

For any such cover `φ` in the window, `φ_*φ^*=[deg φ]` and `0<deg φ<109`, so pullback is injective on the selected order-109 subgroup. Its target is `J(Y)(K)`. No rational-function cover or evaluable subgroup map into `J(Y)(F3)` has been produced. [pcc:unverified]

## Complete necessary-condition reduction

Translate the rational Weierstrass point at infinity to the elliptic origin. The hyperelliptic involution then intertwines with elliptic negation, so `u=A(x)/B(x)`, with coprime `A,B∈K[x]`, `deg A=5`, and `deg B=5−e∞` for an odd `e∞∈{1,3,5}`. A finite branch pole has odd denominator multiplicity; every nonbranch pole has even multiplicity. Since `f=(x+1)Q1···Q5` over K, with the `Qi` distinct irreducible quadratics, the rational branch root cannot occur in an even-degree denominator. The complete monic denominator list and assignment counts are:

| `B` shape | Denominators | Galois-compatible assignments |
|---|---:|---:|
| `1` | 1 | 243 |
| `Qi` | 5 | 405 |
| `(x−c)²`, `c∈K\{−1}` | 242 | 58,806 |
| `QiQj`, `i<j` | 10 | 270 |
| `Qi(x−c)²`, `c∈K\{−1}` | 1,210 | 98,010 |
| `H²`, monic quadratic `H` coprime to `f` | 58,801 | 14,288,643 |
| **Total** | **60,269** | **14,446,377** |

Every finite Weierstrass point maps to elliptic 2-torsion. The rational branch point has the unique K-rational finite 2-torsion x-coordinate as image; each unpoled `Qi` pair has three Galois-compatible choices. Let `M=(x+1)` times the product of unpoled `Qi`. For every listed denominator, `deg M≥7`; therefore a degree-five numerator satisfying the branch conditions is unique. The producer forms it as `(B*h) mod M`, where `h` is the CRT residue of the selected 2-torsion coordinates. The independent replay instead solves the direct coefficient equations `A−e_j B≡0` in each quotient ring and checks their left kernels. [pcc:unverified]

For the dominant `H²` shape, the producer tests five high-degree coefficients of `(B*h) mod f` as linear equations in the four lower coefficients of a monic quartic B, then checks whether its unique candidate is a quadratic square. For all consistent rank-deficient assignments it directly enumerates every allowed H. The replay instead builds the eleven-by-ten system in six numerator and four denominator coefficients, with the same direct-enumeration fallback. The producer's rank histogram is 232 rank-four, 10 rank-two, and one rank-zero assignment; the replay's corresponding matrix ranks are 232 rank-ten, 10 rank-eight, and one rank-six. Both test 646,811 fallback H values. [pcc:unverified]

## Exact surviving numerators and full-equation obstruction

Both source-bound methods examined all **14,446,377 logical cases** and retained the **same two** degree-five `(A,B)` pairs, both with poles at factor indices `[1,4]` in the zero-based scripts (`Q2Q5` in one-based notation). Their complete coefficients are retained in `degree5_survivors.magma`, `degree5_screen.json` and `degree5_linear_replay.json`. For either pair, a curve map would require

`f*C² = B*(A³+2A²B+aB³)`

for some `C∈K[x]`: writing the anti-invariant elliptic coordinate as `v=yR(x)`, the pole parity above makes `B²R` regular at every finite point, hence polynomial. Both Sage runs show that `f` divides the right-hand side but the degree-eight quotient is not a square. A separate [public Magma 2.30-1](https://magma.maths.usyd.edu.au/magma/) run on the exact hard-coded pairs confirms this with success, exit code zero and untruncated output. It factors the first quotient into irreducibles of degrees `2+2+4`, each with exponent one, and the second into degrees `1+1+2+4`, each with exponent one. Thus neither survivor satisfies the elliptic equation. The Magma source and response are `degree5_survivors.magma` and `degree5_survivors_magma.json`. [pcc:unverified]

The producer source SHA-256 is `3cf3888f8647eec45de2b2de51d0a7c398d94fe0b87568daae6f772b40d95af0`; the direct-linear replay source SHA-256 is `203b913db59d1649c8b587a1f8705a540ace38c22a0cfe402a2bd15669529440`; the Magma input SHA-256 is `7135f4dbb4b509d25c7a28d1554a0617e05483c8af51a5d03b7bcb1be97f91b7`. The producer's first execution had an output-only Sage-preparser source-binding error; `DEGREE5_AMENDMENT.md` preserves that receipt and the committed correction. The successful corrected producer matches the first result field by field except for the repaired hash. [pcc:unverified]

## Scope, prior work and next falsifier

This excludes a degree-five map for **this exact source curve, this exact elliptic target and the field K**. It does not exclude degree five for another elliptic curve in the isogeny class, another genus-five carrier, a map defined only over a larger field, or a non-cover correspondence. It does not prove a GHS speedup. The general branch-point interpolation method is prior work; for example, [Fan 2022](https://onlinelibrary.wiley.com/doi/10.1155/2022/9833393) uses it for genus-two quadratic-extension covers. [Tian](https://arxiv.org/pdf/2012.07173) already gives isogeny-based transfer for cubic extensions. The repository KB revision `9141e386254340a9bdba3036f2bb706a` predates this result, and targeted exact-equation searches did not establish publication priority. [pcc:unverified]

The next constructive test is a degree-six-to-twenty cover or an explicit `F3`-rational correspondence from the Weil restriction into `J(Y)`, evaluated on the exact order-109 subgroup. A complete relation/decomposition, linear-algebra, descent and verification cost must then beat matched rho before any vulnerability claim. The fifteen checked declarations in `Odd5Ledger.lean` cover finite arithmetic counts and bounds; the geometric denominator theorem and the finite enumeration are not Lean-formalized. [pcc:unverified]
