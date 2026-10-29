# Symbolic signed-graph reduction for pencil relations

This is a mathematical implementation step toward the still-incomplete
1,260-target pencil cohort. It contains an exact finite-field row reducer,
independent unit tests, and four field-identity Lean lemmas. Its library
returns symbolic linear constraints. An optional producer integrates the
library into the existing synthetic pencil cohort path; it is compile-tested,
not yet run as a cohort.
No cohort, EKS job, or timing experiment was run for this packet. Candidate/rho
cost and the remaining 717 candidate outcomes are unmeasured here.

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
were added before delivery. Current validation is 13 default library tests,
14 producer-feature library tests and three producer adapter tests. Cohort
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
