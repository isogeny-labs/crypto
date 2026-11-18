# CT20: compatible elliptic gluing and a genuine odd Jacobian continuation

Status: prospective construction/proof plan, not an executed gluing test or an
accepted Jacobian7-map. Recovered from the session handoff after the execution
workspace became unavailable on 2026-11-11 around14:29UTC. The prior local
CT20_PLAN.md could not be read after that event. No numerical prediction below
is a new measured result. All mathematical standing remains unsigned.
[pcc:unverified]

## Objective and fixed inputs

Continue the user's Chained ECDLP Transfers objective from cryptoPR3877:
actual DLP-preserving Jacobian chains, coefficient descent to the original
prime field, independent correctness and complete cost controls. Do not
replace this by a model change, an abstract module count or rational halving
without a geometric map. A computational advantage remains a separate open
requirement after correctness. [pcc:unverified]

CT19 construction and independent replay are durable at cryptanalysis
ef9a542598c1b0ebfde03cf27f77929b928391cd, PR1483:
experiments/chained-transfer-reference/ELLIPTIC_SEVEN.md,
src/seven.rs, tests/seven_independent.rs and
evidence/elliptic-seven-witness-01.tsv.
The focused run43038507853 completed successfully at14:33:50Z:
native25, reference51 unit passes plus two standalone integration passes,
one pre-existing benchmark ignored. This CI observation has no new signed
PCC verdict. Existing mathematical records include a51d74/beeac7/45f608/2f70a0.
[pcc:unverified]

Keep native crypto runtime75a737b8366ec6c78ed8672886423fcbe69853a2 unchanged.
The separate102-input Kohel construction and five-input Montgomery verifier
manifests/pins already exist; do not fold them into the seven native inputs.
Reuse the old isogeny_algos FpR/genus2::glue implementation, rather than
introducing another elliptic, field or Cantor engine. [pcc:unverified]

Fixed p101 source:
C0: y²=t6+6t4+13t²+56, #J0(F101)=12317=109*113.
Its two elliptic factors are E1short(1,46),order109,trace−7,j12 and
E2long[5,33,13,1], with X=x+38 giving short(44,21),order113,trace−11,j58.
The original E109 uses X=t²+2. Select all four pairs of the CT19 elliptic
degree7 maps, ordered lexicographically by kernel coefficients.
The targets on E1 are (5,39),j91 and (26,58),j87; on E2 they are
(69,50),j92 and (13,42),j89. Dual scale u=1/7=29; point recovery
scalars78 mod109 and97 mod113. These are existing CT19 assertions,
not new CT20 observations. [pcc:unverified]

## Primary geometric source and sign audit

Howe–Leprévost–Poonen, Large Torsion Subgroups of Split Jacobians of Curves
of Genus Two or Three, arXiv:math/9809210v1 (8September1998),
Section3.2, Propositions3/4 and their proof:
https://arxiv.org/pdf/math/9809210 .
The session read these pages before interruption; the recorded PDF digest
was1730ca25f09f57eeda0a703ed6917b58c9c924d497e132f2e6565b2dbacd095c.
The downloaded PDF itself is not recovered in this checkpoint. Recheck the
source if the original file/receipt is not restored. Proposition3 constructs
a genus2 Jacobian from a Galois-equivariant graph on2-torsion not induced by
an elliptic isomorphism; Proposition4 gives explicit gluing formulas.
This is prior art, not a novelty claim. [pcc:unverified]

The paper's direct quadratic quotient goes to the SECOND elliptic input F;
the reciprocal quotient goes to FIRST input E. Expect a component swap
relative to the source SplitQuotients API and verify it explicitly.
The paper has h=−product of three quadratics. The existing code's a1,b1
are negatives of the paper's because the denominator differences are
reversed: Acode=−Apaper,Bcode=−Bpaper; three negative factors contribute
the compensating external minus. Do not retwist from a comment alone.
Validate actual point counts, quotient equations and coordinate maps.
This sign derivation is a plan-stage audit, not an executed new gluing.
[pcc:unverified]

## First discriminating construction

Use FpR::from_modulus(101,[56,13,6,1]); call its root a.
The paired source2-torsion x-coordinates in short models are
A_i=a_i+2 and B_i=56/a_i+38.
Lift the actual CT19 rational map coefficients to Fp3 and evaluate these
points (ordinate0), checking poles, distinctness, Frobenius cycle and both
target curve equations. Feed each paired image triple to unchanged
genus2::glue. [pcc:unverified]

Inventory all four map pairs, with every successful and failed chart.
Require coefficient descent, smoothness, the actual expected L-polynomial
from traces−7/−11 and #J=12317. Under the cited gluing hypotheses a genus2
quotient is expected; the existing chart can still be unsupported. If raw
leading coefficient L is nonsquare, record an unsupported normalization
boundary; never silently change to a quadratic twist. If L is square,
choose the least rho with rho²=L and put raw w=rho*Y to make the sextic
monic. Choose the first fully supported model in the declared lexical
order and preserve the full inventory. No gluing attempt has yet run.
[pcc:unverified]

For monic new C1:Y²=x6+a2*x4+a4*x²+b, construct existing SplitQuotients.
The first cubic is [b,a4,a2,1], the reciprocal one [b²,a2*b,a4,1].
For each long cubic y²=x³+c2*x²+c1*x+c0, X=x+c2/3 is its short coordinate.
Match actual quotient curves to the actual Kohel target curves, allowing
the necessary component swap. Use checked isomorphisms satisfying BOTH
A_target=u4*A_source and B_target=u6*B_source. The old iso_exists_over
function only returns an unchecked scale candidate and cannot certify this.
If X_K=u²*X_internal,Y_K=u³*Y_internal, invert by
x_long=X_K/u²−c2/3,y_long=Y_K/u³.
Check complete point-set isomorphisms and orders109/113, not j alone.
[pcc:unverified]

## The first genuine (7,7) map: global kernel inclusion

Let N0:J0→E1×E2 and P0 its dual be the old norm/product-pull maps;
N1,P1 are new gluing maps in consistently matched coordinates.
Write alpha for the product of the actual degree7 elliptic maps.
A=P1*alpha*N0 has polarization multiplier28 and degree784.
Prove A kills the full geometric J0[2]. Only then factor A=Phi*[2]
by the abelian-variety quotient property, obtaining separable degree49
and polarization multiplier7. Rational-point halving alone is insufficient.
[pcc:unverified]

Compute N0 on every geometric branch-pair2-torsion class. Its image is
the four-element matched graph: opposite branch pairs map to zero, and
the remaining index pairs map to the corresponding elliptic2-torsion pair.
For each of the three nonzero images under alpha, convert to the actual
new quotient coordinates U,V. The first pull is u=x²−U,v=0; the second
is u=x²−b/V,v=0. Check U*V=b for each pair, so P1 kills the graph.
This supplies the full geometric2-kernel inclusion; retain the written
HLP/Milne bridge and every exceptional/support case. [pcc:unverified]

On the odd F101 rational groups evaluate
Phi(D)=P1*alpha*(N0(D)/2), halving the109/113 components by55/57.
Evaluate PhiHat(Q)=P0*alphaHat*(N1(Q)/2) with the checked component
isomorphisms and swap. Check both entire12317-class domains:
coefficient descent, image cardinality/bijection, paired additivity,
both dual norms[7] and all109 original-source relations.
The global dual identities require the geometric diagram and cancellation
in Hom, not just these finite rational checks. [pcc:unverified]

## Phi on all geometric2-torsion without Jacobian halving

The global diagram gives Phi*P0=P1*alpha after canceling[2].
For each D∈J0[2], use Z∈(E1×E2)[4] with P0(Z)=D and
2Z in the four-element graph KerP0. For each graph element take all four
halves on each elliptic factor:64 product lifts in total.
They should cover all16 geometric J0[2] classes, four preimages apiece.
P1*alpha(Z) must be independent of the chosen preimage. This computes
Phi|J0[2] without dividing Jacobian2-torsion by2. Match against the15
nonzero branch-pair classes; check all256 additions, Frobenius
equivariance and the sixteen-element bijection. [pcc:unverified]

Ei[4] lies in Fp6 here: Frobenius³ is identity on Ei[2], hence mod4
Frobenius³=I+2M and its square is I. For Ti=(ri,0) use
x=ri±sqrt((ri−rj)(ri−rk)) and both ordinate signs sqrt(f(x));
verify all four distinct points actually double to Ti. Halves of O
are the four Ei[2] points. Do not trust a formula without checking its
curve equation, completeness and doubling. [pcc:unverified]

Reuse the existing Fp6 bridge: flat modulus[33,46,2,18,56,12,1],
gamma=T+s with s²=2,
s=(gamma³+6gamma²+19gamma+68)/(3gamma²+12gamma+15), T=gamma−s.
Embed flat coefficient polynomials into nonsplit_map::Field by Horner
at gamma=T+s. This basis bridge was independently checked in CT15;
reuse its immutable witness rather than inventing another field.
A new seven_glue.rs child of nonsplit_map can access the existing
private Field/Element/Jacobian/Divisor code. CT19 SevenEdge APIs are
pub(super) and the balanced SplitQuotients fields are already available
within the test crate. Validate visibility and pin every new source.
[pcc:unverified]

Extension pullbacks for C:y²=x6+a2*x4+a4*x²+b:
first quotient(U,Y)→u=x²−U,v=Y;
second(V,Y), V≠0→u=x²−b/V,v=(Y/V)*x.
The second quotient V=0,Y=±b gives the balanced n2/n0
infinity-difference classes; elliptic O maps to Jacobian zero.
Include zero/infinity cases and validate pairs. Old short→long shifts
are2/38; new ones require the actual checked isomorphisms above.
Use coefficient embedding at gamma and check canonical tails.
[pcc:unverified]

## Move beyond the accepted six-cycle target

The old R0 cross-kernel classes have branch-pair labels
(0,4),(1,5),(2,3) in the existing ±Frobenius root ordering.
Use the64-lift computation to identify their Phi-images among the new
fifteen branch pairs. This selects the rational Richelot kernel R1 on J1.
The current RichelotMap::from_assessment asserts the frozen first cross
pairing. Generalize only with an explicit selected-pairing constructor that
checks factor product=source, Frobenius stability, roots, determinant
nonzero, smoothness and valid normalization. Never bypass those checks.
New roots may still be expressed in the old cubic coefficient field
[56,13,6,1]; validate the new branch ordering and Frobenius rather than
assuming it. Preserve any nonsquare/inert normalization boundary.
[pcc:unverified]

For the already accepted six-cycle target T0, put
C=R1*Phi*R0Hat. Since R0Hat(T0[2])=KerR0 and Phi maps that kernel onto
KerR1, C kills T0[2]. Factor C=Psi*[2] globally to obtain a genuine
(7,7) map Psi:T0→T1 of degree49. On the odd rational group compute
C after halving the input by6159 mod12317; the dual is the reversed
diagram with the same verified halving. Require the actual geometric
kernel argument, both complete-domain dual norms[7], descent,
bijection/additivity and original109 preservation before acceptance.
[pcc:unverified]

The full source chain E109 --f*→J0 --R0→T0 --Psi→T1 has reverse
N0*R0Hat*PsiHat equal[28] on E109; inverse28 is74 mod109.
The new target elliptic coordinates N1*R1Hat of the transferred source
should be ([4]*alpha1(P),0), giving a direct elliptic7-map control.
Check every109 scalar, including zero. Derive these identities from
the global diagrams and compare actual coordinates, never scalar-read
the output. This is a planned assertion, not executed CT20 evidence.
[pcc:unverified]

## Distinguish unpolarized F101 Jacobians correctly

The first elliptic j changes12→91/87, the second58→92/89.
That alone does not establish Jacobian nonisomorphism.
First prove elliptic pullbacks injective: their kernels lie in E[2],
and all three nonzero elliptic2-torsion images are explicit nonzero
opposite-branch classes. Check the Richelot cross-kernel intersects
those pullback subgroups trivially, so the relevant R0/R1 restrictions
remain injective. The distinct traces−7/−11 force
Hom_F101(E1,E2)=0, since a nonzero elliptic homomorphism would be an
isogeny preserving Frobenius trace. This should imply uniqueness of
the embedded elliptic subvariety of each isogeny type, via the identity
component of the other projection's kernel.
Any unpolarized F101-Jacobian isomorphism would then restrict to an
elliptic isomorphism E1≅E1', contradicted by different j.
Write this kernel/uniqueness proof and check the actual intersections
before asserting it. No extension-field nonisomorphism is presumed.
[pcc:unverified]

## Validation, limits and next decisions

1. First implement only image2-torsion/gluing inventory and actual quotient
   matching. A correct supported new model unblocks the full kernel proof.
2. Implement the factor/dual diagrams and all12317 classes both ways.
   Immutable four-thread contexts are permissible for correctness if useful;
   this is not a benchmark or a measured concurrency advantage.
3. Implement the64-lift geometric2-torsion action, select R1, and complete
   the actual Psi chain and unpolarized-isomorphism argument.
4. Preserve every failure; maintain written proofs and faithful feasible
   Lean helpers with the original geometric theorem still open when partial.
   Use the existing PCC CLI to register new records; never manufacture old
   lost claim JSON or rewrite ledger history.
5. Freeze changed inputs before publication. Run changed targeted tests,
   formatting, strict Clippy and guards, then the applicable full focused
   consumer matrix. Existing workflow has a25minute cap; assess the full
   chain's correctness cost and split/manual timeout scope if necessary.
   Do not add expensive PR triggers or waive verification to meet a timeout.
6. Publish source-linked report, diagram and fully inspected PDF; index the
   actual result/negative and graph disposition. Do not substitute this
   plan for those deliverables.
7. Only after successful S3 admission and live IsoLab allowance/capacity,
   preregister complete construction/transport/solver/recovery/verification
   comparisons with original elliptic, direct elliptic7 and best applicable
   ECDLP/IC controls. No solver or performance experiment has been launched.
   KB/ecdb and encrypted lead-atlas integration remain blocked.
[pcc:unverified]

Execution is stopped by workspace unavailability. GitHub file reads/object
publication still worked for recovery; no local compiler/proof process can
currently be invoked. Resume after a working checkout is restored, inspect
actual refs/diffs and the interruption checkpoint first. No CT20 code,
gluing test, odd Jacobian map, benchmark or signed scientific acceptance
is claimed by this file. [pcc:unverified]
