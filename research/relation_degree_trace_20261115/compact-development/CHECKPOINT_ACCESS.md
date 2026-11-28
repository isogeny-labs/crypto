# Checkpoint access restoration: current task

The user requested restoration in the current environment. Authorization is
not the missing piece; the environment has no credential binding and the
available environment connector exposes status/diagnostic reads only.

Actions completed without printing credential values:

- Installed official AWS CLI 2.40.3 outside the repository at
  /workspace/relation-tools/bin/aws.
- Tried the standard provider chain with STS get-caller-identity; it found no
  credentials. No IAM identity was created, permission broadened or credential
  exported.
- Prepared the task-specific resume and ran the unchanged checkpoint gate
  with that CLI available. The gate failed for missing profile/token; no
  checkpoint was uploaded and no benchmark launched.
- Read current managed-environment status: configuration observations are
  current, runtime is connected, network policy is unrestricted/enforced,
  and both secret bindings and outbound identities are empty.
- Searched available environment tools: request_changes/configuration-write
  tools are unavailable. The Cloud Environment runtime guidance directs the
  user to the running chat's environment settings in that situation.
- Checked the repository's documented routes. The existing APS source-only
  checkpoint workflow is scoped to another approved source/lease; it was not
  repurposed. GitHub App access cannot list repository secrets (HTTP 403).
  No workflow was created to extract a secret.

The required next action is an environment-setting change: bind an
existing CHECKPOINT_UPLOAD_TOKEN vault secret to the current task, or attach
an already-authorized AWS outbound identity. Secret values must be entered
only in settings, not chat, Git, logs or artifacts. Once the binding is ready,
recheck environment status and run the standard checkpoint gate with a fresh
process. A token grants checkpoint transport; it is not an experiment result.

The compact constructor, frozen qualifiers and predeclared 15x protocol are
published in PR #4349. The archive migration workflow completed, and both
compact archive manifest entries match their frozen envelope hash/size.
The runtime target remains unmeasured; source/ledger merge reconciliation is
separate from credential restoration. The task-specific .resume/RESUME.md
replaces an unrelated inherited branch handoff only in this worktree; the
other worktrees' local notes were not modified.
