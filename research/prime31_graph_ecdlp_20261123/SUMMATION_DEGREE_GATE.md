# Prime31 summation-polynomial degree and novelty gate

Date: 2026-11-23. Scope: the two ordinary elliptic curves over
`p = 2^31 - 1` already used in this repository. This is a mathematical
screen, not a measured relation-finding improvement.

## Geometric obstruction for the fixed abscissa map

Let `R` be a fixed point with `2R != O`. The exact four-summand locus in
abscissae is the image of the morphism

`(P1,P2,P3) -> (x(P1),x(P2),x(P3),x(R-P1-P2-P3))`.

Because `E^3` is irreducible, its image closure is irreducible. For three
generic input abscissae, each of the eight sign choices for `P1,P2,P3`
gives a distinct fourth abscissa. Indeed, equality of two output
abscissae implies either equality of the signed sums or that their sum is
`2R`. The former holds generically only for identical sign choices; for
opposite choices the latter would require `2R=O`, and all other choices
impose a nongeneric algebraic equation. Thus the exact hypersurface has
degree eight in the fourth abscissa. The specialized Semaev `S5` supplies
that hypersurface and is irreducible up to scalar under this condition.
This argument is an inference from the geometry, not a theorem explicitly
stated for this specialization in the cited papers.

This excludes a lower-degree polynomial in the **same four abscissae**
whose zero set over the algebraic closure is exactly that locus. It does
not exclude selector-restricted equations, auxiliary variables, rational
branch conditions, or a different morphism. A lower input degree or a
lower partial F4 degree is not by itself a lower completed solving degree.
Faugere et al. prove uniqueness of the summation polynomial for a fixed
morphism and classify degree-two maps as translated abscissa maps followed
by a Mobius transform (Propositions 2 and 5, respectively).

Source: Faugere, Huot, Joux, Renault and Vitse,
[Symmetrized Summation Polynomials](https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf).
Kosters and Yeo's [Notes on summation polynomials](https://arxiv.org/abs/1503.08001)
is a further caution against identifying first-fall degree with completed
solving degree.

## Exact equations on a finite selector

Let `S` be a public set of `B` distinct **liftable** abscissae and let
`H(X)=product_(s in S)(X-s)`. For a fixed public target `R`, write
`F_R=S5(X1,X2,X3,X4,x(R))`. Because `H` is split and squarefree, the
selector coordinate ring is

`A = F_p[X1,X2,X3,X4]/(H(X1),H(X2),H(X3),H(X4))
   ~= product_(s in S^4) F_p`.

Suppose a proposed single equation `C_R` accepts **exactly** the same
ordered tuples in `S^4` as `F_R`. In `A`, there is a unit `u` with
`C_R=u*F_R`: at each tuple where `F_R` is nonzero take the ratio
`C_R/F_R`, and at each common zero choose any nonzero unit value.
Conversely, a unit multiple has the same zero set. Consequently
`(C_R,H(X1),...,H(X4))` and `(F_R,H(X1),...,H(X4))` are the **same ideal**
in the polynomial ring. They have the same quotient algebra and reduced
Groebner basis for a fixed monomial order. This is a finite-product-ring
deduction, not a new summation-polynomial construction.

A lower-degree representative `C_R` could still change the work of a
particular F4 presentation; equality of ideals does not make generator
choice free. The candidate must be built from public selector and target
data without first enumerating its accepted tuples, preserve the same
distinct-index and rational-point semantics, and finish an equal-input
cost comparison. The statement applies to the **actual** liftable base;
the B677 algebraic selector's larger off-curve superset needs its own
lifting and rejection account. Auxiliary-variable systems and
non-squarefree selector schemes are outside this unit-equivalence claim.

### B677 split-selector support before target equations

The frozen B677 split circuit uses 1,024 Dickson abscissae, of which 509
lift to rational curve points and 515 do not, plus 168 liftable Y7
abscissae. Before imposing the public target equations, its 2+2
factor split has `C(1024,2)*C(168,2) = 7,347,529,728` unordered
root choices. Exactly `C(509,2)*C(168,2) = 1,813,624,008` of those
choices use four liftable leaves, or 24.683% of the raw choices.
The remaining 75.317% contain at least one off-curve Dickson root.

This is a **support count**, not an F4 branch count or a measured solver
slowdown. The four-leaf anti-Frobenius argument in
`research/prime31_four_summand_20261122/scaling-b677/MATHEMATICS.md`
excludes such roots from genuine relations when the target is nonidentity
and outside the selector and all four abscissae are distinct. Its
repository note remains unsigned; the saved split-circuit executions are
resource-limited before root extraction. The count identifies algebraic
work that the compact selector may have to eliminate, without asserting
that any off-curve choice survives the target equations.

## Curve-specific symmetry screen

The direct factorization of `x^3 + ax + b` over `F_p` is irreducible on
both repository curves:

| Curve | `a` | `b` | `j mod p` | Rational nonidentity 2-torsion |
| --- | ---: | ---: | ---: | --- |
| B677/RR5 | 847514846 | 2087444724 | 2145262698 | absent |
| Registered graph curve | -3 | 21 | 1390704497 | absent |

Both discriminants are nonzero. Neither `j` is zero or 1728. The rational
2-torsion symmetry used in the cited symmetrized construction therefore
does not apply directly to either curve. This says nothing about extension
field torsion or selector-specific structure.

Reproduction: with Python FLINT, factor
`nmod_poly([b,a,0,1], 2147483647)` for each row; compute
`j = 1728*4*a^3/(4*a^3+27*b^2) mod p`.

## Existing presentations and the next falsifier

The balanced S4/S3 eight-leaf presentation and the S4-to-two-S3 auxiliary
chain are already documented in
`research/prime31_ic_20261115/native/BALANCED_S4_M8_INPUT.md` and
`research/prime31_ic_20261115/native/S4_S3_CHAIN_TARGET_DEPENDENT_GATE_20261116.md`.
The latter's exact Rust execution reached degree 10 for the original
single S4 and degree 6 for three chained pairings; all four were
oversize/undetermined. Its conditional eight-leaf arms reached degree 10
versus 5, again with no completed ideals. These are censored partial
degrees, not a completed solve-cost gain. Symmetric-coordinate roots also
have prior repository controls in
`research/prime31_ic_20261115/native/M8_SYMMETRIC_ROOT_RESULT_20261116.md`.
Auxiliary-variable decomposition has published precedent in characteristic
two: Karabina, [Point Decomposition Problem in Binary Elliptic Curves](https://arxiv.org/abs/1504.02347).

A qualifying new lead on this fixed M31 field must identify a genuinely
different exact locus or representation, prove full point and exceptional
branch semantics, then finish a same-input solver comparison. The measured
quantity is completed solving degree **and** total work through useful
independent relations, including failed calls, roots, sign lifting and
rank; the cold single-target cost is compared with the same-target rho
control. For fixed `p`, an asymptotic speed claim needs a separate family
and analysis rather than a fit on this one field.
