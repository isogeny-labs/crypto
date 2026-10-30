# Dataset campaign operations

The native entry point is `crypto dataset`. Read [the design](DESIGN.md) before
using the data for a learning experiment. Each CLI invocation has a finite cell
budget. A planned cell, a saved known-answer instance, a solved point decomposition
and an independently recovered discrete logarithm are different records.

## Planning

`dataset plan` constructs a mixed-radix index without materializing the corpus.
Its default cross-product has 24 field sizes, 14 arities, three curve families,
two twist choices, 16 curve seeds, eight generators, 64 target keys and ten
methods: **165,150,720 method cells**. This is a derived planned count, not a
count of completed solutions. The checked-in
`research/ml_dataset_20261028/full-campaign-plan.json` materializes these axes
without expanding the cross-product. Its protocol digest is intentionally null:
it is a planning artifact and generation refuses it until a matching full-campaign
protocol and finite execution budgets have been frozen. Every curve/target configuration has both twist
choices. The prime field at n=61 is exactly 2305843009213693951.

Freeze an `experiment.protocol/v1` record with the shared research protocol
checker and commit it before generation. The plan binds the SHA-256 of the
complete protocol file. That file hash differs from the protocol's canonical
body hash in `freeze.protocol_sha256`; retain both.

```sh
crypto dataset plan --out campaign-plan.json --protocol-sha256 FILE_SHA256
crypto dataset capabilities
```

For a smaller frozen pilot, set axes explicitly:

```sh
crypto dataset plan --out pilot-plan.json --n 23 --m 2,4,15 \
  --families koblitz,binary,prime \
  --solvers bsgs,enumerate,root,sat,f4,f5,f6,weil,pkm,double-large-prime \
  --curve-seeds 1 --generators 2 --keys 2 --factor-base-size 4 \
  --degree-bound 3 --work-limit 20000 --protocol-sha256 FILE_SHA256
```

This pilot has 720 method cells. Unsupported method/parameter combinations
remain visible in results. A budget exhaustion is censored evidence, not UNSAT.
Natural random targets are kept separate from deliberately planted test
fixtures; passing a planted unit test does not make a natural pilot PDP solved.

## Execution and recovery

Build the binary from a committed source snapshot with
`CRYPTO_BUILD_GIT_COMMIT` set to that commit and `CRYPTO_BUILD_GIT_DIRTY=false`.
The generator refuses missing or dirty provenance. An output directory is
bound to the exact binary, plan, source, resource limits and shard.

```sh
crypto dataset generate --plan pilot-plan.json --protocol protocol.json \
  --out /work/dataset --max-cases 36 --shard-index 0 --shard-count 19 \
  --seconds-per-case 30 --memory-mib 2048 --trace-mib 64 --publish \
  --remote s3://setec-astronomy-artifacts/ml-dataset/20261028/store \
  --budget-policy /work/budget/policy.json --budget-grant /work/budget/grant.json \
  --budget-owner /work/budget/owner.json --budget-state /work/budget/worker.sqlite \
  --budget-cell-envelope /work/budget/cell-envelope.json
crypto dataset verify --out /work/dataset
crypto dataset status --out /work/dataset
```

S3 execution requires the activated, source-bound budget session described in
[BUDGET-CLI.md](BUDGET-CLI.md). The example assumes that finite grant has already
been issued and activated; it does not create spending authority. Grant issuance
uses preflight's `plan_sha256` for the native parsed-plan binding and
`protocol_file_sha256` for the protocol binding. Preserve the separate file
hashes as described in [BUDGET-CLI.md](BUDGET-CLI.md#prepare-one-authoritative-campaign).

Use a digest-pinned image on the repository's EKS Spot lab. Assign disjoint
shards before submission, record every Job identity, and inspect the existing
Job and S3 receipts before retrying a submission. Worker-local process leases
do not provide a distributed cloud lock. The ordinary throughput Job does not
establish CRIU recovery, isolated benchmark clocks or comparative speed claims.
The launcher must reconcile actual Job/Pod UIDs with each grant and distribute
it only to that worker; a matching supplied owner document is not Kubernetes
attestation. Follow the [launch-adapter contract](LAUNCH-ADAPTER.md) for this handoff.

Run `crypto dataset preflight --plan pilot-plan.json --protocol protocol.json`
as the actual container UID before launch. It checks readable inputs, the frozen
protocol, clean compiled source identity and configured lab receipt routing
without performing cryptography. It does not establish output-directory or
cloud permissions, validate or activate a budget grant, attest a Pod, or reserve
a cell. Input preflight does not replace admitted worker conditional PUT/GET
verification. Container input modes must grant UID 1000 read access.

The current lab service supplies a legacy `ISOLAB_RESULTS_URI`. Dataset receipts
must use the approved bucket: explicitly set a unique campaign/job prefix under
`s3://setec-astronomy-artifacts/ml-dataset-lab/runs/`. Keep the original service
URI and the effective override in the launch record. The EKS role must permit
get/put for the actual data, budget-control and receipt destinations; the native
budgeted transport does not list objects. The prepared policy also includes scoped
listing for older tooling. In every case,
a local role's successful upload does not establish worker access. See the
prepared [role policy proposal](../../research/ml_dataset_20261028/worker-s3-policy-proposal.md).
It has not been applied. Its scoped data remote is
`s3://setec-astronomy-artifacts/ml-dataset/20261028/store`.

The first pilot uses 19 shards so a bounded sequential shard crosses method,
twist and other axes. Derive observed coverage from saved configurations;
strides divisible by the ten-method axis can accidentally select only one method.

Reissuing the same generation command skips verified completed computation.
With `--publish`, it performs fresh publication checks for completed cells too;
those reads and retries consume budget although `--max-cases` counts only new
attempts. Keep the original per-cell envelope on resume. Standalone
`dataset publish` uses unallocated Work capacity rather than capacity held inside
existing Cell reservations; admit separate publication allowance for that route.
A partial cell requires explicit reconciliation:

```sh
crypto dataset reconcile --out /work/dataset --cell CELL_INDEX \
  --reason "Original worker terminated; external Job and owner reconciled"
```

Reconciliation checks local Linux processes and preserves the partial directory
under an immutable attempt name before allowing another attempt. It cannot
prove that a worker on another host has terminated. Retain all interrupted
attempts when moving a campaign to another host. Worker journals are bound to
their original absolute path and Linux host boot identity. A replacement host
needs a separately issued and activated grant after owner/usage reconciliation;
retain the original allocation without resetting or refunding it. Audit
checkpoints are evidence, not a spendable replacement journal.

## Restore from durable receipts

Publication saves a full portable `ml-dataset.publication/v1` receipt in the
remote catalog at `ml-dataset/catalog/NAMESPACE/MANIFEST_SHA256.json`, relative
to the data remote's prefix. Use the catalog key returned by publication; a
completion marker, catalog locator, status summary or enclosing lab receipt is
not itself the full receipt accepted by `restore`.

For the scoped pilot remote, download the exact catalog object to a local
ordinary file, then restore one cell at a time. On a fresh host, the following
budget paths refer to the reader's own separately issued and activated grant,
bound to its executor and the original data plan, protocol and shard:

```sh
crypto dataset budget fetch \
  --remote s3://setec-astronomy-artifacts/ml-dataset/20261028/store \
  --key ml-dataset/catalog/NAMESPACE/MANIFEST_SHA256.json \
  --sha256 CATALOG_SHA256 --bytes CATALOG_BYTES \
  --out /work/receipts/cell-publication.json \
  --budget-policy /work/budget/policy.json --budget-grant /work/budget/grant.json \
  --budget-owner /work/budget/owner.json --budget-state /work/budget/worker.sqlite
crypto dataset restore --receipt /work/receipts/cell-publication.json \
  --out /work/restored-dataset \
  --budget-policy /work/budget/policy.json --budget-grant /work/budget/grant.json \
  --budget-owner /work/budget/owner.json --budget-state /work/budget/worker.sqlite
crypto dataset verify --out /work/restored-dataset
crypto dataset status --out /work/restored-dataset
```

Create the local receipt directory before the download and replace the uppercase
components with the returned namespace, manifest digest and catalog locator
digest/byte length. The fetch requires exact expected content and does not list
or discover keys; its requests consume the same active grant as restore. Do not
substitute an unaccounted transfer command when claiming cumulative caps.
Restore uses the
remote recorded in the receipt. Its optional `--remote` argument must identify
the same remote; it cannot redirect proof to a mirror. The supported S3 bucket
is `setec-astronomy-artifacts`; `file://` remotes are also supported for local
round-trip validation. Restore requires an already activated journal: the separate
`budget activate` command writes its immutable control object; restore itself
reads remote data and retains local accounting checkpoints.

Each invocation freshly downloads the completion marker and all cell objects,
checks lengths and SHA-256 digests, and replays native public-instance and
solution certificates before activation. All cells accumulated under one
output must share the exact plan, protocol, source, producer binary, resource
caps and shard binding. The producer executable is not needed for read/replay;
resuming generation or reconciling a producer attempt still requires its
original clean build and binary.

Activation currently requires Linux `renameat2(RENAME_NOREPLACE)`. Restore never
replaces an existing cell. An identical completed cell is rechecked remotely and
replayed; conflicting or incomplete destinations fail. Failed and successful
download attempts remain under `.restore/` for inspection. Allow disk space for
retained retries as well as active cells. Earlier completed cells receive hash
and provenance checks on each append; their arithmetic is not replayed again
unless they are the target of that restore. Repeated individual restores still
perform repeated local hashing and are not a batch-scalability benchmark.

## Reader and export limits

Native JSON metadata readers accept ordinary, non-symlink files of at most
**32 MiB (33,554,432 bytes)**. This includes frozen protocols, plans, run and cell
manifests, restore receipts, and JSON artifacts parsed during verification or
feature projection. Raw trace files need not fit this JSON limit when they are
only copied and hashed. Size declarations do not substitute for fresh length
and digest checks.

Restore also has a fixed **1 GiB (1,073,741,824 bytes) aggregate budget per
receipt**, counting its completion object and every listed cell object. It
checks declared sizes before downloads and each remote object's reported size
before fetching it. Larger receipts fail explicitly; no data is truncated and
there is currently no CLI flag to raise this ceiling. The producer's
`--trace-mib` does not override a reader's limits. The ceiling bounds a single
restore's declared object volume, not total retained retry storage or process
RSS. This stricter restore ceiling is separate from publication's per-object
4 GiB limit.

Training export caps each serialized row at 64 MiB and uses 16 MiB sort chunks
with merge fan-in 8. Coverage reports cap metadata and rows at 32 MiB and use
8 MiB chunks with fan-in 8. Individual source JSON values are materialized;
these bounds are not a total process RSS guarantee. An oversized required input
or output row causes an explicit error rather than a partial success record.

## Artifact and measurement contract

Each sealed cell includes protocol, plan, run binding, configuration, process
records, logs and a manifest binding every emitted file by length and SHA-256.
Successful construction adds the exact public curve and target, restricted
scalar/provenance and factor base. Backend artifacts and phase telemetry reflect
the work that actually ran; a sealed failure need not contain a constructed
instance or a completed solver trace. Save raw witnesses and seeds as restricted
labels, never model inputs.

Publication uses the content-addressed store and a namespace derived from the
complete run binding. Uploads are verified by downloading and hashing every
object, including the final completion receipt. `status` checks the recorded
acknowledgement; `publish` performs a fresh remote check. No receipt is evidence
that the underlying algorithm completed unless its outcome and replay say so.
Objects larger than 4 GiB must be chunked; this implementation rejects them
because the existing multipart store path does not guarantee immutability.

S3 can return `403 Forbidden` for a missing key when the caller lacks
`s3:ListBucket`; that response does not prove absence. The dataset publisher
handles this ambiguity only for the approved bucket: it attempts the existing
backend's conditional single-object `PUT` with `If-None-Match: *` and a SHA-256
checksum, then requires a successful fresh `HEAD` and full download/hash check.
It applies to both content-addressed objects and portable catalog receipts.
An existing key cannot be overwritten, and denied writes, failed reads,
conflicts or mismatched bytes cannot produce a successful publication receipt.
Other `HEAD` errors fail without this fallback. This behavior is local to the
dataset publisher; it does not change the shared store or the unapplied IAM
proposal. Bucket-wide listing is not required for this creation path; object
read/write permissions are still required. Retain the exact catalog locator
returned at publication: the budgeted transport disables LIST. External catalog
discovery is outside this native accounting path and must not be represented as
metered by it. See AWS's [HeadObject permissions](https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadObject.html)
and [conditional-write permissions and behavior](https://docs.aws.amazon.com/AmazonS3/latest/userguide/conditional-writes.html).

Hardware counters use the existing native PMU instrumentation for construction,
known-answer replay, factor-base replay and solver phases. Save raw counts,
scaled estimates, time enabled/running, inheritance and errors separately.
An unavailable PMU produces a null instruction label with its reason. Counts
cover user-space work in the calling thread and newly spawned descendants when
inheritance succeeds. They exclude process startup, publication and unrelated
preexisting threads. Multiplexed counts are estimates, not exact instruction
labels. Interrupted phases retain their start record but may lack a final PMU
read. Wall times on shared workers are diagnostic.

The supervisor polls worker output before manifest sealing and stops work when
the trace threshold is exceeded. This can overshoot and is not a hard checked
filesystem-write limit. Manifest and publication metadata are accounted
separately; stdout, retained retries and accounting checkpoints also need space.
The supervisor preserves partial files after a killed call; complete native
algorithm internals still require
explicit trace hooks. Capability records name those gaps. Exact first-fall and
last-fall degrees remain null until a checker establishes their stated algebraic
definition and all required closure/rank conditions.

The prime F4 increment at source `69e4c9763bed508f0ab0d2f424873d2e748a7229`
passed 11 focused native/adapter tests, the full release library suite
(5,153 passed, zero failures, 123 ignored), release build and actual CLI checks.
The separate endpoint test passed all 360 adapter combinations, including
structured unsupported and capped outcomes; it does not establish 360 solves.
The earlier strict-lint blocker was three unregistered startup helpers.
Source `7a1bd350b26eaeccf51bf9078915649a7c4335fc` registers the startup CLI and
central stored-input verifier. It passed 142 dataset tests, 26 F4 unit tests,
one native checkpoint integration test, the full release library suite
(5,170 passed, zero failed, 123 ignored), strict maintained Clippy, a release
build and eight actual CLI checks. A separately frozen two-twist CLI fixture
passed ten commands, including rejection of rehashed equation/domain corruption.
These are software checks; cloud validation remains open.

Combined runtime source `f4249f62df082b401717a03ae2cae1b7b1aeddd3` integrates
main `5a250bd47f7674b1f8fbc8a5ca577805aa235ecc`. Fresh validation passed the full
release library suite (5,170 passed, zero failed, 123 ignored), strict maintained
Clippy, both changed hyperelliptic catalog tests, a rebuilt release CLI, eight
CLI checks and the ten-command two-twist fixture. Independent audit verified
158 inventory files and 159 archive members. Evidence is retained separately in
`research/ml_dataset_20261028/validation-startup-main-20261029/`; earlier-source
records remain unchanged. PR [#2836](https://github.com/isogeny-labs/crypto/pull/2836)
carries this increment; exact-head remote checks and merge status are separate
from these local results. This adds no research corpus rows or verified cloud
cells.

`f4` keeps its existing m=2 reference path and selects
`prime-affine-s3-chain-f4-visitor/v1` for prime m=3..15. This new strategy records
leaf membership equations and unrestricted auxiliary-variable roles before
solving. Native elimination requires p<2^32. For larger moduli, including
2^61-1, the adapter retains exact equations computed with wide intermediates
and reports an unsupported native modulus; it does not substitute a smaller
field or invoke the narrow reducer.

The new lane writes `prime-f4-input.json`, a bounded
`prime-f4-chain-events.jsonl` prefix, and `prime-f4-chain-result.json` termination
metadata. Events expose recursive systems and fixed-variable bindings, native
bases/useful-row schedules, branches, candidate validation and independent
point-sign lifts. Event/byte caps and sink errors stop traversal explicitly;
root work, elimination observations and group additions retain separate units.
A native F4 call materializes its own rows before serializing a bounded event.
These event limits therefore do not bound native matrix memory; retain the
worker memory/time supervisor. Useful-row traces omit private pivots, zero rows
and internal univariate factorization/GCD states. The inherited top-level
`checkpoint_mid_call` flag reports live exposed-prefix sink persistence; it
does not certify native-call or process recovery. Read the separate supervisor
checkpoint field and explicit trace gaps before interpreting recovery support.

Read `encoding_search_completed`, `algebra_completed`, `termination` and the
trace/lift limits together. An encoding traversal may complete after truncated
native elimination without establishing a complete Groebner basis. Infinity
exceptions are not all encoded, so completion without a verified lift never
establishes PDP impossibility: `exhaustive_no_decomposition` remains false.
A verified positive witness can coexist with a censored trace. If live persistence
fails after a positive, the chain stages its certificate and returns explicit
`live_persistence_errors` for a separate canonical worker write. A true
read-only-event-descriptor regression verifies this path; total storage failure
still cannot establish durability. Neither state
establishes scalar recovery or an exact first/last fall degree.

Generation and central `verify_cell` invoke the public equation/domain
reconstruction verifier. Semantic replay tests on source `7a1bd350b26` cover
rehashed corruption and omitted inputs with stripped strategy tags. The frozen actual-CLI fixture passed replay on both twists and rejected
rehashed corruption in a copied twist=false cell; cloud restore remains a
separate validation step. The frozen
[prime-chain contract](PRIME-F4-CHAIN-CONTRACT.md) records required checks and
mathematical limits; no new cloud-generation evidence follows from this code.

Create a coverage snapshot in a directory outside the source run:

```sh
crypto dataset report --out /work/restored-dataset \
  --destination /work/dataset-report-001
```

The report writes `coverage.jsonl`, `measurements.jsonl`, `artifacts.jsonl`,
`observed-cells.jsonl` and `summary.json` with schema
`crypto.ml-dataset.coverage/v1`. It includes active sealed cells, active partial
attempts and retained interrupted attempts, without inventing unattempted cell
rows. It hash-checks saved records without rerunning solvers or arithmetic
certificates; its solution labels are explicitly recorded labels. Raw PMU
values, enabled/running times and scope stay in the measurement records. Use
`dataset verify` for arithmetic replay. Local publication metadata in a report
does not establish a fresh remote round trip.

Report inputs must be idle locally and source/destination trees must be
disjoint, with no symlink ancestors. Output files are immutable: repeating the
same snapshot is allowed only when the bytes match. Choose a new destination
when new cells, interrupted attempts or publication records change the snapshot.
Reports contain restricted source references and telemetry; they are not model
input files.

Native report completeness is distinct from the standard research export: the
full pilot maps 720 method cells to 144 arm-instance bundles, while a 36-cell
wave remains partial. The pinned `experiment.export/v1` checker has no
partial-panel exemption. Neither `dataset report` nor `dataset export` produces
that research schema or claims to pass its checker.

## Before training

Create the derived training projection from a local restored or generated run:

```sh
crypto dataset export --out /work/restored-dataset \
  --destination /work/dataset-training-001
```

The v6 implementation replays sealed cells and writes `features.jsonl`,
`prefix-features.jsonl`, `algebraic-features.jsonl`, `restricted/targets.jsonl`,
`restricted/index.jsonl`, `restricted/prefix-records.jsonl`,
`restricted/algebraic-records.jsonl` and `manifest.json`.
It performs no remote download. Only the three feature files are eligible model
inputs; targets
contain known-answer and recovered scalars, solver labels and certificate
references, while the restricted index retains source manifests and artifact
references. Keep the restricted directory access controls when copying it.

Feature identities hash the strictly projected public instance, actual factor
base and `pdp_domain`; solver names and source provenance do not enter the hash.
Ordinary methods on the same inputs can share a feature row. PKM uses its
separately verified Kummer base and the `prime-kummer-full-curve` domain, so its
outcomes cannot be matched to `cyclic-subgroup-factor-base` routing rows.
PKM cells without a saved, verified alternative base retain restricted target
and provenance rows but have a null feature identity and an explicit exclusion
count. Large-prime rows use the actual saved small and residual domains, checked
against the independently replayed relation transcript. Signed Frobenius closure
and identity/sign columns have distinct domain identities; neither is matched
to ordinary PDP rows. Their structural inputs retain public column actions and
state that each query decomposes `aG+bQ` into `m-2` small points and two residual
points. Scalar certificates must agree with that transcript. Missing domain or
transcript artifacts leave a null feature identity, an explicit exclusion count
and retained restricted provenance; conflicting domains fail export. Unsealed
attempts appear in the restricted index without fabricated feature or target rows.

Use a separate destination outside the source tree. Export refuses the source
root, its ancestors and paths inside `cells/`, and rejects symlink ancestors.
Exports are immutable snapshots: identical retries verify existing bytes; a
changed source or export policy requires a fresh destination. The current
implemented policy is `strict-structural-allowlist-v6`. It adds the consequence
input file while retaining the static and branch input identity rules. Keep
historical v5 exports immutable; use a new destination for v6 and verify the
manifest policy before selecting model inputs. Completed source-bound software
validation is recorded in the [consequence implementation status](../../research/ml_dataset_algebraic_targets_20261030/STATUS.md).

Prime-chain recursive events and termination records stay in restricted
artifacts/provenance. The v4 exporter adds a `prime-chain-graph/v1` static equation
graph for eligible replayed prime/F4/m3..15 cells, with lossless decimal values,
shared finite/symbolic domains and signed-point incidence. Its inverse decoder
must recover the exact reconstructed input before publication. Missing input or
a graph-size refusal retains restricted provenance with no generic feature
fallback; malformed input fails export. Earlier v3 snapshots stay unchanged.
Source `1565b1fecd7` passed full-library, strict-lint and actual-CLI checks; see
[retained software evidence](../../research/ml_dataset_20261028/validation-prime-graph-20261029/README.md).
The new v5 prefix path is implemented with validation in progress. It certifies
both polynomial-ideal containments, reconstructs recursive specializations and
legal iterators, and projects the state before each observed branch. It excludes
future bases, branches, lift results and source identifiers. Restricted prefix
records retain explicit certificates, observed actions, joins, cuts and bounded
unknown suffixes. The public action domain describes the remaining iterator;
labels do not establish an optimal policy. Root completeness and semantic state
certificates do not verify all native operations, a complete Groebner basis or
instruction totals. See [contract and limits](PREFIX-REPLAY-CERTIFICATES.md).

The v6 consequence task is `prime-polynomial-consequence/v1`. Its public rows
contain the initial public projection, ordinary polynomial ring, active original
variables, fixed bindings and the pre-native source equation graph. Field values
are decimal strings. Multiple targets from the same context share a feature
identity. `restricted/algebraic-records.jsonl` contains the target equation graph,
generator-multiple proof, direct identity audit, source joins and nullable degree
labels. A model may propose polynomial consequences and multiplier witnesses;
every proposal still needs exact identity replay. Targets, costs and provenance
must not be concatenated into model inputs.

Consequence extraction currently accepts only retained native prime-chain nodes
whose two ideal containments have already been certified, with m=3..15 and
p<2^32. It directly checks each identity `g = sum(c_i X^a_i F[j_i])` using the
pre-native system F. The searched representation is not a witness of the native
F4 operations. A failed whole-node equality search supplies no consequence rows;
this increment does not recover individual provable rows from that failure.
The lower-level identity API supports p<2^62, including p=2^61-1, but that does
not extend the recursive adapter's supported domain.

Nonzero verified targets produce public feature occurrences and restricted
labels. Zero targets remain restricted with null result degree and no degree-drop
claim. A cap produces a restricted unknown-suffix record with no certified target
or proof and no new feature occurrence; earlier admitted rows remain. Unknowns
are not negative algebraic examples. The channel has separate cumulative limits
of 10,000,000 direct coefficient operations, 4,096 examined target rows and 64 MiB
of serialized feature/record payload before sorter envelopes and deduplication.
Its terminal record has a separate 4 KiB limit. Censoring this channel leaves
branch replay under its existing budget. These are bounded data structures and
counted operations, not a whole-process memory or native-instruction guarantee.

The restricted `witnessed_degree_drop` label compares the nonzero target degree
with the largest nonzero generator-multiple summand in its checked representation;
zero generators cannot inflate that degree. It establishes neither a minimum
derivation degree nor exact first/last fall degree or closure completeness.
Identity work counts direct multiplication and accumulation. The referenced
node equality work includes both search and prior identity checking and is
shared across target rows; do not sum it once per target. Neither counter is a
counterfactual policy cost. See the [frozen consequence contract](../../research/ml_dataset_algebraic_targets_20261030/CONTRACT.md).

The v6 runtime passed254 scoped dataset tests, strict maintained Clippy, a release
build and5459 full library tests (0 failed,123 ignored). Its actual CLI fixture
passed curve/twist export and immutable retry, all three input files, restricted
joins/counts, unchanged static/branch feature bytes and corruption rejection.
Failed earlier attempts remain preserved. See the implementation status for exact
runtime, fixture and validation revisions; later main changes are outside this
validation. Earlier v4/v5 evidence validates only its recorded source.
Native re-export checks direct identities again; it is not an independent
implementation or a standalone validator for externally supplied consequence
records. No new scientific samples or cloud cells are established here.

The current source contains structural routing inputs, static prime-chain
equations, causal recursive prefixes and this bounded consequence channel.
Broader sequential solver policy views, held-out training and efficacy validation
remain open. These additions do not reduce the requested Koblitz/binary/prime,
curve/twist, n=23..31 and 47..61, p=2^61-1, m=2..15 campaign.
Keep answers, future pivots, recovered logs, construction seeds and
future conflict clauses in the target/restricted partition. Group equivalent
presentations, signs, Frobenius orbits, generators, twists and related cases in
the same holdout component. The initial conservative split groups by field size;
cross-size embeddings need an additional component merge before training.

Validate public export, split disjointness and leakage before a training run.
Retain all capped, unsupported and failed rows. Fit success probability and
resource predictors with censoring, and measure learned SAT or elimination
policies against frozen classical baselines with held-out instances. A neural
prediction about hidden algebraic structure becomes a mathematical result only
after independent symbolic or formal verification.

The [durable-launch obstruction record](../../problems/ml-ecdlp-dataset-durable-launch.md)
tracks the remaining worker S3 access and cumulative campaign-budget gates;
software checks and exported local fixtures do not establish a cloud corpus.

## Shared campaign reservation and independent consumer

The existing [campaign budget record in PR 2751](https://github.com/isogeny-labs/crypto/pull/2751)
sets a USD 200 cumulative ceiling through the stated horizon, with USD 188.3888
conservatively reserved and USD 11.6112 unallocated. It includes existing E12
jobs, storage/transfers/contingency and a proposed single c6a.4xlarge worker for
at most 24 hours. These are reservations, not verified billing or an account-wide
spending guard. The reservation does not renew per wave or per day.

That proposed worker was priced in us-east-1; the producer's EKS lab is in
us-east-2. Before another cloud wave, reconcile prior Job usage and inventory,
price and admit the actual EKS node/worker shape, and bind cumulative output
and request accounting. Preserve the recorded proposed limits of 20 GiB unique
output and 100,000 PUT attempts. The original proposal named 200,000 GET requests;
the implemented conservative read ceiling shares those 200,000 attempts across
GET, HEAD and explicitly counted STS bootstrap/refresh. The per-cell trace
threshold alone does not enforce these campaign-wide limits. The large planning
manifest is not an authorization to launch its cross-product. The role-policy
approval and this admission gate must both be resolved before a new pilot.

An independently implemented Rust consumer is maintained in the companion
[PR 2757](https://github.com/isogeny-labs/crypto/pull/2757), under
`research/ml_dataset_independent_validation_20261028/native`. It checks public
positive certificates without linking producer arithmetic, with narrowly stated
limits on order, solver-independence and fall-degree claims. Reconcile and pin
its current revision before using it. Its controller-role software fixture does
not establish worker access or replace a real producer-to-S3-to-consumer replay.
The first durable producer cell must be restored into an isolated directory and
checked by both the native producer and that independent consumer before any
broader scientific claim or training acceptance.


## Merged polynomial-consequence validation, 2026-10-30

PR2964's merged sourcea5343597d835 passed264 dataset tests, strict maintained
Clippy, release binary build, retained curve/twist CLI exports/retries and full
serialized library tests (5537 passed,0failed,123ignored). Source checks before
and after every phase matched inventory03. The14 phase/pipeline/binary/CLI
artifacts have fresh S3 readback records in
research/ml_dataset_algebraic_targets_20261030/attempt-05-readback.json.
These are software checks on retained synthetic fixtures; cloudcells remain0.
