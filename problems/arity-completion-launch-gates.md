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
- **Last updated:** 2026-11-06

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
- **Observed (2026-11-04 01:52–01:56Z, fresh container after the owner
  reported setting AWS credentials):** the container exports
  `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` (no `AWS_SESSION_TOKEN`,
  no `AWS_REGION`, no `~/.aws/credentials`, no profile); `aws sts
  get-caller-identity` → `InvalidClientTokenId` at the global, us-east-2,
  us-east-1 and us-west-2 endpoints; `aws s3 ls
  s3://isolab-results-104186811093-us-east-2 --region us-east-2` →
  `InvalidAccessKeyId: The AWS Access Key Id you provided does not exist in our
  records`. AWS itself answers, so the request reaches it; the key id it sees
  is not an active key of any account. Remote Control folders of the owner's
  Mac are listed `not_available` to cloud sessions, so the laptop route is
  closed from here too.
- **Observed (route audit, same session):** the GitHub-identity route
  `isolab-experiment.yml` takes an `ecbench.spec/v1` file and builds an image
  holding only `ecbench` and `isolab-fresh-owner` (`taskq/deploy/eks-karpenter/build-bundle`);
  it cannot run `pkm_tower_pilot`, `n83_msweep_cell` or `run_shard.sh`. The
  arbitrary-command route is `isolab_k8s_run` (EKS MCP, argv array,
  digest-pinned image with UID 1000 support) and needs both an AWS identity
  and a custom image carrying the two example binaries, `bash`, `coreutils`
  (`timeout`, `prlimit`), `bc` and `/usr/bin/time`, pushed to the lab's ECR.
- **Derived or proved:** not applicable.
- **Suspected explanation:** the key pair the container receives is not an
  active IAM access key (a temporary `ASIA…` pair set without its
  `AWS_SESSION_TOKEN` gives exactly these two errors; so does a deleted or
  mistyped key, or a placeholder that the environment did not resolve). The
  checkpoint token was never minted for this session.
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
  branch (or `CHECKPOINT_UPLOAD_TOKEN` is set from `presign-upload`). For a
  Claude Code on the web session that means the environment's settings
  (Edit → Network secrets, or environment variables) carry
  `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_SESSION_TOKEN` when the
  key is temporary, and `AWS_REGION=us-east-2`, and a new session is started;
  the session verifies with STS before anything else.
- [ ] A digest-pinned image for the study (the two example binaries built
  at the frozen commits, `bash`, `coreutils`, `bc`, `time`, UID 1000) in the
  lab's ECR, or an executor host with a repository checkout and `cargo` on
  which the build cards run as written. Without one of these the eight cards
  have nowhere to execute even with a valid identity.
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
  environment secrets were refreshed (checked 2026-11-04 01:52Z after such a
  note: still rejected, see Evidence).

## Resolution and history

- 2026-11-04: opened with the two frozen protocols and the development builds
  in `research/arity_completion_20261104/`.
- 2026-11-04 01:52Z: owner reported AWS credentials set; a fresh container
  still fails STS and S3 with the key it holds; route audit added (GitHub
  route is ecbench-only, arbitrary commands need `isolab_k8s_run` plus a
  custom image). Still open.

## Update 2026-11-04 20:10Z: launched; now blocked on Spot capacity and the retry budget

The identity gate cleared at 07:26Z (`coreuser`, account 104186811093) and both
studies were launched through the new command-Job route
(`research/arity_completion_20261104/launch/README.md`, PRs #3421, #3429).
What ran is in `launch/receipts/attempts/`; what blocks the rest:

- **Spot nodes live 10–21 minutes.** Five pods were lost to
  `instance-terminated-no-capacity` on c8a.2xlarge (3) and c8i.2xlarge (1)
  Spot, and m8a.2xlarge and m7i.2xlarge Spot requests were refused outright
  (`UnfulfillableCapacity`); placement scores 1–3 of 10 across every 8- and
  16-vCPU candidate (`launch/receipts/attempts/SPOT-LOSSES.md`).
- **Retry budget.** The prime-tower cell `isogeny-m5-t2` has been lost to
  infrastructure twice, the protocol's `max_attempts` = 2 (per cell).
  Koblitz's m = 5/6 shards have not started (attempt 4 Pending on capacity).

What would unblock it (owner decisions, none taken):

- [ ] Allow on-demand capacity for these two Jobs (the Iso Lab pools are
  Spot-only by policy, `SpotOnlyEC2Launch`), at roughly USD 0.4/h per 8-vCPU
  node; the Koblitz worst case is about 17 h and the prime tower about 12 h
  plus t = 4.
- [ ] Or route them through the CRIU/EBS resumable contract
  (`isolab.eks.resumable/v1`, `.claude/skills/isolab-eks/references/recovery.md`),
  which preserves computation across Spot loss; it needs the command Jobs
  adapted to that contract.
- [ ] Or wait for Spot capacity and file a protocol amendment raising the
  prime tower's per-cell infrastructure retry limit before the next attempt.

Results so far (exact, every file matched its SHA256SUMS): ten Koblitz m = 2
negative controls (chain-s3 4/4 proved_unsat on each of k0n23, k1n29, k0n31,
k0n37, k0n41; chain-s3-orbit 4/4 incomplete at its conflict budget); the three
prime-tower controls under protocol v2 (D 6, 7, 4); kummer-m5-t2 D = 6;
kummer-m5-t3 capped at 8 GiB (inconclusive_cap). Verdicts await each study's
`analyze.py` and the director's assessment.

## 2026-11-06: conjugate-pair and radix follow-up shares the access dependency

`research/lead_followup_20261106/PROTOCOL.md` and blocked cards
`XP-20261106-conjugate-chart-followup` / `XP-20261106-radix-depth-followup`
reuse this launch-gate record. The managed Codex runtime reports no configured
AWS identity, secrets or EKS tool. With a complete `.resume/RESUME.md`,
`scripts/checkpoint.sh gate` reports no upload transport (no AWS profile
`ecc-challenges-new`, no checkpoint upload token). GitHub access works, but
the current `isolab-experiment.yml` dispatch takes ecbench specs; the example
payload requires an authenticated arbitrary-command route and a matching image.
No scientific job was submitted. These are access observations, not mathematical
obstructions or a change to any earlier run's budget/attempt allowance.

Exit conditions for these two cards: reconnect the existing scoped capability,
pass the checkpoint and publication checks, pin the command image/source/binary,
verify live budget/capacity under their shared proposed ceiling, and qualify the
worker controls. Development tests and a merged preparation PR do not satisfy
those conditions or complete either scientific panel.
