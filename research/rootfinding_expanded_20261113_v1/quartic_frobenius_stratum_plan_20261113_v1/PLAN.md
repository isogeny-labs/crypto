# Quartic Frobenius stratum: an exact affine support slice

UNEXECUTED next hypothesis. No measured source, input, manifest, review or raw evidence is modified. This is one coefficient stratum of a binary norm search, not a compression of the whole73-parameter family. Frobenius linearity and Gaussian elimination are classical; no novelty or speed claim is made.

## Exact support criterion

Work in the identified K=GF(2^83) with polynomial basis alpha and modulus X^83+X^45+X²+X+1. Let V=span_F2(1,alpha,...,alpha^17). Restrict to monic quintics

    H_c(X)=X^5+cX.

The inverse fourth-power Frobenius is a=c^(2^81), so H_c=X(X+a)^4. Consequently H_c has all algebraic roots in V, including repeated roots, iff a belongs to V, equivalently c belongs to C={v^4:v in V}. When c=0 this includes the multiplicity-five zero root. When c is nonzero the ordered source multiset has one zero and four equal copies of a; it has five distinct orderings. These are source-coordinate tuples only, not original curve/auxiliary lifts.

For every v in V, its unreduced fourth power has degree at most68, below83, with exponents only0,4,8,...,68. Therefore

    C=span_F2(1,alpha^4,alpha^8,...,alpha^68)

has EXACT dimension18: the listed canonical polynomial-basis columns are distinct and have no reduction/wrap. Inside the69-dimensional degree-at-most68 product-span relaxation this is exactly51 additional zero-coordinate conditions. In the full83-dimensional coefficient space it imposes65 zero coordinates;14of those are already the high-degree restrictions. The phrase51constraints applies ONLY to this restricted stratum and ambient span, not to51independent constraints on the actual global73-bit norm family.

The supplied c=alpha witness passes the degree-at-most68 coefficient bound but fails C membership, since exponent1 is not divisible by4. Its odd trace syndromes vanish due to four repeated nonzero eigenvalues, while the exact support test rejects it. No assertion is made that this witness lies in the actual norm affine family.

## Conditional affine intersection without2^73 enumeration

Suppose an ACTUALLY ADMITTED fixed(d1,d0) norm cell has an affine coefficient map H(t)=H0+sum_j t_j H_j over F2, with at most73binary parameters. First impose e1=e2=e3=e5=0 so it lies in this stratum; then require e4 in C by the explicit coordinate-zero equations. These are linear equations in the SAME global t variables, so one exact Gaussian elimination either proves the stratum empty for that cell or supplies its complete affine parameter solution space. Do not choose different parameter assignments for different coefficient projections.

The field affine-map premise must be proved against the literal norm expression, using its fixed-d Frobenius-linear structure and complete symbolic admission. Matching constants and basis samples alone does not prove absence of nonlinear terms. The current note provides no actual cell matrix or rank. It does not predict that the intersection is nonempty or lower-dimensional.

If inconsistent, retain a row-combination contradiction certificate and conclude only that this particular stratum/cell is empty. If consistent, retain pivot/rank/nullspace witnesses and the full affine parameter family. A parameter-space count2^k is not an original-root count, especially if the coefficient map has a kernel. Recover a by inverse Frobenius, then retain all ordered source tuples and every original function/rationality/exceptional/auxiliary lift through a separately proved admission/recovery interface. A zero source may be a ramified/torsion chart and cannot be discarded. Rational point ordinates for repeated abscissas may choose different sign branches; do not replace them all with one point.

The eight polynomial lifts also simplify only within this supported stratum: actual source polynomial representatives are zero and the unique canonical a(T), so the literal constant coefficient is zero and the supported product uses q=0. This does not eliminate the eight shared-q obligation outside the stratum.

## Concrete next bounded experiment

Freeze fresh source IDs, actual field representation, target/cell matrix, literal-affine-map admission, parameter ordering and all hashes. Independently exhaust small binary fields and small source subspaces, comparing C-membership with every original ordered supported source multiset, including c=0 and the odd-trace counterexamples. Compare the affine intersection against direct enumeration only in tiny cells, checking inconsistent row-combination proofs and complete consistent solution sets.

Then admit ONE actual fixed(d1,d0) cell and compute the exact rank/intersection without enumerating2^73states. Preregister at most100parameter columns and500binary equations,90seconds,4GiB-per-process and onecore. All guards or affine-admission failures return unknown. Do not replace the cell after seeing rank. Preserve every source/output map, elimination certificate and original recovery obligation. An empty stratum is a scoped branch closure, never a global GF(2^83) UNSAT result or a qualified genuine F4/F5/F6 gain.

If the branch is inconsistent, the next question is whether the obstruction admits a proved broader repeated-factor stratum cover. If its residual dimension remains large, examine coefficient-map kernel/rank before adding selectors. Neither outcome justifies an identical retry with more time.
