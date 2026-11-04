# Arity-completion runs blocked at the launch gates (AWS identity, checkpoint token, budget)

- **Status:** `open`
- **Kind:** access/tool limit; resource limit (budget gate); separate from any
  mathematical obstruction
- **Affected work:** `research/arity_completion_20261104/` (prime-tower m = 5
  cells; Koblitz chained-S3 m = 5, 6 cells at n = 23..41 and the n = 83 cap
  lift); `research/dreg_high_arity_20261101/` (protocol v4 refreeze and
  execution)
- **Next action owner:** owner (credentials and budget); then the session that
  holds them runs the frozen cards
- **Last updated:** 2026-11-04

## Problem

We cannot launch the preregistered arity-completion cells from the Claude Code
cloud container because every launch gate that needs an AWS identity fails
there: `aws sts get-caller-identity` returns `InvalidClientTokenId`, no
`CHECKPOINT_UPLOAD_TOKEN` or `~/.config/checkpoint/upload-token` exists, and
so `scripts/checkpoint.sh push` cannot reach S3, which makes
`.claude/hooks/checkpoint-hook.sh` block every run launcher (`ecbench`,
`cargo run --release`, sweep and bench scripts, and any backgrounded Bash).
isolab-eks submission (`remote/launch-remote`) and the artifact store
(`artifacts.py push`) need the same identity (account `104186811093`,
region `us-east-2`). The dreg v4 study additionally records that both USD 25
daily AWS thresholds were exceeded on 2026-11-01 and that STS answered from
account `590183823895`, so its budget and account gates are failed on their
own terms.

## Evidence and uncertainty

- **Observed:** `scripts/checkpoint.sh push` on this branch (2026-11-04 00:43Z) → `InvalidAccessKeyId: The AWS Access Key Id you provided does not exist in our records` on `PutObject` to `s3://setec-astronomy-artifacts/checkpoints/crypto/claude__arity-completion-20261104/…`, so `checkpoint.sh gate` returns 2 and the hook blocks launchers; `aws sts get-caller-identity` →
  `InvalidClientTokenId` (2026-11-03 20:16Z and 2026-11-04 00:20Z, this
  container); `scripts/checkpoint.sh status` → `RESUME.md: NOT READY`, `last
  push: stale or none`; `artifacts.py push` of two report PDFs → `403` on
  `HeadObject` (recorded in cryptanalysis PRs #1384 and #1385 READMEs);
  `research/dreg_high_arity_20261101/README.md` account and budget gates.
- **Derived or proved:** not applicable.
- **Suspected explanation:** the environment's `AWS_ACCESS_KEY_ID` /
  `AWS_SECRET_ACCESS_KEY` are stale or belong to a principal without the
  lab's roles; the checkpoint token was never minted for this session.
- **Missing capability:** a valid AWS identity for account `104186811093`
  with the roles in `taskq/deploy/eks-karpenter/ACCESS.md`, or at minimum an
  upload token from `scripts/checkpoint.sh presign-upload`; budget headroom
  under `taskq/deploy/eks-karpenter/COST_CONTROLS.md`.
- **What this evidence does not establish:** nothing about the arity
  question itself. The protocols, cards, builds and development smokes do not
  depend on the identity and are in the study directory.

## What would unblock it

- [ ] `aws sts get-caller-identity` in the launching session reports account
  `104186811093`, and `scripts/checkpoint.sh gate` passes for the study
  branch (or `CHECKPOINT_UPLOAD_TOKEN` is set from `presign-upload`).
- [ ] Recorded budget headroom for the cells' ceilings (stated in each
  `protocol.json` `execution.placement`), confirmed against the cost controls
  before submission.
- [ ] For dreg v4: the refreeze steps a–h in its README, in order.

These conditions are necessary; meeting them lets the frozen cards run and
the cells report. They establish nothing about the predictions until the
runs are replayed and audited.

## Next discriminating step

- **Action or proof obligation:** run the frozen cards in
  `research/arity_completion_20261104/cards/` through isolab-eks from a
  credentialed session; Haiku executor, Sonnet director assessment.
- **Inputs and controls:** as the two protocols state (planted positive
  controls, A/A duplicates, m = 2 negative control, m = 4 recorded cells).
- **Execution limits and stop condition:** the per-cell caps and ceilings in
  the protocols; no cap is raised after the freeze.
- **Outcomes and next actions:** supported predictions close the m ≥ 5
  lines on prime towers and Koblitz chained SAT; refuted ones name the cell
  to scale; capped cells stay inconclusive and are reported as such.
- **Recheck trigger:** a credentialed session, or the owner's note that the
  environment secrets were refreshed.

## Resolution and history

- 2026-11-04: opened with the two frozen protocols and the development builds
  in `research/arity_completion_20261104/`.
