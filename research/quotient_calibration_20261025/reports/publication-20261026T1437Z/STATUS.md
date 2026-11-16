# Approved raw-log archive publication

The exact approved archive was uploaded and independently downloaded on
2026-10-26 at 14:37 UTC. This resolves raw-log publication only; no new
target-calibration or inversion experiment ran.

| Field | Verified value |
| --- | --- |
| File | `reports/run-20261025T1845Z/runs.tar.xz` |
| Bytes | 67,540 |
| SHA-256 | `ae5e8b6973db769db3ccef01fb81ba5837702a26221206134091a8bdff3bad2a` |
| Destination | `s3://setec-astronomy-artifacts/store/sha256/ae/ae5e8b6973db769db3ccef01fb81ba5837702a26221206134091a8bdff3bad2a` |
| Manifest | [Generated artifact manifest](../run-20261025T1845Z/.artifacts.json) |
| Operation receipt | [STATUS.json](STATUS.json) |
| Delivery validation | [VALIDATION.md](VALIDATION.md) |

## Verification

The original archive's size and SHA-256 were checked before network work.
The repository's `tools/artifacts/artifacts.py push` stored exactly one
object and generated the manifest and ignore entry. Its single-object
upload uses `If-None-Match: *` and a SHA-256 checksum; the tool checks S3's
reported checksum. The subsequent remote verification exited 0.

For readback, the original local archive was temporarily moved aside and
`artifacts.py pull` used an empty cache with the S3 transport. It hydrated
one file, rather than accepting an existing local copy. The downloaded
file's SHA-256 and size matched the approved values, and final
`verify --remote` reported one hydrated entry and zero problems. The
original local copy was restored. All four artifact operations exited 0.

Credentials were supplied only to the temporary process and its children.
They are absent from the archive, tracked files and operation receipt.
No bucket policy or repository secret was changed.

## Sources checked

- [Repository artifact policy and commands](../../../../docs/artifacts.md):
  use the content-addressed store and commit its generated manifest.
- [Artifact tool implementation](../../../../tools/artifacts/artifacts.py):
  conditional upload, remote checksum verification and hashed hydration.
- [AWS conditional-write documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/conditional-writes.html):
  `If-None-Match: *` prevents replacement of an existing object key.
- [AWS object-integrity documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html):
  S3 checks the supplied upload checksum; downloading and hashing the bytes
  provides an additional end-to-end check.

## Scope and remaining work

The current repository is `isogeny-labs/crypto`, repository ID 1221029056,
and the existing delivery is PR #2569. Auto-merge is enabled subject to
normal gates. Conflict resolution and hosted CI/review are tracked on that
PR; archive publication does not establish a merge or a passing CI run.

Frozen source inputs, challenge coordinates, protocol, native source and
historical reports are unchanged. Target fitting, all 32 target challenges,
inverse fibers and Profile B initialization remain unexecuted. No new
vulnerability, map identity or speedup is claimed.
