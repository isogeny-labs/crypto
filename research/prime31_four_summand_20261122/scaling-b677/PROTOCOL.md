# RR5 structured B677 extraction protocol

Frozen design before the first build or execution in this continuation.
Branch: `lane/prime31-rr5-b677-extraction-20261122`; initial tracking PR: #4967.
Parent: `7f5752d6e38e3b43de886959c16136c0d443304f`.
The preceding B8 source and evidence are immutable inputs, not new measurements.

## Question and scope

Can a compact quadratic modular circuit express exact four-summand membership
in the original PRIME31 hybrid base, and can the existing native F4 traversal
extract its public signed solutions within explicit degree, time and memory
bounds? Construction correctness, extraction completeness, relation replay,
useful authentic-RHS rank and complete cold cost are separate gates.

The curve is p=2147483647, A=847514846, B=2087444724,
n=2147446831, G=(625037715,2100627251). The requested base is the original
509-point Dickson fibre plus the frozen 168 Y7 points, canonically signed,
sorted and deduplicated. It is not a natural-x base. The Y7 CSV is
`research/prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv`, SHA-256
`fc484615394ee423190747d05001cea526580fb0df0279fbcf789ab6cafc0c46`.

## Presentations and mathematical controls

The reference presentation is U*W=h, with monic quartic U and the exact
degree-677 base polynomial h. The candidate uses the public degree-1192 selector

H=(D_1024(X,1)-terminal)*product_i((X^3+A*X+B)^7-c_i^2).

Every selector factor must split simply over the base field. Its on-curve
nonzero-y abscissae must equal the requested 677-point base exactly. The
anti-Frobenius argument in MATHEMATICS.md is a required justification for the
extra noncurve selector roots. The norm identity, target-outside-base condition,
monic divisions, every coefficient equation, and signed group replay remain
mandatory. A quadratic gate graph does not imply a solving-degree or cost bound.

Construction controls independently form the dense selector and perform scalar
monic long division for fixed RR5 parameter triples (0,0,0), (1,1,1),
(p-1,p-1,p-1), and (17,31,43). Every auxiliary is perturbed individually by one;
its defining equations must reject the mutation. Fixture controls independently
recompute selector membership and exact canonical point sets. There is no
four-tuple or pair-table enumeration on the B677 base.

## Frozen extraction requests and stopping rules

The positive correctness request is the sum of the first two canonical Dickson
points and the first two canonical extra points, with signs (+,-,+,+). A
grader-only integer implementation constructs the public target; only curve,
base, target and limits reach the candidate. The fixture is rejected, without
adaptive replacement, if the target is infinity or has a base abscissa.

The public-target follow-up consists of H0=(18,221278090) and
W0=(101000006,1077058598), from the original deterministic x-walk requests.
They are distinct from scalar-seeded and later target corpora.

At most two positive extraction calls are planned: one per presentation. Run
the structured call first. Run the cofactor reference once even if the candidate
stops, so the record distinguishes the input formulations. H0 and W0 run only
if the structured positive completes with independent exact replay. No further
target, limit increase, parameter search or retry with unchanged inputs is
authorized by this protocol. A source correction requires a new immutable
build/receipt and a documented reason; all earlier outcomes remain intact.

Per call: maximum solving degree 10; shared construction/solver/root deadline
30 seconds; maximum polynomial terms 20,000; maximum assignments 4,096;
maximum admitted critical pairs 20,000; maximum symbolic monomials 8,192;
maximum matrix entries 2,000,000. The matrix bound includes traced/contributor
copies where allocated. One worker; outer wall limit 180 seconds; address-space
limit 2 GiB. Allocations must be checked before admission. A resource, degree,
time, build, output or forbidden-enumeration stop is incomplete, never Empty.

## Execution and evidence

All scientific construction, arithmetic, verification and extraction is native
Rust. Ordinary compiler, static checks and unit correctness controls may run
locally; public-target research calls use the repository's EKS Spot workflow
and require a successful checkpoint gate and authenticated admitted job.
If that route is unavailable, preserve its exact failure and publish the
reviewable implementation and runnable fixed requests without claiming those
calls executed. No change of name or launcher waives an admission requirement.

Build with at most two compiler workers; execute tests with one test/Rayon
worker. Before each run bind the exact recursive source closure, CSV, protocol,
compiler version and separately copied read-only executable. Never replace a
pre-call manifest. Retain command, start/end, stdout, stderr, exit status and
post-call source/executable hashes. Unit regression includes the previous 19
native controls and eight public-example tests, plus new fixture, circuit and
forced resource-stop controls. The original-root library gate remains separate.

## Acceptance, cost and interpretation

The positive gate passes only on Complete, the expected signed row present,
all returned rows independently on-curve and summing to the public target, zero
forbidden branch/domain evaluations and complete original-equation replay.
It does not establish that a selected target is representative. A public query
may return Complete or Empty only after complete algebraic traversal and replay.
No authentic scalar label is invented for an unlabelled x-walk target.

Fixture setup, symbolic construction, solver, roots, replay, adapter and rank
work remain separate in receipts. Prior generic collision rows are not free
setup or imported rank. Phase accounting is incomplete until each phase is
actually counted; unmeasured timing, recovery and complete-cost fields stay
unset. No speed, asymptotic or complete-ECDLP comparison is made by this gate.

## Source checks and evidence standing

KB unavailable: no loaded research-KB tools and the repository wrapper reports
no executable. Primary sources checked: the RR5/Semaev source references in the
parent study, Faugere--Mou Sparse FGLM (arXiv:1304.1238), and Paterson--Stockmeyer
(1973), DOI 10.1137/0202007. FGLM assumes an available finite quotient basis;
blocked polynomial evaluation supplies a circuit upper construction only.
Same-team source review is unsigned and is not a formal or signed proof.
