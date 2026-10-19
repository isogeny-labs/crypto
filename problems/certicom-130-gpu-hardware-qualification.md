# Physical CUDA and Metal qualification for the two Certicom 130 aliases

- **Status:** `open`
- **Kind:** hardware access limit; device correctness unverified
- **Affected work:** [crypto #2250](https://github.com/aburan28/crypto/pull/2250), [cryptanalysis #968](https://github.com/aburan28/cryptanalysis/pull/968), and [the shared native workers](../gpu/certicom/README.md)
- **Next action owner:** the PR author once existing authorized GPU access is attached
- **Last updated:** 2026-10-19

## Problem

Physical execution of rho, BSGS and index calculus on both NVIDIA CUDA and
Apple Metal cannot yet be qualified because neither device nor an authorized
remote GPU connection is attached to this task.

The exact instances are the published ECC2-131 and ECCp-131 fields, models,
generators, subgroup orders, cofactors and targets, exposed under the requested
ECC2-130/ECCp-130 aliases in both repositories. ECC2K-130 remains distinct.
CPU support, native GPU code and compilation checks do not discharge the
requested physical-device validation. The separate [nine-binary-curve rho
campaign problem](certicom-binary-gpu-rho-integration.md) has broader curve,
durable-store and performance requirements; it is not resolved by this work.

## Evidence and uncertainty

- **Observed:** at published heads `8ea1c141c6f66e41e78abb50b85de7ff8f100caf`
  and `6552da5ed5f3715bfe5c3b9c8a9fa23f1e1356fe`, the full CPU library suites
  passed 4,420 and 3,150 tests with loopback enabled. The original restricted
  runs' nine/eight localhost failures and the passing reruns remain disclosed
  in the PRs. These are earlier-head results, not qualification of later fixes.
- **Observed:** complete CUDA workers compiled locally with nvcc 13.3.73 for
  sm_80, sm_90 and sm_120. A native device probe exited 3 with an insufficient
  driver error. Explicit CUDA CLI selection returned an error, no scalar and
  no CPU fallback. This workspace has no NVIDIA device or Metal runtime.
- **Observed:** macOS 14 arm64 compiled the Objective-C++ worker, Metal 3.0
  shader and metallib in [crypto CI](https://github.com/aburan28/crypto/actions/runs/39197392278/job/117882993610)
  and [cryptanalysis CI](https://github.com/aburan28/cryptanalysis/actions/runs/39197404995/job/117883031024).
  Those jobs compile only. The optional hardware job was legitimately skipped.
- **Observed access limit:** GitHub self-hosted runner enumeration returned
  HTTP 403. The task runtime reports no GPU capability, secret binding or
  outbound identity. The suite's `cloud/fleet.json` describes `rp-gpu-1`,
  but a configuration name does not establish a live authorized connection.
- **Derived or proved:** no mathematical obstruction is established.
- **Suspected explanation:** no device correctness failure has been observed;
  actual device behavior remains unknown until execution is possible.
- **What this evidence does not establish:** emulation and compilation do not
  establish physical GPU use, throughput, natural IC relation yield, or a
  solution to either public challenge. Constructed rho `Q=-G` and prepared
  full-rank IC inputs are correctness controls.

## What would unblock it

| Requested component in both repositories | Current evidence |
| --- | --- |
| CPU rho, BSGS and IC on both exact curves | Verified constructed correctness controls and bounded public-Q runs; full library and release CLI checks pass on this Linux host. |
| NVIDIA CUDA rho, BSGS and IC on both exact curves | Implemented; native compilation and device-source host emulation pass. Physical execution remains unverified. |
| Apple Metal rho, BSGS and IC on both exact curves | Implemented; native macOS compilation passed at the earlier heads. Hosted device probe/qualification is pending; physical execution remains unverified. |

- [ ] Attach existing authorized NVIDIA and Metal-capable Mac connections,
  or usable GitHub runner labels, with the native compiler/runtime available.
- [ ] On both backends and in both repositories, run the checked-in device
  tests and release CLI tests on both exact curves and all three methods.
  Retain source/binary hashes, OS/architecture, compiler/runtime and device
  identity, explicit backend reports, launch counts and complete pass/failure
  logs. Every returned scalar and IC relation/log must pass independent CPU
  replay. Include bounded incomplete runs on both published targets.
- [ ] Resolve any device defects without fallback or curve substitution,
  preserve shared-source parity, and satisfy current-head CI/review gates.

Passing these checks establishes device correctness for the tested workloads
and hardware. It does not establish a speedup or practical public-target solve.

## Next discriminating step

- **Action:** build/probe the selected native worker and run the hardware
  job in `.github/workflows/certicom-devices.yml` on an existing authorized
  runner. It executes `bash gpu/certicom/qualify.sh`, including device and
  release CLI tests. The hosted macOS job also attempts a native Metal probe
  and runs this qualification when its runner exposes a device; that attempt
  is pending and is not a completed physical run.
- **Inputs and controls:** the exact published instances; full-width random
  arithmetic against independent Rust; constructed known answers; public Q
  with small search budgets; worker identity, malformed-input and timeout
  controls. `gpu-emulator` stays explicitly labelled host execution.
- **Execution limits:** checked-in per-walk/query/table/matrix budgets and
  whole-session deadlines; serial hardware tests; a 45-minute workflow limit.
  Stop on any mismatch, worker error or deadline. No benchmark campaign,
  paid provisioning or public-challenge solve is authorized by this record.
- **Outcomes:** success supplies the missing physical receipts; a mismatch
  requires a bounded repair and repeat validation; missing hardware or a
  launch error remains infrastructure evidence. Bounded no-solve results do
  not prove impossibility.
- **Recheck trigger:** an attached existing GPU connection or runner label,
  a physical execution receipt, or new current-head CI/review feedback.

## Resolution and history

- 2026-10-19: recorded the cross-repository physical qualification gap and
  successful compilation controls. Added the CLI checks to the hardware job;
  scheduling that job is not a completed hardware experiment.
- 2026-10-19: reproduced the session-deadline defect with both a native-source
  worker and the actual release CLI: expired sessions could return verified
  scalar zero. The monotonic-deadline repair passes both regressions, all 14
  focused solver/device tests per repository, and the complete release library
  suites (4,421/0/118 existing ignored; 3,151/0/92 existing ignored). Seven CLI
  cases pass through each of `icx`, `ca-icx` and `crax icx` on CPU and explicit
  host emulation: 42 test executions. These remain host correctness evidence.

Unresolved. Merging these PRs or completing this record does not establish
physical-device correctness.
