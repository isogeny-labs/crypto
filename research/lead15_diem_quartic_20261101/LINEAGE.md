# LEAD-15 launch and collection contract

This contract is part of the draft protocol. It does not authorize a build,
freeze, checkpoint, submission, or target-size run.

## Why the request hash is not in the launch argv

The frozen source manifest contains an eight-element `argv_template`. Its only
placeholder is `${SOURCE_MANIFEST_SHA256}`. After the manifest is frozen, the
launcher substitutes the actual manifest byte hash and writes the resulting
`request.json`. The completion receipt binds the SHA-256 of those exact request
bytes.

Putting the request hash in its own command, or putting a request hash in a
manifest whose hash is in that request, would be a self-referential hash cycle.
Neither is permitted. The collector instead proves the complete chain:

`frozen manifest -> argv template -> exact request bytes -> raw Job/Pod -> local
read-back bytes and S3 object versions`.

The manifest's `pre_manifest_package_receipt` has the fixed scope
`pre_manifest_package_excluding_source_manifest_and_requests`. It covers the
source/protocol/schema/build package produced before the final manifest exists;
it cannot cover the manifest that embeds the receipt. Final manifest and request
bytes are retained separately. Its sole admitted object URI is
`s3://setec-astronomy-artifacts/lead15-diem-quartic-20261101/pre-manifest-package.tar`.
Each request URI is exactly `.../requests/q-<q>.json`, coupled to that job's `q`.
Every request is uploaded with S3 SHA-256 checksum
metadata after its manifest-hash substitution, and its completion receipt binds the
version-specific checksum-enabled head to the locally rehashed request bytes.
The completion-receipt schema independently couples each admitted `q` to the exact
Job-name suffix, `argv[1]`, and request URI; changing any one to another admitted
size is invalid rather than a merely incomplete cell.

## Evidence trust boundary

The offline directory is not self-authenticating. A trusted operator must acquire
the request read-back/head, submission, raw Kubernetes objects, status, complete
result listing, version-specific object heads, and object bytes through the
intended Kubernetes and AWS provider sessions, then preserve those bytes without
rewriting before invoking the collector. The offline collector and native analyzer
prove that those retained bytes are internally consistent with each other and with
the frozen identities; copied ETags, version IDs, checksums, and local hashes are
not independently signed provider attestations. An attacker controlling the whole
directory could forge the documents and their copied checksums together.

Loss of trusted acquisition/provider-session custody invalidates the study. It
cannot be demoted to an ordinary infrastructure terminal, and no K1--K4 or
scientific completion claim may be made. Any legacy code or field name using
“authenticated” means lineage-validated under this trusted-acquisition assumption,
not standalone cryptographic authenticity of the local bundle.

## Deadlines and environment

- The Kubernetes Job has `activeDeadlineSeconds=7200`, `backoffLimit=0`, and a
  60-second termination grace period.
- The tracked wrapper is the sole container command. It starts one native child
  under a 3600-second timeout and never starts a replacement.
- The Iso Lab launcher injects `RAYON_NUM_THREADS=4` and `OMP_NUM_THREADS=4`
  because the Job requests four CPUs. The wrapper overrides both to `1` before
  starting the scientific child and records those child values in
  `worker-start.json`.
- The child records the injected Job name and Pod name/UID/node. Kubernetes does
  not expose the owning Job UID through the admitted downward-API environment;
  that UID is therefore bound only by the locally retained submission, raw Job,
  status, raw Pod owner reference, and completion receipt, never invented in a
  child record.
- Raw Job admission is the exact standard-launcher contract, not a permissive
  Kubernetes subset. It requires integer `metadata.generation=1`; permits
  template creation time, name, and namespace only absent/null; and binds the
  86,400-second cleanup TTL, generated
  controller selector and current/legacy labels, Karpenter annotation, CPU
  toleration, required anti-affinity, non-root pod/container security contexts,
  and the launcher's `/work` and `/tmp` `emptyDir` mounts. Resource maps contain
  exactly CPU, memory, and ephemeral-storage requests/limits; resource claims or
  extended device resources are forbidden.
- A retained Pod has exact `v1`/`Pod` type metadata, server-emitted integer
  `metadata.generation=1`, and, when scheduled, matching integer
  `status.observedGeneration=1` from the version-pinned EKS 1.36 deployment.
  An unscheduled terminal Pod may omit the status field but cannot report a
  mismatch. The Pod also binds its namespace, a single complete
  Job owner reference, the controller/job label identities, the Karpenter
  annotation, and exactly one `task` container. Only the observed coherent
  `us-east-2` region/zone label pair may supplement those labels. The admitted
  Pod must contain the `runner` service account's exact IRSA mutation: fixed role
  ARN/token-file environment entries and the read-only `aws-iam-token` projected
  volume/mount with the STS audience and 86,400-second token lifetime. Its normalized
  spec must equal the Job template; normalization removes only that fixed IRSA shape,
  enumerated API defaults, and the scheduled node name. Args, other environment sources, lifecycle
  hooks, working-directory changes, volume/device overlays, init/ephemeral
  containers, sidecars, or any unlisted spec/metadata field are rejected.
  Downward-API `fieldRef.apiVersion` may be absent or exactly `v1`; explicit null
  and every other value are rejected.
  The role binding is sourced from `taskq/deploy/eks-karpenter/workloads.yaml`; retained
  production status at
  `taskq/deploy/eks-karpenter/validation/pr2342-premerge-20261021/launcher/status.json`
  independently shows the admitted third `aws-iam-token` mount.

## Build and image identity

`build-info.json` is validated by `build-info.schema.json` and binds the source
revision, Cargo lock hash, full compiler version, target triple, digest-pinned
base image, image-recipe hash, exact release command, native binary hash, and
worker hash. `lead15-image.Dockerfile` copies those exact three artifacts and a
copy of itself into the image, verifies their hashes while building, and leaves
the final paths read-only under UID 1000.

The build-info bytes deliberately do **not** contain the final OCI image digest:
an image cannot contain its own content digest without creating another hash
cycle. The frozen source manifest binds the build-info byte hash and separately
binds the final digest-pinned image URI. The raw Job, raw Pod, and Pod
`imageID` checks then verify that this externally bound image is the one that
ran.
- Worker outputs go only to the launcher-injected
  `s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod>/` prefix. The
  `setec-astronomy-artifacts` prefix is reserved for the preregistration package
  and versioned request artifact; it is not the Job results prefix.

## Exact local evidence directory

For each size, retain the following byte-for-byte artifacts in one directory.
Do not rewrite, pretty-print, or copy selected fields into replacement JSON.

Always required:

- `request.json`: the `lead15.isolab-request/v1` envelope containing the exact
  `isolab_k8s_run` arguments;
- `request-head.json`: raw `s3api head-object` JSON for the uploaded request,
  including `ContentLength`, `ETag`, nonempty `VersionId`, and
  `ChecksumSHA256`, obtained with checksum mode enabled for that exact version;
- `submission.json`: the exact submission response;
- `job.json`: the raw Kubernetes Job object after creation;
- `status.json`: the terminal `isolab_k8s_status` response;
- `results.json`: the complete, pagination-reconciled `isolab_k8s_results`
  listing, represented with `next_token: null` only after all pages are merged;
- `object-heads.json`: `lead15.object-heads/v1`, with `results_prefix` and an
  `objects` map keyed by downloaded basename. Each value preserves that
  object's `ContentLength`, `ETag`, nonempty `VersionId`, and
  version-specific `ChecksumSHA256`.

Required if a Pod was created:

- `pod.json`: the raw Kubernetes Pod object. An unscheduled terminal Pod may
  have a null node and no container status; it can establish only an
  infrastructure failure.

Retain every available worker object, including a strict prefix left by an
interruption:

- `records.jsonl`, `worker-start.json`, `worker-exit.json`,
  `worker-upload.json`, `runtime-provenance.json`, `build-info.json`,
  `binary.sha256`, `stdout.log`, `stderr.log`, and `SHA256SUMS`; and
- `timeout-events.log`, the separate C-locale `timeout --verbose` stream. It is
  empty for a non-timeout exit, contains the exact TERM event for exit 124, and
  contains exact TERM then KILL events for exit 137.

The object listing uses full keys such as
`runs/<job>/<pod>/worker-exit.json`. The corresponding `object-heads.json` map
uses the local basename (`worker-exit.json`). A downloaded worker file is
admissible only when its local size matches both the listing and head, its ETag
matches the listing, the listing names SHA-256 as a checksum algorithm, the head
carries a nonempty version ID, and the head's `ChecksumSHA256` decodes to the
SHA-256 independently computed over the local bytes. Acquire each local file
with a version-specific `get-object`; a same-length file and an unbound head are
never sufficient.

## Collection and analysis

Run `lead15-collect.sh Q source-manifest.json EVIDENCE_DIR` only after the
original Job is terminal. The collector is offline: it performs no Kubernetes,
AWS, or submission calls. It refuses symlinked inputs, locally hashes every
byte artifact, verifies the exact launch semantics and identities, and creates
`EVIDENCE_DIR/completion-receipt.json` with no-clobber semantics.

A proper subset of valid worker artifacts is preserved as
`infrastructure_failure`; it is not discarded. Only a complete coherent worker
set can establish `completed`, `deadline`, or `scientific_failure`. A deadline
flag and worker timestamps remain false/null when `worker-exit.json` is absent;
the receipt exit code is then only the raw scheduled Pod termination code, or
null for an unscheduled or zero-Pod attempt. A deadline also requires an exact
timeout-event proof, a failed Job, and a raw terminal Pod
whose `exitCode` agrees and whose reason is `Error`, never `OOMKilled`. A completed
or scientific-failure classification likewise binds the worker exit to the raw Pod.
A deadline or scientific failure may legitimately have no `records.jsonl` if the native
child terminated before creating it. `completed` requires a read-back payload
with a final newline.

The worker-owned JSON envelopes are closed shapes. `worker-exit.json` has exact
top-level and payload keys and must give a nonnegative integer elapsed time plus
a coherent tuple of terminal class, deadline flag, timeout flag, and exit code:
completion is exit 0 without deadline/timeout; a deadline claim requires both
flags, exit 124 or 137, and at least 3599 seconds, and promotion to the deadline
terminal additionally requires the matching timeout-event bytes. Scientific
failure is a nonzero exit other than the wrapper-reserved 130/137/143 signal
exits, without deadline/timeout. Infrastructure-class receipts cannot
promote a scientific result. `worker-upload.json` uses bounded integer child and
upload exit codes and requires `uploaded` to be exactly equivalent to
`upload_exit == 0`. `runtime-provenance.json` has an exact field set, positive
integer CPU/memory values, and binds the raw Pod name, Pod UID, node, and frozen
Job name. The collector and native analyzer apply the same checks; unknown
fields and contradictory lineage-validated bytes are rejected. When both worker
start and exit records are present, their `started_at` values must be identical;
the values are lifecycle provenance and are never scientific wall time.

The native analyzer must not trust the collector's copied metadata. It resolves
each `lineage_artifacts.*.path` relative to the completion receipt, rejects
symlinks and non-regular files, and requires the sibling path to be absent when
an optional descriptor is null (including no dangling symlink). It recomputes
SHA-256 and byte length, reparses all control-plane and worker JSON, and repeats
the request/Job/Pod/S3/payload cross-checks. Any mismatch makes the size
invalid/incomplete.

`lineage-contract-tests.sh` contains schema-only positive and adversarial
fixtures. `collector-adversarial-tests.sh` exercises the offline collector with
synthetic request/Job/Pod/S3 bytes and rejects deadline, image, version,
payload-hash, terminal/elapsed, upload, exact-shape, mixed-start-time,
controller/owner/label, device-resource/claim, execution-injection, and Pod-name
mutations. Its positive Job/Pod pair includes the actual standard-launcher
mounts, volumes, security contexts, affinity, tolerations, labels, annotations,
and named Kubernetes defaults.
`result-schema-tests.sh` checks all four completion-receipt q-coupling branches,
rejects cross-q Job/argv/request combinations, and rejects extra keys, missing
certificates, wrong types, malformed field elements, reordered cost bands, and
fictional formula-reference rows. None runs the binary or generates scientific
data.
