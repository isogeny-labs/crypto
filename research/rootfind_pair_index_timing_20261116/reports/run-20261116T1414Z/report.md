# Koblitz pair-index component timing — run-20261116T141419Z

This is a descriptive run report only. It records observations from one shared-host L0 process; it does not review a hypothesis or establish a complete-call ECDLP performance claim. The study has no assigned EXP, HYP, or GOAL identifier. Its frozen [protocol](../../PROTOCOL.md), [control amendment](../../AMENDMENT-1.md), [run record](../../RUN_RECORD.json), [raw stdout](../../run.stdout), and [derived results](../../RESULTS.json) bind the scope.

## Run and controls

- **Measured:** One of one requested focused release tests passed. UTC 2026-11-16 14:14:19–14:14:20; seven untimed warmups and 101 timed repetitions for every one of five arms on each of K13, K19, K23. All 1,515 timed rows and three case records are present. No timed cell failed or timed out.
- **Fixed inputs:** Actual Koblitz E0 signed Frobenius-orbit bases with dimensions 5/7/8, 272/4,830/18,906 pair attempts, 261/4,800/18,842 retained first keys, 11/30/64 duplicate keys, and 2,112/8,896/17,600 precomputed probe keys. Public residuals used Q=[641]G. The full curve and factor-base digests, pair/probe hashes, and expected checksums are in `RESULTS.json`.
- **Controls:** `hash` matches the production u64-key MixHasher and a*a+1 capacity per Amendment 1; `hash-aa` is an identical A/A arm. Every duplicate pair passed a full group-equation check; every arm returned the same first witness and lookup checksum. These are source-harness checks, without independent replay.
- **Command:** `gtimeout -s TERM -k 10s 600s /usr/bin/time -l /Volumes/SSD990-2/crypto/wt/codex-rootfind-k13-local-f456-screen-20261116/target/release/deps/rootfind_pair_index_timing-79a07ef92fe2f245 --exact fixed_koblitz_pair_index_build_probe_timing --nocapture`.
- **Source and host:** Protocol commit `46faae79f8`, amendment `71d5af799b`, corrected source commit `1b91f79a5c`; clean run tree at `92bf1257275f`. Executable SHA-256 `4f8e218be771a8cf58cc1005c9aa49561ae85701d3846f5af0f266ca862ec7ff`. Darwin 27.2 arm64, Mac16,7, 14 logical CPUs, Rust 1.99.1. Whole diagnostic process: 0.38 s wall, 0.12 s user CPU, 50,593,792-byte maximum RSS. These process measures are not per-arm values.

## Recorded component timings

**Measured medians [minimum, maximum], nanoseconds**, of fresh index allocation/insertion plus the fixed precomputed lookup sequence. Each cell has 101 repetitions per arm. The [editable plotting source](plot.py) produces the [vector chart](chart.svg) from `RESULTS.json`; panel scales differ.

| Cell | Hash | Hash A/A | Direct flat u32 | Paged | Inline |
| --- | ---: | ---: | ---: | ---: | ---: |
| K13 | 5,667 [5,583, 15,126] | 5,667 [5,583, 7,418] | 4,666 [4,375, 5,833] | 8,791 [8,459, 12,584] | 7,208 [6,458, 12,999] |
| K19 | 37,708 [35,375, 56,833] | 37,334 [34,875, 56,750] | 31,500 [29,208, 38,875] | 45,000 [41,751, 55,417] | 47,500 [43,458, 62,750] |
| K23 | 112,416 [101,459, 153,249] | 111,541 [100,625, 131,999] | 226,749 [189,500, 477,125] | 118,083 [103,167, 274,834] | 125,541 [113,333, 182,001] |

The hash A/A median absolute differences were 0, 374 and 875 ns at K13/K19/K23. `RESULTS.json` records separate build and lookup medians, plus the frozen component screen's derived ratios and threshold flags. The direct flat index has lower observed combined medians on K13/K19 and a higher observed median on K23. Paged and inline observed medians exceed hash in all three cells. This is a description of the recorded sample, not an independently verified advantage.

## Scope and retained deviations

The timed section begins after elliptic-curve pair sums, target and probe-key generation. It omits relation rank and linear algebra, verification, process startup and per-arm memory. It is not the production collector's complete call; in particular, the standalone direct flat build differs from the hash-then-convert dense path in PR #4414. No matched F4/F5/F6 arm ran in this study. The separately frozen L2 panel remains unrun while its budget and isolation gates are unmet. No canonical performance graph or leaderboard was changed because this run does not supply admitted complete-call evidence.

The first compile-only command failed on a harness `usize`/`u32` argument mismatch; `build.*` retains its output. The corrected source was committed and the second compile-only command passed (`build2.*`). Neither compile check is a timed cell. The run itself exited zero under the 600-second watchdog. The run record also binds the pre-run S3 checkpoint, command, hashes, environment and validity flags. There is no independent replay yet.
