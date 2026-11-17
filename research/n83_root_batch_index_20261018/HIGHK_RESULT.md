# K2816 batched-root index result

Batching the 83 relative S3 roots of each representative pair through a
shared inversion reduced the K2816 N83 index build from a 359.302-second
scalar mean to a 234.476-second batched mean in the frozen four-arm cloud
panel: a 1.532× ratio of means and 124.826 seconds saved per index. All
329,090,432 compact states and 658,165,181 indexed roots matched across the
arms, including full BLAKE3 digests of state storage and table slots.

| Arm, execution order | Index build (ms) | Setup total (ms) | Peak RSS (KiB) | Exit |
| --- | ---: | ---: | ---: | ---: |
| Scalar 1 | 377,382.834 | 378,166.185 | 9,379,008 | 0 |
| Batch 1 | 234,755.920 | 235,549.029 | 9,379,024 | 0 |
| Batch 2 | 234,197.065 | 234,978.337 | 9,378,880 | 0 |
| Scalar 2 | 341,221.780 | 342,006.543 | 9,378,980 | 0 |

The paired scalar/batch index ratios were 1.608 and 1.457. The base was the
exact `construct:83:1:2816` x-scan with 467,456 usable points, 2,816 orbit
columns, and base hash
`9208460bf63be634f0aee6091abaaed855e05004fc2894e28a9c1fd394c35ec4`.
Every arm produced state digest
`9ede6e8fd34a44327295f8edcfd5d9a69d894c8ce44270170f2284c0c8f4386f`
and table digest
`dca3450096b79bfa6a9b64facd265740d5905e3564a7c9df4329b866caf046b5`.

The [cloud run](https://github.com/aburan28/crypto/actions/runs/39094570462)
used source commit `64e46a90a2b6088344bdd1e622667ab21035bd29` and source SHA-256
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`.
The [raw artifact](cloud_k2816_stage_39094570462/) contains the four stdout
rows, process exit codes, `/usr/bin/time -v` resource receipts, host topology,
affinity, source and binary hashes, and the analyzer result. The
[SHA-256 manifest](cloud_k2816_stage_39094570462/manifest.sha256) pins every
retained file. The local native analyzer reproduced `analysis.json` byte for
byte as `local_replay.json`; both have SHA-256
`78abe95bc3238706e1e13b78cb697b227144e94f2827e4f14b88e51c239a5765`.

This panel measured target-independent index construction and keeps
`candidate_id: null`. It ran on a four-vCPU GitHub hosted VM with visible
affinity `0-3` and process cgroup member
`/system.slice/hosted-compute-agent.service`. The stage runner sampled cgroup
files at the mount root; this artifact therefore lacks per-arm cgroup
throttling and OOM deltas from the process member and remains a cloud stage
diagnostic under the benchmark tier contract. The primary one-target online
selector remains open. The next complete experiment pairs the batched-index
candidate with same-point rho and independently checks the full resource,
calibration, relation, and scalar receipts; the separate K1107/K2304 bridge
tests the unresolved smaller-base comparison.

The [first K2816 attempt](first_highk_attempt_39093280175.json) stopped before
its first arm because the shell source lacked executable permission in the
workflow. Its failed receipt remains separate from this completed panel.
