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
