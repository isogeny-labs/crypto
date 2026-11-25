# Launch status — 2026-11-24

The collector now has an exact-input command wrapper, committed-source exporter
and a locally built static image. The original radius-12 plan was checked in
that image without allocating its production table. A small known-answer image
control ran as UID 1000 with networking disabled; the separately compiled
affine/Euclidean checker checked its twelve base logs and target equation.
Producer evidence is in [the command component](command/README.md),
[report PDF](command/REPORT.pdf) and [receipt](command/validation/component.json).
[pcc:unverified]

The previous controller's 8Gi maximum was below the unchanged 9,566,497,092-byte
payload. This source change adds explicit 14Gi admission with tests for legacy
identity preservation and both computation/fresh manifest resources. No
controller/runtime was deployed. The local image configuration ID is not a
registry manifest digest. Both candidate and original-base controller suites
have the same unchanged process-owner test failure; all 15 contract and 3
top-level cloud tests pass. [pcc:unverified]

The current one-shot `sts:GetCallerIdentity` check returns
`InvalidClientTokenId`. This supersedes the older authentication observation
below; no new CodeBuild or EKS request was attempted. The next live step needs
working authorized execution access, current source-checkpoint and budget
admission, published tested payload/runtime image digests and collector-specific
CRIU/EBS worker-loss continuation qualification. The prepared recovery PVC
requirement is 64Gi, not the old 20GB development estimate; it is unallocated.

The replacement filesystem does not contain the old private working directories
or original local-only archives. Their recovery remains an explicit task;
published source/results are available, and no permanent-loss conclusion is
drawn. New private control journals and build attempts are retained locally
outside Git. No original-width production table or research job was started.

## Observations recorded on 2026-10-28

The following is the preserved October access snapshot, not a current identity
or command-support assertion.

**No original-size experiment has started.** Implementation and development
controls in [PR #2707](https://github.com/isogeny-labs/crypto/pull/2707) are merged.
The authorized AWS key now authenticates to the intended lab account
`104186811093`. The source-only checkpoint gate **passed**, and its encrypted
S3 archive was downloaded and verified. Missing credentials are no longer the
checkpoint blocker.

| Check | Actual outcome on 2026-10-28 |
| --- | --- |
| STS identity | Correct lab account |
| Source-only checkpoint gate | Exit 0; 2,739-byte archive, AES256 storage encryption |
| Checkpoint readback | Hash and both member contents verified; only handoff and metadata |
| `codebuild:StartBuild` on `isolab-experiment-launcher` | Definitive `AccessDeniedException`; no build ID or build created |
| `eks:DescribeCluster` on `isolab` | Access denied |
| Tested-source custom image and standalone-collector recovery | Still unimplemented/unqualified |

The checkpoint SHA-256 is
`165a4d53dee7ad9e24b8adcaff1e7687c825bd6d7a39e286c3a33e8019bf9a59`.
It binds public source `6187036c049b6de2b1521b014f575952e4f1705c`;
all twelve entries in `SOURCES.sha256` still match. This is a **source/handoff
checkpoint**, not a CRIU process checkpoint or a restore qualification.
[Machine-readable access evidence](validation/access_20261028.json).

The single CodeBuild request replaced the normal launcher buildspec with two
read-only STS/EKS inspection commands, no source/artifact publication, no
privileged mode and five-minute execution/queue bounds. AWS rejected it before
creating a build because the caller has no identity-based policy allowing
`codebuild:StartBuild` on that project. No IAM policy was changed. Neither a
default fixture nor an EKS Job/Pod was submitted. There is no ambiguous launch
to retry.

Next execution requires an already authorized launcher identity (or a scoped
grant by the account administrator), a tested-source immutable custom image and
qualification of this standalone collector's CRIU/EBS recovery. The current
ecbench-only GitHub/recovery contracts cannot execute it. Do not widen IAM,
silently substitute a local/non-resumable run, or dispatch the unrelated fixture.

## Earlier checkpoint history

1. Initial checkpoint-gate invocation was rejected before execution by automatic
   approval review: its generic upload path could include private worktree data.
2. After publishing the code, the checkout was audited: clean tracked files, zero
   unpublished commits and no included untracked files. The reviewed checkpoint
   contained only the recovery handoff and generated metadata, referring to the
   public source commit. Original campaign/private development journals are
   outside this checkout. No scalar witness or raw randomness was in that payload.
3. A retry of the **same gate**, with this concrete audit, was permitted. It
   actually executed and stopped with:

```text
checkpoint: no way to upload: no AWS profile 'ecc-challenges-new', no CHECKPOINT_UPLOAD_TOKEN, no /home/agent/.config/checkpoint/upload-token
checkpoint: cannot upload, so nothing was checkpointed
```

That access failure was historical. On October 28 the same gate succeeded using
the authorized lab credentials in a one-shot process. The clean checkout had
zero unpublished commits, modified tracked files or included untracked files;
the audited payload excluded private research journals and all credentials.
A changed worktree must be audited and checkpointed again before an experiment.

The PDF is the implementation/control-stage snapshot made before this access
rechecks. Its development conclusions are unchanged. This status note supersedes
its launch-readiness wording, not its frozen workload or measured controls.
