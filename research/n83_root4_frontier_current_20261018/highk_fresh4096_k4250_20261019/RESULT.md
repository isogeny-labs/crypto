# N83 K4250 wide3 factor-base control

## Measured result

K4250 constructs 705,500 distinct usable factor-base points and 4,250 folded columns. On the same 4,096 fresh public subgroup points used by the merged three-arm panel, its mean verified target interval was **6.685 ms**. K4300 measured 6.381 ms and K4405 measured 6.804 ms. The frozen paired 99% interval for K4250 minus K4300 is −0.435 to +1.048 ms; for K4250 minus K4405 it is −0.912 to +0.702 ms. Neither comparison resolves a target-stage improvement at that threshold.

The K4250 process wall was 627.526 seconds, including cold setup and all 4,096 targets, with 19.803 GiB peak sampled RSS and 45% minimum sampled free memory. This single local cold wall is a diagnostic, not a calibrated setup-inclusive ranking. All 4,096 scalars and four-point target relations passed independent replay. The selected online candidate remains null while the Tier 1 same-point single-target IC/rho gate is pending.

## Frozen method and exact identity

The curve is EC1N83Ckb1a1haacb69ee9f37, and the target fixture is the ordered 4,096-point set with SHA-256 281a088cc49658973375ab88359d65a3b4d51d2819a297872b5fe5da01783895. The frozen protocol pins source/executable hashes, 2 GiB wide3 Bloom lookup, exact 5 byte packed root table, four rank threads, two target threads, an 1,800 second cap, and 24 GiB sampled RSS cap. Its retained base count is 705,500, with 4,250 folded columns and 1,499,152,664 occupied root table entries.

The K4250 candidate is IC1N83Ckb1a1fb705500PDP4rootRCguidedLAgaussTDdirectISO0h600eebde16ef. candidate.json pins its exact base and algorithm. The workload ID is 55d566dc37f8, shared with the preceding three-arm panel. ic1_row.json gives the full run ID and cost row. identity_audit.json verifies the canonical candidate digest, base SHA-256, source base hash, and run ID.

k4250_plan.json was frozen before timing. It specified two paired comparisons on the same ordered points, with 20,000 circular moving block resamples of 64 consecutive fixture indices and PCG64 seed 20261021. k4250_analysis.json retains means, intervals, and tail probabilities. scripts/recompute_statistics.py reproduces both intervals from committed target certificates.

## Evidence and replay

The new ordered base dump was compressed with gzip mtime zero and published to the repository store as evidence/n83-wide3-fresh4096_20261019/k4250_base_dump.jsonl.gz. The remote object passed crypto store verify --remote. base_manifest.json pins 16,851,506 compressed bytes and SHA-256 6f56a7ea520e8f5fd965a105231220b3b4f919707dc0e26193310f97d58604be, plus the raw digest. store/index.json records the remote location.

The K4250 run directory retains source, host, resource, output summary, stderr, independent analysis, and every target certificate as a deterministic gzip. The portable verifier reloads the stored base, the preceding panel's frozen fixture, and this run's certificates; it checks every scalar, four-point relation, and phase sum.

From the repository root, download the K4250 base gzip into one directory and run:

    python3 research/n83_root4_frontier_current_20261018/highk_fresh4096_k4250_20261019/scripts/verify_portable.py --repo . --base-dir /path/to/base-gz-directory

The exact executed local runner and first independent verifier are retained under scripts. portable_replay_receipt.json and statistics_replay_receipt.json record the package checks.

## Next experiment

Use a Tier 1 Pod or cloud instance to run the unresolved K4300 and K4405 finalists on the reserved holdout in alternating order, with A/A and noise receipts, then perform a verified one-target same-point IC/rho comparison. Further local K increments are unlikely to resolve host-dependent timing without that gate.
