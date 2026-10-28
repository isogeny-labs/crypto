# E12 dataset persistence

The user requested live S3 persistence on October 27, 2026 Pacific. The original
runner copies `job.log` every 15 seconds, but copies output datasets only on exit
or Spot notice. SSM inspection confirmed that no `out/` objects existed yet for
the active continuations. Immediate archives were uploaded first; Amendment 16
records a periodic uploader for the existing hosts without restarting solvers.

## Destinations and capture format

Bucket: `ephemeral-runs-104186811093-us-east-1`, region `us-east-1`.

| Part | Job prefix under `jobs/` |
| --- | --- |
| E12a | `20261028-021829-fd-e12a-r6a-20261028-r3-0c1d7f/` |
| E12b | `20261028-031317-fd-e12b-spot-20261028-r3-24148b/` |
| E12c | `20261028-021036-fd-e12c-20261028-r3-914d46/` |

Each job's `snapshots/latest.json` identifies the newest committed archive, its
SHA-256, byte count, and member list. The immutable
`snapshots/<UTC timestamp>-<unique suffix>/output.tar.gz` contains `/work/out`
and available host/resource metadata. Its `snapshot.json` is written only after
S3 verifies the archive checksum. Archives and manifests use conditional
create-only writes; only the latest pointer is updated. The original runner's
final dataset upload remains under `out/`, separate from all live snapshots.

The initial snapshots use suffix `-initial` and a plain `sha256` sidecar.
SSM command `fe4fcb90-c33a-4440-a0cd-de464130bf0c` copied those before timer
installation. Inspection command `992acfe2-c0e9-4259-a6b2-b7da89c8a68b` found
80–92 KiB of output files per host, including nonempty E12b partial draw data.

## Uploader and guarantees

`snapshot-to-s3.sh` validates the exact job ID against its allowlist and local
job record, observes the original deadline, serializes uploads with `flock`,
creates an archive, and checks S3's returned SHA-256 before publishing a manifest.
The timer runs 60 seconds after the previous invocation finishes, at low CPU/I/O
priority. Each service invocation is limited to 60 seconds. A failed upload
leaves the latest committed pointer unchanged and writes an error to `job.log`;
the next timer invocation retries artifact capture, never the experiment.

`install-snapshot-timer.sh` verifies the script hash and installs the isolated
service/timer on the three existing instances. It neither restarts nor signals
the solvers, changes their inputs, extends instance lifetimes, nor launches a
replacement host. The working cache is bounded to one local archive/manifest.

These are copies of live files. They can contain an incomplete final record and
do not capture solver memory or provide mid-closure resume. Only completed draw
records and all existing acceptance gates can support later scientific results.

The observed bucket lifecycle contains one enabled 30-day expiration rule for
`cache/` only. There is no current expiration rule for these `jobs/` datasets.
No bucket policy, lifecycle, IAM policy, or versioning setting was changed.

## Validation and handoff

Local tests verified archive byte/hash preservation, unchanged latest pointers
after upload failure or checksum mismatch, rejection of unregistered jobs, and
the original deadline guard. Shell syntax and whitespace checks passed. No solver
source changed and no mathematical claim is made. Runtime receipts and S3
readback verification belong in `persistence/` beside this document.

Follow-up: verify a committed manifest, download its `archive_key`, compare the
archive's SHA-256 with `sha256`, then inspect extracted raw files. For completed
experiments also retain the original `out/` directory and all original gates.
Conductor task: T-675. Existing E12a/E12b/E12c job identities remain unchanged.
