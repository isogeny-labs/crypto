# R27 prospective exact computation

Continue every lead listed in R26/NEXT.md. Start with cusp valuations, which
are an input to the complete RR and actual Prym transfer construction; then
derive source-bound norm/collector identities and pursue node187 membership.
Each dependent extension requires its own prospective source freeze.

Reconciliation: new boot 8fc63b83-b328-41c5-9d73-c9bd94597e78. Approved receipt
and managed Diem inspected: deadline-expired,50/52,pendingfalse; legacycomplete.
EKS overview3succeeded2failed0active,unrelated; no submission/retry. Conductor
clears study/tools/problempaths; daily log held T729, use SESSION.md. R26
publication9b8a05bb4 is clean and complete, do not repeat its experiments.

Literature before method: R22 CM/base divisors/generic recovery; R23 CM
SS5/Z5; R24 charts/series; R25 quotient; R26 exact T source orbit/proof map.
Sutherland18.782 lecture22, Cor22.23:
https://www.ocw.mit.edu/courses/18-782-introduction-to-arithmetic-geometry-fall-2013/5859c5f7d2ad1eae5339d87dce144823_MIT18_782F13_lec22.pdf
The RR dimension requires the proper geometrically integral curve and genus
bridge. Singular official manual jet/truncation semantics reviewed via
https://www.singular.uni-kl.de/ftp/pub/Math/Singular/src/4-1-1/singular.pdf;
the selected latest manual URL was inaccessible. No new novelty claim.
Generic protocol checker/schema and lead-atlas unavailable in this checkout;
this remains the inherited custom exact-computation contract, not generic lint.

LAURENT27: start from the R24 four O-chart lifts,768 absolute series precision,
increased fixed lifting iterations; preserve original five residual checks.
Represent a Laurent approximation as (unit polynomial,exponent,absolute P),
meaning error O(w^P). For addition take min P; multiplication takes
min(Pa+vb,Pb+va), inversion P'=P-2v. Normalize after every operation.
All arithmetic is native Singular; Python/shell only assemble/store inputs.
Evaluate original source polynomials termwise with truncation and propagated
precision; require zero residual with strictly positive certified precision.
Iterate exact original T five times; record lambda,nu,S,N and all registered
entries at all20 cusp branches. Require the fifth iterate equals sigma at
positive certified precision and b,c orders agree with the complete R22
base table times e. Nonzero leading terms must lie below certified precision.
Helper controls include inversion of a pole, cancellation, multiplication
precision, and exact zero. A negative control changes the actual lambda
update by1; unchanged original source residual detector must reject it.
No global divisor completeness is inferred before degree/support exhaustion.

Planned2native calls positive/corrupt, each4GiB virtual/1200s fixed cap,
native Singular4.4.1 local inherited lane. Native exit0+finalmarker+no error
diagnostic required; failures retained even when process exits0. No random,
timed/comparative/statistical claim, sampling, A/A or power calculation applies
to these deterministic identities. Freeze amendments before affected data.
No new EKS,Diem,waiting-freeze,fifth Conductor claim,merge or auto-merge.
Checkpoint current source/raw to setec-astronomy-artifacts before each native
call. Never mutate repo concurrently with commit/checkpoint/gate. Native raw
must be durable before continuation. New faithful Lean identities will expose
geometric bridges rather than assuming missing geometry as axioms.

Prospective correction before LAURENT02:01 retains native exit0/late marker
but wrapperrejects the collision between seed poly entry and new int entry.
Rename only the new loop index to index27. No chart or precision change.

## Prospective norm/geometry and RR extension after accepted LAURENT02
LAURENT02 accepts all20cusps and original residuals with positive propagated
precision. Retain01 collision failure. Make checkpoint-live transport visible
to checkpoints; no raw lost (01/02 present in the newly included archive).
NORM27 adds one exact native call for actual lambda/nu norm-one ratios,
nontriviality, all8registered invariant ratios=1, constructive Hilbert90 and
an explicit ideal membership1 in (b,A,b*ib+1). These algebraic certificates
support, but do not formalize, the proper free-action argument below.

Potential proper quotient bridge: outside five cusps b is a unit; lambda is
integral from monic G and nu is regular except possibly SS5/Z5, whose original
charts already certify regularity. Thus sigma(nu)-nu=b excludes fixed geometric
points there. At O,lambda residue swaps0/1, and T transports this to every
cusp while commuting with sigma=T^5. A free constant Z/2 action gives an etale
proper quotient; Riemann-Hurwitz would give genus31 and Prym dimension30.
Check coverage and state the geometric bridge explicitly before concluding.
References read before deduction: Stacks0BXX,07S7,0C1B. In particular no
prime-to-characteristic ramification assumption is silently used in char2.
Lean certifies only universal ratio/fixed-action algebra and conditional
integer genus/dimension calculations, not quotient schemes or RH.

RR construction proposal: all original lambda/nu are regular away cusps under
these same charts. Candidate rational monomials with denominators only
u(u+1),lambda/nu powers, are therefore regular elsewhere. Impose ALL20cusp
principal-part bounds by linear algebra over F2 using certified precision.
If68independent valid functions are found, RRdimension68 certifies completeness;
fewer do not. An actual source-basis independence calculation is mandatory.
Freeze precise monomial range and matrix source before RR data; do not claim
completeness from a kernel count without independence and pole exhaustiveness.

Before NORM27data, extend it with native integer sums of literal accepted
LAURENT02 orders: check pole degrees lambda64,nu132,S308,N462;lambda cusp
zero50leaves14,nu cuspzeros132 andS/N cusp+Z5zeros exhaust308/462. Completeness
uses regularity away cusps via monic G and existing SS5/Z5 recovery charts.
For remaininglambda support, compute actual mu=b^4+bcA^3+c^2A^4,
clearu^28, take its exact elliptic-base quadratic norm and factor it with an
exact product check. No residual degree guessed from a scalar model.
Negative Laurent control rejected original3/fifthlambda despite native0/late
marker, as required. Its full diagnostics retained.

## Prospective LAMBDA14 and complete RR search, before affected data
NORM01 accepts all actual norm/collector identities and exact degree sums.
The residual lambda base norm factor is the irreducible degree14 polynomial
q14=u14+u13+u9+u5+u4+u3+1, exponent1, other factorsu22(u+1)16.
LAMBDA14 checks the unique base residue using (E,mu) overF(2^14), recovers
original b,c,lambda0,nu and all5source residuals/all8registeredunits. Require
u=(b+c)/c²,nonzero nu, nonzero lambda derivative and no exceptional denominator.
A simple degree14source zero plus50cuspzeros exhausts64 under the DVR bridge.

RR27 fixed candidate families K=8,12,16, stop on jet-rank68 or afterK16;
all at same4GiB/1200s native invocation cap. Candidates
lambda^i nu^j v^e u^k/(u+1)^K with0<=i,j<=3,e=0,1,-K<=k<=K.
Regenerate certified20charts as input, retain their full series. Impose all
negative coefficients below -7e atA,-e atB,0elsewhere. Fixed coefficient row
range[-512,-1]; reject any candidate below-512 or with uncertainty at/below0.
Require129known coefficients at the chosen unramified O point. All additions
in the Gaussian reducer are F2 vector XOR; polynomials encode finite bits,
NOT polynomial-module syzygies. Track original-candidate combinations and
independently recombine every accepted vector's constraints and evaluation
jet from original columns. Distinct jet pivots certify independence. Candidate
family independence is not assumed. Under complete pole and genus bridge,
nonzero valid section cannot vanish to129 sincepoledegree128. Thus rank68
certifies a complete basis. Lower rank only records this family's deficiency;
rank>68 forces bridge/implementation audit. No map search until completeness.
Keep exact candidate index,coefficient bitsets,constraint columns and jets.
All branch/source and precision controls remain in the same native call.

RR01 rejected collision of seed poly ee with coefficient int ee; operator
stopped this already-failed invocation, native reported0. Rename the new
integer rrExponent only beforeRR02; no candidate/constraint change.

Prospective CUSP27: exact divisor ofnu/sigma(nu) has degree34 and ramified
cusp anti-pair vector(11,0,7,11,5), orderedO,A,C,D,B. Transport byT gives
negacyclic moduleT5=-1, sinceT5=sigma. Compute resultant of this actual
relation andt5+1 overZ, and mod11 the full ideal/lift. Test gcd=t-6 and
allfour primitive values2,6,7,8, not a sampled/coefficient-box search.
This bounds the rational ramified cusp module; it does NOT certify a nonzero
Jacobian class or its inclusion into the intended factor. The value6 matches
R26's retained rational diamond character but that agreement is not a map.

Lean01 failed map_div (group rather than group-with-zero lemma),additive
order in a denominator proof,and incomplete denominator simplification.
It finished exit1 after a migrated-NFS RPC wait. Source/log retained; replace
map_div bymap_div0 and explicit div_eq_iff denominator reasoning before02.
No theorem statement or geometric bridge is strengthened by the correction.

## Prospective O-pole coefficient amendment after RR02
RR02 reaches native finalmarker and all3width ranks19, no native errors, but
its native exit was NOT captured: the live launcher was expanded forCUSP
while its child ran and Bash resumed with a truncated command. This is a
retained wrapper failure, not a strict accepted invocation. Never edit a
running script, including after its gate. Freeze runner until processexit.

The first family keeps base coefficient O-poles bounded by3 for everyK:
u^k/(u+1)^K withk<=K is regular atO,apart fromv. This was an incomplete
coefficient family, not a complete RR proof. NewRR-O family permits
-K<=k<=2K, widths8,16,24,32, stop at68 or after32/cap. All other source,
constraint[-512,-1],precision,129jet and independent recombination checks
unchanged. No input-polynomial independence assumption. Preserve lower-rank
results, full candidate indices and all source/residual controls. One fresh
native call at4GiB/1200s; runner copied/hashed as evidence and never edited
while active. Native exit must actually be recorded for strict acceptance.

BeforeLean02,add finite cusp-evaluation/Bezout/common-root checks and actual
integer determinant factorization/11valuation,conditional source genus from
the complete tame different,129jet degree obstruction. Geometry remains
written. Source count18named theorem declarations. Local cache is a byte-
verified copy of pinned compiled dependencies; an initial overinclusive
source-comment parser copied extra modules,not a missing-library result.

## Prospective export reconciliation after RR-O01
RR-O01 strictaccepts nativeexit0/marker/noerrors and internalrank68 atK16.
However bulk write(list) exports have one fewer record:19chart records,
67basis vectors,1567jet columns. Do not silently synthesize missing records
or treat an exported67-vector file as a complete68basis. First IO27 tests
3explicit elements with internal size/print,implicit list write,explicit
closed-link list write,and element-by-element closed-link writes. This is
an I/O experiment, no new arithmetic claim. Reconcile actual in-memory list
counts and export framing before any dependent basis-action/torsion run.
A necessary export-recovery replay must assert20charts,68basis vectors and
1568candidate columns,print all finalbasis vectors,and explicitly close each
output link. Retain all originals. No result is promoted from a receipt count.

IO01 strictPASS: internal list has3elements; implicit and explicit-closed
bulk list writes retain only2, while individual closed-link writes retain3.
This isolates a native serializer behavior, not missing in-memory arithmetic.
Before dependent calculations, necessary RR-export replay uses ONLY the
successfulK16family, unchanged768precision/constraints/129jets/caps. Assert
20charts and1568columns, print allbasis vectors and write individually framed
native source assignments for all20charts,68basis vectors,1568constraints
and1568jets, with closed links. No bulk list writer. Retain originalfiles.
Use the frozen generic custom-source runner branch for future inputs; never
edit that runner while a process uses it. This replay recovers unavailable
required outputs; it is not a new or enlarged RR search.

Lean02 local dependency-root copy fails before proof elaboration with unknown
private Lean initializer declaration. Keep this runtime failure and exactsource.
Next attempt uses the original pinned dependency search roots,with only source
and output copied to task-local disk. No changed theorem or enlarged cap.

## Prospective actual cusp 11-torsion test
CUSP01's actual relation matrix M has an integral vectorz=(1,2,4,8,5)
with Mz=11d,d=(-9,-11,-4,8,10). Thus11[d]=0 in the actual Jacobian,
but nonvanishing is unproved. Also Tz-6z=-11(1,1,2,4,2), soT[d]=6[d]
by actual principal relations. MOVE27 verifies these integer/source products
then searches k=1,...,10 andm in[-4,4]^5 in fixed lexicographic order for
moved=k*d+M*m withO,C,D,B in[0,11] andA in[-28,21]. Stop atfirst, or
report the bounded moving-family failure without any global obstruction.
For a found certificate D'=sum moved_i delta_i-div(lambda) <=D128.
Then L(D') is a linear subspace of the newly complete68space. Impose all
cusp orders and simple vanishing at the degree14 lambda-zero point. Since
D' has degree0,dimension0 proves actualnonzero11torsion,dimension1 produces
a principal-function witness. Freeze the exact dependent linear source
before doing that test. This is a decisive actual divisor-class question,
not merely a nonzero finite lattice quotient. No factor inclusion follows
from matching the residual diamondvalue6.

Literature lead: Achter-Casalaina-Martin, Putting the P back in Prym,
https://www.math.colostate.edu/~achter/math/prym1224.pdf (December18,2024),
TheoremA(2a),Theorems5.12/6.4. Its arbitrary-base characteristic2 treatment
applies to an etale double cover once our proper-curve bridge is established:
the30dim complement has a principal polarization overF2,restriction is twice
it. This is an additional Prym carrier,not the old det19 quotient embedding,
not a30genus Jacobian,and not an explicit theta/projective model.

## Prospective ACTION27 after complete RR export and MOVE01
MOVE01 strictaccepts k4,m=(0,-1,-1,-3,-2),moved=(1,-3,10,2,0),
principal exponents(4,-3,5,-1,-2),after205833 bounded visits. Define
D'=sum moved_i delta_i-div(lambda);11D'=div(product g_i^c_i/lambda^11),
where g_i=gNu composed withT^(-i) (pushforward divisor convention).
D'<=D128,hasdegree0,positive/negative degree64. Nonvanishing unproved.

ACTION27 imports individually framed20charts,68vectors,1568jets andactual
moving certificate, bound by action-inputs.sha256; no stochastic samples.
Regroup actual candidate bitsets into rational numerators in(n,l,v,u) over
common denominatoru16(u+1)16. Reconstruct all68sections atall20charts and
check their old divisor bounds and129Ojet agreement with independently saved
original column jets. Every requested coefficient must be below propagated
absolute precision. Recover sigma on68basis using Obranch1 versusObranch0,
verify sigma²=I, require rank(sigma+I)=34. Solve traceone and retain its full
basis coefficients and actual cusp pole degree. Record every function in an
individually framed symbolic numerator file; no bulk list serialization.

Then evaluate at the actual degree14lambda-zero point. Use all14absolute
trace pairings against1,a,...,a13; verify this detector has rank14. Impose
all20moved-divisor bounds plus that degree14zero. Check D'<=D128 anddegree0.
Compute exact F2 matrix rank/kernel on68sections, verify each retained pivot
and kernel vector by recombination from original columns. Ldimension0 proves
nonzero actual11torsion;dimension1 gives a principal-function witness;>1 is
an error requiring audit. This uses the written complete-RR/divisor bridge,
not an unformalized geometry axiom in Lean. No desiredCM projection follows
from the shared residual eigencharacter alone. Same4GiB/1200s native caps.
Input source/raw is explicitly in checkpoint-live before invocation; runner
and all inputs immutable for its full lifetime. Store all errors/rejections.

Lean03 on the unchanged18theorem source with original pinned dependencyroots
and task-local source/output compiles andaudits exit0:18named/18audited,
0failures,0trusted-native. Lean01proof errors andLean02import runtime failure
remain retained. Standard foundational axioms only; no geometric theorem is
silently promoted. Future proof extensions require new source freeze.

Before ACTION27data, additionally export individually all68O-jets through
order388,34invariant basis vectors and129jets, and exact representations of
1,b,c,lambda in the full basis. Require the actual propagated precision for
all requested389coefficients. This enables later complete degree-two equation
construction; it does not yet claim those equations or whole ideal saturation.
Very ample degree bounds and finite-field quadratic generation checked in
Marquez-Corbella, Martinez-Moro and Pellikaan, On the unique representation of very strong algebraic geometry codes,Proposition11:
https://ruudp.win.tue.nl/paper/65.pdf. It explicitly includes finite fields.
A future quadratic-relation experiment needs its own source freeze. These
are new proper curve models; old coupled-quotient saturation/polarized abelian
embedding are separate, not discharged by changing coordinate models.

## Prospective low-genus and direct relation filters
Primary LMFDB spaces read:77.2.a (new dimension5,fournewform orbits;total cusp7,
old2) and77.2.b (primitive quadratic character conductor77,dimension6,
newform degrees2and4). Under R15/R16's written moduli bridge, the free
cyclic10quotient is X0(77),genus7; quotientbyorder5 hasgenus13 and adds the
quadratic-character space. Every trivial-character constituent has dimension
atmost2; the only additional4dim constituent has chi(2)=-1,so its degree8
Euler polynomial is even (complex conjugation pairs a2 with-a2).
LOWGENUS27 factors the actual two degree8tracezero polynomials overQ,
requires irreducibility and non-evenness, checks the generic conjugate-pair
Euler parity. Hence these particular genus7/13quotients cannot contain either
A5factor up toF2isogeny, under that modular decomposition bridge. This is
stronger than the already retained norm-annihilation for the selected factor;
it does not exclude other low-genus curves.

Also certify source base-translation fixed-locus ideal1 and allunramified
quotient genus arithmetic. The actual integral correspondence
E=(1-T5)(5-(1+T2+T4+T6+T8)) obeysE²=10E, image dimension24. Check it acts10
and10E acts1 on the residualT=6line mod11. This is a sourceJacobian mixed
carrier projection with denominator10unitat11,not a map from Koblitz points.
Its inherited polarization type and the old det19polarized embedding remain
uncomputed. The knownPrym30principal polarization does not supply them.

For the direct elliptic-coordinate route, factor the normalized ordinary
elliptic cubic x3+(z²+z+aCurve)x²+1 oververifiedF32 forbothaCurve0,1.
The target z=y/x has degree3. Constant shifts of actual trace-one coordinates
nu/b,lambda/A andthe ACTION canonical trace-one section have degrees34,80,110.
Their nondivisibilityby3 excludes these direct matches,not all nonconstant
invariant adjustments. No coefficient search or IC efficiency claim.
The higher-degree quadratic-generation reference was retrieved bycurl after
webtimeout;fitztextreadactualProp11 confirms the finite-field statement.

## Prospective complete projective equation construction after ACTION01
ACTION01 strictPASS:68reconstructed sections at20charts, original Ojet-column
agreement;actual sigma²=I,rank(sigma+I)=34;34invariant sections andtraceone
sectiondegree110. Degree14trace detectorrank14. Moveddegreezero membership
matrixrank68,kernel0, independent recombination. Thus D' is nonprincipal and
11D' is principal: an actualorder11sourceJacobianclass withTaction6. It lies
in the Prym by the difference/11torsion argument. This is not a proven nonzero
projection to either desiredCMsignfactor;the residualsystems overlap at11.

QUADRICS27 uses all68Ojets through388 and all34invariantOjets through128.
Compute every symmetric product:2346source columns truncatedat257 and595
quotient columns truncatedat129. Require multiplication ranks196 and98,
independently recombine ALL kernel vectors and retained pivots, verify
strictly distinct leading candidate indices. Expected kernel dimensions2150
and497. Export each homogeneous quadratic and bit-vector certificate
individually with explicit closedlinks. All data bound byinputSHAmanifest.
Positivecontrol is actual (b+c)*(1/u)+c²=0 via retained firstRRbasisfunction;
corrupt with the actual constant section squared and require residue1.
No sampling,numericalrank,timing or IC-relation-rank claim.

A source quadratic has poles<=2D,degree256;257zero coefficients atO prove
zero. A quotient quadratic is sigma-invariant,so129zeros atO force129at
sigmaO too,exceeding256. Equivalently its quotient poledegree is128. The
invariant34space is complete onH withdegree64. Forboth128>=2*61+2 and
64>=2*31+2, finite-fieldProp11 gives a closed embedding and idealgenerated
by degree-two relations. A complete nativeI2basis therefore generates the
prime saturated homogeneous ideal in THESE new projectivecoordinates.
Original field coordinates1,b,c,lambda are recovered linearly inC;H has
fieldF2(E)(Z) andnativeinvariance identifies its quotient. Oldcoupledgraph
ideal equality,saturation and the det19polarizedembedding are separate
obligations; do not transfer the new-model saturation claim to those rings.
All4GiB/1200s caps/checkpoint/immutable-runner rules unchanged.

NewKoblitzTorsion27source formalizes actual five integerprincipal-relation
combinations,11annihilation,diamond6,movingby4,recoveryby3,conditionalnonzero
andmixedprojector10recovery. The actual RR nonvanishing and curve/Picard
bridges remain native/written,not Leanaxioms. Freeze before compiling it.

Before newtorsion-module compile,include the actual128/64very-ample degree
bounds,2*129>256 and expected2346/595product counts/2150/497kernels,and
free-quotient genus/mixed-dimension arithmetic. Newmodule14named theorems,
combinedwith18previousnamed results; no full curve/rank formalization claim.

QUADRICS01 rejected before multiplication: kernel is a Singular reserved identifier. Actual-source positive/corrupt controls returned0/1; neither target rank accepted. Rename local list quadraticRelations; same inputs, cap and acceptance tests for retry02. Lean04 stopped at checkpoint credential discovery before compilation; retain gate error and use frozen inherited credential wrapper for Lean05.

Lean05 exited1 before elaboration: failed to create thread, Resource temporarily unavailable. Freeze single CPU0 affinity in addition to existing -j1 and4GiB/1200s caps for retry06. This changes execution resources only, not theorem bytes. Interrupted checkpoint-quadrics02 before native launch to regenerate the assembled input with the corrected identifier; no quadrics02 run directory exists.

## Prospective actual node187 proper-chart certificate
Bind all4 original Q41 graphs,4 actual degree19 Miller functions each,5
coefficient conjugates:80actual functions. Native hex decoding in original
F2[beta]/(beta360+beta5+beta3+beta2+1), verified Rabin gcds and final square.
Check every successive coefficient conjugate by actual Frobenius squaring,
all monic S8 and degP9. AtO t=x/y,z=1/y=t3q verify
 t19 q9 F(x,y)=U(t,q), U(0,q)=1,
via the exact toric ideal (xt2q+1,yt3q+1), not a scalar model.
For BOTH ordinary group laws, complete local line-intersection/Vieta and
inverse-unit blocks give a lifted identity tSum in (tP,tQ) modulo chart
relations. Independently multiply the full lift. Its32nd power proves
nilpotence transfer. Retain geometric-series explicit unit certificate for
1+tSum*w when tSum32=0. Combine with the actual node187 boundary ratios,
all coefficient components and proper descent P2=P0+sigmaP1.
This is a proper-chart polynomial separator for the actualsource boundary,
not yet global h^k f membership in the original affine homogeneous ideal.
Do not silently discharge global saturation from it. Caps4GiB/1200 unchanged.

## Prospective arbitrary cusp reverse evaluator
Using the ACTUAL five ramified cusp principal divisors, verify overQ an
integral5x5matrix W with M W=134292 I-d(1,6,36,216,1296), d=(-9,-11,-4,8,10).
Check every denominator1 and each product entry independently. Bind arbitrary
formal coefficients e0..e4, not forward-generated examples. Along with actual
nonzero order11 proof this identifies the full11primary cusp subgroup and
its coordinate e(6) after prime-to11 scalar134292. Corrupt one witness.
This does not evaluate ordinary degree460/367 supports, nor isolate a CM sign.
Same4GiB/1200s native caps. Proper principal-divisor bridge remains written.

QUADRICS02 also rejected before multiplication: rank is reserved. Rename multiplicationRank27; original controls0/1 retained, no rank accepted. Lean06 still failed before elaboration despite CPU0 affinity; the runner incorrectly added a process-address-space ulimit to the inherited -M4096 compiler cap. Remove that extra virtual-address restriction, preserve Lean -M4096 and1200s/CPU0. Lean memory limit is managed allocation, not a4GiB limit on mapped runtime address space. Native Singular retains original4GiB virtual limit.

Written mixed-carrier construction recorded in POLARIZATION.md, based on
actual free cyclic10 tower and characteristic2 Prym/duality literature.
Proposed type(1^18,5^6) is a geometric deduction, not a measured lattice or
explicit theta-section computation. Exact integral E and actual residual
class are independently native-bound; actual CM-sign projection remains open.

Lean07 reached elaboration after removing the extra address-space restriction, failed five scalar-zero goals because Abel does not import Group.Basic. Add the actual Group.Basic import and zsmul_zero; retain all failed source/logs. Add three conditional additive/numeric mixed-carrier lemmas, now17named inTorsion plus18Collector=35expected, not a formal polarization theorem.

## Prospective ordinary460/367 paired-image recovery
Import the4actual ordinary pushed supports from originalM15/M16:436(simple),
12(double),284(simple),83(simple). Freeze exact original JSON bytes/hashes.
Native verify every modulus byRabin, original curve equation, actual recovered
norm A+B*y=0, nonzero denominators. Product-plus-one forces secondY=1/firstY;
asymmetric-plus-one forces secondY=1/firstX. Factor actual second-X cubic
 X^3+X^2+secondY*X+1+secondY^2,
reconstruct all factors/multiplicities, check every rational root on E49.
At a SIMPLE pushed zero the norm valuation formula forces a unique source
zero of residue degree1/order1 (under existing proper-correspondence bridge).
If cubic has exactly1rational root, this uniquely identifies its second image.
At the degree12 double zero additional residue-degree2 possibilities remain.
This does not construct the scalar-C5 lifts or a complete reverse divisor.
No finite candidate set is promoted as selected images or relation rank.
Four4GiB/1200s caps may be applied individually; initial frozen invocation
processes these4inorder under the same single cap. No replacement of targets.

## Prospective complete RR F32 equivariant elliptic-map equations
Build actual34invariant basis h_i withh_1=1 and exact trace preimages l_i,
all from checked68sigma matrix and389jets. Every equivariant candidate in
L(D) is X=sum a_i h_i,Y=sum a_i l_i+b_i h_i, sigmaX=X,sigmaY=Y+X.
Native coefficient construction of the entire elliptic identity
Y2+XY+X3+c0X2+1 uses385O coefficients (poles<=3D degree384). Add all68F32
field equations, c0inF2, and Rabinowitsch nonconstant-X block (33auxiliaries).
455equations cover both ordinary target signs in this EXACT prescribedspace,
not a coefficient box. Native same-field genus3-to-elliptic polynomial
identity and actual corruption provide controls. Every trace preimage checked.
Producer4GiB/1200; solver separately4GiB/1200, no silent cap increase.
IfGBunit, require explicit unitlift and full independent multiplication.
Nonunit/cap is unresolved, not an evaluated map or universal exclusion.
Any successful point still needs function-field identities,descent and actual
nonzero11 action; no CM inclusion is assumed. Checkpoint assembled producer
and its output before each native call; retain every attempt.

After next host replacement boot a578e16f, reconciliation found no native/proof processes, Diem deadline-expired, legacy complete, EKS one unrelated failed ML job and no active jobs. No launches duplicated. Lean08 completed exit1: broad simp converts integer scalar-zero to natural scalar-zero without closing it. Replace ONLY simplification by explicit zsmul_zero/add_zero/neg_zero list. Rebuild Collector object because host-local verified object was lost, not because prior proof evidence failed. REVERSE01 full integral witness passed but final control used polynomial remainder by11 inQ (always0), not integer remainder. Check exact evaluated integer14469 instead. Original failures/source retained; no mathematical rejection.

Post-migration checkpoint first failed before launch because shasum was absent; installed standard perl-Digest-SHA, retry checkpoint succeeded. DIRECT01 rejected number/poly assignment at linear roots; all4cubic factor counts are3 but no point accepted. Extract constant coefficient with leadcoef inretry02. MAPBUILD01 rejected reserved argument monomial; its late455-equation marker is unaccepted. Rename symbolText27, add >1000coefficient-term guard. Basis34/trace preimages remain independently checked in retry; no empty-map verdict from invalid producer.

MAPBUILD02 strictPASS:34invariants andpreimages,646907binary coefficient terms,
455equations. Before solver data, add TWO independent actual-source-jet
specialization checks coveringALL455equations: (a1=1,allothercoeff0) and
(a2=beta32,b3=1,c0=1,auxv1=1/beta32). Recompute X/Y fromoriginal68jets and
savedH/Lbasisvectors, compare every cubic coefficient to the generated
polynomial equation evaluated by independent ringmaps. Constant/nonconstant
block must be1/0, allfield constraints0. No fixed vectors are solutions or
map claims. Solver can start ONLY after both complete controls pass.

MAPSOLVE01 rejected after SIGTERM request (Singular caught it and returned native0) after cross-ring indexing errors. No GB conclusion accepted. Whole polynomial ideals must first be evaluated while mapTarget iscurrent, then transfer ONLYconstant results to validation ring. Freeze910 explicit successful checks and0failures with nativequit-before-solver on mismatch; independent mcAllowed admission guard in solver. Native ERROR alone is not a control-flow stop. Retain exactfailedinput/log/exit; sameequationsandcaps inretry02.

MAPSOLVE02 completed admission910checks/0failures, then hit original4GiB
virtualmemory cap: Singular no-more-memory,halt14,nativeexit14. No map,
nonexistence,orGBunitcertificate. Keep full455-equation input andboth actual
jet controls for a future better elimination strategy; do not raise thiscap.
R28proof accepted3named/4audited including generatedcofactor congr_simp;
0fail/0trustednative. R28ordinaryimageevaluation accepts436,284,83selected
pointpairs, same seconddegrees;degree12bothfirstadjugates0,R29extensionfrozen.
