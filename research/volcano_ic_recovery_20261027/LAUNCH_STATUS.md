# Launch status — 2026-10-28

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
