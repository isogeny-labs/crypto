# Proposed checkpoint-only upload authorization

No checkpoint has been uploaded. This is a reviewable proposal for the newly
required pre-run resume checkpoint, not a reuse of the four-artifact approval.

Current main [AGENTS.md](https://github.com/aburan28/crypto/blob/20e8f44bba1f04e84a151e6f77e8fa752ebf7ad5/AGENTS.md#resume-checkpoints-in-s3-before-every-run)
requires: “Other runtimes (Codex) run `scripts/checkpoint.sh gate` before
launching and do not launch if it fails.” It also requires pushing checkpoints
when runs start/finish, results arrive or the plan changes. The official
[script](https://github.com/aburan28/crypto/blob/20e8f44bba1f04e84a151e6f77e8fa752ebf7ad5/scripts/checkpoint.sh)
validates RESUME and then invokes its uploader; it is not a read-only gate.
The current explicit user constraint excludes further uploads/credential reuse,
so this dependent action needs new authorization. No checkpoint-only token is
configured in the documented environment variable or token-file location.

Proposed scope is **checkpoint-only writes for this existing PR branch**:

    s3://setec-astronomy-artifacts/checkpoints/crypto/codex__quadratic-twist-evidence-import/

Per timestamped checkpoint, the unmodified script creates:

- `checkpoint.tar.gz`: resume/metadata plus changed specs, an unpushed Git bundle,
  worktree patch and eligible untracked files if any;
- `RESUME.md` and `meta.json` copies;
- the branch prefix's `LATEST` pointer.

For the initial checkpoint after publishing all prepared work, a clean pushed
worktree needs only RESUME/meta: no unpushed bundle, patch, untracked payload,
research asset or binary is necessary. The committed protocol/source can be
retrieved from the public Git head. A local initial payload preview/manifest is
prepared separately for exact review. Uploader-generated timestamps may change.
The same branch-scoped authorization would cover the required start/finish/
plan-change checkpoints, not arbitrary asset uploads or other branches.

The resume document includes the goal, atlas lead, exact preregistered protocol,
fixed curve/point identities, completed prior results clearly dated, current
zero-execution state, next commands, no research runs in flight, budgets/stop
conditions and real access/approval holds. It contains no credential values,
presigned token/URL or chat transcript. Future checkpoints must keep that rule.

Access should use the repository's **checkpoint-scoped upload token**, configured
through `CHECKPOINT_UPLOAD_TOKEN` or its documented secure token file. No prior
AWS key, token minting, profile reuse, deletion or unrelated destination is
requested. Configuring access is separate from granting permission to upload.
No paid compute or extra research-asset upload is requested.

After authorization/access, use the exact unmodified gate. Do not fabricate a
`.resume/.last-push`, use a skip flag, weaken the rule or count local staging as
a successful S3 checkpoint. The uniform experiment remains unexecuted until
that gate succeeds and its independent audit/proof implementation is ready.

## Concrete initial preview

[Manifest](checkpoint-preview/MANIFEST.json) and [archive](checkpoint-preview/checkpoint.tar.gz) are a **local preview only**, bound to published parent88e6846d4cae5e68840f579d2c02a8be7381297d. Four proposed objects total10,789bytes. There is no unpushed bundle, patch or untracked payload. Metadata is restamped for the actual authorized head/time; no object has been uploaded and this preview does not satisfy the gate. The approval proposal is branch-scoped checkpoint writes, not permission inferred from a matching hash.
