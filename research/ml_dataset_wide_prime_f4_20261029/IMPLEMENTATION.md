# Native implementation handoff (not validated)

Design commit: 28e892aa8c0. Runtime and tests were written without compiling or
running tests, per parent instruction. Scoped rustfmt only. Independent bounds
review and parent-owned compilation are required before integration/delivery.

## API

f4_tower_generic_bounded(input: &[GPoly<u64>], ring: &GTowerRing<Fp64>,
opts: &TowerF4Options, limits: &BoundedF4Limits, observer: &mut dyn
FnMut(&BoundedF4Event)->Result<(),String>) -> BoundedF4Report.

Defaults: work16777216, accounted bytes67108864, rows8192, columns8192,
basis256, pairs32768, steps64, events4096. Callers may lower these only. Native
allows prime5..2^61-1 for small-prime differential controls; planned dataset
routing remains p>=2^32. No tower rules, two free variables, <=3 input polys,
<=75 terms total/<=33 per poly, input degree<=32. Exponents at product admission
are <=64. Degree limit<=12; staircase/full-rank early exits and legacy callbacks
are disallowed in the bounded profile. The bounded observer is sole callback.
Deadline is min(original deadline, entry+5s), with existing cooperative checks.

Report includes explicit stop, native producer stats/basis, work_reserved,
bytes_peak_reserved, acknowledged_events, native_backend_invoked. Reserved work
is deliberately NOT completed arithmetic or instructions. bytes_peak_reserved
is cumulative NONREFUNDED reservations: no release/decrement occurs, so it also
bounds simultaneous requested storage conservatively. It is not RSS or actual
allocator bytes. This is stricter than the initial design's live-budget idea.

## Gates and conservative formulas

Initial input clone/monomial map/sparse rows/echelon: R=input.len(), C=sum(input
term counts), pre-reserve K=64*(R+1)*(C+1)*(R+C+130) work and
B=256*(basis_slots+16*(R+1)*(C+1)+pair_slots+1) bytes. All arithmetic computing
these permits is checked. Gate rejection returns no allocated input basis.
Within echelon <=R pivots, each <=C coefficients; the initial accumulator C,
row/map clones, normalization/inversion and scans are contained by this envelope.

The same K/B formula is admitted BEFORE columns/maps/pivot descriptors and
eliminate, using R=all symbolic row descriptors, C=examined monomials. Row forms
cannot exceed C nonzeros; B' has <=R*C coefficients, echelon <=min(R,C)*C,
and partial/reduced buffers across ALL parallel tasks <=R*C per copy. The factor
16 covers simultaneously retained original row forms, reducer rows, residual
accumulators, returned copies and final basis. Fp64 coefficient/accumulator sizes
are8/16 bytes, plus Mono/index/Vec/map bookkeeping; accounting multiplier256 is
conservative capacity accounting, not a proof of Rust allocator overhead.
K covers row normalization (fixed inversion tariff), AXPY coefficients, pivot
and residual scans, and quadratic sorting/comparison overhead. Finite permits
are granted before whole kernels; inner kernels are not primitive-counter exact.

Before pair minimum/count/partition/seen/jobs: reserve64*pending_pairs work and
1024*(pending_pairs+1) bytes. Selected degree batch<=4096, giving <=8192 jobs.
State::insert: before mutation reserve pending_pairs+h slots and polynomial
backing; h<256 and total retained coefficient+monomial slots<=262144. Work tariff
8*(h+1)^2*65^2 +16*pending_pairs covers pair criteria and divisor enumeration;
byte tariff256*(slots+8*(pending_pairs+h+1)) covers retained basis/Arc backing,
pair groups/kept arrays/maps. Inactive basis is counted too. No refunds for pairs
dropped by criteria. Per-insertion rejection leaves that insertion unapplied.

Before symbolic product scans: sum ALL stored source tail slots (zero tails
included); reserve8*terms before checking exponent sums. For empty towers each
nonzero source term yields at most one multiplied term: no rewrite expansion.
Reject exponent sum>64 before ring.mul_elem. Admit rows<=8192, cumulative terms
<=1048576 and examined+new-term-upper-bound<=8192 BEFORE scan_products. Reserve
(65^2*8+256)*terms work and256*(16*cumulative_terms+basis_slots+8*pending_pairs)
bytes. It covers product batches/clones, sorting and examined/frontier growth.

Before each frontier reducer lookup reserve8*65^2*frontier.len() work and
4096*(cumulative_terms+pending_pairs+1) bytes. No tower masks; reducer_for visits
at most65^2 free divisors. Existing deadline check occurs before the frontier.
All reservations accumulate, so overlapping previous structures remain charged.
Final active-basis copies are covered by prior state/kernel envelopes (including
inactive backing); stopped paths do not allocate a new unbounded fallback basis.

## Failure and caller behavior

Bounded observer calls: begin, initial_echelon, each completed step, terminal.
First observer error is sticky (message bounded512chars), no callback or later
kernel after it. A terminal refusal overrides successful admission. A kernel
already running is only cooperatively deadline-cancelable; no hard per-primitive
cancellation claim. Rejected work/storage is an explicit stop, not Complete.
Stopped symbolic admission does not falsely set native timed_out. Some stopped
native stats/basis describe partial work; only the explicit outer stop determines
completion admission. A derived unit is not an independent UNSAT certificate.

Old f4_tower_generic delegates to the same internal algorithm with admission=None.
No old limits/defaults/scheduler/arithmetic/report/callback/deadline behavior is
intentionally changed. The original on_step/on_stop and stopped-step paths remain;
only admission-aware branches add bounded behavior. No other native caller file
or dataset source is modified. Tests compare bounded to legacy basis on fixed
small/wide controls, but those tests have NOT been executed.

Seven added tests: fixed positive/negative systems at17 and2^61-1; wide coefficients;
initial work/storage refusal; sticky observer refusal at begin/initial/step/terminal;
expired deadline/invalid ring/composite field rejection; exact reserved-work boundary
and one less; dimension/event caps. No random search or scientific data generation.

Remaining: independent review of conservative formulas, compile/focused/full tests,
additional targeted dynamic-fill/representation-limit tests if review requires;
dataset adapter/profile serialization, exact input/outcome bindings and replay,
separate extraction/lift budgets, canonical persistence and source-bound CLI checks.
Do not mark requested m2..15/n47..61 corpus complete from this native increment.

## Pre-build review correction

Parent/independent review caught missing canonical reconstruction of packed Mono:
require m==mono(0,free_of(m)) in addition to empty mask/free2/degree checks. Added
forged-degree/exponent255 rejection before observer/backend. Restore selected
pairs on pre-symbolic/pre-kernel/insertion admission stops; count selected-but-not
finished pairs against insertion cap. New native_statistics_complete is false
on every noncomplete outer stop, including observer refusal; legacy partial
statistics must not be interpreted as completed pair accounting. Eight tests now.

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
