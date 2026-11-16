# R58 complete100-section doubled-pole space and trace contraction

The full L(2P) has100 actual source-bound sections. All196 selected
quadratic products pass complete ambient bounds and257 F4-jet independence;
the96 local binary conditions have rank96. The actual involution has
N=sigma+I of rank50 on100-space. An explicit binary operator Q satisfies

    NQ+QN=I100, Q²=0.

The normalized old24-space embeds injectively/equivariantly. Allfour old
fixed-modulo-trace representatives now have explicit trace preimages in
L(2P), with actual function-field identities. This is scoped to the two
prescribed function spaces; it is not a target-factor identification.

## Complete pole geometry and actual descended products

R56 gives div(A)=P-D, where P consists of four reduced degree20 points
and D=28R07+4R01+7R17+R11, all four infinity points of residue degree2.
R27/R57 give the complete D0=49R07+7R01+7R17+R11 basis of dimension68.
Thus2D0 has degree256, h0=196 on the retained proper genus61 curve.
The new2D has degree160, h0=100 since160>2g-2=120.

R43/R50 provide196 actual products of descended sections. At each retained
ordinary cusp chart, form G_i=sigma(f_i)+a*(f_i+sigma(f_i)) with
 a²+a+1=0. MULTIPLY the G_i to evaluate the actual product packet.
Linear descent of f_i*f_j would be a different expression, so it is not
substituted for these products. Coefficient conjugation produces sigma(G_i)
and is checked for all1360 section/chart pairs.

Each F4 coefficient q is split by imag(q)=q²+q and real(q)=q+a*imag(q),
both required0/1. The196 product columns have rank196 in257 F4 jets
(514 binary rows), and all3920 section/chart pole bounds pass. Completeness
uses this independence plus RR dimension196, not the candidate count.
No normal-generation theorem is assumed. The original196 packet suffices;
no fallback to all2346 pairs was necessary.

At Q0 remove42 ramified coefficients -98..-57 and6 unramified coefficients
-14..-9, each with two binary components:96 conditions. Their rank is96.
The196x100 kernel K has complete products C K=0,R K=I100,K R+H C=I196.
All exported matrices are checked to have only binary entries. Controls1,A²
lie in the kernel; actual b² reconstructs in the ambient and is excluded
in6 restricted rows. The196-column actual involution and100-column
restriction square toI and satisfy the full stability product.

## Whole-source functions and finite-pole division

FIELD58 binds all196 product pairs to the actual R50 field, all100 kernel
functions and all100 divisions byA². It checks all100 actual sigma
substitutions lambda->lambda+A, three control identities and a changed
section negative control. The source has degree32 overF2(u). Division
byA² identifies complete L(2D) with complete L(2P), using div(A)=P-D.
All source equations are exact in the full field, not finite-fiber tests.

## Trace contraction and disappearance of the four old classes

The old L(P) inclusion into L(2P) sends old numerator f toA*f. INCLUDE58
reconstructs all24 numerator inclusion columns with257-jet identities,
checks rank24 and equivariance, and checks all24 normalized source identities.

For new Nrank50, choose50 image columns and50 actual preimages. Their
100-column Jordan basis has a complete two-sided inverse. The resulting
Q satisfies NQ+QN=I andQ²=0. Consequently kerN=imageN; the retained
R57 complete_kernel theorem applies withK=N,R=Q,H=Q,N²=0. Q gives
explicit preimages for EVERY fixed function in this space.

ApplyQ to the embedded four old fixed-modulo-trace columns. Allfour
source identities sigma(h)+h=f_old/A pass. Addinglambda/A changes trace
by1 and is rejected as the original preimage. CHECK58 independently
replays ranks,kernel recovery,the trace identity andinclusion products,100 source
reconstructions,100 divisions andfour actual preimage identities.

## Complete finite support and conditional map constraints

FIBER58 rechecks the source-bound norm80:four degree20 F2 fields become
20 simple degree4 points overF32. All five coefficient-conjugate factors
per component and all product identities are checked. On every quartic,
Frob32² acts as w->w+1,lambda fixed,b/base fixed,nu->nu+b. This is the
actual source sigma action on this support, extending the retained R54
finite-fiber identity to the full labelled F32 support packet.

The following are CONDITIONAL constraints, not constructed maps. For a
nonconstant map overF32 to E_a:y²+xy=x³+a*x²+1 with both x andy/x in
L(2P), the double poles of x atO andy/x atT=(0,1) force ramification1
at those fibers. Their supports are disjoint subsets of20 degree4 points.
Hence degree=4k,1≤k≤10. Allten allowed degrees4..40 are coprime11;
integer Bezout certificates are retained. The target-subgroup action must
still be checked on an actual candidate.

If additionally phi*sigma=[-1]*phi, then at these points Frob32² phi(Q)
=-phi(Q). Every four-cycle point trace therefore cancels. Degree-zero
closed divisors supported entirely on these20 points have zero pushforward
under such a map. Use them as pole support and reverse-evaluator negative
controls; collectors need other supports. The actual map/equivariance
hypotheses are not supplied by a rank or by a shared Hecke packet.

## Proofs, attempts and sources

Pole58 formalizes local-unit order doubling, exclusion of simple poles,
binary coefficient extraction, four-cycle trace cancellation, ramification1
under a double bound and the degree-unit implication. PROOF_MAP.md names
the actual geometry/map premises. Read results.json for terminal compile/audit
counts and every failure. The first Lean attempt lacked an explicit CharP
import; the second left closed Coprime goals unsolved. Both are retained.
The repaired finite cases use kernel-checked decide; no theorem is accepted
from source text alone.

Reviewed R27 complete RR/cusps,R43/R50 actual product packet,R54 finite
Frobenius/sign family,R55/R56 complete zero quotient/poles,R57 complete
restriction and trace functions. Primary reference:
[Stacks RR and duality](https://stacks.math.columbia.edu/tag/0B5B).
No novelty claim is made for linear algebra,RR or finite-field factorization.

## New contained map test: eighteen stable-support patterns

The alloweddegree filter leaves18 F2-stable O/T patterns on thefour
originaldegree20components:12 ordered single-component pairs (degree20)
and6 complementary two/two partitions (degree40). This cohort is smaller
than all F32point subsets and will be labelled as such,without replacing
the broader map search. For each pattern, div(x)=2T-2O is a degreezero
principal-divisor question. In100-space, f=A²x must vanishorder4 atT
andorder2 outsideO/T. These are COMPLETE linear local conditions.
Constructing normalB/A^4,dimension320, supplies all pointjets. A nonzero
solution is unique up toscalar by propercurve constant-field facts.

For each surviving x0, enumerate all31 nonzero F32scalars and bothordinary
signs. The Artin-Schreier equation t²+t=x+a+1/x² is F2-linear on250
trace-compatible bits:write t=lambda/A plus acombinationofthe50fixed
functions overF32. Its expectedrank249 has kernelthetwoconstants0,1;
verifythis usingthe completepole/jetbound andwhole-source equations. This avoids starting
with a500-variable quadratic solve for this specifically bounded cohort.
The generalF32-support quadratic compiler remains a separate retained lane.
Final whole-source identities,allfivecoefficientconjugates andintended11action
remain acceptance checks. This is the next experiment,not an executed map.
