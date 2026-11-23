# K600 m6 native precharge child intervals — 2026-11-15

The earlier [arm budget amendment](ARM_BUDGET_CONTROLLER_AMENDMENT.md)
identified externally declared build, stage, freeze and preflight wall times
as an accounting gap. The native `n83_k600_m6_budget` example now supplies a
bounded `charge` command for each of these four child processes and an
`assemble` command for the four-phase precharge manifest:

```text
charge build|stage|freeze|preflight NEW_CHILD_RECEIPT NEW_OUTPUT_DIR OUTER_SECONDS PROGRAM [ARG...]
assemble BUILD_OUTER STAGE_OUTER FREEZE_OUTER PREFLIGHT_OUTER ARM_LOCK PREFLIGHT_RUN NEW_PRECHARGE_JSON
run PLAN_JSON NEW_OUTPUT_DIR
```

`charge` requires a new absolute output and child receipt path outside the
source checkout. It records clean source HEAD, program path and SHA-256,
arguments, child exit, stdout/stderr hashes, the child receipt SHA-256, a
monotonic outer interval rounded up to milliseconds, and a terminal status.
It enforces an outer seconds cap and rechecks the source and program after
the child. For preflight it rejects a pre-existing fixed-name container and
inspects/kills any remaining container with bounded Docker commands.

`assemble` derives rather than accepts the four interval values and child
hashes. The v2 precharge verifier binds each outer receipt to the current
controller binary, exact phase, source revision (`C` for build and `H` for
later phases), allowance, child exit 0, cleanup, wall and child hash. It
also checks the phase child schemas and statuses, the arm lock's staging
and preflight hashes, and the source-matched build receipt's image and
binary digests. A changed outer receipt is rejected. The arm runner rejects
the previous v1 externally declared precharge schema.

**Validation:** Rust 1.99.1 macOS arm64 release example tests passed 5/5,
including a native child that creates its receipt, a missing-child failure,
the four-phase v2 manifest and a changed outer receipt. The production
release example built; `git diff --check` passed. No Docker child or K600
input ran in these tests. The root release library gate previously exited
101 with 6,306 passed, 59 failed and 138 ignored on unchanged library
source; this change touches only the example and study documentation.

**Remaining accounting gate:** A native source-matched Linux build producer
must emit the required `n83.k600-m6-linux-build/v1` child receipt. The
preflight lock commit, source-matched image, real host preflight and arm
sequence remain unexecuted. The `charge` interval covers its child through
child receipt creation and cleanup, but its own outer receipt write and
process exit occur afterward. The assembled sum therefore remains a lower
bound until a full-invocation outer receipt charges those intervals and
the arm controller's final write/exit. No total cold IC runtime or factor-
base winner is established. No graph or leaderboard row changes because
this amendment contains no K600 performance measurement.
