# Exact factorization of the W1 genus-fourteen descent Jacobian

16 November 2026. This checks the specific published-family cover `C/F2` returned by `WeilDescent(E,F2,z^94)` for

`E/F128: y²+xy=x³+x²+z³`, with `z⁷+z+1=0`, trace `−9` and order `138=6·23`.

The [protocol](PROTOCOL.md) froze the predicted factorization before the new Magma call. [Magma 2.30-1](factor_receipt.json) computed the **complete** genus-fourteen L-polynomial

`L_C(T)=1+175T¹⁴+16384T²⁸=(1+9T⁷+128T¹⁴)(1−9T⁷+128T¹⁴)`.

The two degree-fourteen factors are respectively the Frobenius polynomials of the W1 source Weil restriction `A_- = Res_(F128/F2)(E)` and its W0 constant twist `A_+`. They are also the L-polynomials of the separate hyperelliptic genus-seven carriers `Y_-` and `Y_+` recorded in `../ghs_w1_twist_bridge_20261116/RESULT.md`. Thus `J(C) ~ A_- × A_+ ~ J(Y_-) × J(Y_+)` **up to F2-isogeny**. The prior [class-group receipt](../ghs_n7_explicit_map_20261115/map14_class_finite_receipt.json) gave `#J(C)(F2)=16560=138·120`, and evaluated an exact order-23 source point into an exact order-23 class of `J(C)(F2)`. The new polynomial resolves the seven-dimensional factors; the earlier class number alone could not.

Because `#A_+(F2)=120` is coprime to 23, the order-23 image belongs to the `A_-` isogeny factor. On 23-torsion, write `U=Frobenius⁷`. The factor polynomials modulo 23 are `U²+9U+128` and `U²−9U+128`, with roots `{1,13}` and `{10,22}` respectively. The explicit idempotent `e_-(F)=6F²¹+6F⁷+12 (mod 23)` takes value one on the first roots and zero on the second. [`audit.py`](audit.py) checks the receipt hashes, sparse polynomial multiplication, factor orders, and this finite projector in independent Python arithmetic. [`PrymFactorArithmetic.lean`](PrymFactorArithmetic.lean) passed the repository proof gate with six theorems and zero trusted exceptions for the coefficient and finite-projector arithmetic. The Lean file does **not** formalize the zeta computation, Tate isogeny or cover geometry. [pcc:unverified]

The source image lies in the seven-dimensional Prym factor in Hess's description. [Hess 2004, Theorem 15 and Section 8.3](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf) already gives the general factorization framework and proves that the norm through the natural intermediate-field quotient kills the source image. The polynomial projector is evaluable *inside* the genus-fourteen Jacobian when Frobenius action is available; it is **not** a coordinate isogeny from its Prym to `J(Y_-)`. The separate hyperelliptic carrier is only identified up to isogeny. That is the remaining map-construction gate before a genus-seven relation/decomposition and complete same-target GHS-versus-rho run. [Magma's L-polynomial API](https://magma.maths.usyd.edu.au/magma/handbook/text/1471) computes the polynomial of the projective normalization of a plane curve.

## Novelty and cost status

This is a new **exact decomposition certificate for this recorded curve** in the project, derived from Hess's published method. The W1 source family and its genus-fourteen cover are previously known, so the result is neither a newly eligible family nor a new attack method. The exact polynomial does not establish that the genus-seven carrier is reachable by an evaluated subgroup map. It supplies no new full attack cost and no new vulnerability verdict. Focused exact-polynomial web searches found no indexed occurrence, but a missing search hit is not a publication-priority claim. The existing full-cost result for a different trace-minus-26 known cover orbit remains separate.
