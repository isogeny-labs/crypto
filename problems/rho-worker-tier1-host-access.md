# Full-worker rho confirmation after the physical-host panel

- **Status:** `open`
- **Kind:** prospective empirical confirmation
- **Affected work:** [worker experiment and decision table](../experiments/rho-worker-phase-profile-20261017/README.md), [worker PR #2137](https://github.com/aburan28/crypto/pull/2137), and [arithmetic PR #920](https://github.com/aburan28/cryptanalysis/pull/920)
- **Next action owner:** AWS quota review for another dedicated host; rho worker task for fresh-target frozen replay and independent audit
- **Last updated:** 2026-10-19

## Result so far

The secp256k1 serial and opt-in batch worker paths replay the same frozen walk outputs. The rebased release library suite passed 4,396 tests with zero failures and 118 ignored, and the [32-Q local replay](../experiments/rho-worker-phase-profile-20261017/local-all32-rebased-main-worker-replay/summary.json) completed 64/64 arms and 32/32 exact-output pairs. The [source receipt](../experiments/rho-worker-phase-profile-20261017/shared-lock-source-validation.json) pins public bundle SHA-256 `ccc4c83128510e145bed6e8c04b26eb9a63f617bb2d3665826856ccd91902904` and verifies the fresh build, queue lock, and auditor controls.

Five hosted attempts ran on RunPod Pod `94uxi8f876o5gz`. The [decision table](../experiments/rho-worker-phase-profile-20261017/README.md#hosted-worker-decision) and attempt directories retain the frozen plans, raw or partial archives, noise observations, and independent audits. Every aggregate is `unknown_gate_failed`:

| Attempt | Completed arms / exact pairs | Gate observation |
| --- | ---: | --- |
| [1](../experiments/rho-worker-phase-profile-20261017/tier1-pod-20261018/README.md) | 768 / 384 | Three A/A ratios outside `[0.90, 1.11]` |
| [2](../experiments/rho-worker-phase-profile-20261017/tier1-pod-20261018-r2/README.md) | 768 / 384 | One A/A ratio outside range |
| [3](../experiments/rho-worker-phase-profile-20261017/tier1-pod-20261018-r3/README.md) | 47 / 23 | Web-terminal threads could run on the measurement core and its SMT sibling |
| [4](../experiments/rho-worker-phase-profile-20261017/tier1-pod-20261018-r4/README.md) | 8 / 4 | Host-parent `runc` was eligible on those CPUs before the observer armed |
| [5](../experiments/rho-worker-phase-profile-20261017/tier1-pod-20261018-r5/README.md) | 96 / 48 | Host-parent `runc` became eligible about 701 seconds after barrier release; observer stopped the run |

Attempt 5 had no A/A exception in its completed pairs. Its host-affinity observation still invalidates the aggregate. All five archives were copied off the Pod and committed to PR #2137. The Pod had no persistent volume, was idle after the fifth audit, and was stopped through the RunPod API; a subsequent API read reported `EXITED`.

The [physical-host r5 panel](../experiments/rho-worker-phase-profile-20261017/tier0-ec2-20261019/README.md)
then ran on a dedicated `m5zn.metal` core with NUMA, IRQ, frequency,
exclusive-cgroup, and per-arm noise receipts. Its
[independent audit](../experiments/rho-worker-phase-profile-20261017/tier0-ec2-20261019/audit-r5.json)
checked all 768 arms and 384 pairs: no invalid arms, invalid pairs,
structural issues, or memory failures. One of 256 frozen A/A controls
had ratio 1.123159, above the 1.11 limit, so the aggregate decision is
`unknown_gate_failed`. The comparison-only diagnostic median Q-cluster
ratio was 5.110, with diagnostic 95% interval `[5.041, 5.165]`.
These diagnostics focus the next experiment on confirming the batch walk
speedup; serial routing remains the default until a prospective gate passes.

## What would unblock it

- [x] Establish and audit physical-host CPU, SMT, NUMA, IRQ, frequency, and per-arm noise isolation; preserve the r5 raw archive and failed A/A control.
- [x] Generate 32 fresh public Q points with zero overlap and independently replay all 4,096 expected outputs. The [confirmation plan](../experiments/rho-worker-phase-profile-20261017/tier0-ec2-20261019/CONFIRMATION_PLAN.md) freezes a new aggregate A/A rule that applies only to a new panel.
- [ ] Wait for AWS quota and allocate a separate dedicated host; keep the existing research jobs running. Rebuild the pinned source there and commit the allocation, binary, fixture, plan, and source hashes before timing.
- [ ] Run the new 768-arm, 384-pair panel through one shared queue and independently audit exact replay, A/A, memory, host noise, and the 32-Q paired interval. A qualified 95% lower bound above 1.05 permits curve-guarded batch routing; otherwise retain serial routing and the complete failure ledger.

## Resolution and history

Open pending a fresh physical-host confirmation. The prior Pod access
and host-isolation gaps were addressed. The r5 A/A excursion is
preserved under its original gate. AWS's additional on-demand quota
request remains open at the last check; termination of the dedicated
r5 host was requested after its complete archive was copied and verified.
