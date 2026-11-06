# Symbolic signed-graph reduction for pencil relations

## Observed first gate — 30 October 2026

The fresh p739 gate now contains all 60 prescribed workloads, 1,080 execution
records and 300 measured candidate/reference pairs. All outcomes verified;
native replay reproduced all 900 measured records. The unchanged native finite
analyzer admits all 300 pairs in every one of the 36 price cells. Exact ratios,
summed costs and model boundaries are in the
[PDF report](validation/accounting/reports/run-20261030T0015Z-finite-costs/report.pdf),
[graph](validation/accounting/reports/run-20261030T0015Z-finite-costs/cost-grid.svg),
[CSV](validation/covered_gate_results_20261030/cost-grid-exact.csv) and
[structured receipt](validation/covered_gate_results_20261030/covered-analysis-receipt.json).

The fresh timing gate was rejected: the 360 pencil records reached L2, while
the 720 rho/control records were L1 with sub-millisecond solve intervals.
Qualified paired clocks remain unavailable. The separate computation epoch is
retained with invalid clocks and is not additional distinct target coverage.
[Canonical packaging and reconstruction](validation/canonical_gate_export_20261030/receipt.json)
passed with no findings and `qualified:false`. Frozen v5 remains unchanged;
the report-provenance correction is explicitly post-data. Raw S3 object versions
and hashes are in the [locator manifest](validation/actual_gate_results_20261030/versioned-raw-locators.json).

The first gate's approved four-hour envelope and original deadline applied only
to that gate. Its two infrastructure attempts are terminal. The frozen stop rule
does not permit extension after the rejected measurement. The upper gate and
full 1,260-target pencil cohort remain incomplete; a prospective continuation
must retain the old outcome and resolve its protocol and resource conditions.
The preparation and implementation sections below retain earlier validation
history; their original unrun statements do not describe the completed gate.

## Current native preparation and review tools

The supported preparation, export and report paths are Rust:

- [Native preparation](native_preparation/README.md) extracts the retained
  archive projection, generates the prescribed specifications, and checks
  native plans against the archived workload identities and prepared bytes.
- [Native canonical export](native_canonical_export/README.md) packages
  hash-bound observations with explicit execution history and canonical
  validation. It recomputes the complete native analysis from the bound inputs
  and reconstructs exports from immutable requests. Packaging does not qualify
  a clock or infer missing counts. The [original review](validation/native_export_review_08e8ab38d/review.json)
  and [correction review](validation/native_export_correction_fc96a3d1/review.json)
  preserve the findings and their scoped software checks separately.
- [Native review reports](native_review_report/README.md) renders the
  preparation evidence as JSON, SVG and PDF using Rust and native LaTeX.
  Its current contract is a zero-observation preparation report, not an
  empirical cohort report.

The earlier Python preparation scripts, canonical adapter and report generator
are retained only as historical source and evidence. They are superseded
execution paths, not alternatives to the native tools. The native replacements
must retain their input, validation and accounting contracts; a software test
result is never relabelled as a mathematical cohort measurement.

The user approved the first gate's 25,000,000 coefficient cap and 3,600 seconds
per execution for all 60 prescribed p739 targets on 29 October 2026. The
[authorization record](protocol_draft/accounting_amendment_20261029/first_gate_authorization.json)
limits that approval to the first gate. The later four-hour authorization,
frozen gate protocols and publication receipts resolved the first-gate execution
conditions. The full-cohort execution envelope remains unresolved; the current
observed gate status is recorded above.

## Implementation and retained validation history

This is a mathematical implementation step toward the still-incomplete
1,260-target pencil cohort. It contains an exact finite-field row reducer,
independent unit tests, and four field-identity Lean lemmas. Its library
returns symbolic linear constraints. An optional producer integrates the
library into the existing synthetic pencil cohort path; it is compile-tested,
not yet run as a cohort.
No cohort, EKS job, or timing experiment was run for this packet. Candidate/rho
cost and the remaining 717 candidate outcomes are unmeasured here.

## Benchmark adapter integration (T-807 / T-808)

This work concerns mathematical algorithms on public synthetic instances.
It does not assert recovery of deployed keys. The root method
`ic.three_leads_genus2_pencil_symbolic_v2` requires explicit small-base
selection and coefficient cap, repeats cold setup, and retains the full
native and symbolic ledgers. Its group-only GAE is a lower bound; full costs,
qualified online intervals and candidate/rho ratios remain unavailable.

The root dependency enables only the `reporting` feature. The shared solver
uses the root arithmetic engine and installs no allocator. Replay binds the
deterministic JSON ledger to four exact SHA-256 words in audited counters;
only explicitly named clock and allocation observations are excluded. These
digest words are identifiers, not operation counts. Independent source review
checked routing, replay coverage and the cost boundaries. The root release library check passed. The full root run recorded 5,146 passes, seven failures and 123 ignored
tests. Six failures lacked tracked checkout fixtures; one reported a test-lock
conflict. After materialization, all eight selected tests passed serially on
the same compiled binary. Both logs are retained; the initial run is not
relabelled as passing.
The retained reporting-feature test log has 15 passing library tests.

The root harness now uses the existing exclusive phase clock. Target-blind
setup precedes target validation; query/control owns gaps between sieve,
relation verification, elimination and the first source check. A checked
export invariant requires the five phases to sum to the window. Additional
adapter verification stays outside online and inside cold group counts.
The matched rho hook opens at the first target multiplication or target
transfer. A regression checks one hook call and identical recorded work in
plain, negation and transferred modes. The standalone driver does not start
a clock session, so these hooks do not yet export online times there.
Instrumentation overhead is included; qualified runtime evidence is still absent.

## Reusable per-target solver (T-802)

`native_solver::solve_one` accepts a source instance, public target, algorithm
seed, explicit denominator/coefficient cap, and an optional installed allocator
observer. It does not read an archive, environment variables or a planted
answer, print output, or invoke rho. The shared source imports caller-provided
`engine` and `kernel` aliases for later ecbench integration without crossing
incompatible engine types. The standalone binary owns input selection, known
answer checks, matched rho, and the global allocator.

Output is now `pencil-symbolic-cohort-target/v2`, with nested per-target cold
setup. Setup is physically repeated for each target. Target validation now follows
target-blind setup and is charged in the candidate query stage. The exact 16-walk rule and signed relation processing are
unchanged; archived trial_cap remains rho's cap. Result storage retains graph
and solver allocations, but setup temporaries drop before rho. This is a new
live-memory baseline, not an identical v1 allocation boundary. No v2 cohort
has been run. Missing observer fields are null, not zero. Panics are terminal
for a measured execution even though a guard restores the observation flag.

New development checks reject unsupported configuration/source metadata,
check unavailable-observer serialization and flag cleanup. There are 14 default
library tests and 28 passing producer-feature library tests with one ignored
shared-clock test (including eight solver tests); binary tests now live with the shared implementation. Root method registration and the serialization-only kernel feature are
implemented, with root validation and prerequisite rechecks retained. Full cohort execution remains open. Costs remain incomplete and ratios null.

## Execution-route reconciliation (T-801)

The earlier access diagnosis incorrectly treated the phrase "research EKS"
as namespace `research`. The repository access guide and retained October 24
access audit specify namespace `isolab`, with the existing host identity in
`isolab:submitters`. On October 29 the documented harness connection returned
permission to create Jobs and listed nine terminal Jobs (seven succeeded,
two failed, none active). The installed MCP overview agreed. No identity,
permission, or client configuration changed. The default context's launcher
AssumeRole denial persists, but it does not block the documented connection.

`EXECUTION_RECONCILIATION.json` records this scope and the separate expired
Diem/freeze lane. No historical task-owned EKS Job ID was found in the inspected
four-lead records; absence is not a historical result. No Job was submitted.

The next execution dependency is integration: the current producer is a
standalone CLI. The maintained resumable contract accepts native ecbench
workloads, not arbitrary commands. The existing `ecbench/three_leads.rs`
adapter and method registry provide the integration point; reuse the exact
producer arithmetic through a per-target API, retain source checks, and add
an explicit method ID. Do not substitute a non-resumable command job for the
required recovery path. Protocol, image/binary identity, controls, pricing
and the existing upper gate must still be satisfied before a cohort starts.

## Phase attribution and scalar pricing boundary (T-798)

The producer now rejects exports when named phases do not sum to field,
native arithmetic and symbolic totals. Cold setup is added only after that
check. Deterministic walk-label generation is inside the generation phase.
`candidate_phase_overhead_ns` retains the nonnegative difference between the
whole candidate interval and its phase intervals, including bookkeeping and
control flow. It does not make unisolated clocks eligible for a speed claim.

A source audit found asymmetric inversion instrumentation: the pencil
`pow` and symbolic inverse helpers count their internal multiplications,
whereas the reference `Ring::inv` records an inverse and calls unmetered
`modpow`. Thus the historical raw scalar event sum is not a common atomic
inverse model. The native `scalar_ledger` module removes the pencil inverse
multiplications before applying a common atomic scalar-I valuation. It retains
non-inverse multiplications, including primality work where observed. Scalar-I
weights are not automatically equated to base-field inversion weights.

`COST_MODEL_DRAFT.json` gives exact formulas and unresolved admission gates;
it is explicitly not a frozen experiment protocol. A regression compares the
actual pencil inverse and actual reference Ring inverse, including their
normalized ledgers. Another rejects mismatched field/native/symbolic phase
sums and impossible elapsed-time sums. Historical artifacts are unchanged;
no revised numerical ratio has been computed from them.

Source references: `src/cryptanalysis/three_leads.rs` (`Ring`, `modpow`),
the frozen `pencil_sparse_20261022/src/bin/price_sparse.rs` analyzer, this
producer's `pow`, and the symbolic library's inverse implementation. No
claim of complete logical accounting follows from normalized scalar counts.

## Sparse map observations (T-797)

The symbolic maps now record map API calls, ordered-key comparisons, yielded
entries and cloned entries. Numeric key ordering and JSON coefficient maps
are preserved. Iterators forward the underlying exact size hint so collecting
rows does not add growth allocations merely because instrumentation was added.
Serialization bypasses metering; equality, length queries, dropped iterator
remainders and internal tree-node movement are not charged by these counters.
These are separate semantic observations, not a count of all CPU instructions.
Comparison counts depend on the Rust standard-library implementation; the
recorded toolchain identity is part of the measurement contract.

The implementation follows the official
[BTreeMap API](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html):
ordered traversal and consistent lookup ordering. A read-only peer review
identified lost iterator size hints; forwarding and regression assertions
were added before delivery. Current validation is 14 default library tests,
28 passing producer-feature library tests with one ignored shared-clock test,
including eight shared-solver tests. Cohort
execution and a complete frozen pricing model remain open.

## Inputs and outputs

Each input equation is homogeneous, `sum a_i X_i + sum b_j S_j = 0`, with
at most two distinct surviving large variables `X_i` after duplicate terms
are combined. Small variables `S_j` remain symbolic. An arbitrary prime
modulus fitting `u64` is accepted, including characteristic two; integer
arithmetic uses `u128` intermediates. The constructor checks primality using
the deterministic seven-base Miller--Rabin test for 64-bit integers.

The forest stores `X_i = scale_i X_parent + offset_i(S)` and uses union by
component size. A root may be anchored to a symbolic expression. Inserting
an equation returns a component join, a new root anchor, or a constraint
among small variables only. Empty constraints are dependent rows. All
emitted constraints must be retained by the caller. Malformed indices and
more than two large variables return errors before modifying the forest.

No relation is silently dropped. A future pencil adapter must explicitly
account for the collection work of rows with three large variables and any
admission rule. Its small-base selection and stopping rules need a frozen
protocol before new performance data.

## Algebra and proof boundary

Substitution into an input row leaves zero, one, or two free root variables.
With two roots, solving for one nonzero coefficient creates an equivalent
substitution and decreases the number of free roots by one. With one root,
the equation determines an anchor. With no roots, the equation is exactly
the emitted small-variable constraint. These reversible row substitutions
preserve the projection of the solution space onto the small variables.
Equivalently, the small-only part of the input row space is spanned by the
emitted constraints. A signed cycle need not create a small-only relation:
when its residual root coefficient is nonzero, it instead anchors the root.

`SignedReduction.lean` checks the pivot equivalence, substitution identity,
balanced-cycle equation, and unbalanced-cycle anchor over any field, with
nonzero pivot assumptions explicit. The induction over forest state and
the bridge from Rust operations to those field identities remain open.
The full implementation is therefore tested, not formally verified.

The independent Rust reference uses dense RREF and brute-force inverses in
small fields; it does not call the graph's field or elimination helpers.
The fixed regression suite compares the emitted and eliminated row spaces
after each of 40 prefixes for 128 deterministic streams over each of
`F_2,F_3,F_5,F_7,F_101`: 25,600 prefix checks. Additional tests cover signed
cycles, duplicate cancellation, malformed inputs, composite rejection and
a prime near the upper `u64` limit. These are development tests, not cohort
measurements, random-sampling claims or performance evidence.

An additional targeted test builds 32-variable components in balanced merge
order, anchors one root, and queries all remaining variables while preserving
four independent symbolic small variables. It emits only zero constraints
until a final intentional rank-one constraint. This avoids the weakness of
generic streams whose small-column rank can saturate early. It also exercises
a union chain over the near-`u64`-limit prime; the dense reference remains
restricted to the five small fields.

## Sources and reuse assessment

- Gaudry, Theriault and Thome, *A double large prime variation for small
  genus hyperelliptic index calculus*, February 2005 version, sections
  2.3 and 3.1: [primary paper](https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf).
  Its graph construction distinguishes a full relation from a cycle that
  leaves one large prime. Its complexity result assumes genus at least
  three and its stated relation model; no complexity claim from that paper
  is transferred to this genus-2 pencil stream.
- `src/cryptanalysis/three_leads.rs`, `SignedGraph`: existing scalar-label
  affine potentials. This packet extends the label type to symbolic terms.
- `src/cryptanalysis/gaudry_cubic.rs`, `LargePrimeEliminator`: existing
  arbitrary-coefficient elimination with protected small columns. It uses
  pivot chaining; this packet uses size-balanced forest substitutions.
- `research/pencil_markowitz_20261022/PROOF.md` and `REPORT.md`: the prior
  upper gate and replay already ended at the 25-million-coefficient cap.
  This packet neither repeats those attempts nor overrides their stopping rule.

The deterministic Miller--Rabin base set is the established 64-bit witness
set `(2,325,9375,28178,450775,9780504,1795265022)`; its primality theorem is
not part of the Lean proof. Its use is an implementation dependency, tested
on the suite's listed primes and composite regressions.

## Reproduce

```
cargo test --locked --offline --manifest-path research/pencil_large_prime_20261029/Cargo.toml
cd research/pencil_large_prime_20261029
lake build
```

The Lean toolchain and mathlib revision are pinned in `lean-toolchain` and
`lakefile.toml`. The retained direct compiler log uses that same dependency
environment from the existing projective-law proof checkout.

## Remaining integration work

### Allocation observations (T-796)

The producer now forwards global allocation calls to Rust `System` through
`allocation::ObservedSystem`. Hooks use only atomics, do not allocate or
unwind, and preserve successful and failed reallocation ownership. The test
uses an independent observer to check zero initialization, grow/shrink payload
preservation, successful requested-layout replacement totals and peak reset.
Twelve library tests and three producer tests pass.

Each phase and the shared cold setup record allocation/deallocation/reallocation
calls, failures, successful requested/released layout bytes, absolute peak live
requested bytes, and growth over entry. An independent outer candidate envelope
includes between-phase work and bookkeeping; inner phase resets do not erase
its peak. Rho has a region spanning its computation and source check. Timing
envelopes remain diagnostics until the protocol's execution/isolation gate.

These observations are not RSS, physical memory, bytes copied, or allocator
internal transient storage. The optimizer may remove calls. Rho's absolute
peak includes still-live candidate state and shared inputs; growth-over-entry
is net process-wide growth, not exclusive algorithm ownership. Reallocation
counts replacement of the requested old layout by the successful new layout;
failed requests change failure/call counts only. Phase aggregation sums events
and takes maxima of phase peaks, whereas the candidate envelope is measured
directly. Graph teardown after verification and output serialization are
outside the computational envelope. Observation regions must be sequential
and single-threaded; one outer envelope may contain the sequential phases.

The implementation follows the [Rust GlobalAlloc contract](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html),
including its reentrancy, non-unwinding and optimizer caveats. Independent
read-only review checked the pointer/layout forwarding and interpretation.
No prices are assigned to bytes or allocator events yet; native index work
and the frozen price/export contract remain necessary. No cohort was run.

### Symbolic arithmetic and phase accounting (T-795)

`meter::Snapshot` records actual calls to scalar add, multiply, negate and
explicit inverse helpers, standalone modular reductions, sparse accumulation,
payload visits, parent hops, partition visits and primality checks. It is
thread-local and includes rejected-row work. Multiplications inside inversion
remain counted; Miller--Rabin powers never count as inversions. Standalone
reduction events exclude the remainders already included in add/multiply.
Residual diagnostic inversion/update counts overlap this ledger and must
not be charged again as separate arithmetic.

The producer now records `solver_setup` (node map, partition, both primality
checks and graph/basis construction), plus symbolic deltas on every measured
phase. It asserts that these phase deltas sum to the target's symbolic total.
Direct pivot lookup replaces the two whole-basis scans. No performance gain
has been measured. Once-per-curve geometry setup remains charged cold to
each target under the existing convention.

These are semantic events, not CPU instructions or complete allocation and
index accounting. The attribution invariant covers metered events only.
Full cost remains marked incomplete. Tests now include eleven library tests
and three producer tests, with explicit inverse/primality/rejection and
phase-attribution regressions. Independent read-only review found no concrete
instrumentation or lookup-equivalence bug.

### Optional geometric producer (T-793)

Build with `cargo build --locked --release --features producer --bin
pencil-symbolic`. The producer is a scoped fork of the frozen pencil source
blob `e974121a123e13316ce3fcc820b4eca3f65a8a3a`; the original stays unchanged.
It preserves the rational-point sieve, complement factor checks, Jacobian
relation identity, source verification and same-process rho control. There
is no independent dual-norm check in this cohort path. The source's 16-point
subgroup projection diagnostic is retained; it is not a universal proof.

The only execution mode is `--cohort BITS P`, run from the repository root.
`PENCIL_SMALL_DENOMINATOR` must be explicit (2, 4, 8 or 16). The partition
selects original identity-ordered columns divisible by that denominator;
both symbolic label columns remain small. Node order must be identity. No
value has been chosen by measurement or frozen for an experiment yet.

Rows rejected for three large variables retain their collection and
geometric-check work in the producer phases. New setup and target schema IDs
are `pencil-symbolic-cohort-setup/v1` and `pencil-symbolic-cohort-target/v1`.
Both explicitly report `incomplete_graph_and_allocation_accounting`; target
cost ratios are null. Old price analyzers must not consume these records.
Residual counters do not include graph work and are not total solver counts.

At T-793 delivery the optional feature passed ten library tests and two producer tests.
The latter compare signed coefficient encoding against the frozen eliminator
and ensure a constant-only inconsistent equation fails. They use algebraic
fixtures, not sampled curves or performance data. Clippy passes with warnings
denied. Review caught the missing constant-only check and an inaccurate copied
dual-norm comment; both were corrected. Legacy peeling allocation and dead
execution modes were removed. No geometric cohort was run. Full graph
accounting, process resource limits, protocol amendment and experiment
admission still precede execution.

### Composed relation adapter (T-792)

`collector::Collector` now owns both stages and one immutable global-column
partition. Its constructor binds their field and small-column numbering.
It accepts already checked homogeneous coefficient rows; it does not verify
points or generate pencil geometry. The caller supplies which columns are
small, including any symbolic labels. There is no adaptive partition or
performance-selected default.

Duplicate terms cancel before the graph decides whether a row has more than
two large variables. Such rows return `RejectedThreeLarge` and are counted
without entering either stage. Residual cap outcomes stop the collector
permanently and preserve the unincorporated constraint for inspection. Further
calls return `AlreadyStopped` without processing. The pending constraint and
graph payload remain outside the residual basis limit; this is still not a
whole-process memory cap.

The independent prefix test now also checks the composed collector's row
space at all 25,600 prefixes. Three adapter regressions cover duplicate-driven
admission, terminal rejection, interleaved partitions and a negative cycle.
Ten tests pass. These fixtures are development checks, not additional
cohort outcomes. The next producer integration must preserve the existing
Jacobian relation checks, freeze a partition on the original workload IDs,
charge rejected geometry and all symbolic work, and respect the upper gate.

### Retained residual basis (T-791)

`residual::Basis` retains the emitted small-row space as a normalized sparse
echelon basis. Feed every `Outcome::Constraint` to it; only `Independent` and
`Dependent` mean the equation was incorporated. `WorkingCap` or `RetainedCap`
leaves the basis unchanged and requires the caller to stop or preserve the
unincorporated input. Continuing while discarding it would lose information.
The graph is not rolled back by a residual rejection; an integrated collector
must treat rejection as terminal unless it explicitly retains the row.

Limits count coefficients, not bytes. The working limit applies after every
map update, including input canonicalization; input order can therefore affect
when a cap fires even when later terms would cancel. Caller-owned inputs,
forest storage, tree-node overhead and allocator memory lie outside these
limits. They are not a process memory guarantee.

Logical counters retain attempted input terms, pivot lookups, coefficient
updates, normalization multiplications and inversions, including failed
attempts. They omit map comparisons, allocation costs, primality setup and
the underlying multiplication work of exponentiation. They cannot establish
a full cost ratio. All 25,600 prefix comparisons now also check the retained
basis against the independent dense reference; two new tests check caps,
duplicate cancellation and malformed-input behavior. Seven tests pass.

This implements downstream symbolic elimination only. Collector wiring and
complete cost accounting remain open; the existing four Lean lemmas do not
prove this additional Rust module.

1. Freeze the small-base partition and admission policy on the original
   1,260 public workload identities. Preserve Jacobian relation checks.
2. Account for every vector traversal, coefficient operation, parent hop,
   allocation and peak retained payload, plus downstream small-row elimination.
   Union by size bounds tree depth, not symbolic payload density.
3. Add an independently checked collector adapter and matched rho control.
   The existing operation-price grid cannot be copied onto this new kernel.
4. Run admitted cohorts under their resource and recovery contracts and
   retain every cap, failure and null cost. Report full cost only after
   completion and independent source verification.

The report, graph and `checks.csv` expose these boundaries for review. No
claim of lower memory, lower runtime, sub-rho cost, or full-cohort completion
is made.

## Retained-data admission audits (2026-10-30)

The [enrollment audit](validation/p739_enrollment_audit_20261030/README.md)
verifies the60 gate workloads, three resolved arms and1080 exact execution slots
against the cohort16 subset. Its correctedv2 preserves900 differing sequence
positions, rejects unexpected execution fields and retains the original packet.
See its [PDF](validation/p739_enrollment_audit_20261030/v2/output/report.pdf),
[graph](validation/p739_enrollment_audit_20261030/v2/output/mapping.svg) and
[exact mapping](validation/p739_enrollment_audit_20261030/v2/output/audit.json).

The [cross-session audit](validation/p739_cross_session_audit_20261030/README.md)
checks deterministic solver outcomes and operation accounting between the retained
computation and fresh sessions. It joins records to their original plans and
checks distinct measured replay coverage. Its [PDF](validation/p739_cross_session_audit_20261030/output/report.pdf),
[graph](validation/p739_cross_session_audit_20261030/output/equivalence.svg) and
[JSON](validation/p739_cross_session_audit_20261030/output/audit.json) preserve
timing, observed resource and environment exclusions. This is same-instrument
consistency, not independent-implementation agreement or clock qualification.

These audits execute no solver and generate no new plan or measurement. They do
not by themselves admit gate reuse, change native aggregation/export enrollment,
or authorize remaining cohort execution. The prospective continuation contract,
resource ceiling/deadline and full1260-target pencil completion remain open.

## Disjoint enrollment preparation (2026-10-30)

The [preparation packet](validation/disjoint_enrollment_20261030/README.md)
preserves one native remainder-plan generation and its successful solver-free
verification. The original seven plans and enrolled eight plans match across
1,260 workloads and 22,680 slots, including 18,900 measured executions and 3,780
warmups. The gate and remainder do not overlap. Original and enrolled sequence
numbers and workload indices remain in the exact mappings.

Review the [PDF](validation/disjoint_enrollment_20261030/report/report.pdf),
[graph](validation/disjoint_enrollment_20261030/report/enrollment.svg),
[standardized report](validation/disjoint_enrollment_20261030/report/report.json)
and [artifact publication receipt](validation/disjoint_enrollment_20261030/artifact-publication.json).
Both mapping files were uploaded and downloaded again to verify their hashes.
This is preparation equivalence, with zero solver executions. Scientific reuse
admission, analysis/export integration, later-stage resources, full pencil data
and timing requirements remain unresolved.
