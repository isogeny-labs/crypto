# Exact fixed-cell paired-factor norm branch

This is a conditional norm-function algorithm, not original S3 root recovery,
global enumeration removal, novelty or a speed comparison. The already frozen
quartic_cell worker supplies the exact literal norm and coefficient formulas.
For fixed h,k,z,y,A,B, its monic H is globally F2-affine in raw r,s: with
deltaF=r(X+z)^2+s(X+z), deltaH=r²(X+z)^3+rXG(X+z)+s²(X+z)+sXG.
The target equation and fixed monic divisor X+z are preserved by that identity.

Write b=e1=h²+h+z and restrict H=(X+b)(X²+sigma X+tau)². This is equivalent
to e3=b e2 and e5=b e4. Their coefficient maps are F2-linear because b is
fixed, and the two scalar field equalities give 2n binary equations in 2n raw
parameter bits. Row-combination certificates and a complete particular/kernel
basis establish every parameter solution, including the case of inconsistency.

The source-support premise is b in V and z outside V. Thus h is neither zero
nor one. Put p=h²+h, C2=k+h²A+zb. From e3=b e2,

    s=(kr+k²+p(r²+hr+C2))/h.

Its r² coefficient is p/h=h+1. The equation e5+b e4=0 has leading s²
coefficient z+b=p. Substitution therefore yields an affine linearized polynomial
in r of degree four with nonzero leading coefficient p(h+1)²=h(h+1)³.
It has at most four field roots; each fixes s uniquely. Consequently a
consistent binary parameter system has nullity at most two. This is a proved
cardinality bound, not a generic-rank assumption; the implementation checks it
before materializing its at-most-four parameter candidates.

For each candidate, literal norm replay and both coefficient identities are
required. The unique square roots sigma=sqrt(e2),tau=sqrt(e4) define Q. If sigma
is nonzero, Q roots are sigma times the complete solutions of
u²+u=tau/sigma². Binary Gaussian elimination gives the full solution set in odd
and even extensions; it never incorrectly uses odd half-trace in even degree.
If sigma is zero, the unique Q root sqrt(tau) has multiplicity two. Support is
exactly b in V and all Q roots in V. An irreducible Q is rejected. The multiset
is (b,a,a,c,c), or (b,a,a,a,a) in the inseparable case. Every distinct ordered
permutation is retained, including all-zero and coincident-root cases.

The worker exhaustively compares complete supported parameter sets against
literal norms for selected tiny on-curve targets, every h and k=0,1, and every
raw r,s. Whole-field quadratic evaluation is independent of the candidate's
linearized solver. Constructed positive factors check all source orderings even
if a selected norm cell is empty. Actual cells are the previously chosen h and
k=0,1; neither is replaced after seeing its result. The actual k=1 literal
constant and all166 coefficient columns are replayed; symbolic affinity is the
global proof, while the replay is an independent audit.

This source covers only a repeated-factor stratum. ALL its source multisets
overlap previously covered affine-source-rank-at-most-two patterns: the centroid
of (b,a,a,c,c) is b, and the translated values are 0,a+b,c+b. Their F2 span has
dimension at most two. Raw linear span before centroid translation must not be
confused with this rank definition. Thus this is a new conditional norm-parameter
representation/crosscheck, not new rank-three original-source coverage. Rational
ordinates, nonrational lifts, original auxiliary variables, torsion and every
global h,k branch require separate complete recovery. Norm-parameter and source
tuple counts are not original all-roots counts. Final evidence is serialized
before completion, demoted if late, and supervised by the90s runner with4GiB
per-process address space and one core. Late or incomplete final reports clear
all nested accepted completion flags and candidate/support counts; verified
candidate arrays may remain only as lower-bound evidence. Per-process RSS is not aggregate tree
memory. No performance gain is asserted by this controls/actual-cell component.
