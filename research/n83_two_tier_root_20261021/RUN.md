# N83 split root-index controls and K5500 base enumeration

The base-only constructor enumerated 913,000 distinct subgroup-usable N83
points at K5500. Independent replay passed for every representative and
signed Frobenius orbit, and the frozen K4405 base matched an exact ordered
prefix. The split ID31 root index and online solve remain separate stages.

The split five-byte root index preserved the same 21,248-point factor base,
680,000 symmetric states, and 1,359,488 exact-root entries as the monolithic
index in four paired K128 controls. The two orders gave opposite timing
directions, so the index-build comparison remains inconclusive.

The dual-filter pairs ran monolithic then split in
[k128_a](runs/k128_a/result.json) and split then monolithic in
[k128_b](runs/k128_b/result.json). The shared-filter pairs used the same
orders in [k128_shared_a](runs/k128_shared_a/result.json) and
[k128_shared_b](runs/k128_shared_b/result.json). Each directory also retains
`input.json`, the frozen runner, child stdout/stderr, and process receipts.
The `status` in every paired result is `PASS_INDEX_STAGE_CONTROL`.

| Pair | Monolithic index build (s) | Split index build (s) | Earlier arm |
| --- | ---: | ---: | --- |
| dual A | 22.393 | 15.919 | monolithic |
| dual B | 14.285 | 15.502 | split |
| shared A | 18.030 | 28.390 | monolithic |
| shared B | 24.862 | 14.632 | split |

All four controls ran on the shared macOS host. The stage executable stopped
after index construction, before rank, target relation search, recovery, or
scalar replay. The target file was an invocation fixture only. The chart
[index_build.svg](index_build.svg) and its
[PNG render](index_build.png) display these exact stage intervals. The
[run PDF](RUN.pdf) and [result PDF](RESULT.pdf) were generated from the four
versioned `result.json` files by [render_report.py](render_report.py).

The split tables allocated 13,107,200 bytes versus 10,485,760 monolithic.
At K128 the union Bloom matched monolithic's 2,097,152 bytes, saving
524,288 bytes relative to separate filters on the split tables. At K4405,
the frozen layout calculation predicts a 128 MiB Bloom saving and
671,088,640 additional table bytes. Those K4405 values are capacity
calculations; [RESULT.md](RESULT.md) retains the qualification boundary and
the next full-size control.

The separate [K5086 ID30 and K5500 ID31 capacity calculations](CAPACITY.md) are illustrated by
[capacity.svg](capacity.svg) and its [PNG render](capacity.png). It is shown
on the second page of [RESULT.pdf](RESULT.pdf) with measured K128 timing kept
on the first page.

## Four-thread root-construction control

The [four-thread K128 run](runs/k128_root_threads4/result.json) used the
same source and target as the serial shared-Bloom control, a release
executable, and `KIC_INDEX_ROOT_THREADS=4`. Its monolithic and split arms
passed the index-stage control. The independent
[serial/parallel replay](runs/k128_root_threads4/verification.json) matched
the base hash, all 680,000 states, both exact-table digests, all 1,359,488
root entries, split allocation, and Bloom bytes for each layout. The
four-thread result SHA-256 is
`573e666233744f093a2808180d6f4ef0b03d8c2b7a605b1116a72bf2c35577ca`.
The prior serial executable used a debug profile and the host was shared,
so these runs establish deterministic index output rather than a calibrated
thread-count speedup. The [runner](run_index_smoke.py) and
[verifier](verify_root_threads.py) are versioned.

## ID31 small-prefix replay

The [K128 ID31 run](runs/k128_id31/result.json) used the same public-point
fixture and K64 split, four index-construction threads, and the opt-in
`KIC_INDEXED5_ID31=1` slot format. Both monolithic and split arms passed
`PASS_INDEX_STAGE_CONTROL`: each built the same 21,248-point base,
680,000 states, and 1,359,488 root entries. Their table byte counts,
per-table entry counts, and state digest match the earlier ID30
four-thread control. The source SHA-256 is
`6876cfa98aac31b007a91437c2be9afc0d7c3532d23f0374acd8a780cbce5b13`,
the native release executable SHA-256 is
`2227fe7031d375ec4b3b1cb4b7b081e9c5493d155709c76e8a576efdc05284db`,
and the result SHA-256 is
`a330adee412481992c57cd91f14219738c591b4e8fd2bccd75aa9013ac21dd12`.
The [input](runs/k128_id31/input.json), frozen runner, per-arm stdout,
stderr, and process receipts are versioned. This shared-host replay is an
index-stage correctness diagnostic; it does not supply a paired online
speedup or the K5500 base enumeration.

## Base-only enumeration and independent replay

The opt-in `KIC_BASE_BUILD_ONLY=1` path writes the ordered point set and
exits before normal-basis setup, S3 root indexing, rank, or target work.
The [bounded runner](run_base_stage.py) froze the source and executable in
each run directory and limited child wall time to 1,800 seconds and sampled
RSS to 8 GiB for the two larger bases. The [independent verifier](verify_base_stage.py)
checked exact curve and subgroup data, every representative's subgroup
membership, all orbit coordinates and labels, closure, and distinctness.
The versioned K4405 and K5500 dumps are gzip-compressed without changing
their uncompressed SHA-256 digests.
The [post-change K128 ID31 control](runs/k128_id31_baseonly_regression/regression_check.json)
matched the prior monolithic and split base, state, exact-table digest,
entry, allocation, and Bloom fields: `PASS_EXACT_DEFAULT_INDEX_REGRESSION`.

| K | Actual points | Scanned abscissae | Base construction (ms) | Child wall (s) | Sampled peak RSS (bytes) | Replay |
| ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 128 | 21,248 | 292 | 1,351.088 | 4.168 | 8,077,312 | [PASS](runs/base_only_k128_20261021/verification.json) |
| 4,405 | 731,230 | 8,907 | 3,228.355 | 6.410 | 386,383,872 | [PASS](runs/base_only_k4405_20261021/verification.json) |
| 5,500 | 913,000 | 11,041 | 6,925.237 | 10.330 | 479,117,312 | [PASS](runs/base_only_k5500_20261021/verification.json) |

The K5500 [ordered dump](runs/base_only_k5500_20261021/base_dump.jsonl.gz)
has uncompressed SHA-256
`823782912c75d782cdd7e6c9074c94282e28a686b1dd98ea978a367514d3acdd`
and producer BLAKE3 base hash
`0789049953a67b51dfe23f6fa2788d11e0bb4b35d52180fc038a3ffd2c0efb5f`.
Its [prefix receipt](runs/base_only_k5500_20261021/prefix_check_k4405.json)
matches the first 731,230 coordinates and labels and all 4,405
representatives to the frozen K4405 manifest. These shared-host times are
construction diagnostics; the memory estimate in [CAPACITY.md](CAPACITY.md)
still needs a full-index measurement before any Pod admission.

## Full-size local K4405 split control

The [shared-filter K4405 input](runs/k4405_split_shared_local_7200/input.json)
froze the same one-point fixture used for the monolithic local control, a
7,200-second child wall cap, 26 GiB sampled RSS cap, and an independent
8 GiB internal-scratch free-space guard. The split child exited by
`SIGTERM` after 5,098.402 seconds; its sampled peak RSS was
12,649,693,184 bytes. The [guard receipt](runs/k4405_split_shared_local_7200/n83_split_k4405_disk_guard_result.json)
records `DISK_GUARD_STOP` at 8,064,487,424 free bytes, below its
8,589,934,592-byte threshold. The [guard code](runs/k4405_split_shared_local_7200/n83_disk_guard.py),
[raw process outcome](runs/k4405_split_shared_local_7200/process_result.json),
and [verification classification](runs/k4405_split_shared_local_7200/verification.json)
are retained together. The raw runner recorded `FAILED_PROCESS` because
the external guard sent the signal; verification classifies the bounded
resource stop as `BOUNDED_UNKNOWN` with reason `DISK_GUARD_STOP`.

Both stdout and the target result file are empty, and no base dump was
emitted, so index completion, rank, and one-target online cost remain
unmeasured in this control. The earlier monolithic local run reached its
7,200-second cap. Neither shared-host run qualifies a Tier 1 speed
comparison; the exact Pod envelope in [MAXK_PROTOCOL.md](MAXK_PROTOCOL.md)
is the next execution gate.
