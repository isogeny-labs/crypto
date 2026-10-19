# K4405 wide3 Bloom screen: replayed N83 root-index diagnostic

On the exact `icv1-f2m83-t6151469093347-9eba6cd5` model and the same 16 previously exposed public subgroup points, the K4405 `wide3` Bloom profile reduced the mean measured target interval from **11.710498 ms** with `standard` to **3.880216 ms**. The ratio of those means is **3.0180**. The paired wide3-minus-standard mean is **−7.830281 ms**, with an exploratory 95% resampled-point interval of **[−10.765576, −5.181497] ms**. All 16 scalars and four-point relations passed independent replay in both K4405 arms and in the K4300 wide3 control. The factor-base dump is byte-identical between profiles; the filter is the controlled algorithmic difference.

This is a local-host diagnostic with shared target-independent preparation across 16 points. It tests the filter and high-K factor-base construction. The selected online candidate remains `null` pending the sealed Tier 1 fresh-point comparison and single-target, same-point rho holdout. The field is polynomial-basis GF(2^83), modulus `0x800000000000000000095`; the curve is `y^2 + xy = x^3 + x^2 + 1`, subgroup order `8569786107849059`, generator `(0x5789eebc7f493ecfe93af, 0x755ed5c31e037ed90b549)`. Its cross-repository EC1 alias in these candidate manifests is `EC1N83Ckb1a1haacb69ee9f37`.

| Arm | Actual base points | Folded columns | Root entries | Bloom GiB | Mean target ms | Cold process s | Peak sampled RSS GiB | Replay |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| K4096 standard | 679,936 | 4,096 | 1,392,477,548 | 1 | 7.044594 | 542.045 | 18.705 | pass, 16/16 |
| K4300 standard | 713,800 | 4,300 | 1,534,634,033 | 1 | 6.079159 | 612.270 | 18.717 | pass, 16/16 |
| K4300 wide3 | 713,800 | 4,300 | 1,534,634,033 | 2 | 5.200633 | 632.922 | 19.964 | pass, 16/16 |
| K4405 standard | 731,230 | 4,405 | 1,610,495,415 | 1 | 11.710498 | 639.784 | 19.445 | pass, 16/16 |
| K4405 wide3 | 731,230 | 4,405 | 1,610,495,415 | 2 | 3.880216 | 646.502 | 20.448 | pass, 16/16 |

K4300 wide3 had a 5.200633 ms local mean versus 6.079159 ms for K4300 standard; its paired wide3-minus-standard interval was [−1.151190, −0.599935] ms. Its comparison with K4405 wide3 crossed zero: the K4300-wide3-minus-K4405-wide3 interval was [−0.374375, 3.170544] ms. The K4300-minus-K4096 paired mean was −0.965435 ms, interval [−5.247441, 2.318617] ms. K4300-minus-K4405 standard was −5.631339 ms, interval [−9.368534, −2.142649] ms. K4405 wide3-minus-K4300 standard was −2.198943 ms, interval [−4.178018, −0.432672] ms. These intervals are exploratory point-resampling diagnostics on the local host, not Tier 1 selection intervals. The complete 100,000-resample, fixed-seed results and per-point values are in `highk16_paired_diagnostic_v2.json` under `audits/`; the earlier four-arm diagnostic remains frozen beside it.

The source file SHA-256 is `4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`; the release executable SHA-256 is `68a8f51df94be895e1a189f950da9304ecb1689b9b74dd72994ec1ca2a1f46d0`. The 16-point fixture SHA-256 is `fadc5caf48c0241c1bae6c7059de93abbb3cdc14e8900c4d3152b200b2e5ea25`. Each `runs/<arm>/` directory holds producer source/flags, host admission, process usage, summary, per-point rows, and independent analysis. The executed scripts and frozen protocols are retained in `scripts/` and `protocols/`. The local scripts preserve their execution paths; source and input hashes identify their contents independently of those paths.

The exact candidate IDs are:

- K4096 standard: `IC1N83Ckb1a1fb679936PDP4rootRCguidedLAgaussTDdirectISO0h55e77acbaacf`.
- K4300 standard: `IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0hc7fc083df612`.
- K4300 wide3: `IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h394bfa10df77`.
- K4405 standard: `IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h457bb5631ea6`.
- K4405 wide3: `IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h2f81ad5da3e7`.

The independently checked capacity formula is `states(K) = 83 K(K−1)/2 + 42 K` and `request(K) = ceil(4 states(K)/3)`. Lean 4.34.1 checked the K4300 and K4405 arithmetic and the structural monotonicity proof showing K4405 is the last K whose request fits the current 2^30-entry request bucket; `proofs/` retains the modules, commands, output, and `#print axioms` results. `lean4checker` was unavailable.

## Raw factor-base replay objects

The three distinct base dumps are compressed with gzip `mtime=0` and published through the repository store. `store/index.json` pins their bytes, SHA-256, location, and provenance. K4405 standard and wide3 used the same raw dump, SHA-256 `d5e172ea5ac62e99770607a1470ea95c52d8a53dedbd5cfced7b2a3173421e9e`.

| Base | Store name | Compressed SHA-256 | Raw SHA-256 |
| --- | --- | --- | --- |
| K4096 | `evidence/n83-root4-frontier_20261019/k4096_base_dump.jsonl.gz` | `ca24c2cbeaac2dc81035b254922e2e29cc25bcfa84f26fceebfe995e268a23e4` | `cb56c16620afc771d377102f9c4dc53d285cb9c5c2b38606d8cb202782fe39ed` |
| K4300 | `evidence/n83-root4-frontier_20261019/k4300_base_dump.jsonl.gz` | `43274cecad77b29806f15404109bb7ae9beb07214b5d470f651704be4048d7f6` | `06cf0ee56506c5674afac988b2809e1287d6be86de4cb0d8738e8dc989d1fd72` |
| K4405 | `evidence/n83-root4-frontier_20261019/k4405_base_dump.jsonl.gz` | `7e9009da4315ca14445184c4830a9c9e640ecba4d5151a5dc713b930ef15858b` | `d5e172ea5ac62e99770607a1470ea95c52d8a53dedbd5cfced7b2a3173421e9e` |

All three remote objects passed `crypto store verify <three names> --remote`. `store_blobs_manifest.json` records sizes and both digest layers. The portable five-arm verifier `scripts/verify_stored_runs.py` independently reloaded the three gzip objects, checked both digest layers, reconstructed every base, and replayed all 80 public-point certificates and phase sums. Its `PASS_INDEPENDENT_REPLAY` receipt is `audits/portable_stored_replay.json` (SHA-256 `3c3a1d05ace2a8958e95a0e58230670c3eb8a1d4b1adde9dd408f42753d8cd88`); the verifier source SHA-256 is `d6101d7393c7ba278f486fbe4fe385908453b85a9a52940f8d3ed1577d26c2c1`. Run it with `--repo <checkout> --evidence <this directory> --base-dir <directory containing the three downloaded gzip files>`.

## Branch verification

At branch commit `c4f8154e`, the complete `cargo test --release --offline --lib -- --test-threads=2` run passed **4,162 tests, zero failures, 117 ignored**. The packed-root example passed 21/21 release tests; the frontier selector and provider-job scripts passed 9 and 2 cases. `verification_receipt.json` pins the commands, outputs, source and replay hashes. The complete passing library log is `store:evidence/n83-root4-frontier_20261019/cargo_lib_pr_final.log.gz`, gzip SHA-256 `e289da386fd8f8f604a8d67318f912ad73b77d10a5d0999873d1a6476d561831`; remote verification passed. The receipt also identifies three immutable first-attempt logs: missing sparse checkout fixtures, a run with seven fixture/Z3 failures, and a run with one intermittent unrelated false-acceptance test failure. Those cases passed in isolation before the final complete run.

## Execution status

Seven bounded RunPod allocation attempts produced no usable running container for this high-K panel; their individual failure receipts are in `cloud_attempts/`. The approved AWS Spot fallback was rejected by the provider as `Blocked` before any instance was created. Those are infrastructure outcomes with zero Tier 1 measurements. The first K4405 pilot had a local telemetry-permission failure and was stopped; the corrected run passed. The first wide3 candidate materializer left one flag at `standard`; the independent audit rejected that identity. `audits/` retains the failed manifest and correction, and the corrected manifest passed exact-flag and hash audit. The measured wide3 producer run did not change during that correction.

The next run is the already frozen two-stratum Tier 1 comparison on fresh public points, with alternating arm order, full resource/noise receipts, independent relation and scalar replay, then a one-target IC/rho holdout on the selected arm. K4300 wide3 passed independent replay and exact candidate audit; its fresh-point Tier 1 comparison remains open before selection. `scripts/verify_stored_runs.py` accepts a checkout and three downloaded gzip objects to replay the public controls from the stored evidence.
