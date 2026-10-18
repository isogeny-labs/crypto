# Frozen Tier 1 worker panel after Pod thread pinning

The third 128-walker panel stopped after 47 arms when a live affinity audit
found RunPod web-terminal threads eligible to run on measurement CPU `5` or
its SMT sibling `29`. Its complete partial archive and `rejected` summary are
retained in the [attempt 3 record](../tier1-pod-20261018-r3/README.md).
Process-leader pinning was insufficient for the multithreaded web terminal.

Before this fourth attempt, every visible Pod thread was checked. Sixteen
web-terminal threads were moved to housekeeping CPUs `2,26`, with zero
permission failures and zero remaining non-benchmark threads eligible on
`5,29`; see the [thread repair receipt](attempt4-thread-affinity-repair.json).
The [affinity observer](watch_pod_affinity.py) is a separate process pinned
to CPU `2`. It polls every 250 ms and sends `SIGINT` to the runner if any
non-benchmark Pod thread becomes eligible on `5` or `29`. A complete
`passed` observer receipt is an additional required host gate; its absence
or any violation leaves aggregate speedup unknown. This monitors visible Pod
threads, not other tenants or physical-host IRQ placement, so the tier
remains Tier 1.

The [plan](attempt4-plan.json) was frozen before dispatch at SHA-256
`b07762f13fb34c22f11ea7afd790fadc90de300bb002cf5799a50f54b0928c4c`.
The [allocation receipt](attempt4-allocation.json) was frozen at SHA-256
`8899d1a721f4e9e11cf641a70f0f4d8f5e21c7cf097f162c318da7dd565c16de`.
The observer and thread-repair receipts are SHA-256
`c1904e9dd10d4f87c1c10bff6bfb9b688fbd4140616e458a8eeaee6ef973fe8a`
and `d091df29ece6c3544597067e5a0bec3193d218a9f912300ab2ffa328f8d34b2c`.

The measured workload remains 128 walkers per arm on the same 32 public Q
points, with four alternating repetitions, 384 serial/batch and A/A pairs,
and 768 arms. Source bundle SHA-256
`ccc4c83128510e145bed6e8c04b26eb9a63f617bb2d3665826856ccd91902904`
and executable SHA-256
`1bbbef507589180facc928ef6689bc474adadce5b3389a45d5116828479ef2a3`
remain unchanged. The independent audit still requires exact output, valid
host receipts, every A/A ratio in `[0.90, 1.11]`, both RSS limits, and a
Q-cluster bootstrap 95% lower charged-time ratio above 1.05 for default
worker routing. Every partial, failed, noisy, and valid arm is retained.
