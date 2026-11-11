# Prospective degree7 construction and genuine Jacobian continuation

This is an acceptance plan,not an accepted map or speed result.
[pcc:unverified]

## CT19: reuse existing elliptic engines first

Use p101,original E1:y²=X³+X+46,order109,and the second shifted quotient
E2:y²=x³+13x²+33x+5,order113. Translate E2 to short Weierstrass using
X=x+13/3;derive coefficients in Rust. E1 coordinate bridge to the accepted
cover is U=X−2. Fixed degree7 is mathematically motivated by CT18,not a
prime search. No benchmark or solver run.

Read existing isogeny_algos/src/find/divpoly.rs,kernel/kohel.rs,curve.rs,
poly.rs,fpr.rs,genus2.rs and src/cryptanalysis/isogeny_walk/kernel.rs.
Existing engines already provide division polynomials,factorization,Kohel
rational maps,checked Fp3 fields and compatible two-torsion gluing. Reuse
them with immutable source custody;do not reimplement kernel arithmetic.
Do not accept an old four-random-x kernel heuristic as the final verifier.

For each elliptic factor,construct every degree3 kernel polynomial at7
with the existing factorization engine. Require degree/squarefree/no source
branch intersection,and coprime map numerator/denominator. For Kohel
x-map N/D,put S=N'D−ND';check the exact polynomial identity

    f_source*S² = D*(N³+A_target*N*D²+B_target*D³).

Degree7 nonconstant maps extending infinity to infinity yield separable
elliptic isogenies in characteristic101;that geometric bridge stays written.
Retain all kernels,codomains,equivalences and failures. Freeze a small TSV
with kernels,map coefficients and actual model/order/coordinate metadata.

Construct each dual from the codomain's7-kernels. The normalized dual raw
codomain should scale back to the source by u=1/7,x→u²x,y→u³y. Require
actual coefficient matching and exact rational-x composition=[7] in both
directions;determine the ordinate sign with full point checks. Existing
division polynomials give x([7]P)=x−4*f*g6*g8/psi7². Homogenize rational
composition with fixed degrees7/6 and compare cleared polynomial equality.
Check all109/113 source/codomain points,all scalar relations and group
additions,dual[7],and source recovery by inverse7 modulo the known order.
No map reads unknown scalars. A gcd-based rational inverse is an optional
separate control,not required if the certified dual provides recovery.

Independently replay frozen kernels/codomains/map polynomials using unchanged
crypto isogeny_walk field/poly/kernel verifier,whose Montgomery arithmetic,
division-polynomial recurrence and subgroup closure differ from Kohel's
engine. Do not conflate local replay with signed scientific standing.

If consumer CI imports the existing isogeny_algos crate,use a separate
source manifest/pin and add its exact checkout closure. Keep the seven
native runtime inputs/pin75a737b8 distinct and preserve old receipts. Its
library has no external dependencies;do not invoke benchmark harnesses.

## CT20: compatible genus2 gluing,not a rank-one detour

Over existing checked Fp3,use accepted source cubic[56,13,6,1] with root a.
Original E1 roots are a_i+2. E2 long-model roots are56/a_i;translate them
to short coordinates by13/3. Apply each actual7-map to these three
two-torsion x-coordinates. Feed paired image roots to the unchanged
isogeny_algos::genus2::glue formula,which uses Howe–Leprévost–Poonen
normalization. Check descent,smoothness,twist/order and all degeneracies;
never divide a nonsquare leading coefficient without a valid coordinate map.

For a supported monic even codomain,derive/check both elliptic quotient
coordinate isomorphisms to the Kohel codomains and their actual pull/norms.
Match gluing on full geometric2-torsion. With N_old:Jac(C)→E1×E2,
product7-map alpha,and P_new:E1'×E2'→Jac(C'),the composite
P_new*alpha*N_old has type(28,28). Compatible gluing should force it to
kill fullJac(C)[2],so it factors as a genuine(7,7) map after[2]. Prove
that inclusion and factorization instead of accepting a rational halving
formula alone as global geometry. On these odd rational groups,the
factored map can be evaluated by halving input before the composite.
[pcc:unverified]

Require actual geometric kernel/dual identities,both full rational-domain
compositions[7],descent,nonzero original109 transport,and written/compiled
algebra. Transport the old non-split Richelot branch through the odd map
only after its compatible-kernel diagram is checked. That may supply a
genuine next odd-degree Jacobian hop from the accepted6-cycle target;
none is accepted in advance. Nonsquare/inert chart conversion is an explicit
engineering boundary if the first codomains require it.

Keep direct elliptic7-isogeny and original-source controls in any eventual
cost comparison. A new Jacobian model/chain alone proves no advantage over
existing ECDLP algorithms. All experiment/S3/budget,CI/merge,PCC and vault
constraints continue to apply. [pcc:unverified]
