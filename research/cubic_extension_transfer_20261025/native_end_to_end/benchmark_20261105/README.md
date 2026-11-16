# Native cold benchmark launch preparation, 2026-11-05

The requested benchmark is not submitted. The proposed panel and resource
ceiling are in [launch-plan.json](launch-plan.json); this is a launch plan,
not a frozen protocol or a measurement result. No scientific run or cloud
build was launched by this preparation. [pcc:unverified]

The read-only AWS preflight uses the supplied lab identity and records budget
and launcher denials in [preflight-attempt1/](preflight-attempt1/). Its statement
record is `PCC-20261105-a6723f`; standing is unverified. Raw command exit codes
and stderr are producer observations, not independent execution evidence.
[pcc:unverified]

The expanded [preflight-attempt2/](preflight-attempt2/) captures the two daily
budgets, monthly budget, CodeBuild polling, ECR authentication and SSM discovery.
Each probe returned AccessDenied. Claim `PCC-20261105-a6723f` pins this summary
and the proposed panel; its standing is unverified. No budget balance, build
permission or runner authorization was established. [pcc:unverified]

The plan preserves the original p109/p157 source curves, eight separately
generated public targets per curve, five independent nuisance seeds, one warmup
and five measured rounds, source rho with/without negation, an A/A arm and real
full-transfer IC/target-rho arms. Every timed execution must start a fresh
process and charge the complete requested cold pipeline. Known-answer scalars
stay outside solver APIs. Timing fields remain absent until actual execution.

Continuation details are in [HANDOFF.md](HANDOFF.md). The shared root resume
file is preserved; this handoff is specific to the cubic benchmark task.

[preparation-checks.txt](preparation-checks.txt) records shell syntax, panel
arithmetic, scoped prose lint, ledger and whitespace checks. Claim
`PCC-20261105-a43948` pins this unsigned transcript; its standing is unverified.
These checks do not execute the crypto pipeline. [pcc:unverified]

## Access needed

Use an already authorized IsoLab operator role/runner where possible. The
maintained [ACCESS.md](../../../../taskq/deploy/eks-karpenter/ACCESS.md) defines
the scoped CodeBuild/S3 caller contract. A native custom image also needs the
existing `isolab/tasks` ECR publication path, and cost admission needs current
budget read access. No IAM policy, trust relationship, pool limit or provider
is changed by this branch.

`capture-access.sh FRESH_DIRECTORY` performs only read-only probes. It never
submits a build or Job and prints no registry tokens. Existing captures are
refused rather than overwritten. An empty stdout file from a failed API call
is not a budget or capacity snapshot.

## Measurement admission

The command route now documented in ACCESS supports a pinned image and an L2
placement profile. It does not automatically supply per-execution L2, replay
or A/A admission. Those observations still need the native measurement adapter;
the profile name and a successful Job are insufficient.

The original requirement is a cold comparison including all construction,
inversion, chart failures, target solving and source verification. Partial Rust
target-call counters keep their existing scope. Online intervals or complete
native GAE prices must not be invented from those counts. Prior correctness
receipts are pilots and are not new timing data.

The [IsoLab skill](../../../../.agents/skills/isolab-eks/SKILL.md) requires:
“Stop new launches when the budget is exhausted or its remaining balance is
unknown.” Current headroom has not been established, so no paid launch is
admitted. [pcc:unverified]
