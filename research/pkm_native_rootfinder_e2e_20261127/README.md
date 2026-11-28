# Native Rust PKM end-to-end measurement

Status: preparation only. No benchmark execution or performance observation
is recorded here. PR #5303 continues the performance work after
the native implementation landed in PR #5283.

The immediate question is which of the merged native F4, quadratic-scan and
resultant-GCD backends minimizes the complete cold cost of solving one target
on the three historically selected degree-32 PKM instances. The three specs
are prospective diagnostic inputs, not frozen protocols. The earlier reported
3.695x factor-base comparison is separate evidence, not a gain from PR #5283.

## Comparison and boundary

Each spec selects one historical curve with a 13-bit field by its original constructor
seed, then creates eight fresh public single-target workloads. The solver
does not receive a planted logarithm. The native planner must confirm the
historical curve identity, and registration must be completed before any new
result is cited. Selection was originally based on equal factor-base size;
this panel cannot support a scaling or current `vs_rho` claim.

| Input | Choice |
| --- | --- |
| Curve constructor seeds | 20261185, 20261221, 20261276 |
| Public-target seed | 2026112701 |
| Algorithm seed | 2026112702, paired across arms |
| Interval base | `prime-abscissa:below=32` |
| Degree-32 base | `prime-isogeny:t=5,seed=5,aux=ordinary` |
| Expected historical size | 18 lifting abscissae for both bases; verify before qualification |
| Backends | `pkm-semaev:m=2,engine=f4`, `engine=scan`, `engine=resgcd` |
| Common controls | chain mode, degree cap 8, walk targets, incremental Gaussian elimination, 1,000 relation trials |
| Reference | native `rho.negation` on each same public target |
| A/A | identical duplicate of `velu-scan` |
| Measurement | one warmup, six measured rounds, alternating arm order, required L2 |
| Timeout | 60 seconds per complete execution; no per-solver wall cutoff |

There are eight arms per spec: six IC combinations, rho, and the duplicate
scan. Across all three specs, plan 1,152 measured executions and 192 warmups.
Preserve failures, timeouts and partial sessions. Do not replace targets,
drop slow arms, tune parameters, or restart with a new seed after observing
data. Exhaustion is an outcome, not permission to silently increase a cap.

The primary boundary is `cold_end_to_end`: factor-base construction (including
auxiliary-curve search), oracle setup, all successful and unsuccessful relation
trials, matrix work, recovery, and internal scalar verification. Keep the
separate online window and phase costs as secondary diagnostics. Curve/target
construction belongs to the specified workload, outside the solve window.
Every returned scalar must satisfy `[k]G = Q`, and every deterministic record
must pass a full native replay.

Report per-curve mean S, the native generic floor, the ratio to matched rho,
and paired ratios of totals with intervals. Report wall median/minimum and
A/A spread only after L2 admission; a difference within the measured noise
floor is unresolved. Apply the repository's 0, 0.1 and 1 GAE pricing band to
unpriced work on every arm, including rho, and retain peak RSS. No arm is
ranked from incomplete runs or missing counters.

The primary contrast is `velu-scan` against `velu-f4` separately on each curve.
A positive primary result requires complete, replayed pairs on all three
curves, an upper 95% paired ratio bound below one on each, no ordering reversal
in the pricing band, and any wall difference outside the A/A noise floor.
The resultant-GCD and interval contrasts are secondary exploratory readings;
report every contrast and do not select a favorable one as a new primary.
This design alone does not qualify a globally fastest backend.

The existing PKM counters were corrected by PR #5283. Old versus new GAE
numbers alone cannot attribute a gain to its arithmetic changes. A separate
before/after comparison must bind baseline
`73266d27a75e3b1c294c12fa6e0285e16027ff82` and candidate merge
`addead2953bbe8b637351a0220f32a4e8a3d84d0`, use matching instrumentation, and
retain identical-output checks. Native `ecbench compare --b-dir` supports
paired count comparisons but deliberately refuses cross-session wall claims.
The present backend panel will not be represented as that patch comparison
or as qualification against every external rootfinder.

## Local instrument check

The native release build completed, and all three specs were accepted by
`ecbench plan --doc`. Each reproduced its intended historical curve, eight
public workloads, 384 measured execution slots and 64 warmup slots. Every
workload/round has eight arms sharing one algorithm seed. The binary SHA-256 is
`e5a99edc17710b0eb597d2a06b11e4497e7f82e79cee2d0dd0efd3d98fb8e5cd`.
The build command, toolchain, complete schedules and hashes are retained in
[local-validation.json](local-validation.json) and [plans/](plans/).
[pcc:unverified]

The planner reports `registered: false` for all three curves, with no EC1
representation binding. Add their native constructor records and registry
entries before freezing or citing new results; retain the historical inputs.
These local glibc/Rust 1.98 checks are preparation, not an admitted musl task
image, solver run, isolation result, or performance observation.
[pcc:unverified]

## Current launch blockers

The read-only budget workflow at
[run 46072715236](https://github.com/isogeny-labs/crypto/actions/runs/46072715236)
used trusted `main` revision `e2bd589690231236954267e3228b1b4395e3894a`.
The existing role was configured, but AWS denied
`sts:AssumeRoleWithWebIdentity` at 2026-11-27T21:49:11Z. Budget reads were
skipped; `launch_admitted` and `aws_compute_started` are false. The extracted
non-secret receipts are in [access/](access/). Their ZIP SHA-256 is
`428a39774d0e4ea6dcf5398d40c21a8ad8d1b2f83ccb49de68b2eedcef5d4604`, matching
GitHub's artifact digest. [pcc:unverified]

The failed role is
`arn:aws:iam::104186811093:role/isolab-remote-launcher-GitHubRole-FaCZneVBkF6D`,
session `isolab-budget-46072715236`. The error alone does not establish which
IAM/OIDC condition rejected it; an administrator must inspect the existing
trust configuration and audit trail. [pcc:unverified]

The local `scripts/checkpoint.sh gate` also returned exit 1: no AWS profile
`ecc-challenges-new`, no `CHECKPOINT_UPLOAD_TOKEN`, and no configured token
file. The exact output and input-note digest are retained in
[access/checkpoint-gate.json](access/checkpoint-gate.json) and
[access/checkpoint-gate.txt](access/checkpoint-gate.txt). No S3 checkpoint was
created. The pre-existing resume-file state was restored after this check.
[pcc:unverified]

The current workspace has no bound AWS identity or checkpoint upload token.
An existing authenticated Iso Lab host/role, or an administrator's repair of
the configured GitHub OIDC trust, is required. No identity, trust policy,
resource quota, or branch protection was changed.

The trusted launcher also has a schema compatibility gap: `checkpoint-source`,
the MCP `benchmark_args`, and `run-ecbench` accept only `ecbench.spec/v1`,
whereas new native runs require v2 with an explicit curve policy. These specs
use v2 diagnostic policy. Do not relabel them as historical v1 reproduction;
review and deploy compatible launch validation before submitting them.

The canonical `experiment-protocol` skill, schema and checker in the connected
`isogeny-labs/crypto-autoresearcher` repository are vault ciphertext in this
environment, with no vault key bound. The plaintext `VAULT.md` documents the
normal authorized unlock route. Until the canonical instructions and checker
are available, this design remains a draft and has no fabricated
`experiment.protocol/v1` freeze receipt.

## Proposed resource envelope

Use the existing Iso Lab x86 L2 Spot lane, one job at a time, four guaranteed
benchmark CPUs, at most 16 GiB pod memory, and the existing c8a.2xlarge worker
class. The native runner sets `RAYON_NUM_THREADS=1` and `OMP_NUM_THREADS=1` for
each measured child. Cap each of the three jobs at 4,800 seconds; zero retries/restores.
Proposed total ceiling: USD 10, including builds, queue/idle capacity, failures,
storage and transfer, subject to the existing USD 200 daily and USD 1,000
monthly shared allowances. Remaining account and lane budget is currently
unknown, so no cloud build or benchmark is admitted. The proposed original
absolute deadline is 2026-11-28T04:00:00Z. A changed deadline requires a recorded
protocol amendment, not a renamed retry.

## Resume sequence

1. Retain the completed native build and plans. Add the missing native curve
   constructor records and ICV1/EC1 registry entries, then recheck the changed
   instrument. Planning is an instrument check; it supplies no performance
   observation.
2. Resolve the authenticated route and read current budget, reserved/charged
   lane spend, existing jobs, live pool limits and quota. Resolve v2 launch
   compatibility without weakening L2, A/A, replay or source/image identity.
3. With the canonical protocol tool available, validate and freeze this design
   before data. Publish the full source/spec/protocol/launch closure, then pass
   `scripts/checkpoint.sh gate` and retain a matching S3 readback.
4. Build from the published source without AWS credentials; use the trusted
   publisher and digest-pinned image. Submit once per spec within the original
   envelope. Capture build, workflow, source, binary, job and pod identities.
5. Download and hash-check complete sessions. Run native `verify --replay-all`,
   `compare`, `table`, `band` and `resources`; retain rejected runs. Produce the
   experiment export, report/figures and benchmark/dashboard evidence before
   making any performance claim. Keep #5303 draft until this work is complete.

## Requirements and evidence

| Requirement | Current evidence / missing work |
| --- | --- |
| Rust production integration | merged PR #5283; prior affected-module tests retained in the November 27 session log |
| Complete cold pipeline | prospective native `ic.pipeline` specs in this directory |
| Matched targets and noise control | common target/algorithm seeds, six rounds, duplicate scan arm |
| Exact build and native planning | release build and all three plans accepted; [receipt](local-validation.json); registration still needed |
| Checkpoint and admission | blocked; fresh OIDC failure and unknown budget recorded above |
| Measured e2e result and full replay | not run |
| Patch-specific speedup | not measured; corrected accounting requires matched controls |
| Fastest external-backend qualification | remains open as a separate requirement |
