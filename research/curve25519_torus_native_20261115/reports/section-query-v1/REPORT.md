# Curve25519 supplied-section queries: native implementation

2026-11-23. PR4984. Source03bc5f4fad; native PCC-20261123-be5351,
prospective PCC-20261123-1aa48b. All standing is producer unsigned/unverified.

The Rust component computes geometric products in a finite free quotient
algebra, restricts occupied label intervals and lifts a supplied section to
original indexed witnesses. At the original Curve25519 torus parameters
q=213156431, a=60824497, k=20633, the fixed control extracts the six labels
(j,t)=(1,0),...,(6,0) from the supplied coefficients, then checks the unchanged
six-member circuit and independently replays its indexed group row. Its
public API inputs are Spec,R,Q=[8]R,c; no original point list, labels, signs
or witness are query inputs. The control constructs c from known points;
it is not coefficient discovery or a blind public-target cohort. [pcc:unverified]

## Exact algebra and algorithm

Use Fp2[X]/S_c, monic degree6, then adjoin
v²-(4T_q(lambda)²-2)v+1, lambda=A/(2X+A). This extension is free even when
S_c is nonreduced; v is a unit, with inverse4T_q(lambda)²-2-v. Scalar
Karatsuba convolutions implement the homothety/baby-step/giant-step/chirp
recurrence. No unknown factor is inverted. Conjugation supplies the second
geometric product, with explicit base-projection checks. The uncapped
fixed-degree loop family has the classical upper bound
O(M(sqrt(k))+sqrt(k)); this backend uses M(n)=O(n^log2(3)), giving
alpha=log2(3)/2≈0.7925. This is a source-level arithmetic bound, not a fitted
scaling result or the earlier hypothetical quasi-linear backend. [pcc:unverified]

Rational support, nonzero rational ordinate and coset membership use sixth
powers, retaining multiplicities. gcd(S,H_J^6) restricts intervals. Bounded
factor splitting, local Hasse orders, trace sign lifting and the unchanged
HAC3.56 index decoder recover witnesses. The toy original case includes a
leaf at -R: R=(21,77), leaf=(21,24) over F101, exercising removal of the
forced norm zero. Raw aliases stay distinct: toy labels(1,0)/(2,2) share
image(71,71). A repeated-prepoint section has a nonzero nilpotent H remainder
with H^6=0; support passes while original repeated raw labels refuse.
Identity-pretarget and arbitrary-sign coverage are not established by these
fixed query controls. Prior operator controls remain separately frozen.
[pcc:unverified]

## Fixed control flow and scope

| Planned case family | Attempted/completed | Result |
|---|---:|---|
| Scalar convolution, two fields/seven lengths |14/14|All exact schoolbook comparisons equal|
| Full algebra remainders, two fields/three degrees/eight n/two offsets |96/96|All direct-product comparisons equal|
| Nilpotent,16 guards,three ordinary pipelines,alias,repeated,full-length degree1 |8/8|All declared checks pass|
| Total fixed arms; completion separate |118/118|Exit0,119 records including completion|

No final arm failed, timed out or was excluded. Transport success/collision
and four focused release tests are separate from the118 arms. The actual
k31/k20633 controls retain q213156431; neither substitutes q1. Actual t is
fixed0, toy alias includes t1/t2. The full n20633 direct-product comparison
is degree1; the separate full-remainder degree6 comparisons have n<=31.
The actual degree6 pipeline has complete original root/circuit/row replay,
not a separate degree6 n20633 direct-product reference. References share field/quotient-reduction primitives;
scalar and algebra multiplication layouts differ. [pcc:unverified]

Recorded new-arithmetic Fp2 multiplication calls: toy k8=19691;
actual k31=105416; actual k20633=1756642. These are partial counters on
single known-answer inputs, not normalized field work, bitop/v1, time,
RSS or speedups. Existing field inversions, AD lifting, group/circuit/index
work, parsing/serialization and complete allocator costs are not priced.
Cumulative reserved coefficient slots are likewise partial, not live bytes.
No external speed ranking or benchmark row is admitted. [pcc:unverified]

## Exponent-bearing successor

For k=B^theta and B^chi explicitly attempted sections, the conservative
seed-test upper budget is B^(chi+alpha*theta), alpha=log2(3)/2.
chi+alpha*theta<3 is a sufficient phase-budget target under the earlier
conditional six-sum model; it is not a necessary lower bound/disproof.
Charge setup to one external target and retain public query/[8]-fibre,
useful yield/rank, growing family, indexing, selection/private-tag, kernel,
recovery and memory costs. [pcc:unverified]

The concrete next candidate is to compile these recurrences with five
unknown normalized section coefficients, rather than enumerate signed
triples. Extension/projection correctness and guarded inverses come first;
small input count/quadratic gates do not imply low solving degree. A compact
bulk solver needs its own representation, root-selection and full cost
bound. No such coefficient solver or new campaign is scheduled here.

## Evidence, provenance and remaining gates

The30-input58112-byte source archive, eight compiled hashes and1533160-byte
binary eaa3e8c6 match pre/post. Full259139-byte raw JSONL is stored losslessly
as61929-byte controls.jsonl.gz, SHA42d9cfbe. Original November17 four-test,
fmt, strict Clippy/build logs and failed literal-type build source/log remain
unchanged. A copied immutable qualifier and actual wrapper transport controls
preceded the single180second-cap native call; shell-only orchestration.
No timing or isolation measurement is inferred. [pcc:unverified]

Primary sources: Bostan--Yurkevich arXiv2012.08656v1 section2.1 Algorithms1/2;
Bostan--Schost2005 Lemma1/section5. Methods are classical; KB unavailable
is recorded, not a novelty clearance. Earlier residual identity
IDC1h5aa2ca2649f9d3c6 and the unit-cancellation Lean endpoint retain their
narrow scope; no new formal proof of this native algorithm/cost is claimed.

Evidence: ../../section-query-v1/{PROTOCOL.md,PRECALL.json,POSTCALL.json,
SOURCE_QUALIFICATION.json,native-v1/,FROZEN.md,NEXT.md}. Diagram pipeline.dot
and counts.svg/CSV describe this component. Canonical scoreboard/timeline
and prior criteria were checked and left unchanged: no admitted performance
value, completed overall-goal edge or solver theorem was added.
