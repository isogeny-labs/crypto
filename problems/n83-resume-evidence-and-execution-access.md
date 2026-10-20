# N83 resume evidence, execution access and historical accounting

- **Status:** open
- **Kind:** current storage access regression; historical accounting gap
- **Affected work:** [primary factor-base sweep](../research/koblitz_n83_factor_base_sweep_20261008/RESULT.md)
- **Next action owner:** N83 sweep maintainer
- **Last updated:** 2026-10-20 UTC

## Problem

The resumed primary `a=0` GF(2^83), 81-bit-subgroup study has working
guarded Docker execution. Earlier S3 publications passed custody checks,
but the migrated host's current role is denied S3 writes. The historical
campaign charge ledger has not been fully reconciled here, so a whole-
campaign balance or complete cold-runtime total is not asserted. Fresh
bounded continuation is recorded separately from prior allowances.

## Evidence and uncertainty

- **Observed:** the initial PR head was `8f971585692c702d2804668b322e2440177f80d2`,
  without the later run. During recovery upstream advanced to
  `63844b3b2b674d31ef07e8dec8d0f63ffe66bc52` and published its retained receipts.
  All 31 K2048 and 26 K4096 manifest file hashes passed; five K2048 source
  snapshots, worker consistency and independent public-query replay passed.
  Its expected Mac path is
  `/Volumes/SSD990/crypto/artifacts/n83_k2048_natural_highcap_20261018/`.
  That path has not been inspected from this executor.
- **Observed:** HEAD on the exact K2048 S3 key returned 403. No AWS profile
  selector or OIC identity manifest was provided; Docker and Conductor are
  unavailable. These observations do not identify the cause of S3 denial.
- **Recovered:** [native reconstruction](../research/koblitz_n83_factor_base_sweep_20261008/verification/cloud-input-reconstruction-20261018/RESULT.md)
  reproduced the exact archive and passed every-point generic replay and a
  semantic mutation rejection. Lack of archive bytes is therefore resolved.
- **Remaining audit limits:** original static binary bytes and live Docker
  cleanup have not been rechecked on this executor. Retained audit receipts
  report their earlier checks; current recovery does not repeat them.
- **Not established:** natural relation rate, useful rank, complete cold
  runtime, minimum-runtime base, or any mathematical obstruction.

## What would unblock it

- [x] Recover the retained Mac protocol/source/run receipts and check their
  manifest hashes, source snapshots and public-query group replay.
- Original static-binary and live-cleanup rechecks remain unavailable here.
  The retained earlier full audit remains evidence; repeating it is not a
  prerequisite for a new experiment with its own verified worker.
- [x] Restore the retained supervisor's one-CPU, 4-GiB, zero-swap,
  networkless execution environment and verify it live. The packaged
  Docker service was started through authorized sudo, and all six retained
  stages have live filtered Docker inspection and cleanup evidence.
- [x] Verify authorized storage access: the user-selected
  `setec-astronomy-artifacts` destination passed conditional PUT and fresh
  HEAD/GET checks for the exact K2048 archive and compact receipt bundle.
- [ ] Reconcile the campaign capacity ledger before further search work.

These gates permit audited continuation. They do not establish a winner;
natural rank, column logs, target recovery, matched references and full
cold accounting remain separate requirements.

## Next discriminating step

The [native continuation](../research/koblitz_n83_factor_base_sweep_20261008/verification/cloud-matched-k2048-k4096-20261018/RESULT.md)
has now frozen and executed K2048/K4096 construction, natural-query and short
control cells. Its separate accounting records the declared 2040-second main
and 360-second short-control worker envelopes, observed stage intervals and
unknown ancillary/global totals; no unused historical allowance was inferred.
Reconcile older charged intervals before asserting a campaign-wide total.
The next execution diagnostic is the
[native K4096 terminal-observability gate](n83-native-k4096-terminal-observability.md).

## Resolution and history

2026-10-18: exact input reconstruction resolved archive recovery. Later-run
retained receipts were subsequently published upstream and passed hash/source/
query replay. Full original-binary audit, execution access and full-runtime
selection remain unresolved.

2026-10-18 publication update: [verified new-bucket custody](../research/koblitz_n83_factor_base_sweep_20261008/verification/setec-publication-20261018/RESULT.md)
resolves S3 storage access for the user-selected destination. The earlier
403 on the old bucket remains historical. Conductor now returns a clear
scope check. Docker's client is available, but its explicitly selected
managed daemon is unreachable and no dockerd process was observed. UID 1000
cannot start the managed daemon under the runtime guidance; environment
recovery remains necessary for the retained container protocol.

2026-10-18 Docker continuation update: authorized sudo is available outside
the sandbox. The installed systemd service was inactive and was started
successfully. Task-local sudo Docker access preserves global socket/group
permissions. Live container controls and cleanup now pass, and K4096 is also
stored and freshly verified in the user-selected bucket. The previous
startup/access obstruction is resolved; historical accounting and the new
native K4096 counter-observability gap remain separate open obligations.

### 2026-10-20 UTC: storage access regression on migrated host

Docker readiness was restored by pulling the exact retained image digest;
new native execution is available. S3 access is currently unverified/blocked:
the migrated host role spot-experiments-WorkerRole-G1FZ84yk3gS6 in account
104186811093 receives403 for the known400K raw object's HeadObject and
AccessDenied for GetBucketLocation, explicitly naming a missing identity policy.
No OIC identity selector or AWS profile is configured. Prior verified uploads
remain historical evidence; their success does not establish current access.
New800K raw evidence must remain local until authorized PUT plus fresh HEAD/GET,
payload/native replay and repack succeed. Restore the intended AWS identity or
bucket permissions for this role, then execute those checkable storage gates.
The user has been asked to restore access; no credentials were inspected or replaced.

A direct conditionalPutObject of the retained worker-hash evidence record now
also returnsAccessDenied explicitly because no identity policy allows
s3:PutObject. [Actual readiness receipts](../research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-800k-20261020/storage-direct-put-readiness.stderr)
retain the action, role and exact content-addressed resource. Native store
readiness failed earlier atHeadObject403. Upload completion remains open.
Git CLI credentials are also absent; the connected GitHub app has verified
repository Git-blob write access, so PR publication can use that authorized
connector with exact local source commits preserved separately in a Git bundle.
