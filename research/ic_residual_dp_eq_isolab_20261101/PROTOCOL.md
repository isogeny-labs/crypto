# Isolated (isolab-eks, L2) replay of the priced DP/equivariant residual study: frozen before the run

Date: 2026-11-01. Replays `research/ic_residual_dp_eq_20261101` (frozen spec `specs/E-p36.json`, run locally at L0) on Iso Lab / EKS Spot with native L2 admission.

## Design
`spec-p36.json` is the study's L2 spec for p36 (curve, 11 arms: rho.negation reference; baseline res1/res2/res4; candidates res{1,2,4}dp and res{1,2,4}eq; A/A copy res2aa; 16 targets; target seed 20261601;
measurement seed 20261602; alternate_measured) with one change: `measurement.rounds` 3 -> 5, because the lab's admission requires at least five measured rounds, a warmup, a reference and a matching A/A control.
The code is the main-branch commit named in the dispatch (it contains `residual-fused-dp` and `residual-fused-eq`, merged in PR #3119); the image is built from that exact commit by the `Iso Lab experiment` workflow.

## Claims
- R1 (replay): counted GAE of every record is a deterministic function of the algorithm seed and instance, so records of rounds 1-3 reproduce the local L0 session's counts exactly (same target seed, measurement seed and
  arm parameters); rounds 4-5 are new draws. If rounds 1-3 differ, the study's counts are not host- or isolation-independent and that is reported.
- R2 (isolation): the lab's native L2 gate, replay audit and A/A admission pass for the session. Wall-clock is reported only as the lab records it; no wall-clock claim is made about the algorithms here.
- R3: the paired candidate/baseline S ratios at w = 0, 0.1, 1 and the distance to rho are recomputed on the 5-round data with the harness's `compare` and `band` and set beside the L0 numbers; they are not pooled.
Class: infrastructure/replay evidence; not an algorithm speedup claim.

## Decision rule
Report the lab's result.json state verbatim. A failed admission (L2, replay or A/A) is reported as failed with its receipt; nothing is retried or edited to pass. One dispatch only (a duplicate or rerun is a new experiment).

## Amendment 1 (2026-11-01, no experiment had run)
The first valid dispatch (workflow run 41337767031, source commit 428c146add196cdc925c22033f4714353c77feab) failed in the `build` job; the `experiment` job was skipped, so nothing reached the lab.
Cause (log of job 124941431851): `src/ml_dataset/telemetry/whole_worker.rs:258` calls `libc::ioctl(fd, PERF_EVENT_IOC_ID, ...)` with `const PERF_EVENT_IOC_ID: libc::c_ulong`, but the lab image builds for musl where
`libc::Ioctl` is `i32`: E0308 "expected i32, found u64". That file belongs to the ml_dataset telemetry work already on main; main fails the same way, which is also why other recent dispatches failed to build.
Fix applied on this branch only (type of the constant, same bit pattern; checked with rustc on x86_64-unknown-linux-musl and -gnu, and the old form fails on musl): `const PERF_EVENT_IOC_ID: libc::Ioctl = 0x8008_2407_u32 as libc::Ioctl;`.
It touches no code path of the study. The tested commit becomes main@aff5fa4975 + this protocol/spec + that one line. One redispatch with the new commit (no measurement had taken place, so this is not a rerun of a failed measurement). A separate PR carries the same one-line fix to main.

## Amendment 2 (2026-11-01, no Job had been created)
The second dispatch (workflow run 41339933622, source commit 717de7f1cb34599d8325db8531afb377bea77d8d) built and published the image (digest sha256:29d03c52ed3708caa625e8587f16e09ae5428c66e2144247d4421519cbf6c9a6, ecbench sha256 5bca91b1fa0b0ab27625f9b692b12eeb7d3dd0b9513255f8a6973230b92150d5)
but the launcher refused the request before creating a Job (`submission.stderr`: "benchmark requires an L2 spec, >=5 rounds, >=1 warmup, reference and matching A/A control"; `status-after-error.json`: jobs.batch "gh-41339933622-1" not found; receipt
s3://isolab-results-104186811093-us-east-2/runs/gh-41339933622-1/launcher/994c480b-2810-4f1b-b2bf-641cede3c969/, result state `submission_unknown`, no Job/Pod UID).
Cause: the lab's admission rule (`taskq/deploy/eks-karpenter/mcp/src/main.rs`, `benchmark_args`) requires the A/A copy to have `role: "control"` with the same method as another non-control arm; my `res2aa` had `role: "baseline"`, the local harness convention.
Fix: `res2aa` role `baseline` -> `control` (the only change; method, parameters, seeds and every other arm identical; the native planner accepts it, and the lab's rule evaluates true on the new spec and false on the old). Counted results do not depend on the role label.
No measurement had been taken, so the redispatch is not a rerun of a failed measurement. This is the third dispatch of this experiment (first: mistyped SHA, cancelled before any build; second: build fixed by Amendment 1 but rejected by admission). No further dispatch if this one fails after a Job exists.
