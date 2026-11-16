# N83 factor-base storage transition, 2026-10-27

**Status:** proposed routing for *new* artifacts; no N83 object has been copied or uploaded to the new bucket by this change. The user's clarification about whether to mirror the frozen D18 input and source package is pending. This document changes no frozen S3 identity, run protocol, cost row or factor-base selection.

## Authoritative destinations and existing custody

The repository's [artifact-store configuration](../../.artifacts/config.json) and [storage policy](../../docs/artifacts.md) name `s3://setec-astronomy-artifacts/store` as the current write-once SHA-256 artifact remote. The [report publisher](../../docs/reports-s3.md) uses the same bucket under `reports/crypto/`; its documentation declares `us-east-2`. A [prior native archive publication](../koblitz_modular_ic_20261018/rounds/21/ARTIFACT_PUBLICATION.json) has an upload and fresh-pull receipt for `setec-astronomy-artifacts/store`. These facts establish the repository convention, not this session's write access.

Historical N83 factor bases, including the full D18 producer object, were sealed in `s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/`. Their exact URIs and hashes remain in the versioned [study](README.md), [D18 custody handoff](verification/d18-full-orbit-20261025/AWS_RESUME.md) and run receipts. The [v1/v2 exporter](../../examples/koblitz_n83_factor_base_export.rs) constructs those URIs with a fixed `PREFIX`, and the [native adapter](primary_adapter.rs) requires the old bucket and matching upload/download receipt. Replacing the constant or editing those historical manifests in place would invalidate the frozen replay. The ongoing [Linux D18 protocol](verification/d18-full-orbit-20261025/native-cadence-replay-20261026/LINUX_SPOT_PROTOCOL.md) still requires GETs of the **original** two old-bucket input keys.

## Forward routing proposal

For new N83 bases and large run artifacts, use the existing native [artifact-store CLI](../../docs/store/README.md): `CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/store crypto store push FILE --name factor-bases/<ICV1-slug>/<FB-id>.jsonl.gz`. It writes immutable bytes under `store/sha256/<first-two-hex>/<full-sha256>` and records the name, SHA-256 and byte count in tracked `store/index.json`. Also commit an N83 panel manifest mapping that store name to the exact curve identity, base specification, orbit/point-set hashes, producer commit and the explicit astronomy remote. The environment override is scoped to this publication and replay; do not silently redirect historical store names or replace `store/config.json` for all callers. A named `factor-bases/` S3 layout is another option, but this proposal uses the repository's existing content-addressed store.

Implement the N83 panel/adapter change as a successor schema and dual reader: historical v1/v2 records continue to demand their old-bucket keys and receipts, while new records resolve a `store:` name through `crypto_lib::store::resolve_path`, demand the new-bucket object, exact byte count, SHA-256, existing independent point-set and orbit checks, and a fresh-download receipt. Do not reinterpret an old `s3_uri` as a new one or mix curve `a=0` and `a=1` custody. The native store already uses conditional S3 writes and rejects a key collision unless the hash and length match; publish the N83 panel manifest only after every object and its independent replay pass. Count construction, hashing, S3 I/O and any failed uploads in the cold one-target cost; artifact placement alone cannot select a winner.

The already prepared 58,011,243-byte private D18 source package (SHA-256 `dcc242fc0c4e6b4af99a854b885b0dece5d18ef69716a82cc5b5cc98d86044cd`) is **not** a factor base. Its [old-bucket upload proposal](verification/d18-full-orbit-20261025/native-cadence-replay-20261026/linux-build-smoke-20261026/SOURCE_DELIVERY_PROPOSAL.md) remains unapplied. If the owner wants this package and the historical D18 input mirrored into the new bucket, first freeze exact new keys and a two-bucket access/provenance amendment; verify both complete copies independently, preserve the original input GET in the current protocol, and obtain approval for the newly specified private-source upload and temporary runner access. A mirrored copy is additional custody, not retroactive replacement of the original.

## Access and execution gates

At the read-only check on 2026-10-27, the configured default and `ecc2k130` AWS profiles each received `403` for `HeadObject` on a known published `setec-astronomy-artifacts/store` object. The default profile also received `AccessDenied` for `GetBucketLocation` and `403` for `HeadBucket`. No local `ARTIFACTS_AWS_ACCESS_KEY_ID`/`ARTIFACTS_AWS_SECRET_ACCESS_KEY` pair was present. These observations establish this executor's current access limit; they do not assert that the bucket or prior object is absent. Do not use credentials from another account or change bucket permissions by inference. Use a specifically authorized artifact identity, then read back its exact policy and test a bounded write-once object/fresh GET before a bulk N83 upload.

The next implementation PR should pin the successor panel schema and dual adapter, exact bucket/region and identity, test old-record replay plus new-record rejection controls, and publish one verified public synthetic known-answer base through the existing native store to the new bucket. Its raw checksum and provider receipts must be committed or stored durably with a hashed manifest. Only after that gate should new grid candidates be routed there. The existing `selected_best_total_runtime=null` stays unchanged until complete cold same-target IC runs and matched references justify a selection. This routing correction yields no new performance observation, so existing runtime graphs are unchanged.

## Read-only access update — 2026-11-15 UTC

The earlier `403` observations above remain historical. A fresh read-only
check found profile `ecc-challenges-new` to be IAM user `coreuser` in account
`104186811093`. With that profile, `HeadBucket` succeeded for
`setec-astronomy-artifacts` in `us-east-2`, `ListObjectsV2` returned keys
under `store/`, and `HeadObject` found the previously published
`store/sha256/e9/e9c89a33a4cf96ae075e015e02829d0ad40165634c0317d40d5c2091ed576fa2`
at 14,861,500 bytes. A fresh `GetObject` of that exact key returned 14,861,500
bytes; local SHA-256 was
`e9c89a33a4cf96ae075e015e02829d0ad40165634c0317d40d5c2091ed576fa2`,
matching the committed prior publication receipt. The temporary download was
removed after hashing.

IAM `SimulatePrincipalPolicy` evaluated `s3:GetObject` and `s3:PutObject` as
`allowed` for a placeholder key under this store prefix. This is an IAM-side
simulation, not a successful `PutObject` or a proof that bucket policy permits
publication. `HeadBucket` on `ctech-astronomy-artifacts` returned `404` under
both inspected profiles, while the old-account `default` profile still got
`403` on `setec-astronomy-artifacts`. These checks support the repository's
spelling and identify one usable read principal; they do not authorize a
new-bucket write or historical mirroring. The owner has been asked to confirm
the exact destination, writer identity and mirror preference. The storage
diagram and runtime graphs are unchanged because no N83 object moved and no
algorithm arm ran.

## Native successor reader — 2026-11-15 UTC

The `primary-store-v3-check BASE_DIR NEW_OUTPUT_JSON` Rust command now imports
one primary signed-Frobenius base from a `store:` name. Its new
`n83.factor-base-panel/v3-artifact-store` manifest binds the primary ICV1
curve, producer commit, remote, policy, seed, column and point counts,
point-set hash, store name, compressed SHA-256/BLAKE3 and uncompressed
BLAKE3/byte count. Separate `replay.json` and `storage.json` receipts must
bind the manifest SHA-256; storage must also state the exact fresh-download
SHA-256 and byte count. The reader requires the configured native store
remote to match the manifest, checks the tracked store index, fetches or
rehashes cached bytes through `Store::ensure`, verifies both compressed and
plain hashes, then independently reconstructs every subgroup point and
signed-Frobenius orbit/label through the existing primary importer. It
writes a fresh `n83.factor-base-store-import/v3` receipt only on success.

The release example suite passed 25/25 tests on macOS arm64 with Rust 1.99.1.
Its v3 control published a public two-column N83 base to a temporary
`file://` content-addressed store, forced a fresh pull, verified the import,
and rejected a changed fresh-download digest and a v1 manifest at the v3
entry point. The historical v1/v2 reader and old-bucket keys were not
changed. This is a native reader and local custody control, not a new S3
publication, a new base in the sweep, or a relation/runtime measurement.
The producer-side v3 publisher, independently checked S3 write/fresh GET,
exact authorized destination and writer, and integration into the full IC
pipeline remain open.

## Native successor producer — 2026-11-15 UTC

The new `primary-store-v3-construct-publish NEW_DIRECTORY POLICY K SEED
CONSTRUCTION_BUDGET_SECONDS EXACT_REMOTE` Rust command accepts one primary
curve (`a=0`) signed-Frobenius public-X specification. It requires a clean,
source-matched checkout and an exact match between `EXACT_REMOTE` and the
configured native artifact-store remote. It creates a fresh directory and
`attempt.json`, streams one gzip object, checks the stream hashes, and replays
every point, subgroup membership, Frobenius phase, sign and coefficient with
the generic curve implementation. Only after replay passes does it write the
v3 manifest and replay receipt and call the native store's conditional,
content-addressed `push`. It then opens a separate empty cache, downloads
the object from the configured remote, checks its SHA-256 and size, writes
`storage.json`, and invokes the v3 reader to produce `import.json`. A later
error leaves the partial attempt in its unique directory and writes
`failure.json` when possible; a retry needs a new directory. The tracked
`store/index.json` is changed by a successful push and must be committed
with the manifest and receipts.

The construction budget is 1–7,200 seconds and governs factor-base
construction and gzip finalization through a process watchdog. A cap leaves
`construction-cap.json` with `UNKNOWN_budget` and the partial object. It does
not bound remote network waits or the full index-calculus pipeline.
The CLI is a publication command: use it only with the intended remote and
writer identity. The local positive control uses a temporary `file://` store,
two signed orbits (332 points), a fresh download, and an import; controls
reject a reused evidence directory and a corrupted fresh-download digest.
`RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc
cargo test --release --example koblitz_n83_factor_base_export --locked --offline`
passed 26/26 on macOS arm64. The repository release library gate is recorded
separately in the PR validation notes.
Actual S3 publication, exact writer confirmation, historical mirroring,
large-base memory behavior, and full cold same-target runtime comparisons
remain open. The new command records construction and custody timings as
informational L0 values only. No runtime graph or leaderboard row changes.

## Native successor trial bridge — 2026-11-15 UTC

`primary-store-v3-trial BASE_DIR FIXTURE_DIR K M MAX_TRIALS BUDGET_SECONDS
NEW_RUN_DIR` connects the verified successor base to the existing native
index-calculus enumerate path for `m=2..6`. It requires a clean, source-matched
checkout; the v3 manifest, replay, fresh-download and store-index checks; an
exact requested column count; and a separate public target corpus with a
validation sidecar opened only after a candidate log. Each run has a new
directory, an explicit wall cap, progress events and a result. The trial
allows at most 1,000 queries and 7,200 seconds. It records the v3 store name
and manifest SHA-256 instead of labeling that digest as the historical
panel's BLAKE3.

The local file-store control exercises a zero-trial preflight for a two-column
base, preserving null total runtime and rejecting a mismatched requested
column count. This is an input and solver-path integration control, not a
relation-yield result. The trial interval starts after base construction and
publication, and the current result still lacks a fully charged rank,
descent and verification comparison. It cannot select a minimum-runtime
factor base. The historical panel path and its old-bucket receipts continue
unchanged. No S3 write or runtime graph change follows from this bridge.

## Native successor chained-S3 bridge — 2026-11-15 UTC

`primary-store-v3-chain-cold`, `primary-store-v3-orbit-chain-cold`, and
`primary-store-v3-ordered-orbit-chain-cold` accept the same verified v3 base
and public target fixture for `m=4..6`. Each command requires explicit
variable, finite-domain-clause, model, conflict, wall, and hard cgroup memory
caps. The ordered first-pair mode is only available with the Frobenius orbit
domain. The imported store row must match the requested column count and the
policy and seed bound in the run configuration. The historical panel path
retains its separate object and source rules.

The native file-store test now also executes one `m=2` enumerate query and
checks that the trial count and solver-stage flag are set while total runtime
remains null. A separate `m=5` orbit-chain zero-query control checks the
combined store/chain input path and its distinct result schema. These are
execution-path controls; no natural chained-S3 search or useful-rank result
has run from a v3 base. A real chained-S3 run still needs a qualified Linux
hard-memory cgroup, external wall supervisor, frozen public query protocol,
and independently replayed models. Base construction, S3 publication, graph
closure, column logarithms, target descent, and matched rho remain outside
the bounded trial interval. No complete-runtime graph or leaderboard row
changes.

For both v3 trial commands, an error after the new run directory is created
now writes `failure.json` with `COMMAND_ERROR`, the original error, and null
runtime and winner fields, unless a terminal `summary.json` or `cap.json`
already exists. The file-store controls cover rejected policy and column
bindings and verify their terminal error receipts. An admission failure
before directory creation still returns an error without claiming a run.
