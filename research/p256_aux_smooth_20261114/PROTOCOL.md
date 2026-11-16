# Aligned auxiliary-curve factor base: prospective protocol

Frozen before construction/test artifact generation, 2026-11-14. Starting
revision: `0c2cc1b408e33a43e55ee31cb75307e90df9d619`. This is the owner's
P-256, m=2..7 relation-stage baseline and matched 2x improvement request,
with a new auxiliary-curve smooth-subgroup base. No performance result is
asserted by this protocol [pcc:unverified].

## Construction and prior art

Use the registered P-256 model
`icv1-fp256-t89188191154553853111372247798585809583-f188c491` and its
actual 256-bit prime, not a word-sized proxy. Auxiliary model:
`w^2=x^3-3x`. Interpret smoothness as its large 2-primary subgroup;
**do not call the complete group order smooth**. For p=3 mod4 the j=1728
curve has order p+1 (pair x and -x and use the quadratic character of -1).
The implementation must check v2(p+1)=96 and certify a point of exact
order 2^96, with deterministic SHA-256-selected public trials. Clear the
odd cofactor and reject lower-order points. No target discrete logarithm
or target-to-auxiliary group homomorphism is assumed.

Construct H of order 2^d, a complete rational 2-isogeny chain phi with
kernel H, and R with phi(R) finite and not 2-torsion. The fibre base is
the target lifts of x(R+H). Both signs are retained and negation folded
into canonical columns. Retain every rejected kernel/coset trial. Reject
empty fibres explicitly. Canonical FB1 identity/point codec and the wide
dump schema are the existing P-256 ones. A certificate accompanies, not
extends, that schema. Rebuilding must reproduce bytes and IDs.

Primary development fixture d=4; exhaustive crosschecks d=1; bounded map
unit checks d=1..6. These are construction/unit tests, not the requested
performance experiment. Intended benchmark base d=8; scaling cells d=16,
18 are separate and must pay construction/storage costs. Never present
d=4 or d=1 timings as the d=8 experiment. Freeze the actual FB1 ID emitted
at each depth before relation-instance generation.

The framework is prior art: Petit–Kosters–Messeng, *Algebraic approaches
for the Elliptic Curve Discrete Logarithm Problem over prime fields*,
PKC 2016, sections3.1,3.4,3.5:
https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf .
Existing native implementation: `src/cryptanalysis/ic_framework/prime_tower.rs`.
The candidate here aligns the auxiliary linear coefficient with the
target, giving `(y+w)(y-w)=B`. Novelty remains unestablished; j=1728 and
PKM themselves are not claimed as new. KB unavailable: no KB MCP tools,
and `scripts/research-kb status` exits127. Fallback sources above and the
existing wide dump/ICV1 codecs were read; a search miss proves no absence [pcc:unverified].

## Matched arms

All arms use the identical base, signed points, targets and complete group
addition relation. Input integers are exact modulo the P-256 prime.

1. Dense membership product over the actual target abscissae, plus the
   target curve equation and complete addition charts (reference).
2. Quadratic rational 2-isogeny membership chain with target ordinate.
3. The same chain with paired lift variables u,v: uv=B,
   x^3-3x-(u-v)^2/4=0, 2y-u-v=0 (candidate).
4. Before final002 validation: orient the paired lift by propagating the
   auxiliary ordinate to phi(R). For each step use
   w_next*(x-r)^2 - w*((x-r)^2-delta)=0. The x-chain already excludes
   x=r. This adds d-1 ordinate variables per summand and d cubic equations,
   but selects one auxiliary ordinate per x rather than 2^m sign copies
   across the whole relation. Compare against arm3, do not assume a win.

The paired lift has two auxiliary-ordinate preimages per target point.
Account for that multiplicity; it is not two independent relation rows.
All arms compare Fp-rational solutions, not equal algebraic-closure ideals.
Tower roots that do not lift to a target Fp point can still acquire target
ordinates over extensions. Pay for any such solver output and its rational
root rejection. Do not assert ideal equality or append degree-p field
equations while continuing to claim a cubic input system.
No factor-base-domain restrictions, planted-only solving or lost exceptional
addition branches may count as a complete search. Provide Semaev S3-chain
exports separately, explicitly marked incomplete at intermediate infinity;
do not use them as complete-coverage arms.

Use m=2,3,4,5,6,7. Complete group addition must include unequal-x,
doubling, inverse, left-infinity and right-infinity charts, with polynomial
Boolean selectors and denominator nonzero witnesses. Finite base points
have no infinity branch. Include deterministic planted and unplanted
targets; hash selection is domain-separated by FB1 ID, m, replicate and
instance kind. Unplanted means no planting witness supplied, not proven
absence of relations. Replay every reported relation in exact P-256 group
arithmetic and reject all invalid or duplicate projected rows.

Additional development gate frozen before test002: d=2 complete-domain
exhaustive/MITM checks for m=2..4 to exercise multiple base columns, with
OpenSSL replay of every returned ordered tuple and corrupted-ordinate
rejection. Ordered tuples are test objects, not independent relation rows.

## Correctness and degree gates

Native development tests must compare GMP arithmetic against OpenSSL EC,
rebuild bytes, check all fibre maps/points, reject bad order/map/dump data,
and compare exhaustive versus meet-in-the-middle relation references on
the complete d=1 domains for m=2..7. Polynomial exports must be evaluated
at planted assignments in every arm, including inverse and infinity
intermediates. Keep negative tests and all failures.

Maximum input-equation degree is not degree of regularity. Neither a
quadratic membership chain nor uv=B proves a smaller affine F4 solving
degree, homogeneous regularity or faster solution. Report structured
degree of regularity as **unmeasured** until the frozen ring, order,
homogenization, solver completion certificate and full Macaulay/rank
data are available [pcc:unverified]. Capped/incomplete basis computations are censored,
not completed solves. No native-decide/producer self-check is an
independent PCC verdict.

## Measurement admission and success

Use current checkpoint gate and EKS Spot/L2 contract before experiments;
local builds and bounded unit tests are development only. Source, binary,
spec and input hashes must be bound. Warmup, A/A controls, at least five
measured rounds, fixed seed set, fastest available matched complete
algebraic reference, wall/CPU/cycles, field/group counts, allocations,
peak RAM, disk traffic and all return statuses are required. Use
single-cell caps: 60 seconds, 4 GiB, one native process, no retries;
freeze a larger allowance before exceeding these caps. Record unrun and
censored cells, not invented zeroes or successes.

Success is >=2x median complete relation-solving stage speedup on the
same m/instances and exact output relation sets, with spread and paid
conversion/replay/precompute costs. A stage gain is not an end-to-end
index-calculus gain [pcc:unverified]. Full EKS/ecbench P-256 solver integration, a measured
baseline, actual structured regularity and the 2x threshold remain
deliverables; construction alone does not complete the request.

Do not attempt unplanted full-depth P-256 key recovery here. The older
17-term full-depth promotion gates are unchanged and not evaluated by
these m=2..7 development fixtures. Preserve scientific assertions as
PCC records and label unsigned observations `[pcc:unverified]`.

## CAS qualification amendment (before any CAS execution)

The native `magma_adapter.cpp` emits all four arms in Magma syntax over
GF(p), with alphabetically sorted variable names and grevlex order. It
calls `GroebnerBasis` then `Variety` and emits projected target points.
This fixes an execution interface, not the structured-regularity
definition or a proof of successful solving. Syntax, licensing/image
availability and actual CAS completion still require qualification.
The printed affine output-basis degree must not be relabelled as regularity.
Synthetic logs are only parser/replay tests. The reader records full log
hashes and distinguishes CAS solution multiplicity from ordered tuples and
nonzero projected rows; its completion-marker observation is unsigned.
The original d8 baseline and matched 2x goal remain unrun [pcc:unverified].
