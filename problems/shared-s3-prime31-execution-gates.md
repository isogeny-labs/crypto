# Shared-S3 Prime31 validation and measurement access

- **Status:** `open`
- **Kind:** unknown compiler termination cause; access/tool limit; implementation gap
- **Affected work:** [PR #4922](https://github.com/isogeny-labs/crypto/pull/4922), [study report](../research/s3_shared_elimination_20261122/REPORT.md), [matched recovery plan](../research/s3_shared_elimination_20261122/matched_recovery/PLAN.md)
- **Next action owner:** PR #4922 lane; authenticated runner/account owner for dispatch and protocol access
- **Last updated:** 2026-11-23

## Problem

The requested complete cold solving-cost comparison over Fp with
p=2147483647 and relation modulus r=2147446831 cannot yet be established:
the full root-library validation has no passing run, the canonical experiment
protocol is unreadable through the available source route, and live supported
runner admission has not been established. The general production chart
adapter and complete accounting also remain implementation obligations.

These are separate execution and implementation gaps. A passing scoped
actual-source qualifier does not close them, and none is a mathematical
obstruction to the construction.

## Evidence and uncertainty

- **Observed compiler failure:** the retained full-root build and check
  [records](../research/s3_shared_elimination_20261122/evidence/README.md)
  report rustc termination by SIGKILL. The later check stopped during metadata
  compilation, before LLVM code generation. OOM is not established; no kernel
  or cgroup diagnostic was available to identify the cause. Repeating the
  unchanged failing local command is not a discriminating experiment.
- **Observed supported validation route:** the maintained
  [rust-all-targets workflow](../.github/workflows/rust-all-targets.yml)
  has a manual `full_library` job with an exact `source_revision` input. Its
  definition, Cargo.lock and build.rs are unchanged between lane tracking
  commit 3eab1e26187bb7740dc3b4565274c138f27560d7 and inspected main
  1e724d0da0e8e3b5f4b24a29ea77eab1a7ceb3be. Production F4, group and tensor
  dependencies have changed on main, so a later merge requires a fresh gate.
- **Observed dispatch access:** the exposed GitHub connector has no workflow
  dispatch operation. The installed gh CLI reports no authenticated GitHub
  host. The reachable workflow page shows Sign in/Sign up and no authenticated
  Run workflow control. No workflow was launched and no runner cap was changed.
- **Observed protocol access:** the workspace experiment-protocol skill points
  to crypto-autoresearcher's canonical `.claude/skills/experiment-protocol/SKILL.md`.
  Retrieved blob 9bd790292cdb7fe3424e25a9ba2ebf8753d59f04 begins with VLT1
  binary content; it is not readable protocol text. The pointer skill does not
  replace the canonical freeze requirement. No decryption or access bypass
  was attempted.
- **Missing measurement admission:** live EKS budget, supported image/source,
  capacity and L2/replay/A/A evidence have not been established. The local aws
  CLI is unavailable; this alone does not prove the account lacks AWS access.
- **Missing implementation:** the matched fixture uses the finite affine
  S3 chart on declared structured bases. It does not provide the production
  full-chart dispatcher or complete operation/cold-cost coverage.
- **Derived or proved:** no impossibility or complexity conclusion follows.
  Diagnostic duration and partial arithmetic counters are not performance
  samples or a complete-cost result.

## What would unblock it

- [ ] An authenticated invocation of the maintained full-library workflow on
  the exact published lane revision completes its full unfiltered library
  suite. Preserve source identity, build/test exits, writer completion,
  executable hash and the native-library artifact. Do not add ignores,
  filters, silent retries or change runner caps to make this condition pass.
- [ ] The canonical experiment protocol becomes readable through an
  authorized route; its exact source, workload, cold boundary, repetitions,
  incumbent and admission checks are frozen before measurement.
- [ ] The general public-target chart adapter, complete accounting and matched
  input/holdout corpus are implemented and independently checked.
- [ ] The supported EKS run has live budget/capacity authorization, a tested
  native source-bound image and the required L2, replay and A/A controls.

The first condition closes only the library gate. The remaining conditions
are necessary before the requested comparative measurement; completing this
record itself establishes no experiment or research result.

## Next discriminating step

- **Action:** finish and independently inspect the matched Original/Fresh/Shared
  public-input driver, run its separate actual-source development qualifier,
  then use the maintained manual library job when legitimate dispatch access
  is available.
- **Inputs and controls:** the fixed p31 public coordinates, separate subgroup
  modulus, both target signs, exhaustive nonidentity F17 diagnostic and
  explicit cap/interruption controls in the linked plan.
- **Execution limits:** the workflow's maintained limits are 130 minutes for
  the job, 40 for build, 70 for tests and zero automatic retries. Development
  calls use separately frozen local guards; they do not authorize a paid run.
- **Outcomes:** preserve any compiler failure or interrupted tests; only a
  successful source-bound full-library artifact closes that gate. A changed
  dependency closure triggers re-review and requalification.
- **Recheck trigger:** authenticated dispatch or protocol access, a production
  adapter change, or an independently admitted supported runner.

## Resolution and history

2026-11-23: recorded the recurring gates after inspecting current source,
workflow, CLI authentication and the signed-out browser page. Unresolved.
Independent driver implementation and development qualification continue.
