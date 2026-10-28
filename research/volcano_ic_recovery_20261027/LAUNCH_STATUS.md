# Launch status — 2026-10-27

**No original-size experiment has started.** The implementation and development
controls are published in [PR #2707](https://github.com/isogeny-labs/crypto/pull/2707).
Auto-merge is enabled with merge commits; repository gates still apply.

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

Thus the initial payload rejection is resolved for that audited state. The
current checkpoint blocker is missing configured upload access. A changed
worktree must be audited again; this does not authorize exporting private data.
The runtime also reports no ready AWS identity/EKS connection. A custom-command
image/route and qualified recovery remain prerequisites; the ecbench-only GitHub
workflow is not a substitute.

Enable the existing checkpoint token and lab connection through configuration;
do not paste secret values into issues, PRs or chat. Recheck actual readiness,
bind the tested source/image, verify the checkpoint, and only then submit the
frozen pilot. Preserve every request/Job/Pod identity and reconcile ambiguous
submissions before any retry. No unrelated default fixture has been dispatched.

The PDF is the implementation/control-stage snapshot made before this access
recheck. Its development conclusions are unchanged. This status note supersedes
its checkpoint-readiness wording, not its frozen workload or measured controls.
