# Source-bound mathematical workload cost analysis

This native Rust analyzer consumes public synthetic known-answer mathematical
workload records. It runs no experiments, generates no targets and makes no
key-recovery claim. The finite algorithm model retains the inherited 36 price
cells (S/M=0,.5,1; I/M=1,10,100,1000; logical GAE=0,.1,1). All reported prices
and ratios use checked exact u128 fractions; overflow is an error.

Run `cargo run --offline --bin analyze_pair -- INPUT.json` to emit one
`pencil-price-pair-output/v1` JSON object. `--mapping` emits the standardized
mapping contract and its source SHA256; `--mapping-sha256` emits only that digest.
This is a pair-row analyzer, not cohort selection or a cloud launcher.

Input schema `pencil-price-pair-input/v1` contains p,r,protocol_sha256,candidate,
reference,coverage_review. Each arm contains the original ecbench `record` plus
an `expected` identity: session,binary_sha256,seq,round,warmup,arm,role,seed,
workload,method. Expected identities must come from the retained frozen plan;
passing an attacker-edited expectation is not authentication. The mapper checks
exact record identity, matching workload/round, matched source-addition unit,
prime-domain identity and independently verified known-answer outcome flags.
It retains the complete input as raw_input in the output. Missing counters
remain absent, never zero. Changed schema, negative/overflowing counters,
geometry partition disagreement or identity disagreement is rejected.
`session` may be null only while the corresponding record is also null for a
prospective, unexecuted plan; a present record always requires a concrete
session identity.

The importer projects current candidate field/native arithmetic, symbolic and
cold_setup semantic counters, current outer-primality observations, rho counts,
and engine geometry/calibration blocks. Cold setup field/arithmetic is already
included in candidate totals; cold setup semantic counters are a distinct
snapshot and are added once. Scalar inverse-expanded multiplies are removed
once before atomic scalar pricing. Base inverse-expanded multiplies are likewise
normalized; primality powers are real noninverse M/S events in their own domains.
The exact landed outer ledger is detail.cold_setup.outer_primality, with separate
optional base/scalar calls and domain checks. Geometry totals are independently
recomputed from their four adapter segments.

The executable mapper adds actual constructor primality M/S/reductions, disjoint
collection/map primitives, inherited hash/RNG categories and one native copy unit
per cloned serde Value node. Byte counts remain supplemental. Candidate
symbolic.semantic_counts.walk_label_calls and setup equivalent are required;
reference uses actual RNG draws, partition hashes and observed table hash calls.
API-call, hash-probe, loop and row-update aggregates are diagnostic and are not
added over their primitive constituents. The standalone typed low-level price
API remains available for tests; constructing its Coverage struct is not a review.

Admission requires an independently authored
`pencil-algorithm-coverage-review/v1` record with status reviewed_complete,
policy matching the executable, mapping_sha256, protocol_sha256 and protocol_path,
rust_toolchain=1.99.1, distinct producer_identity/reviewer_identity, binary_sha256
array, source_files [{path,sha256}], independent_report {path,sha256}, and nine
category entries. Categories are field_arithmetic,scalar_arithmetic,
outer_and_constructor_validation,setup_transfer,collection_graph_elimination,
hash_rng,source_verification,metadata_clone,calibration. Every entry must have
disposition covered, empty uncovered_sites and nonempty source_sites containing
sha256,line_start,line_end,primitive. Source sites must bind validated source
file hashes. Source, protocol and independent report files are read and hashed;
all binaries and the exact mapper must match. This validates the evidence chain,
not the truth of an arbitrary review or cryptographic authentication of its author.
The caller must obtain the independent review through the repository's approved
review process. No such scientific review is created by this implementation.

Missing review, uncovered categories, missing observations, warmup or any
unverified/capped/failed pair keeps all 36 pair prices and ratios null with
explicit coverage_blockers. Attempted work remains in raw_input. On admitted
pairs, candidate/reference algorithm costs and ratios are provided alongside
calibration costs and explicit algorithm-plus-calibration costs/ratio. Calibration
preparation formerly reset away is included once; the source-addition unit is
only the normalization denominator, not an excuse to omit calibration work.

Report JSON/serialization/digest work, observer traversal, allocation, RSS,
process/harness/audit clocks, infrastructure and retries remain separate units
in raw records; they have no invented GAE conversion. Engine report-assembly
snapshots do not certify all JSON instructions, nor is that the finite original
algorithm-price requirement. Whole cold clocks include instrumentation overhead.
Metadata clone inside fresh() is actual method work and receives the declared
node-copy price. Parent auditing remains explicitly separate.

Cohort publication still requires complete frozen-plan coverage and ratio of
sums over exactly matched cells, including failures as null outcomes, rather
than a mean of ratios or survivor-only aggregate. Isolation/A-A gates, workload
seal authenticity, source coverage review and complete cohort denominator remain
independent requirements. No observed performance result is produced by tests.

Software tests include exact hand prices, 36-cell enumeration, scalar inverse
normalization, malformed/missing counter and identity rejection, checked overflow,
source-review digest mutation, false coverage rejection, geometry recomputation,
and a clearly synthetic positive review fixture that exercises both pure and
calibration-inclusive ratios. Synthetic review fixtures are never experiment
evidence. Raw test/clippy logs are retained in the isolated accounting deliverable.

## Complete prepared-plan aggregation

`cargo run --offline --bin analyze_cohort -- MANIFEST.json` emits
`pencil-price-cohort-output/v1`. Append `--summary` to omit large per-slot,
per-pair and control arrays while retaining their counts, all aggregate cells,
input identities and plan observations. The complete mode retains those rows.

The manifest is `pencil-price-cohort-input/v1` with:

- `prepared_plan_validation`: `{path,sha256}` for the existing
  `pencil-prepared-plan-validation/v1` result (the study's
  `prepared_native_plans/result.json`), containing the actual plan filename/digest
  map and declared execution counts;
- `binary_sha256`, `protocol_sha256`, and the pair mapper's `coverage_review`;
- `plans`: one `{name,path,sha256,session_id,records}` per file in that receipt,
  with `records` either `{path,sha256}` for JSONL or explicit null when absent;
- optional `separate_resources`: `{path,sha256}` for retained JSON resource,
  attempt, audit and infrastructure observations. These are preserved in their
  own units, not summed into GAE or overlapping method clocks.

The aggregate validates actual `ecbench.plan/v1` arms/workloads/executions using
existing `pencil-paired-export::export_with_plan`. It enforces the original
seven plan names and SHA-256 digests before entering the default 1,260-workload
route. It then enforces five measured and one warmup round per workload and the
three exact pencil/rho/rho_AA arms. This yields22680 expected execution slots,
18900 measured executions,3780 warmups and6300 measured candidate/rho pairs.
Actual workload identities and plan bytes are checked; matching cardinality
alone is insufficient. Missing entire targets remain expected missing rows.
All supplied records, including warmups and controls, must match the plan and
session/binary identity. Duplicate records are errors; no attempt is silently
selected or overwritten. Separate retry receipts belong in the retained
resource artifact under the existing frozen attempt-selection rule.

For each price cell the aggregate sums exact per-pair normalized prices before
dividing, across all6300 pairs and separately for each of five1260-pair streams.
It never averages ratios or converts aggregated raw field counts using one
model's source unit. A missing or ineligible pair nulls that scope's sums/ratio;
partial admitted sums remain explicitly diagnostic. Warmups are excluded from
measured numerators/denominators and their missing counts remain visible.

The reused A/A rule is **per target**: each of its five measured rho_AA/rho
pairs must individually lie in inclusive[0.90,1.10], separately for cold and
online clocks. One failing round cannot be rescued by a passing mean. Timing
control status remains distinct from mathematical/count eligibility.

Provided hashes establish byte integrity, not plan authenticity, protocol
freezing or launch authorization. The actual prepared-spec manifest explicitly
says not_frozen_not_launch_authorization; the prepared-plan validation receipt
also is not a substitute for independent review. This tool does not invent a
seal or an approval process. `qualified_ratios` remains null and authenticity
is explicitly unestablished by the tool, while complete, independently covered
algorithm prices may be reported as ratios of sums under the provided plan.
Reviewers must use the repository's existing freeze/export/review evidence to
qualify them. Resource observations are retained separately and missing ones
are null. Software fixtures and plan-only validation with zero records are not
cohort evidence.

### Exact p739 first-gate scope

The default `analyze_cohort MANIFEST.json` remains the 1,260-workload cohort.
`analyze_cohort MANIFEST.json --p739-gate60` selects only the existing prepared
60-target p739 gate. Its manifest must contain `analysis_scope` equal to:

```json
{"kind":"p739-gate60","expected_workloads":60,"prepared_plan_sha256":"bc57819e43862f5d9c9aee0f13b983fec186b2dd14dbc8b921d8999acc4b6776"}
```

Exactly one `gate-p739.json` plan with these verified bytes is required. This
fixes membership, arms, seeds and execution order; arbitrary smaller cohorts are
rejected. The same aggregation arithmetic produces 300 measured pair slots,
1,080 execution slots including 180 warmups, and retains missing rows/null ratios.
The gate route is analysis support, not launch permission or cohort completion.
The canonical exporter additionally requires a frozen protocol with
`sample_size.instances:60` and identical `extensions.analysis_scope`.

### Exact upper-gate scope

`analyze_cohort MANIFEST.json --p186379-target0` accepts only the retained
`gate-p186379-target0.json` plan with SHA-256
`481d795e6208cfb3ca648c463de2d7b232adfb7d28bebc341afa4f58e119dfcf`.
It retains exactly one workload, five measured pairs and eighteen execution
slots. This prerequisite is charged separately and is never enrolled as cohort
coverage.

### Opt-in disjoint p739 continuation

`analyze_cohort MANIFEST.json --full1260-disjoint-p739` accepts only the
reviewed eight-plan enrollment identified by `full1260-disjoint-p739/v1`. It
invokes the separately compiled and hash-bound native-preparation verifier,
then checks the exact request, certificate, 1,260-workload/22,680-slot mapping,
and per-plan protocol/coverage/history lineage. The prior p739 gate must carry
its actual records, session, history and a scoped reuse-admission record. All
seven prospective plan entries must keep `session_id`, `records` and
`execution_history` null until data exist. Missing rows remain in the complete
denominator and all affected ratios remain null. This route neither authorizes
execution nor qualifies timing, and the p186379 prerequisite remains separate.

The opt-in manifest adds `enrollment` with exact `request`, `certificate` and
`protocol_lineage` file bindings, `output_directory`, and a `verifier` object
containing separately hash-bound native binary and source bindings. Lineage
schema `pencil-disjoint-protocol-lineage/v1` binds the continuation protocol,
the known-prior-data declaration, and exactly eight named plans. Every plan
declares its plan digest, `prior_gate` or `prospective_continuation` origin,
full-file data-protocol binding and identity, coverage-review object, session,
records and execution history. A prospective execution triple must be either
entirely null or entirely present and hash-bound; mixed or fabricated identities
are rejected. The gate reuse admission additionally fixes its plan, protocol,
records digest, session and observation-receipt digest.
