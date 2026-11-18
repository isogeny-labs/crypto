# Full constant-row-space audit, v1 (frozen before code/calls)

Goal: discriminate the proposed univariate extraction lead on the ORIGINAL
large quadratic circuit before another bounded solver fork. This is a static
linear-algebra/certificate development check under AGENTS9a, not a solver,
benchmark, sampling campaign, full-base enumeration or exponent measurement.
No original solver/source/criterion/budget is changed.

Input: exact frozen target-circuit-v2/native-v2/six-member-system.json.gz,
SHA256 5b92bf1e7ef2b468175262d0a77e762386f83e364271a34f830624f1f9e87522;
p=2^255-19,1059variables/1089equations, original huge q213156431 membership,
arity6. Input contains no witness. Do not consult the known-answer witness.

Two fixed projections, both retaining EVERY original equation:
1. Keep all six members' z_a,z_b variables (12 indices).
2. Keep all six members' z_a,z_b,selector,slack,branch,raw_v inputs (204 indices).
Identify explicit indices by exact frozen compiler variable stems; retain names
and indices in the certificate. These labels are an adapter,not a group or
compiler equivalence proof. No initial assignments/substitutions are made.

For keep set I, split equations into original input-only rows U and remaining
rows R. Project R onto monomials involving at least one variable outside I,
obtaining matrix A. Compute exact constant-weight sparse row elimination and a
complete lower-triangular origin transform. Every zero projected row gives a
kernel vector and hence an exact input-only polynomial. Record/check them ALL,
including zero images. Independently certify ranks of U and U plus kernel
images. Their rank difference is the dimension of new input-only consequences
in the FULL original constant row space. No row products,degree growth,field
equations,ideal completeness,assignments,root solving or distribution inference.

Certificate checker uses separate direct BigUint weighted recomposition. It
checks source order/coverage,canonical coefficients/monomials,nonzero triangular
diagonal,all previous pivot coefficients zero,normalized new pivots,each full
input-only image,and both input-space ranks. A certificate needs all original
rows; a partial/capped computation is UNKNOWN,never an exclusion/absence proof.
No producer-native checker yields signed/formal PCC standing.

Fixed native development cases: five public Fp29 controls (new quadratic
relation; private auxiliary pivots/no relation; already-input constraint;
zero dependency image; constant contradiction),then the two fixed projections
above. Preserve every case including failures. No parameter/slice tuning after
results. Protocol permits no successor extractor or solver calls this round.

Admission: <=4096variables,8192rows,32768input terms,degree<=2,<=2MiB input;
field restricted to Fp29 or Fp(2^255-19). Each generator/checker independently
caps50million charged visits/arithmetic events,8million cumulative coefficient
writes and1million represented certificate terms/origins; raw JSON<=32MiB.
These are development admission quotas,NOT normalized cost,allocator bytes,
peakRSS,timing or a complete cold attack budget. Resource refusal stays explicit.

Controls: independent ranks expected for the five small hand-derived matrices;
mutated origin coefficient,diagonal,source coverage,pivot,image,keep list and
serialization must refuse. Force work/write/entry/input caps. A new equation
or degree cap change needs a separate version/protocol,not a selective rerun.
First source/build/lint failures are retained. Freeze compiled source/binary,
qualifier and distinct build/pre-call/post-call manifests before the seven-case
qualified call; no native reruns for report/publication metadata.

Decision: zero increment with a complete checked certificate excludes NEW
input-only relations from constant row combinations for that exact input and
keep set. Positive increment preserves the lead and records actual relations.
A budget refusal parks only this implementation. Neither outcome excludes
conditional specialization,higher-degree multiples,new representations,other
targets/families or index calculus. Signed standing remains unverified.
The original V3 gamma<1/family/yield/rank/recovery/full-cost obligations remain.
