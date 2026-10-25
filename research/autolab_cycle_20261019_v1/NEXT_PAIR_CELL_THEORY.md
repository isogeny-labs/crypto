# Beyond unary compression: an exact pair-fiber obstruction and its limit

Motivation if cube/BDD hard searches remain unresolved: unary rationality contributes only marginal source restrictions. Seek correlations between an auxiliary s=u0 and the two source abscissae x,y. The following is a derived hypothesis/review, not a measured solver result.

For x,y in the degree17 binary prefix, put p=x+y (18 coefficients) and q=xy (35 coefficients). Since2*17<83, these product coefficients do not wrap. The first S3 equation is exactly
q^2+s*q+s^2*p^2+1=0.
For fixed s this is an affine F2-linear system in the53 p/q coefficients. Every true pair gives a solution; the converse still requires exact factor/domain recovery of X^2+p*X+q. Do not treat the relaxation as the pair-image oracle.

A dual certificate lambda:K->F2 with lambda(1)=1 and lambda annihilating all53 coefficient columns certifies that no relaxed p/q solution exists. For any fixed nonzero lambda, the set of s values to which it applies is an affine cell:
lambda(T^(2j))+lambda(s*T^j)=0, j=0..34;
lambda(s^2*T^(2i))=0, i=0..17.
All these conditions are affine-linear in the83 bits of s because squaring is F2-linear. Thus a single evaluated matrix contradiction can in principle certify an entire auxiliary cell.

**Important limit proved before experimentation.** The35 functionals s->lambda(s*T^j), j=0..34, are independent. If a nonzero polynomial w=sum c_j*T^j made their sum identically zero, multiplication by w permutes the field and would make lambda identically zero, a contradiction. T^0..T^34 are independent modulo the degree83 irreducible modulus. Therefore every nonempty single-lambda cell has codimension at least35 and cardinality at most2^48. Covering half the83-bit auxiliary field by such cells needs at least2^34 certificates even before overlaps. Cheap enumeration of a few global dual cells cannot establish tractability. This is a limit of this particular certificate representation, not of generalized rootfinding.

A common certificate for a whole affine cofactor C=s0+V can be sought by checking whether1 lies in the span U of
T^(2j)+s0*T^j;
v*T^j for each basis v of V;
s0^2*T^(2i);
v^2*T^(2i) for each basis v of V.
If1 is outside U, exact Gaussian dual recovery yields a lambda proving every s in C infeasible for the relaxation, hence for true source pairs. If1 belongs to U, this common-certificate test is inconclusive; it does not prove the cofactor has a root. In particular, V containing1 always puts1 in U via v=1,j=0. Such prefix cells can never be excluded by one common dual certificate. Condition that direction first rather than retrying those cells.

Next bounded experiment: native rank/dual profile on fixed cofactor families with bit0 of s fixed and free directions chosen before results, dimensions1,2,4,8; include actual pair-generated feasible s and independent small-field exhaustive controls. Check every returned lambda against every column. On small fields expand every cell and verify no true pair is discarded. On n83 record actual rank, certificate existence, cofactor size and coverage, not sampled probabilities multiplied together. Cap<=90 s,4 GiB,one core; no root or speed claim from ranks. If generic cofactors span1, change directions or integrate certificates into a specific partial-assignment controller rather than enumerate2^83 field values. A later SAT integration must explain conflicts on partial states with sound clauses/XOR reasons; rejecting only completed models gives no benefit because original complete models already satisfy these implied constraints.

This gives a structural next experiment beyond further unary truth-table packing. It uses classical linear duality and the S3 identity; novelty is unestablished. Prime and odd-extension analogues require a different derivation (squaring is not additive in odd characteristic).
