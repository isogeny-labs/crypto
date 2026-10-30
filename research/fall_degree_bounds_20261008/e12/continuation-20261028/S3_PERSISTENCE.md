# E12 dataset persistence

**Verified October 27 at 8:51 PM Pacific (03:51 UTC):** E12a and E12c have active
timers and have each produced automatic snapshots after installation. Downloaded
archives match their published SHA-256 digests. E12b completed at 8:45 PM Pacific
and terminated normally before timer installation; all four draw records and
32 output files are saved under its original `out/` prefix. Its final archived
copy is also checksum-verified and referenced by `snapshots/latest.json`.

| Part | Verified archive | Draw records in this snapshot |
| --- | --- | --- |
| E12a | `snapshots/20261028T035020Z-32ec1200/output.tar.gz` | 0; original gates and other output files saved |
| E12b | `snapshots/20261028T034958Z-final/output.tar.gz` | 4; producer completion check passed |
| E12c | `snapshots/20261028T035013Z-0ccd0e54/output.tar.gz` | 0; original gates and other output files saved |

The E12a/E12c latest pointers continue advancing. This table describes the
readback receipts, not a claim about subsequently completed draws. See
[`latest-a-readback.json`](persistence/latest-a-readback.json),
[`latest-b-readback.json`](persistence/latest-b-readback.json), and
[`latest-c-readback.json`](persistence/latest-c-readback.json).

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
All three initial archives were downloaded and checked against the hashes
returned by SSM. E12b's initial archive contained two records; the later final
archive contains all four. The raw final dataset is 1,686 bytes with SHA-256
`055811b722f441ab681172a605f9a540b174734dbb4f23ff9f03b931d948c81b`.

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

Installation command `7e1301ab-8ec0-4507-baed-a113d40ed8b3` succeeded on E12a and
E12c: the timers reported active and their service exit codes were zero. The
first installation request included E12b, but AWS rejected it with
`InvalidInstanceId` because that job had just completed and terminated. After
reconciling EC2, SSM, and the completed S3 outputs, installation targeted only the
two remaining hosts. No solver was restarted and no replacement instance was
launched. The failed request is preserved separately from the successful one.

Receipts remain loose because the handoff and this document reference individual
manifests/readback records. The actual datasets and archives remain in S3; they
are not duplicated into Git. The full producer completion report is preserved,
but independent scientific review of E12b's results has not been performed here.

Follow-up: verify a committed manifest, download its `archive_key`, compare the
archive's SHA-256 with `sha256`, then inspect extracted raw files. For completed
experiments also retain the original `out/` directory and all original gates.
Conductor task: T-675. Existing E12a/E12b/E12c job identities remain unchanged.
