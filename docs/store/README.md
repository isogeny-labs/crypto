# The artifact store: large files without Git

Factor bases, relation tables, DP archives and raw evidence that are too big
for Git live in object storage and are fetched when something needs them.
Code names a file, not a place:

```
ic run --factor-base store:factor-bases/icv1-f2m41-tm2308219-7f48b14a/fb1-0123abcd.json
```

The store turns that name into a verified local path, downloading it once
into a content-addressed cache. A small index tracked in Git records what
each name means, so a checkout, a CI job, a worker pod and the
`crypto-autoresearcher` repository all resolve the same name to the same
bytes.

The store moves bytes and checks them. It prices nothing and makes no
performance claim; the evidence rules in `AGENTS.md` apply to what is built
on the bytes.

## Quick start

```bash
# once per machine: the AWS CLI with credentials that can use the bucket
aws sts get-caller-identity

# publish a file (uploads, caches, records it in store/index.json)
crypto store push fb.bin \
  --name factor-bases/icv1-f2m41-tm2308219-7f48b14a/fb1-0123abcd.bin \
  --description "subspace d=8 factor base" \
  --provenance "ic search --curve ... --out fb.bin (commit abc1234)"
git add store/index.json && git commit      # the index is the manifest

# anywhere else: fetch on demand
crypto store pull store:factor-bases/icv1-f2m41-.../fb1-0123abcd.bin   # prints the cached path
crypto store pull <name> --out ./fb.bin                                # also copy it out
```

`ecbench store …` is the same command. A path argument that accepts a
`store:NAME` or `sha256:DIGEST` reference needs no other change; current
callers include `ic … --factor-base`, `ic solve-prime --route-file`, and
`ic solve-cubic --input`. Any code can adopt it with one call:

```rust
let path = crypto_lib::store::resolve_path(&arg)?;   // a plain path is returned unchanged
```

## How it works

| piece | what it is |
|:--|:--|
| **object** | immutable bytes, keyed by their SHA-256: `sha256/ab/abcdef…` |
| **index** | `store/index.json`, tracked in Git: name → sha256, bytes, description, tags, provenance, superseded digests |
| **cache** | `$CRYPTO_STORE_CACHE`, default `~/.cache/crypto-store`; verified objects, read-only, written atomically |
| **remote** | `s3://bucket/prefix` through the `aws` CLI, or `file:///directory` |

Why content addressing: a cached copy never goes stale, a remote object is
never overwritten with different bytes, and every download is hashed against
the digest the index promised, so a wrong or damaged object is refused and
never reaches the cache.

The index is the manifest `AGENTS.md` asks for when output is too large for
Git: content hash, byte count, durable location (the remote in
`store/config.json`), and the command that produced it (`--provenance`).
A name that already holds other content is not repointed without
`--replace`, and the old digest is kept in `superseded`, so a replaced
figure stays visible as the "was".

## Commands

| command | does |
|:--|:--|
| `store status` | the index, cache and remote this checkout resolves to, and whether the remote answers |
| `store ls [prefix]` | indexed names, with whether each is cached |
| `store info <spec>` | one entry in full |
| `store push <file> --name N` | upload (skipped if the remote already holds the bytes), cache, index |
| `store pull <spec> [--out P]` | make available locally, print the path |
| `store verify [names] [--remote] [--download]` | re-hash cached objects; `--remote` checks existence, size, and digest metadata through S3 HEAD; `--download` also downloads and hashes the remote bytes in a temporary file |
| `store gc [--apply]` | drop cached objects the index no longer names (dry run by default) |
| `store rm <name>` | remove a name from the index |

A `<spec>` is a name, `store:NAME`, or `sha256:DIGEST` (a full digest, or a
prefix of 8 or more digits that the index resolves).

The remote is **never deleted from**: `rm` edits the index, `gc` prunes the
local cache. Deleting remote objects is a decision for the bucket's owner.

## Configuration

Precedence is environment, then `store/config.json`, then the default.

| setting | environment | `store/config.json` | default |
|:--|:--|:--|:--|
| remote | `CRYPTO_STORE_REMOTE` | `remote` | none (the committed config names `s3://crypto-autoresearcher/store`) |
| endpoint (MinIO, LocalStack, …) | `CRYPTO_STORE_ENDPOINT` | `endpoint` | AWS |
| cache directory | `CRYPTO_STORE_CACHE` | | `$XDG_CACHE_HOME/crypto-store`, else `~/.cache/crypto-store` |
| index file | `CRYPTO_STORE_INDEX` | | `store/index.json` at the repository root |
| `aws` executable | `CRYPTO_STORE_AWS` | | `aws` |

A local directory works as a remote (`CRYPTO_STORE_REMOTE=file:///srv/store`),
which is how a lab without S3, a shared volume, or a test uses the store.

## S3 details

Transport is the AWS CLI, as in `ecc2k-merge` and `isogeny_walk::store`, so
credentials, regions, SSO and retries are the CLI's and this crate links no
HTTP stack. Uploads follow the `isogeny_walk::store` conventions:

* **Write-once.** Single PUT and multipart completion use
  `--if-none-match '*'`. A concurrent writer's object is accepted only after
  checking its size and recorded digest; the push report says whether this
  invocation created the object.
* **Checked on arrival.** The upload declares `--checksum-sha256`, which S3
  verifies against the bytes it receives, and the reply's checksum is
  compared again.
* **Large objects.** Past 4 GiB, the store uses explicit multipart creation,
  SHA-256 checked parts, and conditional completion. It hashes the entire
  source during upload and aborts an incomplete upload on failure. The full
  digest travels as user metadata because S3's multipart SHA-256 is
  composite. A fresh download is hashed against the index before caching.
  `CRYPTO_STORE_MULTIPART_THRESHOLD_BYTES` and
  `CRYPTO_STORE_MULTIPART_PART_BYTES` can lower the threshold and part size

`crypto store verify --download NAME` is the byte-level remote check. It
downloads the named object to a temporary file, compares its SHA-256 and byte
count to the index, and removes that file afterward. It does not use an
existing cached copy as evidence. This is useful for multipart objects because
their S3 HEAD checksum is composite and their full-file digest is recorded in
metadata. `--remote` remains a faster HEAD-only check.
  for bounded acceptance runs; defaults are 4 GiB and 64 MiB.
* **Checked on download.** Every object is hashed as it enters the cache, and
  a mismatch is discarded.

For S3, `verify --remote` checks metadata without downloading an object. A
matching HEAD response is an existence and metadata check; a missing digest
is reported as a mismatch. `push` likewise refuses to accept an existing
object without the indexed digest. Use `store pull` into a fresh cache to
hash the downloaded bytes against the index. The file-backed remote hashes
its bytes directly for HEAD and remote verification, and creates
content-addressed keys atomically.

The IAM permissions needed are `s3:GetObject`, `s3:PutObject`,
`s3:AbortMultipartUpload`, and `s3:ListBucket` on the bucket and prefix.
S3 answers `403` rather than `404`
for an absent key when `s3:ListBucket` is missing; the error says so.

## Naming

Names are path-like (`[A-Za-z0-9._/+=-]`, no `..`). Use a namespace that says
what the file is and which curve it belongs to, with the ICV1 slug (AGENTS.md
§11) for curves:

```
factor-bases/<icv1-slug>/<FB1-id>.<ext>
tables/<icv1-slug>/<description>.<ext>
evidence/<topic>_<date>/<file>
```

Do not rename frozen evidence to fit: the store name is a pointer to the
bytes, the committed file keeps the name it was written with.

### Setec bucket layout

For new IC artifact collections in `setec-astronomy-artifacts`, use the remote
root `s3://setec-astronomy-artifacts/ic/v1`. The store appends
`sha256/<first-two-hex-digits>/<full-sha256>` to this root; logical names and
provenance live in the checked-in index. Reserve these name forms:

```
factor-bases/<icv1-curve-slug>/<fb1-id>.<ext>
relations/<icv1-curve-slug>/<run-id>/<stage>.<ext>
isogeny-routes/<icv1-curve-slug>/<graph-id>/<route-id>.json
evidence/<campaign-id>/<run-id>/<file>
```

Use a UTC timestamp plus a random identifier for `run-id`, and record the
curve identity, producing command, source commit, SHA-256 and byte count in
the index entry. An acceptance run uses a separate remote root at
`s3://setec-astronomy-artifacts/ic/v1/acceptance/<run-id>` and an isolated
index and cache, so its cleanup cannot affect a research artifact.

The current committed index and config refer to an existing remote. Switching
that index to the Setec root requires copying and verifying every indexed
object first. New Setec runs can use `CRYPTO_STORE_REMOTE` and a separate
`CRYPTO_STORE_INDEX` without changing the existing index's resolution.
The bucket is in `us-east-2`; the configured `ecc-challenges-new` AWS profile
passed a live push, HEAD, remote verify, fresh-cache pull, SHA-256 comparison,
and cleanup on 2026-10-18.
The exact key, checksums and cleanup outcome are recorded in
[`S3_ACCEPTANCE_20261018.json`](S3_ACCEPTANCE_20261018.json).
The separate isogeny-walk `publish`/`fetch` path was exercised on a public
three-curve route; its marker, three downloaded hashes, route verification,
and cleanup are in
[`ISOGENY_S3_ACCEPTANCE_20261018.json`](ISOGENY_S3_ACCEPTANCE_20261018.json).
An actual verified `icx solve-prime` report was also pushed through the
content store, checked with S3 HEAD and `store verify --remote`, pulled into
a fresh cache with matching SHA-256, and removed from its unique acceptance
prefix. Its receipt is
[`IC_REPORT_S3_ACCEPTANCE_20261018.json`](IC_REPORT_S3_ACCEPTANCE_20261018.json).
The public isogeny route was also published to a separate acceptance prefix,
fetched into an empty cache by `ic solve-prime --route-file store:...`,
checked against its content hash and graph certificates, and used to verify
the scalar on the source and destination curves. The exact HEAD, fetch,
verification, and cleanup results are in
[`ROUTE_IC_S3_ACCEPTANCE_20261018.json`](ROUTE_IC_S3_ACCEPTANCE_20261018.json).
The public cubic-field cofactor-12 target input also passed a live upload,
HEAD checksum, fresh-cache `icx solve-cubic --input store:...` fetch, scalar
replay, remote verify, exact-object cleanup, and final HEAD 404. The first
attempt used the shell's default AWS profile and failed at preflight HEAD
with 403 before upload; the recorded `ecc-challenges-new` profile completed
the test. The receipt is
[`CUBIC_TARGET_S3_ACCEPTANCE_20261018.json`](CUBIC_TARGET_S3_ACCEPTANCE_20261018.json).
The final binary after merging current `main` repeated the isolated push,
matching repeat skip, S3 HEAD, remote verification, fresh-cache download,
SHA-256 check, exact-object deletion, and final HEAD 404. Its binary hash and
unique key are in
[`FINAL_MERGED_S3_ACCEPTANCE_20261018.json`](FINAL_MERGED_S3_ACCEPTANCE_20261018.json).
The current `crypto` CLI head repeated that sequence on 2026-10-21 under a
new isolated acceptance prefix. S3 HEAD with checksum mode enabled matched
the content SHA-256, and the exact-object cleanup ended at HEAD 404. Its
source head, release binary hash, object key, and checks are in
[`FINAL_CRYPTO_CLI_S3_ACCEPTANCE_20261021.json`](FINAL_CRYPTO_CLI_S3_ACCEPTANCE_20261021.json).

## For evidence

When a result's raw output is too large for Git, push it and commit the
index entry with the PR. The manifest an evidence directory needs (hash,
byte count, location, extraction command) is then the entry's `sha256`,
`bytes`, the remote in `store/config.json`, and `provenance`. A reader
reproduces the bytes with `crypto store pull sha256:<digest>`.

## What this does not do

* **Live S3 scope is bounded.** The single-PUT and isogeny-walk receipts
  above record their keys and cleanup. A 9 MiB object exercised the same
  explicit conditional multipart path selected for files above 4 GiB,
  using the threshold and part-size overrides. Its two checked parts,
  conditional completion, composite HEAD, remote verification, fresh
  download, and cleanup passed. Competing-writer runs rejected different
  bytes and accepted a checksum-declared matching object. The receipt is
  [`CONDITIONAL_MULTIPART_S3_ACCEPTANCE_20261020.json`](CONDITIONAL_MULTIPART_S3_ACCEPTANCE_20261020.json).
  An actual file above 4 GiB and interrupted-upload recovery have not had
  live acceptance runs.
* **No migration of existing large files.** Files already in Git history stay
  there; moving them out is a separate, history-visible decision.
* **No remote deletion, versioning policy or lifecycle rules.** Those belong
  to the bucket.
* **No database.** An RDS-backed catalogue of artifacts (and of run yields)
  is the next step; the index here is its source of truth until then.

## Factor bases

Index-calculus factor-base dumps use the namespace above and the build /
publish / run flow in
[`docs/ecbench/FACTOR_BASE_PROTOCOL.md`](../ecbench/FACTOR_BASE_PROTOCOL.md).
Short form:

```bash
ecbench fb --curve '…' --factor-base 'koblitz-orbit:divisor=0;1' --out fb.json
ecbench store push fb.json --name factor-bases/<icv1-slug>/<FB1-id>.json \
  --description '…' --provenance 'ecbench fb … (commit …)'
# then run ic.pipeline with the same plug-in string; join on fb_id
```

## Related

* `src/cryptanalysis/isogeny_walk/store.rs`: the run-level publisher for walk
  outputs (a completion marker over many objects). It shares the write-once
  and checksum conventions; this store is for named files of any kind.
* `ecc2k130/aws/`: the ECC2K-130 campaign's own bucket and tooling.
* `taskq/`: workers need the `aws` CLI (or a `file://` remote on a shared
  volume) to resolve `store:` references.
* Factor-base protocol: [`docs/ecbench/FACTOR_BASE_PROTOCOL.md`](../ecbench/FACTOR_BASE_PROTOCOL.md).
