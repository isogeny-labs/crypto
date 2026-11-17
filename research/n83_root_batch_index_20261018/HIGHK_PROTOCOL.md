# K2816 cloud index-stage comparison

The K512 sparse-selector pilot measured a paired batched-root index gain and
byte-identical root tables. This prospective larger-base panel tests the same
implementation at K2816, the cold-process leader among the four archived
crossfamily arms. It remains an index-stage diagnostic with
`candidate_id: null`.

Use the exact K2816 x-scan factor base `construct:83:1:2816`. Its candidate
manifest records 467,456 usable points, 2,816 folded columns, and expected
base hash BLAKE3
`9208460bf63be634f0aee6091abaaed855e05004fc2894e28a9c1fd394c35ec4`.
Freeze N83 source SHA-256
`4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`,
[runner](run_highk_stage.sh) SHA-256
`5e97f632a8537064af9d636bf1286a77f9c2678131e901f572a2f247a9da530b`,
[native analyzer](../../examples/n83_root_batch_highk_analyze.rs) SHA-256
`961ddb7448ca57f05a341385cbde5bc53179eba9149974fe0e1eaa0d41599cb9`,
and workflow SHA-256
`71e43db235fbb9392da1b7535948d53d8f75b8313afdcc0bc948cf77494fa061`.

The first [cloud attempt](https://github.com/aburan28/crypto/actions/runs/39093280175)
built and tested the frozen producer, then stopped before the first arm because
the shell runner file lacked executable permission. Its preserved artifact
contains the compiler versions; [the attempt receipt](first_highk_attempt_39093280175.json)
records the failure. The revised workflow invokes that same hashed shell
source with `bash`. No index timings came from the first attempt.

Run one frozen `ubuntu-24.04` cloud allocation in scalar, batch, batch,
scalar order. Both builders use `KIC_SYMMETRIC_S3=1`,
`KIC_SPARSE_CANONICAL=1`, `KIC_PACKED_ROOT_TABLE=1`,
`KIC_ROOT_ONLY11=1`, `KIC_INDEXED_ROOT5=1`, and
`KIC_INDEXED5_LOAD75=1`; only `KIC_INDEX_ROOT_BATCH` changes. Each process
starts with empty caches, constructs the base, builds the index, emits its
full compact-state and root-table digests, and exits before relation rank.
Cap each process at 900 seconds and the job at 75 minutes. Retain host
topology, affinity, cgroup CPU and memory files before and after every arm,
process resource logs, source and binary hashes, all exits, and raw artifacts
even when an arm fails or times out.

The native analyzer requires all four successful exits, the expected base
hash and usable-point count, identical state and table counts, and matching
full BLAKE3 digests. It reports both paired scalar/batch index and setup
ratios. Any missing or failed arm leaves the aggregate unavailable. This
stage panel does not use same-point IC/rho measurements; a complete candidate
promotion requires those fresh paired solves and the Tier 1 calibration and
resource gate.
