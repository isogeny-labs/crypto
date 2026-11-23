# APS local measurement preparation

Status: bounded structural implementation and proposed performance design;
not a frozen performance protocol or speed result.

The requested execution backend is the current local machine. This changes
the default EKS destination; the prospective checkpoint, accounting, replay,
protocol and admission requirements remain applicable. Preserve the earlier
frozen source boundaries and receipts.

## Scope of the next implementation

The matched comparisons remain those in [PROTOCOL.md](PROTOCOL.md): checked
trace reuse, the complete line-chart decomposition, unequal profiles and
elimination choices, and retained full versus disjoint frontier growth.
Each comparison uses the same exact input population and resource law.
The supported proposed size classes remain 8, 10, 12 and 14 bits.

The local runtime must bind the source-built ecbench and separate
aps_scalar_check executables, the compiler/features, profiler version and
configuration, source revision and native analysis. Refer to
[ACCOUNTING.md](ACCOUNTING.md) for the inclusive method and scalar-check
frames. Check binary identities independently and retain all failures,
recordless attempts and missing profiles.

The maintained public build contracts reviewed at main
0b4f472f05c4199491ec8d052cb07b3f31a33479 package ecbench and
isolab-fresh-owner. The prospective APS path also needs scalar-check custody
and profiler support. Proposed changes must cover the build bundle, image
contents, artifact transfer, schema-valid build identity and independent
verification together. Extend negative identity tests; do not weaken
existing checks or silently add unsupported schema fields.

## Partition leads and first experiments

The following are candidate mechanisms, not new literature claims or measured
improvements. Cosets and low-degree maps already appear in the
[source review](../aps_leads_20261105/SOURCES.md). Start with structural
correctness experiments; performance collection remains subject to the
admission steps below.

| Lead | Proposed partition rule | Falsification target |
| --- | --- | --- |
| Solver signatures | Bucket coordinates by cheap specialization features: degree/support and leading-coefficient zero patterns under fixed other domains | Reject a speed interpretation if signature construction plus solving costs more than the matched ordinary partition, or yields fewer useful relations without compensation |
| Coloring families | Try a bounded, deterministic family of point-to-slot colorings; assign each represented tuple to its first qualifying coloring | Reject any completeness assertion if exact enumeration finds an uncovered tuple; retain and report partial coverage |
| Rational-map fibers | Bucket known labeled coordinates through a fixed low-degree rational map | Reject if denominator handling, label acquisition or elimination of the graph relation erases the saving |
| Coefficient cancellation | Search a labeled pool for subsets whose vanishing polynomials have prescribed missing coefficients | Charge the search and reject if the reduced relation system is no easier at matched coverage |
| Pair-sum compatibility | Form compatible pair blocks using their actual group sums before algebraic solving | Charge construction, memory and joins; compare with an ordinary group join as well as the polynomial baseline |

### First experiment: coloring coverage, without timing

Question: can a bounded family recover tuples excluded by a single partition
without duplicating tuples in the represented union?

Proposed native scope: a small partition-lab module plus focused tests, with
no change to existing solver identities or frozen receipts. Begin with
synthetic distinct point IDs, arities three and four, population at most
twelve IDs and a family cap of thirty-two colorings. This is a combinatorial
correctness experiment, not a solver or timing experiment. Stop at the cap;
do not silently add colorings until a desired outcome appears.

Controls: one fixed coloring, repeated identical colorings, renamed colors,
an intentionally uncovered tuple and an exhaustive small-domain reference.
Compute the qualifying-coloring predicate separately in the reference.
Require exact equality between the represented union and reference union,
and exactly one first-owner assignment for each represented tuple. An
incomplete family must return its explicit uncovered population. Distinct
point IDs, original slot labels, group signs and coordinate canonicalization
are separate concepts; do not infer one from another.

Only after those checks should real labeled APS pools be integrated. Freeze
the color family before held-out relation outcomes are observed. Retain
original point representations and informative-relation checks. Build and
validate an independent full-domain relation set, then compare the union of
family cells with that reference. A tuple excluded by the family is a missing
decomposition, not evidence that the method cannot solve the target.

### Second experiment: specialization buckets, without timing

Question: can cheap signatures predict compatible certified trace reuse
without using relation outcomes as features?

Use fixed labeled pools and public input polynomials. Begin with degree,
support and leading-coefficient zero patterns only. Do not solve the
specialization to assign its bucket. Freeze the feature definition and
assignment before held-out targets. Compare random partitions, signature
buckets with trace disabled, and the same buckets with certified trace reuse;
this separates partitioning from the existing reuse mechanism.

Require literal-equation and complete-root agreement with the ordinary
reference. Include a poisoned trace, incompatible signature and degenerate
leading coefficient; each must retain the existing certification/fallback
behavior. Acceptance/refusal and coverage are diagnostic observations, not a
speed conclusion. Record all feature construction and fallback work for the
later admitted cost comparison.

### Implementation and execution boundary

The owner extended the Conductor exception to native partition-lab edits and
bounded local correctness experiments. The standalone
[partition lab](../../tools/aps-partition-lab/README.md) now supplies coloring
coverage and specialization-feature experiments, plus a tiny public
known-label group control. Its scope stops before production APS sampler
integration and certified F4 trace reuse. Those obligations are
not replaced with synthetic trace checks. Performance trials still need the
frozen design, prospective checkpoint and accounting/replay/admission below.
Register native claims before public citation of resulting observations;
registration does not authorize publishing the withheld native ledger.

## Before any measurement

1. Obtain the existing project's native Conductor task and current scope.
   Do not invent ownership or create a competing project.
2. Implement and validate the local profiling and binary-custody path.
   Existing parser/correctness fixtures do not establish a performance result.
3. Generate and independently validate exact registered curves, subgroups,
   generators and targets. Freeze disjoint pilot and main populations,
   analysis code, effect threshold, uncertainty, power/sensitivity assumptions,
   masking, multiplicity and finite budgets before collecting performance.
4. Use current cost rules: calibrated field-operation prices alongside the
   governing sensitivity band; eligible counted table work; explicit unpriced
   work and excluded phases. An incomplete accounting record cannot support
   a complete S/GAE ranking.
5. Prospectively checkpoint the actual frozen run source under fresh fenced
   ownership and reviewed payload boundaries. The earlier source1bc
   acknowledgement does not cover a future protocol or runtime.
6. Admit local execution with explicit resource limits and qualify each metric.
   Profiler instruction counts are not wall time. Missing isolation or clock
   qualification must not become an elapsed-time speed claim.
7. Execute the frozen design, preserve unsuccessful outcomes, independently
   replay/export and analyze the complete population. Report bounded toy-size
   comparisons without extrapolating a cryptographic-scale attack.

No performance trial is represented by this document. Local validation
receipts remain separate and must acquire native claim records before public
citation under repository policy. T9/T13 broader measurement goals and T10
checkpoint/readback remain open.

## Publication boundary

This proposal does not publish the native ledger, private histories, raw
dependency/build logs, shared full journals/indexes or detailed host capsules.
The existing ledger publication and reconciliation hold remains. Keep
auto-merge off; opening a PR does not authorize merging it.
