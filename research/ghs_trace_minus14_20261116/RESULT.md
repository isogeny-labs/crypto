# Trace -14 fixed-pair cover-degree result

15 November 2026, local run. This result is computer assisted and has one Sage elimination implementation per degree plus a structural receipt audit. The search algorithms were adapted from the previously frozen trace +26 and +22 screens; an independent algebra-system replay of these new trace -14 cells remains open. [pcc:unverified]

## Exact curves and scope

Let `K=F3[a]/(a^5+2a+1)`. The source and carrier are

* `E/K: v²=u³+u²+(a²+2)`, with `#E(K)=258=2·3·43`, trace `-14`;
* `Y/F3: y²=x^11+x^8+x^7+x^6+x^5+x²+2x`, genus five, with `L_Y(T)=1+14T^5+243T^10`.

The carrier branch polynomial factors over `K` as `x Q_1⋯Q_5` with five distinct irreducible quadratics. The elliptic cubic has one `K`-root and one irreducible quadratic. The finite screen concerns **this exact pair** and morphisms defined over `K`. It does not classify every source model of trace -14, every genus-five carrier in the isogeny class, or arbitrary Jacobian correspondences.

## Degree-four through degree-seven search

Translate a proposed morphism so infinity maps to `O`. For degree `d`, write `u=A/B` with `deg A=d` and `deg B=d-e∞`, where odd `e∞` is the ramification index at infinity. Write `v=yC/B²`; the exact identity is

`f C² = B(A³+A²B+(a²+2)B³)`.

Coprimality and valuation parity at the factors of `f` force the finite pole classes. Every unpoled branch factor is assigned all Galois-compatible elliptic two-torsion residues. CRT reconstructs every allowed numerator residue; remaining coefficients or denominator parameters are enumerated by exact linear algebra, resultants, or finite-field loops. Every surviving candidate is checked against the full identity and squarehood. The degree-seven parity partition and its ten `(number of quadratic branch poles, square-part degree)` pairs are explained in the earlier [fixed-carrier theorem](../ghs_degree7_remaining_20261116/DEGREE_SEVEN_FIXED_CARRIER_THEOREM.md); its geometric argument transfers because this carrier has the same branch-factor pattern.

| Degree | Receipt arms and coverage | Exact maps |
|---:|---|---:|
| 4 | all four denominator types; 59,697 assignments | 0 |
| 5 | all six denominator types; 14,446,377 logical assignments; two numerator interpolation survivors rejected by full equation | 0 |
| 6 | all ten denominator types; 14,506,074 branch assignments, including exact lambda enumeration when the CRT modulus has degree six | 0 |
| 7 | all ten `(s,h)` shapes, partitioned among sparse, linear-square, quadratic-square, cubic-square, two-poles-linear, and three-poles receipts; cubic systems 243/243, zero censored; largest exact bitset arm represents 643,043,610 numerator candidates | 0 |

The nine producer outputs, each bound to its source SHA256, are checked by [`audit.py`](audit.py); [`AUDIT.json`](AUDIT.json) records every structural counter and zero-hit field. The source files and raw `.json`/`.stderr` receipts remain alongside this note. The audit is **not** a second algebraic implementation. Lean checks the arithmetic denominator partition, group factorization, candidate counts, and degree ceiling in [`TraceMinus14Arithmetic.lean`](TraceMinus14Arithmetic.lean), with `lean` exit zero. Lean does **not** formalize the geometric parity argument, CRT completeness, or Sage elimination.

The prior [geometric degree-three exclusion](../ghs_odd5_carrier_20261115/THEOREM.md) covers all 84 recorded genus-five carrier models over an algebraic closure, including this `Y`, and was separately replayed with Magma. Combining it with the new fixed-pair receipts excludes degrees one through seven. [Howe, Theorem 4.17 and Remark 4.18](https://arxiv.org/pdf/2110.04221) applies to this ordinary isogeny class: `14²−4·243=−776` is fundamental, and `5669·776^5 < 4^5·34^10`. It supplies some `K`-morphism `Y_K→E` of integer degree at most 33. Thus the **computer-assisted fixed-pair degree window is `[8,33]`**. The possible degrees in the interval have not been constructed or individually ruled out.

Because the selected source subgroup has prime order 43, every map supplied by this ceiling has degree below 43. Push-pull `φ_*φ^*=[deg φ]` therefore makes `φ^*` injective on `E(K)[43]`. This is an **abstract subgroup-preserving map existence result** for the exact pair, with no coordinate formula to evaluate the map. No decomposition run or complete GHS-versus-matched-rho cost is available for this trace -14 pair.

## Prior work and novelty

[Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf) classifies the direct odd-characteristic `(2,…,2)` descent construction for prime extension degrees through seven. Its degree-five genus-five full-rational-two-torsion case does not contain this order-258 source, whose group order is 2 modulo 4. [Howe 2022](https://arxiv.org/pdf/2110.04221) supplies the cover-existence and degree ceiling mechanism used here. The trace -14 Jacobian isogeny class was already recorded as `has_jacobian=1` in the earlier LMFDB audit, and the local eight-trace genus-five carrier family already included this curve. This continuation adds a **fixed-pair degree obstruction**, not a new eligible family or a new attack method. A targeted literature search did not identify the exact equations, but that does not settle global priority. A practical vulnerability claim requires an evaluable subgroup map and a complete same-target attack cost below matched rho.
