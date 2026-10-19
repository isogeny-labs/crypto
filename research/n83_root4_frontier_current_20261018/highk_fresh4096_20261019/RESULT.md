# N83 wide3 packed root index: 4,096 fresh public points

## Measured contribution

The K4300 signed Frobenius base (713,800 usable points; 4,300 folded columns) averaged **6.381 ms per verified target interval** on the fresh local panel. K4096 averaged 7.535 ms. Their paired difference, K4096 minus K4300, was 1.155 ms, with a frozen 99% circular block bootstrap interval of 0.470 to 1.907 ms. The ratio of means was 1.181. All 4,096 target scalars and four point relations per arm passed independent replay.

K4405 averaged 6.804 ms. Its paired difference from K4300 was 0.423 ms, with a 99% interval of −0.228 to 1.088 ms; these data do not resolve that ordering. The candidate selection field remains null pending the Tier 1, same point, single target comparison with rho.

| Base | Usable points | Folded columns | Mean target interval (ms) | Median (ms) | Cold process wall (s) | Peak sampled RSS (GiB) | Replay |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| K4096 wide3 | 679,936 | 4,096 | 7.535 | 5.467 | 594.380 | 19.161 | 4,096/4,096 |
| K4300 wide3 | 713,800 | 4,300 | 6.381 | 4.661 | 708.285 | 20.149 | 4,096/4,096 |
| K4405 wide3 | 731,230 | 4,405 | 6.804 | 5.108 | 672.380 | 20.574 | 4,096/4,096 |

These are local timing diagnostics after shared base, index, and factor log preparation. The cold process walls are one run per arm and include setup and target processing; their order differs from the target interval order. The 4,096 point means answer a batch diagnostic question and do not replace the primary one target online metric. No Tier 0 or Tier 1 claim is attached.

## Frozen methods and exact identity

The curve is EC1N83Ckb1a1haacb69ee9f37 over GF(2^83), polynomial representation, with prime subgroup order 8569786107849059. The source and executable SHA-256 are pinned in each arm protocol. The ordered fixture SHA-256 is 281a088cc49658973375ab88359d65a3b4d51d2819a297872b5fe5da01783895. It contains 4,096 distinct public subgroup points, generated from the domain in fixture_manifest.json, with zero overlap against 128,048 distinct points from previous and reserved workloads. fixture_audit.json independently checks scalar generation, curve membership, subgroup membership, uniqueness, and exclusions.

All arms use the same source, exact 5 byte packed root index, symmetric S3 states, 2 GiB wide3 Bloom profile, four rank threads, two target threads, 24 GiB sampled RSS cap, and identical point order. The K4096 and K4300 caps are 1,800 seconds; K4405 has a 2,100 second cap. Actual base point counts, folded columns, index sizes, source hashes, and algorithm flags are in the arm protocols and run summaries. The exact K4096 wide3 candidate manifest is in identities; K4300 and K4405 manifests are in the preceding highk_20261019 evidence directory. The 4,096 point workload and three run IDs are in identities.

The paired plan was frozen before K4300 and K4405. It specifies the mean of per point target interval differences and 20,000 circular moving block bootstrap resamples, each using blocks of 64 consecutive fixture indices, with PCG64 seed 20261019. The K4096 extension plan was frozen before that arm, with the same bootstrap construction and seed 20261020. The K4096 minus K4405 interval is 0.006 to 1.519 ms. These intervals describe this local run and do not establish host isolated performance.

The first K4300 verifier failed before replay because it looked up runner_sha256 instead of pilot_runner_sha256 in the frozen protocol. k4300_verifier_repair.json preserves the failed verifier digest and one field repair. The repaired verifier replayed all K4300 outputs; the K4405 and K4096 verifiers used the corrected lookup before those producer runs. No producer output or fixture was changed. The first portable statistics replay failed because it restarted the PCG64 stream for the second extension comparison. statistics_replay_repair.json records that failed digest and the stream-advance fix; statistics_replay_receipt.json then passed all three frozen comparisons.

## Evidence and replay

Each run directory contains source.json, host_before.json, usage.json, stdout.jsonl, analysis.json, the stderr log, and a deterministic gzip of every target certificate. target_blobs_manifest.json pins compressed and raw hashes. The three base dumps are byte identical to those already published under evidence/n83-root4-frontier_20261019 in the repository store; their raw and gzip digests are pinned in the preceding store_blobs_manifest.json. The portable replay accepts a checkout and those three downloaded gzip dumps. It checks source and fixture hashes, deterministic scalars, subgroup membership, every base point, exact group relations, recovered scalars, and per target phase sums.

Run from the repository root after obtaining the three published base gzip files in one directory:

    python3 research/n83_root4_frontier_current_20261018/highk_fresh4096_20261019/scripts/verify_portable.py --repo . --base-dir /path/to/base-gz-directory

The exact executed local runner and verifier scripts are retained under scripts. Their absolute source and output paths record this run; verify_portable.py is the portable independent replay. paired_analysis.json and extension_analysis.json retain the fixed resampling outputs. scripts/recompute_statistics.py recalculates their intervals from the committed target certificates. portable_replay_receipt.json records the repository package replay.

## Next experiment

Run the two unresolved wide3 finalists on the reserved cloud holdout with an exposed Tier 1 resource envelope, alternating order, A/A and noise controls, then solve and verify one previously unseen public target with the selected IC candidate and same point rho reference. The RunPod allocation attempts and AWS Spot provider failure are recorded in the preceding highk_20261019 cloud_attempts directory. A usable Pod or instance is needed for that gate.
