# High-arity closure runner identity does not reach the deployed stack

- **Status:** `open`
- **Kind:** access/tool limit
- **Affected work:** [high-arity Boolean closure-degree discriminator](../research/dreg_high_arity_20261101/README.md)
- **Next action owner:** AWS account/budget administrator; experiment executor after identity, budget, and custody gates pass
- **Last updated:** 2026-11-01

## Problem

The preregistered high-arity closure discriminator cannot be submitted through
the repository's deployed `ephemeral` runner because the current AWS identity
resolves to account `590183823895`, while that runner is deployed in account
`104186811093`.  No scientific draw generation or closure scan was launched.

The repository's 2026-11-01 cost inspection independently reports both USD 25
daily AWS budget thresholds already exceeded.  The blocked prelaunch record at
`research/dreg_high_arity_20261101/launch-request.json` therefore has no
backend submission command and authorizes no spending.  Its USD 20 campaign
ceiling, resource/deadline/retry limits, exact draw-generation commands, and
remaining GitHub-custody obligations preserve a bounded route forward without
turning either operational stop into negative mathematical evidence.

The affected study is the native 32 GB panel at
`(m,n,ell) = (5,11,2), (5,13,3), (6,11,2)`, including its historical identity
replay and satisfiable controls.  Changing the account, backend, source ref, or
registered experiment to bypass this preflight would not satisfy the study.

## Evidence and uncertainty

- **Observed:** on 2026-11-01, `aws sts get-caller-identity` resolved to account
  `590183823895` and user `adam`.  Reading `/ephemeral/config` in `us-east-1`
  returned `ParameterNotFound`.  Repository history identifies the deployed
  runner account as `104186811093`.  No alternate AWS profile or OIC identity
  manifest was available in the session, and no launch command was issued.
- **Observed:** the checked-in cost inspection records both USD 25 daily
  thresholds above their limits; current account-budget and lane-ledger
  headroom has not been re-established.  The exact draw-specific closure
  commands, backend request identity, and GitHub publication-check receipt are
  consequently still unfiled.
- **Derived or proved:** the current identity cannot discover the named runner
  configuration through the tested SSM path.  No mathematical or solver result
  is established.
- **Suspected explanation:** the session received credentials for a different
  AWS account than the one hosting the deployed runner.  Whether the expected
  account has every later EC2, S3, Spot, and checkpoint permission is untested.
- **Missing capability:** an authorized identity in account `104186811093` that
  can read the deployed runner configuration and perform the study's read-only
  preflight.
- **What this evidence does not establish:** it is not evidence about closure
  degree, satisfiability, resource demand, solver correctness, Spot capacity,
  or the feasibility of any index-calculus lead.  It does not consume a
  scientific attempt.

## What would unblock it

- [ ] `aws sts get-caller-identity` reports account `104186811093` under an
  explicitly authorized launcher principal.
- [ ] From the same environment and region, the ephemeral runner's `config`
  and `ls` commands succeed, and
  `/ephemeral/config` is readable without changing the frozen study.
- [ ] The pushed experiment commit equals the requested runner ref and
  `scripts/checkpoint.sh gate` passes before any draw generation or closure
  execution.
- [ ] Account-budget and lane-ledger telemetry proves sufficient unreserved
  headroom under the single USD 20 campaign ceiling.
- [ ] The exact phase-specific launch command and configuration are committed
  and pushed, `check-experiment-git` verifies the live GitHub commit, and its
  receipt is retained and bound before any build or job.

These checks clear only the observed identity, budget, and custody blockers.  Binary
and source identity, durable-output, control, capacity, execution, export, and
independent-review gates remain separate obligations.

## Next discriminating step

- **Action or proof obligation:** after the correct identity is attached, rerun
  the read-only STS/config/list preflight, then compare the local commit to the
  pushed experiment ref and run the checkpoint gate.  Do not launch if any
  check differs.
- **Inputs and controls:** expected AWS account `104186811093`; SSM parameter
  `/ephemeral/config` in `us-east-1`; the exact pushed high-arity study commit;
  and its completed durable resume checkpoint.
- **Execution limits and stop condition:** this step is read-only and stops
  before instance allocation.  Stop on the first identity, configuration,
  source-ref, or checkpoint mismatch.
- **Outcomes and next actions:** a complete passing preflight permits the
  separately registered launch workflow; another missing parameter or wrong
  identity preserves this blocker; a later permission or capacity error must
  be recorded as a new infrastructure observation, never a mathematical
  result.
- **Recheck trigger:** an administrator supplies or confirms an authorized
  launcher identity for account `104186811093`.

## Resolution and history

Unresolved.  Opened 2026-11-01 after the read-only preflight.  This record does
not freeze the protocol, launch a worker, generate study draws, or establish a
closure-degree result.
