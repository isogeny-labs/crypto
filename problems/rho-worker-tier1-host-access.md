# Qualified full-worker rho timing needs host-level CPU control

- **Status:** `open`
- **Kind:** empirical measurement and host-isolation gap
- **Affected work:** [worker experiment and decision table](../experiments/rho-worker-phase-profile-20261017/README.md), [worker PR #2137](https://github.com/aburan28/crypto/pull/2137), and [arithmetic PR #920](https://github.com/aburan28/cryptanalysis/pull/920)
- **Next action owner:** workspace owner for an administratively controlled Linux host and SSH access; rho worker task for frozen replay and independent audit
- **Last updated:** 2026-10-18

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

Attempt 5 had no A/A exception in its completed pairs. Its host-affinity observation still invalidates the aggregate. The charged worker-time ratio, interval, and memory acceptance result remain unknown. Serial routing remains the default. All five archives were copied off the Pod and committed to PR #2137. The Pod had no persistent volume, was idle after the fifth audit, and was stopped through the RunPod API; a subsequent API read reported `EXITED`.

## What would unblock it

- [ ] Provide SSH access to a Linux host with administrative control of benchmark CPUs, their SMT siblings, NUMA memory placement, IRQ routing, frequency policy, and background services. Record the actual host-level isolation configuration and its verification, rather than inferring exclusivity from a container affinity mask.
- [ ] Rebuild the pinned source on that host and freeze allocation, binary, fixture, and source hashes before dispatch. Confirm exact replay and the queue lock there.
- [ ] Run the frozen 128-walker panel with 32 public Q points, `dp_bits=12`, an 8,192-step cap, four repetitions, serial/batch and A/A pairs, one child at a time. Retain every failed or interrupted arm and independently audit ordered check-ins, distinguished points, phase sums, A/A, memory, throttling, steal time, and host noise.
- [ ] Apply the declared integration gate: a Q-cluster bootstrap paired 95% lower serial/batch charged-time bound above 1.05, exact replay, and added peak RSS within both 10% and 64 MiB. A qualified lower result keeps serial routing and closes the worker decision; a failed isolation or A/A gate leaves the speed result unknown.

## Resolution and history

Unresolved. The previously recorded need for a RunPod key and source-transfer approval was satisfied when the Pod was launched and the public bundle was uploaded; it is no longer the access blocker. The current blocker is host-level interference on the measurement CPUs. CI for PR #2137 was still queued at the last check; PR #920's reported checks passed. Recheck the source hash and PR status before dispatch or merge.
