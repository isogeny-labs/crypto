# Selector-target zero row and exceptional right degrees

Date: 2026-11-23 UTC. Separate-role mathematical review for PR #4952. These are written algebraic derivations, not a Lean proof or signed execution claim. No native program was run to obtain them. [pcc:unverified]

## 1. Exact zero-row lemma

Let the working field have characteristic greater than three. Let `V` be the `d` distinct roots of `L(x)=x^d-1` in that field. Define

    N(x,t) = product_(z in V) S3(x,z,t),
    H(x,t) = N(x,t) mod L(x),
    P_V(t) = product_(z in V) S3(t,z,X).

Assume the target coordinate `X` belongs to `V`. Then

    H(X,t) = P_V(t)

as an exact polynomial identity in `t`. Indeed, `N-H` is divisible by `L(x)`, which vanishes at `X`. Symmetry of `S3` then gives

    H(X,t) = N(X,t)
           = product_(z in V) S3(X,z,t)
           = product_(z in V) S3(t,z,X)
           = P_V(t).

Let `F(t)` be any nonzero divisor of `P_V(t)`, made monic using its actual leading coefficient, and write `e=deg F`. Reduction in `t` commutes with evaluation at `x=X` because the divisor is independent of `x`. Consequently

    (H mod F)(X,t) = P_V(t) mod F(t) = 0.

For `e>0`, write the remainder as

    H mod F = sum_(i=0)^(d-1) sum_(j=0)^(e-1) C_ij x^i t^j.

The nonzero row vector `(1,X,...,X^(d-1))` annihilates `C`, so

    rank C <= min(d-1,e).

This coefficient argument does not require `F` to split or be squarefree. It does require a nonzero divisor; a zero polynomial has no monic remainder chart. If `F=1`, the remainder is zero and the rank is zero by the empty-quotient convention.

### Rational-root quotient

For `G_X = gcd(P_V,t^p-t)` made monic, `G_X` is split and squarefree. Evaluating the canonical remainder on `V` and on the `e=deg G_X` distinct roots of `G_X` gives a matrix with a zero row at `x=X`. The two full Vandermonde changes of basis are invertible, so its rank equals coefficient rank and obeys the same bound.

If the left grid is restricted to `B` liftable selector roots and includes `X`, that restricted evaluation matrix still has a zero row and has rank at most `min(B-1,e)`. This is a statement about the restricted evaluation matrix, not about deleting arbitrary coefficient rows of the original matrix.

For `e>0`, the Boolean zero-indicator matrix has an all-one row at `x=X`; its support therefore has at least `e` entries. The zero-indicator rank is a different quantity and is not bounded by the displayed coefficient-rank argument.

This explains an expected loss of rank for ordinary selector targets in the frozen protocol. Such a cell is not a failure of arithmetic qualification or a counterexample to generic rank with an unrestricted target parameter.

### Fixed-subset quotient

For a subset `W` of `V`, put `P_W = product_(z in W) S3(t,z,X)`. It divides `P_V`. Whenever it is nonzero, reduction modulo its monic form also has the same zero row. This holds even if `X` belongs to `V` but not to `W`, so the standard degree-`2|W|` normalization chart is valid.

The generic fixed-subset theorem treats `X` as an independent parameter. Restricting that parameter to the selector roots is a proper exceptional locus and can force the displayed rank loss without contradicting the generic theorem.

## 2. Actual degree and normalization

As a polynomial in `t`, the leading coefficient of `S3(t,z,X)` is `(z-X)^2`. Thus the raw product `P_W` has degree `2|W|` and leading coefficient `product_(z in W)(z-X)^2` only on the chart `X` outside `W`. On that chart the monic product is obtained by dividing by this nonzero scalar.

At `z=X`, direct substitution gives

    S3(t,X,X) = -4 f(X) t + (X^2-A)^2 - 8BX,
    f(u) = u^3+Au+B.

For a rational target with nonzero ordinate, `f(X)` is nonzero and this factor is linear. A full-selector target then has `deg P_V = 2d-1`, and a subset containing `X` has degree `2|W|-1`.

For a nonsingular 2-torsion target, `f(X)=0` and `f'(X)=3X^2+A` is nonzero. The diagonal factor is the constant `f'(X)^2`. A full-selector target then has degree `2d-2`, and a subset containing `X` has degree `2|W|-2`.

These statements use nonsingularity in the 2-torsion case. At a singular point with both `f(X)=0` and `f'(X)=0`, the diagonal factor can be the zero polynomial, so ordinary monic division must not be silently applied. The frozen cusp witness has `X=0`, outside `mu_d`, and avoids this defect.

## 3. Rational squarefree degree at 2-torsion

Suppose `f(X)=0`, `f'(X)!=0`, and `z!=X`. Direct substitution gives the exact square

    S3(t,z,X) = (z-X)^2 * (t-X-f'(X)/(z-X))^2.

Every such factor has the single rational root

    t_z = X + f'(X)/(z-X).

The fractional linear map `z -> t_z` is injective on the selector roots other than `X`, since its nonzero numerator is fixed. Therefore the rational squarefree quotient has exactly

    deg G_X = d       if X is outside V,
    deg G_X = d-1     if X belongs to V.

For `d=1` and `X` in `V`, the latter gives `G_X=1`, as expected. These controls can include roots associated with nonliftable selector coordinates. They should not inherit an ordinary non-2-torsion rational-lifting interpretation.

## 4. Custody and native-check scope

The zero-row lemma and 2-torsion observation were initially appended to `PRIOR_SCOPE_REVIEW.md` just before the parent informed this reviewer that its earlier bytes had been pinned as an unsigned claim subject. No signed verdict or publication of the changed file occurred. The additions were moved here and the prior review restored to its original SHA-256 `642c0dfa9e4ccba858c74706d55c161d30913d2a0a95769436e2fb34fe6bd413`. The separately pinned `THEORY.md` was not edited.

The producer was notified before its source freeze and reports adding zero-row and full/liftable rank-bound assertions within the protocol's existing selector-target cells. This report is not an observed test outcome. Native source review and frozen replay are recorded separately when available.
