# S3 publication with prefix-scoped listing permission

Dataset publication checks whether each immutable object exists before writing
it. S3 HEAD can return 403 for an absent object when the caller's ListBucket
permission is conditional on a prefix. This change lets the store establish
current absence using a successful, complete, empty listing for the exact full
object key. It does not change IAM, publish scientific results, or launch workers.

Initial base revision: `f06eb186b28d8b5343ce0fc776fb173ad7a61a31`.
Before publication the branch was fast-forwarded to
`1a90e5bb7f8195cee17266e32dbfd0711585ae85`. That update changes no root Rust
source, Cargo manifest/lock, or local dependency source used in these checks.
The backend at this revision is identical to the backend in dataset producer
`f0863aefd6e1a416929f8acc02283d46f9c69936` (Git blob
`7340de7effb3a0fbd90240dbeed2e9e5b369257d`). Conductor task: T-708.

## Behavior and boundaries

Only a failed HEAD with the AWS CLI's explicit `(403)` marker can invoke one
ListObjectsV2 request. Its prefix is the full requested key, its maximum is one
key, and CLI pagination is disabled. The configured executable and endpoint
remain in use. Empty keys and directory buckets skip the probe.

Absence requires an exact bucket and prefix match, `MaxKeys: 1`, `KeyCount: 0`,
`IsTruncated: false`, and absent or empty `Contents` and `CommonPrefixes` arrays.
Typed deserialization rejects missing, duplicate, or wrongly typed required
fields, including explicit null arrays. Continuation, start-after, and delimiter
state is rejected even if empty or null. Unknown response fields are allowed.

Denied or failed LIST, malformed output, nonempty or truncated listings, and
identity mismatches preserve the original HEAD error. A sibling whose key starts
with the requested key is deliberately inconclusive. LIST never substitutes for
object metadata, checksum verification, or GET. The existing conditional single
PUT and collision verification remain in place because absence is not a lock.
The existing subprocess wrapper has no additional wall-clock or output bound;
this patch limits request count and pagination, not subprocess duration.

## Sources checked before implementation

- [AWS HeadObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadObject.html):
  distinguishes missing-key 403/404 behavior and checksum-related KMS permissions.
  Therefore HEAD403 alone cannot establish absence.
- [AWS ListObjectsV2](https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListObjectsV2.html):
  defines prefix, response identity, key counts, pagination, and directory-bucket
  restrictions. The absence decision requires a complete empty response.
- [AWS CLI ListObjectsV2](https://docs.aws.amazon.com/cli/latest/reference/s3api/list-objects-v2.html):
  automatic pagination requires explicit `--no-paginate` here.
- [AWS prefix conditions](https://docs.aws.amazon.com/AmazonS3/latest/userguide/amazon-s3-policy-keys.html):
  supports narrow listing permission rather than bucket-wide listing.
- [AWS conditional writes](https://docs.aws.amazon.com/AmazonS3/latest/userguide/conditional-writes.html):
  the write condition, not the earlier listing, arbitrates concurrent writers.
- Repository `src/store/backend.rs`, `docs/store/README.md`, and producer
  publication flow were inspected. The ordinary listing parser is intentionally
  permissive and is not used for the absence decision.

## Verification

`tests/store_scoped_head.rs` uses native Rust and a private fake AWS executable
per fixture. It checks complete argument vectors and at most two calls, forbids
PUT/GET, and covers success, denial, malformed responses, duplicate fields,
pagination, KMS-like failures, non-403 gateway errors, and endpoint propagation.
These are software regressions, not cryptanalysis measurements.

The controller performed this read-only response-format check:

```sh
aws s3api list-objects-v2 --bucket setec-astronomy-artifacts \
  --prefix ml-dataset/20261028/store/absence-probe-20261028T0729Z-T708 \
  --max-keys 1 --no-paginate --output json --region us-east-2
```

The captured response is `evidence/controller-empty-list.json`, SHA-256
`2bed0454e7b8948c68020604b7b47ca5c49953cb12abb085eaa08ded4d287337`.
This confirms response shape under controller credentials only. It does not
validate the worker policy or demonstrate a worker publishing a dataset.

Native build/test results and source hashes are recorded in `evidence/`.
Three early build attempts lacked local or transitive dependencies or a
compile-time fixture while the sparse checkout was being initialized. Their
diagnostics are retained. The first complete library run recorded 4,979 passed,
8 failed, and 122 ignored tests. Six failures concerned further missing tracked
GPU/reference inputs; two GPU test failures reported file-lock contention in
the parallel run. Restoring those inputs and rerunning each affected test in
isolation passed. The complete serial retry passed all 4,987 enabled tests,
with 122 ignored and no failures. Both complete logs are retained in the
content-addressed S3 artifact store; the failed parallel run is not replaced.

Independent code review covered the strict parser, exact argument scope,
conditional-write preservation, and error handling. It found the broad
Forbidden trigger, which was corrected. The subsequent review found no
remaining material issue in this scoped diff. This is software review, not an
independent review of a cryptanalysis computation.

The 12 new integration regressions and five existing store CLI tests passed.
`cargo +1.98 clippy --lib --test store_scoped_head -- -D warnings` and targeted
`rustfmt --check` passed. The serial full-library retry invokes the exact compiled
test executable directly while Cargo builds the integration targets; its SHA-256
is recorded in `evidence/library-test-binary.sha256`. A queued Cargo invocation
that was only waiting for the build lock was terminated before test execution;
`library-serial-build-lock.txt` preserves that diagnostic.

## Remaining dataset requirements

The shared worker role's proposed prefix-scoped S3 policy remains awaiting the
user's administrative approval. This patch does not apply it. A subsequent
worker must prove upload and readback with hashes before sustained dataset
production is considered functional. Large native dataset generation, broader
solver coverage, independent replay, and ML export remain separate requirements.
The user-authorized AWS/S3 budget is $100–$200 total through tomorrow, with $200
as the planning ceiling; a reservation estimate is not measured billing or an
account-wide spending guard.

## Session handoff

### What was done (commands, files, features)

Prepared the exact-prefix HEAD403 fallback and independent native regression
fixtures, reviewed AWS primary documentation, and captured a real read-only
empty-list response. Validation and publication receipts are kept beside this
note. No new worker or IAM mutation was performed.

### Decisions made (and why)

Fail closed on every ambiguous response and preserve the original HEAD error.
Restrict the fallback to explicit HTTP403 because an unrelated gateway error
can contain the word Forbidden. Preserve conditional writes for race handling.

### Open threads (unfinished, pick up next time)

Complete PR review, then incorporate this fix in a frozen producer build.
Worker IAM approval and worker-level publication verification
remain open. The shared daily session log initially belonged to T-709; an entry
was appended only after a fresh Conductor scope check reported it clear.

### Errors (what went wrong, how it was handled)

Initial dependency resolution ran before sparse checkout initialization finished;
the failed diagnostics were retained and tracked dependencies and fixtures were
materialized. Parallel GPU lock contention was isolated by serial reruns.
Independent review found the broad Forbidden trigger; it was narrowed and two
regression fixtures were added before execution.

The repair is published in [PR #2763](https://github.com/isogeny-labs/crypto/pull/2763).
After creation, `main` advanced to `189ac727fcc4475b42755d4f1c15f97a4a102070`.
Merging that revision required retaining both appended daily-log entries; it
changed no root Rust, Cargo, or local dependency input used by the passing checks.
The backend and regression source hashes remain identical. Auto-merge is enabled;
hosted validation and review of the resulting revision remain separate gates.
