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

## Deadlines and environment

- The Kubernetes Job has `activeDeadlineSeconds=7200`, `backoffLimit=0`, and a
  60-second termination grace period.
- The tracked wrapper is the sole container command. It starts one native child
  under a 3600-second timeout and never starts a replacement.
- The Iso Lab launcher injects `RAYON_NUM_THREADS=4` and `OMP_NUM_THREADS=4`
  because the Job requests four CPUs. The wrapper overrides both to `1` before
  starting the scientific child and records those child values in
  `worker-start.json`.

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
  including `ContentLength`, `ETag`, and nonempty `VersionId`;
- `submission.json`: the exact submission response;
- `job.json`: the raw Kubernetes Job object after creation;
- `status.json`: the terminal `isolab_k8s_status` response;
- `results.json`: the complete, pagination-reconciled `isolab_k8s_results`
  listing, represented with `next_token: null` only after all pages are merged;
- `object-heads.json`: `lead15.object-heads/v1`, with `results_prefix` and an
  `objects` map keyed by downloaded basename. Each value preserves that
  object's `ContentLength`, `ETag`, and nonempty `VersionId`.

Required if a Pod was created:

- `pod.json`: the raw Kubernetes Pod object. An unscheduled terminal Pod may
  have a null node and no container status; it can establish only an
  infrastructure failure.

Retain every available worker object, including a strict prefix left by an
interruption:

- `records.jsonl`, `worker-start.json`, `worker-exit.json`,
  `worker-upload.json`, `runtime-provenance.json`, `build-info.json`, and
  `SHA256SUMS`.

The object listing uses full keys such as
`runs/<job>/<pod>/worker-exit.json`. The corresponding `object-heads.json` map
uses the local basename (`worker-exit.json`). A downloaded worker file is
admissible only when its local size matches both the listing and head, its ETag
matches the listing, and the head carries a nonempty version ID.

## Collection and analysis

Run `lead15-collect.sh Q source-manifest.json EVIDENCE_DIR` only after the
original Job is terminal. The collector is offline: it performs no Kubernetes,
AWS, or submission calls. It refuses symlinked inputs, locally hashes every
byte artifact, verifies the exact launch semantics and identities, and creates
`EVIDENCE_DIR/completion-receipt.json` with no-clobber semantics.

A proper subset of valid worker artifacts is preserved as
`infrastructure_failure`; it is not discarded. Only a complete coherent worker
set can establish `completed`, `deadline`, or `scientific_failure`. A deadline
or scientific failure may legitimately have no `records.jsonl` if the native
child terminated before creating it. `completed` requires a read-back payload
with a final newline.

The native analyzer must not trust the collector's copied metadata. It resolves
each `lineage_artifacts.*.path` relative to the completion receipt, rejects
symlinks and non-regular files, recomputes SHA-256 and byte length, reparses all
control-plane and worker JSON, and repeats the request/Job/Pod/S3/payload
cross-checks. Any mismatch makes the size invalid/incomplete.

`lineage-contract-tests.sh` contains schema-only positive and adversarial
fixtures. `collector-adversarial-tests.sh` exercises the offline collector with
synthetic request/Job/Pod/S3 bytes and rejects deadline, image, version, and
payload-hash mutations. `result-schema-tests.sh` rejects extra keys, missing
certificates, wrong types, malformed field elements, reordered cost bands, and
fictional formula-reference rows. None runs the binary or generates scientific
data.
