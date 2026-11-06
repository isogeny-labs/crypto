# N83 current-source online screen: complete Tier 1 RunPod replay

Six fresh processes in the frozen A/B/C/C/B/A order solved and independently
replayed 8,000 distinct public points apiece on one RunPod `cpu3g-8-32` Pod.
All six arms passed the 15.25 GiB process-RSS guard, zero-throttling,
zero-OOM and zero-steal gates. The 99% A/B repeatability gate passed.

| Exact factor base | Usable points | Columns | A mean online | B mean online | Mirrored mean online |
| --- | ---: | ---: | ---: | ---: | ---: |
| K3450 wide3 | 572,700 | 3,450 | 17.896 ms | 17.617 ms | 17.756 ms |
| K3500 root4 | 581,000 | 3,500 | 16.098 ms | 16.462 ms | 16.280 ms |
| K3596 root4 | 596,936 | 3,596 | 16.000 ms | 16.081 ms | 16.041 ms |

The candidate IDs are
`IC1N83Ckb1a1fb572700PDP4rootRCguidedLAgaussTDdirectISO0hc7264b83bbd1`,
`IC1N83Ckb1a1fb581000PDP4rootRCguidedLAgaussTDdirectISO0hd41c74f97eea`,
and `IC1N83Ckb1a1fb596936PDP4rootRCguidedLAgaussTDdirectISO0h3da4e8b31c3f`
in table order. Their manifests specify the exact factor-base construction and
index policy.

K3450 is slower than each root4 arm on the prespecified paired 99% block
intervals: K3450 minus K3500 is `[0.781, 2.172]` ms and K3450 minus K3596
is `[1.012, 2.419]` ms. K3596 minus K3500 is `-0.239` ms with interval
`[-0.900, 0.418]` ms. Therefore the frozen online selector returns
`selected_online_candidate_id: null`, `BOUNDED_UNKNOWN`. These 8,000-point
means are a secondary diagnostic. The primary single-target same-point IC/rho
holdout for the selected base remains open.

The Pod ran source commit `f33a526e9bee85f1906db9169cfa1c74f71e8a94`
with the sealed fixture hashes and candidate flags. The first two sparse-clone
bootstrap attempts failed before target timing because root Cargo files and
then the Rust `docs` include were omitted. Their logs, exit statuses and exact
repair scripts are retained under `evidence/` in the raw archive. The third
attempt passed the sealed dispatch check, Rust example test/build, all six
producer arms, and on-Pod independent verification.

An additional local `cargo test --release --locked --offline --lib` attempt
exited 101 because this sparse checkout omitted 23 tracked `include_str!`
fixtures outside the screen directory. The exact log and exit code are
preserved as `local_lib_test.log` and `local_lib_test.exit.txt`; this is a
checkout materialization gate, not a test failure in the screen producer.

The original raw archive is `95,004,360` bytes, SHA-256
`357260ea97c3ddace5cf69e129c5501e33a5da0f8b3b3f8e8a82a50ee6a58e5b`.
`raw_parts_manifest.json` records two SHA-256 checked transport parts.
`runpod_aggregate.py` checked all 82 raw files, including all 54 results,
bound the allocation and machine IDs, and independently replayed every base,
relation and scalar from the extracted archive; `validated.json` records
`PASS_INDEPENDENT_REPLAY`. `local_replay.exit.txt` is `0`, and stderr is empty.
The Pod was terminated at `2026-10-19T04:13:48.442680+00:00`, after
5,183.728 seconds at an observed $0.32/hour, for an estimated compute charge
of $0.461 before provider rounding. The watchdog cap was 165 minutes/$1.10.

| Requested outcome | Status and evidence |
| --- | --- |
| Test the three frozen current-source bases on fresh points | Complete: six arms, 8,000 points each; `analysis.json` and raw archive. |
| Apply the Tier 1 cloud resource and repeatability gates | Complete: `host.json`, per-arm usage/telemetry, `analysis.json`. |
| Independently replay and select under the frozen rule | Complete replay; selection unresolved between K3500 and K3596. |
| Establish the best primary one-target online factor base | Open: run a fresh same-point IC/rho holdout after selection. |

Recheck the raw binding with `runpod_aggregate.py` using the committed
allocation, archive, part manifest and a safe extraction of `results/`.
The complete extraction helper is `runpod_retrieve.py`; its SSH path and
identity are configurable. `analysis.json` and `validated.json` allow quick
inspection without unpacking the 95 MB archive.
