# Tier 1 full-worker rho timing awaits an authorized CPU Pod

- **Status:** `open`
- **Kind:** access/tool limit and empirical measurement gap
- **Affected work:** [frozen collaborative rho worker panel](../experiments/rho-worker-phase-profile-20261017/README.md), [worker PR #2137](https://github.com/aburan28/crypto/pull/2137), and [joint-scalar arithmetic closeout PR #920](https://github.com/aburan28/cryptanalysis/pull/920)
- **Next action owner:** workspace owner for a rotated `RUNPOD_API_KEY` secret and approved source transfer; rho worker task for dispatch and audit
- **Last updated:** 2026-10-18

## Problem

The frozen 32-Q secp256k1 serial/batch collaborative rho panel cannot establish qualified full-worker throughput until its public source and inputs run on a pinned Tier 1 Linux allocation. The workspace has no `RUNPOD_API_KEY` environment secret. Automatic approval review rejected an earlier transfer of the separate Rust source snapshot; its exact rejection reason was not retained. The chat-pasted key is not a workspace secret.

The planned comparison has 32 public Q points, 32 walkers per arm, `dp_bits=12`, an 8,192-step cap, four repetitions, and comparison plus serial/serial and batch/batch controls for every Q: 384 pairs and 768 arms. The integration gate requires exact ordered check-ins and verified distinguished-point records, no failed host-noise or A/A gates, added peak RSS within both 10% and 64 MiB, and a Q-cluster bootstrap 95% lower serial/batch charged-time bound above 1.05.

## Evidence and uncertainty

- **Observed:** The long local replay completed 64/64 arms and 32/32 exact-output pairs on the previous combined source. After rebasing onto repaired main at `abee07ee7337d04a7e734333669ca1bacd6c763a`, the release benchmark built and the [rebased 32-Q replay](../experiments/rho-worker-phase-profile-20261017/local-all32-rebased-main-worker-replay/summary.json) again completed 64/64 arms and 32/32 exact-output pairs. The current source archive was rebuilt identically twice, SHA-256 `abd7a2eaeebca050dcf0a9f2290c02b116dbdd4b0bfb43cb45bf4be1ca5f7025`, then built from a clean extraction with a fresh Cargo target; its q00 serial/batch outputs matched. The complete release library suite passed 4,396 tests with zero failures and 118 ignored. The [rebased-main validation receipt](../experiments/rho-worker-phase-profile-20261017/rebased-main-validation.json) binds those checks. The [auditor policy self-check](../experiments/rho-worker-phase-profile-20261017/worker-auditor-policy-selfcheck.json) exercises one synthetic pass and fail-closed A/A and missing-output cases.
- **Derived or proved:** Local exact replay establishes output equivalence for its frozen traces. It gives no qualified wall-time bound under the observed host contention.
- **Suspected explanation:** None is needed for the access gap. The earlier automatic approval rejection applied to source transfer; its detailed reason is unknown.
- **Missing capability:** A rotated workspace secret and an approved transfer path for the exact public source bundle, followed by a suitable CPU Pod allocation.
- **Measurement limit:** The qualified full-worker speedup and memory change remain unknown until the hosted panel passes its gates. A failed or noisy run remains a retained failed pair, not a performance decision.

## What would unblock it

- [ ] Make a rotated `RUNPOD_API_KEY` available as a workspace secret and approve transfer of the exact public source bundle whose SHA-256 is frozen immediately before dispatch. Do not use chat-pasted credentials.
- [ ] Build the release benchmark from that bundle on the Pod and record allocation, binary, fixture and source hashes, CPU/NUMA/cgroup envelope, and host counters.
- [ ] Execute all 768 arms serially under the host lock and independently audit every raw arm, pair, A/A control, phase sum, point record, memory observation and noise gate.

These conditions permit a qualified decision. A passing lower bound above 1.05 and acceptable memory permit default routing; a qualified result below the gate retains the serial default and closes the worker arithmetic decision. A noisy or incomplete panel leaves the result unknown and requires a repeat under the same frozen plan.

## Next discriminating step

- **Action:** Dispatch the [frozen Tier 1 panel](../experiments/rho-worker-phase-profile-20261017/README.md) after the source and allocation are pinned.
- **Inputs and controls:** Same 32 public Q points for both paths; alternating serial/batch arms with serial/serial and batch/batch controls; exact ordered check-in and distinguished-point replay.
- **Execution limits and stop condition:** 768 planned arms, one child at a time; 180-second per-arm timeout; retain every failed or interrupted arm and stop only when the plan completes or an infrastructure failure is recorded.
- **Outcomes:** A qualified >1.05 lower bound advances the batch default; a qualified lower result keeps the serial default; failed host/A/A/structural gates keep aggregate speedup unknown. Infrastructure failure is recorded separately.
- **Recheck trigger:** Rotated secret plus approved source transfer, or a qualified alternative Linux allocation with an equivalent auditable host record.

## Resolution and history

Unresolved. The source, correctness replay, and audit controls are committed in PR #2137. Recheck the source hash after any branch change before dispatch.
