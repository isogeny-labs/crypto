# EKS admission audit for frontier workstreams 3–7

**Status (2026-11-19): blocked before launch.** No build, image publication,
CodeBuild request, Kubernetes Job, or paid experiment was started by this audit.
The current runtime has a usable GitHub identity, but it has no AWS identity,
Kubernetes client, IsoLab MCP connection, or checkpoint upload transport.
The trusted main-branch workflow is active, but its latest manual dispatch
failed in its `contracts` job before build because artifact hydration failed.

The audit also separates two execution contracts that cannot be conflated:

- The existing standalone GHS package produces a custom image and submits an
  `isolab.eks.command-request/v1` Job. That route can produce deterministic and
  counted evidence under standard isolation, but its wall clock is diagnostic.
- The requested isolated wall comparison needs the audited L2 benchmark
  contract. The current commit-building workflow bundles `ecbench`, not the
  standalone GHS driver, and native command Jobs do not provide L2 replay/A/A
  admission. A later offline contract in `../l2_bridge/` binds the standalone
  inputs and artifacts but deliberately refuses admission because it has no
  independent native isolation grader. The GHS driver must first become an
  ECBench method or the reviewed L2 harness must gain an equivalent trusted
  custom-binary path.

No launch-ready request is included. Generating one before the final source,
protocol, budget ceiling, checkpoint, live budget readback, and all-tenant pool
receipt exist would create a misleading artifact.

## Route matrix

| Route | Read-only result | Disposition |
|:--|:--|:--|
| Installed EKS MCP | `codex mcp get isolab-eks` returned no registered server; no IsoLab tools are exposed to this session | blocked |
| Direct AWS / CodeBuild bridge | `aws sts get-caller-identity` returned `Unable to locate credentials` | blocked |
| Direct pool check | `kubectl` is absent; `isolab_pool_check.sh` exited 3 before any cluster read | blocked |
| Checkpoint gate | no approved AWS profile, upload token, or token file; gate exited 1 and uploaded nothing | blocked |
| GitHub Actions identity | authenticated account has repository admin/push access; workflow 390723163 is active | available for dispatch only after all gates are frozen |
| Main GitHub workflow | run 44374607134 at main `ad2a014c...` failed in `contracts` artifact hydration; build and experiment were skipped | blocked |
| Main generic workflow payload | source-commit build supports `ecbench.spec/v1` and an `ecbench` image, not the standalone GHS executable | implementation gap |
| Standalone GHS command path | request generator controls passed 7/7 locally; custom-image publication and command submission still need an authorized operator | implemented, unexecuted |
| L2 wall path for standalone GHS | offline schema/validator/artifact checks exist and default verification exits 3; command Jobs still lack a trusted independent L2/replay/A/A grader | design-only, non-admitting |

## Open infrastructure work already visible on GitHub

- PR #4695 adds a read-only budget-telemetry workflow. It is open, not on
  `main`, and deliberately always reports `launch_admitted=false`; it does not
  supply pool, checkpoint, image, or lane-reservation admission.
- Draft PR #4705 scopes artifact hydration per job and is relevant to the
  current `contracts` failure. It is not merged and does not by itself restore
  every missing store object or qualify the GHS driver for L2.
- At audit time, task branch head
  `2b88bdd84b2c102da686d4c589b2d6a68cea07d3` was published. Workstream
  implementation has continued since that observation. The hash is a
  historical tracking revision, not the final experiment revision.

## Budget boundary

The retained matched-family M3 protocol has one USD 5 all-attempt ceiling,
including failures and its one allowed infrastructure retry. The R67–R83
180-call worker-scaling matrix requires a separately approved finite ceiling;
none is frozen yet. The shared account policy is USD 200/day and USD
1,000/month, but those policy limits are not available spending balances and
cannot substitute for current telemetry, reservations, or a lane ceiling.

## Protocol conflict to resolve before freezing

The R67–R83 plan currently names `cpu-spot` in its pool-check command while it
requires every wall observation to earn L2. The maintained EKS contract routes
audited native L2 benchmarks to `cpu-l2-spot`; standalone command Jobs use
standard isolation. The final protocol must name one implemented execution
contract and its matching pool. The later offline contract validates this
boundary but cannot launch or admit a result. Changing only the pool label
would not add the missing benchmark audit.

See [ADMISSION_CHECKLIST.md](ADMISSION_CHECKLIST.md) for the exact fail-closed
sequence and [read-only-receipt.md](read-only-receipt.md) for the preserved
observations.

