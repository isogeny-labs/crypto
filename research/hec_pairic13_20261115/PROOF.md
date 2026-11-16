# Exactness of pair lines and large-prime reconstruction

For a polynomial f of degree<=4 overFp, gcd(f,Y^p-Y) is its squarefree product
of rational linear factors. Splitting that product enumerates every rational
root, including roots of f whose multiplicity is greater than1. Restricting
Q1 to each affine x and adding its infinity roots gives the complete point
list without a quadratic grid scan. X^4coefficient1 excludes(1:0:0).

The line through two distinct affine FBpoints has a polynomial restriction of
degree<=4. Dividing by their two known linear factors leaves degree<=2. A
residualdegree2 has no infinity intersection; degree1 has one; degree0 would
have a repeated infinity intersection and is excluded by transversality. After
root and distinctness checks, four points exhaust the canonical intersection
by Bezout. Since two of them were chosen in the smallbase, at most two are
outside. The reference hyperplane's four points are inside the smallbase,
so subtracting it introduces no additional large primes.

Each sparse row is a true principal-divisor relation, with point0anchored.
Elimination is ordinary invertible row scaling/subtraction moduloell, using
minimum-weight LP pivots. The trace retains each normalized pivot row without
changing numerical operations. Reversing it computes a removed variable only
when all dependencies are known. Free/unresolved LPvariables remain None;
unknown values are never interpreted as zero or used in descent.

There are47anchored FBvariables. A rank46kernel is unique up to scale. Its
true log restriction is nonzero:48distinct Abel point classes cannot all lie
in the rational2-primary subgroup of size2 in this order2ell Jacobian.
Consequently every reconstructed known coordinate equals the same scalar
multiple of the true point-log functional. In this execution all81LPvariables
are recovered, but this is an observed property of the fixed relations, not
an assumption for arbitrary inputs. The independent control matrix is used
only after computing the reduced solution, to check this proportionality.

Rational descent subtracts the fixed degree3base and the known translation
contribution, as in mappedic13. The unknown factor-base scale and common
isogeny sign cancel in log(target)/log(generator); final scalar multiplication
on both groups checks the result. Full-reference computation, root-list
comparison and two eliminator copies are controls outside the proposed fast
path. No complexity or complete-cost comparison is inferred from their run.
Diem's rigorous staged construction has extra enlargement/depth hypotheses
not implemented by this fixed48point cohort; see PROTOCOL.md.
