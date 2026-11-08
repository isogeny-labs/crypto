# CM marked-scalar timing admission

- **Status:** open
- **Kind:** resource limit (local contention), access/tool limit (EKS role), and missing measurements
- **Affected work:** [PR 3674](https://github.com/isogeny-labs/crypto/pull/3674), T-1888, [native CM study](../research/cm-marked-scalar/README.md)
- **Next action owner:** T-1888 for arithmetic; authorized measurement host or cluster operator for backend admission
- **Last updated:** 2026-11-08

## Problem

The native relation evaluator has no admitted end-to-end timing comparison because the current host fails the frozen quietness gate and the migrated direct kubeconfig cannot assume its configured role.

The requested result is a verified improvement over matched regular public-scalar multiplication for the exact marked action [mu]P, with cold runtime/table/map/correction/closure/output costs included. P-192 is the primary research target; P-224/P-256/P-384/P-521 remain in scope. The current timing draft covers the initial P-384/P-521 panel; five-curve construction coverage does not silently extend that unfrozen timing design.

## Evidence and uncertainty

- **Observed:** local preflight used 0.74 and 0.75 other CPU seconds in a two-second interval, above the 0.20-second limit. No cryptographic timing child was launched. Receipts: [first](../research/cm-marked-scalar/qualification/native-v9/isolation-controls/smoke.json), [second](../research/cm-marked-scalar/qualification/admission-controls/smoke-idle.json).
- **Observed:** the sleep-only native restoration and SIGTERM software controls passed, always admitted=false. They provide software checks, not measurement admission.
- **Observed:** read-only EKS reconciliation found nine failed jobs and one suspended job. No new submission or restart was requested. [Live-read errors](../research/cm-marked-scalar/qualification/environment/README.md) show STS AssumeRole denied for the configured remote-launcher role. The context name spot-job-submitter does not establish that it uses the authorized job-submitter role.
- **Derived:** the local receipts fail the unchanged timing gate. The EKS overview's compiled-in pool numbers cannot establish live CPU/RAM enforcement or remaining funds.
- **Missing capability:** an authorized quiet backend, source/binary-bound frozen timing record, and (if cloud is used) actual live caps and known remaining shared funds.
- **Scope:** these facts do not show that every host will fail admission, that a measured gain is impossible, or that the new relation kernels/maps lack correctness. They are not timing results.

## What would unblock it

- [ ] An authorized host passes exclusive SMT/core reservation, contention and PSI gates with A/A controls under the unchanged accounting boundary.
- [ ] Before cloud builds/jobs, verify the existing job-submitter role, authoritative live NodePool/quota limits, and known remaining daily/monthly funds; publish exact source/protocol/config/inputs/image/launch with finite limits. Do not self-grant privileges or raise caps.
- [ ] Freeze and publish source, binary, inputs, selected calibration/held-out procedure and analysis before affected timings, then collect complete correct rows, independently replay outputs and apply the frozen A/A decision.

Backend admission is necessary; it does not by itself establish a runtime improvement. A loss remains a recorded loss and directs the next implementation.

## Next discriminating step

Continue the native five-curve exact construction and counted schedules while the measurement host/configuration is resolved. No additional local preflight is scheduled on unchanged conditions; no daemon, monitoring process or agent is suspended to force quietness. No EKS build or job will be submitted while funds/caps/role are unknown. Recheck after an actual backend/configuration change, with the existing two-minute bounded attempt and zero unchanged retries retained.

## Resolution and history

2026-11-08: Unresolved. Native correctness now covers all five curves, with 150 point/window cases and explicit orbit transporters. Native timing remains unadmitted. The original speed objective remains open.
