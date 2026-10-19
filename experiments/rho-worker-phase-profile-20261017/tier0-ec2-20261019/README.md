# Physical-host worker panel

This panel uses a dedicated EC2 `m5zn.metal` host (`i-047f22d5f7ea716c2`,
`us-east-2a`) to test the full rho worker with physical core ownership.
The host is separate from the user's active research instance. Its scheduled
shutdown is 2026-10-19 04:30 UTC; the instance is configured to terminate
on shutdown.

The frozen source archive SHA-256 is
`f32992105d47d63c78a7ba5bb02021ba6ba150e4c388cec55715b979d697f722`.
It was built from PR #2137 with optional `CHECKIN_EVERY=64`, so the requested
batch width 32 is reachable. The 32 public Q fixtures retain SHA-256
`f46ad43788b7f9241927904541cbeb03ea70a96ca95aa84d527a9b22ba8d12b8`.

The [wrapper](worker_wrapper.py) checks every worker's exact DP replay and
charged phase sum, and exposes the raw worker JSON to the strict host runner.
The [manifest builder](build_manifest.py) derives 96 cases from the frozen
same-Q plan: one serial/batch comparison, one serial/serial control, and one
batch/batch control per Q. Four repetitions alternate arm order, for 768
arms and 384 pairs. The [independent auditor](analyze.py) checks raw output,
artifact hashes, paired fields, A/A `[0.90, 1.11]`, peak RSS, host preflight,
and a 32-Q cluster bootstrap. The improvement threshold is a 95% lower
bound greater than 1.05 with every arm and pair valid.

The benchmark CPU is physical core 23 (logical CPUs 23,47) on NUMA node 1.
Boot isolation, the cgroup v2 isolated partition, housekeeping affinity,
IRQ routing, fixed frequency, memory binding, and per-arm noise are checked
by the repository's `scripts/isolated_bench.py`. A failed preflight or arm
remains in the raw ledger with the aggregate decision unknown.
