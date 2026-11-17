# Bounded wide-prime F4 implementation contract (design only)

2026-10-29; inspected c59a5dd8a8a81c4749540b92ebcdf7fb24da4a59 native source.
No edits to source, compilation, fixture generation, scientific draws or cloud
activity. Proposed constants below are conservative SOFTWARE ADMISSION choices,
not measured optimal values, user spend authorization or full-corpus restrictions.
Freeze this design in the eventual owned research scope before implementation.

## API and scope

Add f4_tower_generic_bounded alongside the existing generic entry point, using
shared implementation/kernels with an optional gate. Preserve existing unbounded
callers and mathematical pair criteria. Do not copy a second F4 algorithm.
Initial admitted profile: prime-f4-fp64-finite-domain-m2/v1, zero tower rules,
exactly two free variables, fp_backend::Fp64, 2^32 <= p <= 2^61-1, prime validated
before field construction. Small-prime f4_fp remains the existing route.

Proposed bounded signature takes input, ring, existing options, NativeLimits and
&mut dyn FnMut(&NativeEvent)->Result<(), ObserverError>. Return a typed report
containing last fully committed basis, NativeStop, counters and acknowledged
observer sequence. Stops: Complete, UnitDerived, DegreeLimit, WorkLimit,
StorageAdmission, Deadline, ObserverFailure, InvalidInput. Full-rank/staircase
heuristic exits are disabled in this profile; neither means completed Groebner
basis. No assertion/panic on user-provided ring/exponent/limit input.

New dataset module src/ml_dataset/prime_f4_wide.rs; narrow dispatch/capability in
solvers.rs; reserved artifact/binding verification in mod.rs. Native work stays in
src/cryptanalysis/f4_tower_generic.rs. Add a dedicated design/test directory.
No exporter, policy-axis, curve/cofactor, budget, collector or fp_backend rewrite.
Scope reservation remains a prerequisite, not granted by this document.

## Numeric admission profile

| Resource | Ceiling / derivation | Meaning |
|---|---|---|
| Input | 3 polynomials, <=75 total terms, <=33 per polynomial | Exact S3 plus two membership polynomials for <=32 distinct factor-base x values |
| Coordinates | 2 free; zero tower; each exponent <=255 | Check additions before packed-monomial multiplication; reject overflow |
| Input degree | <=32 | Membership degree; does not assert small solving degree |
| Selected pair degree | min(config.degree_bound,12) | Existing user degree cap remains authoritative |
| Active/retained basis | <=256 polynomials; <=131072 coefficient slots total | Includes inactive retained elements and shared row backing, not only active count |
| Pending pairs | <=32768 | Pre-admit worst new pairs before insertion; no unbounded temporary group map |
| One degree step | <=4096 selected pairs; <=64 committed steps total | Refuse a larger whole degree batch; do not silently discard pairs |
| Symbolic jobs/rows | <=8192 each | Includes reducer and S-row descriptors, todo/frontier storage |
| Columns/monomials | <=8192 | Applies during discovery, not just after sort/dedup |
| Sparse row terms | <=1048576 per step across all retained row forms | Count clones/cached forms separately while live |
| Dense coefficient slots | <=2097152 live aggregate | All echelon/reducer outputs and block buffers, not largest one |
| Wide accumulator slots | <=1048576 live aggregate | Fp64 uses u128; includes each concurrently live buffer |
| Accounted allocation | <=min(64MiB, frozen worker memory_mib / 4 in bytes) | Checked requested vector/map capacity accounting, not allocator/RSS guarantee |
| Native work tokens | min(16777216, config.work_limit * 1024) | Checked multiplication; explicit profile mapping, not retired instructions |
| Domain evaluations | <=min(config.work_limit,1024) ordered x pairs | Separate from native work tokens; 32^2 maximum |
| Group additions | <=config.work_limit | Separate existing independent lift budget; no borrowing across counters |
| Native wall budget | min(5 seconds, remaining original case deadline) | Supervisor retains original process deadline; no reset per step |
| Observer records | <=4096; <=64KiB encoded each | Includes start/terminal, bounded state/step events |
| Restricted adapter bytes | <=min(4MiB, frozen trace_mib/4 in bytes) | Aggregate input/basis/report/events with 64KiB protected terminal metadata allowance |

Zero/overflowed/missing run limits fail admission. All units are explicit bytes,
entries or named logical operations. The input polynomial count is exactly three;
75 follows 9 S3 terms + 2*33 membership terms. No curve/target is changed to fit.
Constants bound one admitted software profile; larger cases stay planned and are
marked unsupported-profile or censored-resource, never silently omitted.
Memory accounting includes vector capacity times size_of and conservative map
capacity reservations plus fixed overhead; reserve before try_reserve/growth and
release when ownership drops. It does not include allocator metadata/OS/runtime;
existing worker memory envelope remains the outer hard boundary. No RSS claim.

## Gate placement in actual native flow

The current entry at f4_tower_generic.rs:1150 clones inputs and performs initial
echelon before its later matrix checks. Place checks before that clone, all-monomial
collection, map/row construction and initial echelon. Ring/check currently asserts;
bounded API must validate first with Result, before any native state mutation.

State::insert currently pushes a polynomial, then builds groups/kept and appends
pairs. Preflight basis entries, worst h new critical pairs, group/kept temporary
capacity and work for active-LM comparisons before any push. Make insertion
transactional: either admitted complete insertion or unchanged prior state.

Pair degree partition currently drains the queue. Admit selected/rest storage
before partition. On any stop during symbolic/matrix work restore selected pairs
or explicitly return the last committed basis with unresolved-pairs status. Never
mark a partially processed degree step completed. scan_products must check every
new discovered monomial/job/row before map/vector insertion, including the initial
S-row scan; current max_nnz check after reducer scanning is too late.

Before pivot_of/q_of/q_start, reducer materialization, Acc::new, bred, block
accumulators and echelon arrays: reserve live slots/bytes with checked arithmetic.
No row factory may allocate before its caller's permit; permit follows the row
until release. Before converting final active basis or serializing output, reserve
its additional live memory and output capacity. Keep last committed basis by
moving owned state where possible, not making an unbudgeted fallback clone.

For the first bounded profile use deterministic serial scheduling of symbolic
frontier/row blocks in the SAME kernels. Existing parallel callers are unchanged.
This avoids thread-local sink loss and concurrent overshoot; it is not a speed
claim. Every loop can check cancellation without waiting for all Rayon tasks.

## Work charging before work

Use a monotone checked token counter and separate descriptive raw counters.
One token per examined monomial/reducer candidate or pair criterion comparison;
one per coefficient conversion/normalization/add/mul/reduction; one per AXPY
coefficient update; reserve 128 tokens per field inversion. Inversion weight is a
fixed admission tariff, not exact arithmetic/hardware cost. Charge sort work with
an explicit comparison counter (fallible bounded sort) or conservative n*ceil(log2
n) permit for a documented bounded sort implementation; do not assume an unknown
library comparator count is exact. Choose the latter only if the implementation's
upper bound is established; otherwise use the fallible comparator-counted path.

acc_sub_tail must reserve row.cols.len()-1 AXPY updates BEFORE row_scatter_axpy,
not add its returned count afterward. sub_row similarly reserves exact suffix
length; acc_load/drain/renormalise charge visited coefficients before loops, plus
normalization multiplication/inversion. Chunk long vectors into <=256 entries,
checking deadline/remaining permits at each chunk. Count scalar loop work and
row-kernel permits consistently without double charging the same operation.

field.inv is one backend call after its fixed tariff; inner exponentiation cost
is not reported as independently counted. Track tokens_reserved and known
completed logical updates separately when interrupted mid-chunk; a partially
used permit is not refunded and is not called completed arithmetic. Existing
rep.muladds remains its narrower native statistic; never relabel it total work.

## Observer failure and canonical artifacts

Observer returns Result and has a sticky first error. After failure no callback,
row work, pair insertion or retry is allowed. Return ObserverFailure even if the
last algebraic operation found a unit. Retain last acknowledged sequence and
last fully committed basis; an unacknowledged event cannot become a trace claim.
Reserve start/terminal record bytes/count before work. If start sink rejects,
native_backend_invoked=false and no elimination starts. If terminal sink rejects,
completion admission fails even though arithmetic may have run; retain both facts.

Dataset stages canonical original input before any live sink submission, following
c59a5dd's initial-prime-chain correction. A live failure preserves staged input and
bounded diagnostic artifacts for a separate canonical worker write, sets
live_persistence_complete=false and canonical_artifact_write_required=true, and
prevents solver-completion labels. Total filesystem failure remains failure;
staged memory is not durable evidence. Do not convert capture failure to a success.

Artifacts (versioned reserved JSON/JSONL roles) must include original exact public
instance/domain/config identity; typed original run/attempt binding; engine/profile
and all effective limits; canonical coefficients decoded from Montgomery; native
stop and bounded step summaries; separately budgeted domain/group lift observations.
Keep complete original input even on unsupported/capped paths whenever canonical
write succeeds. Output basis uses actual generic coefficients, never u32 truncation.
Each certificate/result identifies whether native elimination was entered, whether
it finished, whether extraction finished and whether positive group replay passed.

## Replay and labels

Central replay first checks original S3/membership equations from public curve,
factor-base coordinates and target, then exact profile/limit/config binding and
all reserved artifact roles/paths/encoding/required members. Unknown fields and
rehashed contradictory stop/counter/limit records fail. Validate canonical basis
encoding and any claimed candidate using independent modular arithmetic and
Point group operations. A basis identity/hash alone does not prove that F4 derived
it: keep native_basis_derivation_verified=false unless a later derivation checker
is implemented. Neither [1] nor native Complete independently proves UNSAT.

For this first profile, finite domain enumeration checks ALL <=1024 ordered x
pairs against ORIGINAL equations, then all admitted point lifts/signs, independent
of basis filtering. Charge each original-equation evaluation/group addition under
separate limits. A positive verified point sum is a PDP witness, not scalar
recovery. A negative label requires complete original-domain AND lift coverage;
it is domain-relative only. If either budget stops, report unknown/censored even
if native basis was [1]. Native Complete/UnitDerived is a producer statistic, not
an independent negative certificate. execution_trace_complete_verified and
total_operation_count_verified remain false; no row-operation proof is implied.

## Exact tests to implement without searching for favorable draws

1. Empty-tower ring controls at p=17 and p=2305843009213693951:
   input {x+y-1, x(x-1), y(y-1)} has precisely (0,1),(1,0) over domain {0,1}.
   Compare canonical solutions and independent BigUint evaluations; compare p17
   basis/behavior with existing ordinary F4 after normalizing equivalent bases.
2. Negative control with x+y-3 replacing x+y-1 and domain {0,1}, same primes:
   no original-domain solutions. Unit derivation is recorded separately; independently
   enumerate four tuples. A forced extraction cap of three cannot label negative.
3. Wide coefficient control: {x-(p-2), y-(p-3), x+y-(p-5)} has the fixed
   root (p-2,p-3), verified over those singleton domains. No u32 narrowing.
   This is a three-polynomial native fixture, not the dataset S3 schema.
4. Boundary rejection: composite p=2^32, even/zero/out-of-range modulus, malformed
   exponents, sum255+1, zero/missing budgets, oversized input, and invalid ring.
   No backend entry/callback beyond error record, no allocation after rejection.
5. Threshold tests inject tiny limits: exact capacity succeeds; one less fails
   BEFORE input cloning, initial echelon, symbolic discovery, pending-pair insertion,
   dense/wide buffer allocation and final basis copy. Counters never exceed cap;
   checked overflow is rejection. Use deterministic fake deadline, no sleeps.
6. Observer rejects start, one selected step and terminal separately: operation
   stops once, first error retained, acknowledged sequence stops before rejection,
   no falsely complete trace/solve. Row permit exhaustion and sink refusal retain
   staged input and existing canonical-write path. Test prime-chain compatibility.
7. Dataset bridge reuses fixed retained public fixture recipes (both twists) from
   prime61_f4_retains_public_equations_without_entering_small_prime_backend and its
   exact public point-doubling target; no private witness supplied. This is an
   explicit known-answer SOFTWARE positive control, not a natural-target result.
   Existing test-time curve recipe may be reused only as that frozen fixture,
   not a seed search or runtime resampling. Add independent group-sum verification.
8. Tamper rehashed input/coefficient/domain/profile/limit/basis-candidate/outcome,
   omitted/duplicated/relocated artifact roles and encoding; reject claims that
   exceed the independently checkable boundary. Preserve small-prime, roots,
   exact cofactors, capture admission and all report/public-feature invariance tests.

No tests or draws were executed to select these cases. Full current-source focused
and combined-library gates, then frozen bounded CLI fixtures, precede delivery.
Higher-m wide F4 remains required future scope: m3 needs visitor/extraction despite
fitting four free variables; m>=4 needs a representation change beyond this
profile. F5/F6/PKM/policy axes and durable remote intermediate publication remain
in the original objective. This increment neither removes them nor claims them.

## Native implementation refinement (before source edits)

First bounded entry is specifically Fp64/empty-tower/two-free-variable; generic
legacy entry and kernels remain shared and unchanged in their arithmetic. Retain
parallel kernel scheduling, but pre-admit the aggregate of ALL rows, not worker
count or one block. Fixed numeric limits above remain ceilings, not promises that
every combination is admissible under the conservative aggregate permit.

For a kernel with R total rows and C columns reserve 64*(R+1)*(C+1)*(R+C+130)
logical work tokens before entry and 256*(basis_slots+16*(R+1)*(C+1)+pair_slots+1)
accounted bytes. Terms cover initial input rows/pivots, dense B'/echelon plus
parallel block accumulators and their collected copies, per-row descriptors,
column maps and final basis copies. All block tasks together contain <=R rows;
initial echelon pivots <=R; fill-in cannot exceed C coefficients per row. 256 is
an intentionally conservative accounting multiplier, not exact allocator/RSS.
Cap observed exponents at64 and gate each product's exponent sum before invoking
ring multiplication; empty tower multiplication cannot expand term count.

Symbolic batch reserves all source stored coefficient slots (including zero tails)
and map/frontier insertions before scan_products; cumulative discovered slots are
bounded before scans, not only when final columns are known. Reducer lookup scans
at most65^2 free divisors per monomial (no tower mask), charged beforehand.
State insertion reserves pending_pairs+h and retained coefficients plus worst
h^2*65^2 divisor checks, before State::insert. Old pairs may be dropped but no
refund is claimed. All checked arithmetic overflow rejects admission.

Observers run at admitted native begin, initial echelon completion and degree-step
boundaries, plus terminal completion. An observer failure is sticky and prevents
subsequent kernels and successful terminal state. Already admitted finite kernel
work may finish between boundaries; existing native deadline checks remain, and
this is not a hard asynchronous cancellation guarantee. Reserved tokens are upper
admission tariffs, never independently verified executed operations. These
conservative kernels may censor earlier than finer fallible primitive gates.

## Diagnostic admission correction

Reserve BOUNDED_F4_METADATA_BYTES=4096 before the first observer call. This includes
the fixed2048-byte native diagnostic String capacity (up to512 Unicode scalars)
and result/event metadata allowance. If unavailable, return StorageAdmission
without invoking observer or allocating that diagnostic. Observer implementation
and its returned original error String are caller-owned allocations, excluded
from native accounting; the caller must independently bound its sink. Tests cover
exact4096 and4095, with a four-byte Unicode begin-error control. The documented
kernel expression is corrected to64*(R+1)*(C+1)*(R+C+130), matching code's
r=R+1,c=C+1 and r+c+128; this correction changes no native arithmetic tariff.
