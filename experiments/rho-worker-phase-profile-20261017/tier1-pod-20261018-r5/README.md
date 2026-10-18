# Frozen quiet-window Tier 1 worker panel

Attempt 4 stopped after its first eight arms because a RunPod `runc` task
with parent `0` could run on CPU `5` or SMT sibling `29` while the Pod
affinity observer was starting. Its complete partial archive and observer
violation are retained in the [attempt 4 record](../tier1-pod-20261018-r4/README.md).

This fifth attempt changes only the **start boundary**. The
[barrier wrapper](start_barrier.py) writes its own PID, waits for the
[observer](watch_pod_affinity.py) to see five consecutive seconds with no
visible non-benchmark Pod thread eligible on `5,29`, then releases the runner
under the same PID. Startup affinity events before release are recorded
separately. From release through the final arm, the observer polls every
250 ms; any observed non-benchmark thread on `5,29` sends `SIGINT` to the
runner and leaves the aggregate result unknown. A missing or non-`passed`
observer receipt also fails the host gate. This remains Tier 1 because Pod
visibility does not prove physical-host CPU, frequency, IRQ, or NUMA
exclusivity.

The [plan](attempt5-plan.json) was frozen before dispatch at SHA-256
`203841b789fa5a0ed09ddd51c44d403e7a20716ba26cbac8b268b80e6e81ae40`.
The [allocation receipt](attempt5-allocation.json) was frozen at SHA-256
`aeb2969f77ed052bfdda8517cd938a5142b566adf70570d756ee92e9e9c389e1`.
The observer and barrier source SHA-256 values are
`f283e0545be5bbdf872079288598dcbe9322762e318f25ae4d8c7cd34731a7a1`
and `fc465ed8f4cb3a57a0383974c912e6d326e525be030a8187bdb2b900b9445ed5`.

The 128-walker, 32-public-Q, four-repeat workload and its 384 same-point
pairs remain unchanged from attempt 4. The Rust source bundle and Linux
executable remain SHA-256
`ccc4c83128510e145bed6e8c04b26eb9a63f617bb2d3665826856ccd91902904`
and `1bbbef507589180facc928ef6689bc474adadce5b3389a45d5116828479ef2a3`.
The independent auditor retains exact-output, host-counter, every-pair A/A
`[0.90, 1.11]`, both memory limits, and Q-cluster bootstrap 95% lower
charged-time ratio above 1.05 as the default-routing gates. Every stopped,
failed, noisy, and successful arm remains in the evidence.
