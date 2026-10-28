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
count of completed solutions. Every curve/target configuration has both twist
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
  --remote s3://setec-astronomy-artifacts/store
crypto dataset verify --out /work/dataset
crypto dataset status --out /work/dataset
```

Use a digest-pinned image on the repository's EKS Spot lab. Assign disjoint
shards before submission, record every Job identity, and inspect the existing
Job and S3 receipts before retrying a submission. Worker-local process leases
do not provide a distributed cloud lock. The ordinary throughput Job does not
establish CRIU recovery, isolated benchmark clocks or comparative speed claims.

Run `crypto dataset preflight --plan pilot-plan.json --protocol protocol.json`
as the actual container UID before launch. It checks readable inputs, the frozen
protocol, clean compiled source identity and configured lab receipt routing
without performing cryptography. It does not establish output-directory or
cloud permissions. Container input modes must grant UID 1000 read access.

The current lab service supplies a legacy `ISOLAB_RESULTS_URI`. Dataset receipts
must use the approved bucket: explicitly set a unique campaign/job prefix under
`s3://setec-astronomy-artifacts/ml-dataset-lab/runs/`. Keep the original service
URI and the effective override in the launch record. The EKS role must permit
get/put and required prefix listing for the actual data and receipt destinations;
a local role's successful upload does not establish worker access. See the
prepared [role policy proposal](../../research/ml_dataset_20261028/worker-s3-policy-proposal.md).
It has not been applied. Its scoped data remote is
`s3://setec-astronomy-artifacts/ml-dataset/20261028/store`.

The first pilot uses 19 shards so a bounded sequential shard crosses method,
twist and other axes. Derive observed coverage from saved configurations;
strides divisible by the ten-method axis can accidentally select only one method.

Reissuing the same generation command skips verified completed cells and retries
their publication when needed. A partial cell requires explicit reconciliation:

```sh
crypto dataset reconcile --out /work/dataset --cell CELL_INDEX \
  --reason "Original worker terminated; external Job and owner reconciled"
```

Reconciliation checks local Linux processes and preserves the partial directory
under an immutable attempt name before allowing another attempt. It cannot
prove that a worker on another host has terminated. Retain all interrupted
attempts when moving a campaign to another host.

## Restore from durable receipts

Publication saves a full portable `ml-dataset.publication/v1` receipt in the
remote catalog at `ml-dataset/catalog/NAMESPACE/MANIFEST_SHA256.json`, relative
to the data remote's prefix. Use the catalog key returned by publication; a
completion marker, catalog locator, status summary or enclosing lab receipt is
not itself the full receipt accepted by `restore`.

For the scoped pilot remote, download the exact catalog object to a local
ordinary file, then restore one cell at a time:

```sh
aws s3 cp \
  s3://setec-astronomy-artifacts/ml-dataset/20261028/store/ml-dataset/catalog/NAMESPACE/MANIFEST_SHA256.json \
  /work/receipts/cell-publication.json --region us-east-2
crypto dataset restore --receipt /work/receipts/cell-publication.json \
  --out /work/restored-dataset
crypto dataset verify --out /work/restored-dataset
crypto dataset status --out /work/restored-dataset
```

Create the local receipt directory before the download and replace the uppercase
components with the returned namespace and manifest digest. Restore uses the
remote recorded in the receipt. Its optional `--remote` argument must identify
the same remote; it cannot redirect proof to a mirror. The supported S3 bucket
is `setec-astronomy-artifacts`; `file://` remotes are also supported for local
round-trip validation. Restore reads remote objects and does not upload them.

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
read/write permissions are still required, and catalog discovery may separately
use scoped listing. See AWS's [HeadObject permissions](https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadObject.html)
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

Trace byte caps apply to worker output before manifest sealing. Manifest and
publication metadata are accounted separately. The supervisor preserves partial
files after a killed call; complete native algorithm internals still require
explicit trace hooks. Capability records name those gaps. Exact first-fall and
last-fall degrees remain null until a checker establishes their stated algebraic
definition and all required closure/rank conditions.

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

Export replays sealed cells and writes `features.jsonl`,
`restricted/targets.jsonl`, `restricted/index.jsonl` and `manifest.json`.
It performs no remote download. Only `features.jsonl` is model input; targets
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
count. Unsealed attempts appear in the restricted index without fabricated
feature or target rows.

Use a separate destination outside the source tree. Export refuses the source
root, its ancestors and paths inside `cells/`, and rejects symlink ancestors.
Exports are immutable snapshots: identical retries verify existing bytes; a
changed source or export policy requires a fresh destination. The current
policy is `strict-structural-allowlist-v2`; its domain-aware feature identities
must not be mixed with earlier export identities without regeneration.

Use public equations, factor-base coordinates and admissible prefix states as
features. Keep answers, future pivots, recovered logs, construction seeds and
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
