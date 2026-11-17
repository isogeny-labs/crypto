# Actual-field Singular workload has no qualified isolated runner here

- **Status:** open
- **Kind:** access/tool limit; workload integration gap; unknown budget remainder
- **Affected work:** [torus study](../research/curve25519_torus_native_20261115/README.md), [EKS integration](eks-resumable-l2-experiments.md), [solver obstruction](curve25519-torus-four-point-decomposition.md)
- **Next action owner:** Curve25519 torus campaign / Iso Lab operator
- **Last updated:** 2026-11-15

## Problem

The actual-field native Singular experiment cannot be dispatched through
the available tools with the required isolated runtime and live budget
readback. This leaves the requested full-base solve unattempted.

## Evidence and uncertainty

- **Observed:** local Rust and native Singular functional controls are archived by the crypto and cryptanalysis studies. They are development checks. [pcc:unverified]
- **Observed access limit:** no EKS MCP, AWS client or kubectl is exposed in this workspace. The inspected GitHub EKS workflow accepts ecbench workloads, not this custom Singular command. Research-KB and Conductor executables are also unavailable; documented repository/primary-source fallbacks are recorded in the study.
- **Derived or proved:** no mathematical obstruction follows from missing access.
- **Suspected explanation:** integration and runtime availability, not a measured solver limit.
- **Missing capability:** supported pinned Singular image and upload/execute/artifact path, live identity/capacity qualification and current daily/monthly spend remainder. Historical allowances are not current headroom.
- **Not established:** a full-base timeout, memory failure, inability to decompose or an attack-cost estimate. No paid job or substitute fixture was launched.

## What would unblock it

- [ ] Supply an authorized isolated runner accepting this committed native workload, with immutable binary/image, input hashes and exact command.
- [ ] Record live identity, capacity and spend remainder through the maintained execution path; fit the existing allowance without inventing a new campaign cap.
- [ ] Qualify the native workload with functional controls and independent artifact retrieval before using a full-base timing.

These conditions permit the experiment; they do not predict its outcome or
close the solver, distribution or scaling obligations.

## Next discriminating step

- **Action:** add or attach the maintained native-workload EKS adapter and perform a read-only identity/capacity/budget preflight.
- **Inputs and controls:** committed actual-field scripts, qualifier hashes, M=1 satisfiable and nonmember controls, bounded root recovery; native error/truncation controls.
- **Limits:** no experiment launch while live headroom is unknown. Use explicit per-job limits and existing isolation/Spot rules when available.
- **Outcomes:** a successful preflight permits a separately registered full-base run; failed controls invalidate qualification; absent credentials/image/headroom remain infrastructure blockers, never mathematical negatives.
- **Recheck trigger:** qualified adapter, live runner access or updated operator evidence.

## Resolution and history

Unresolved. 2026-11-15: repeated missing-runtime checks consolidated into
this record so they are not mistaken for repeated failed solver experiments.
