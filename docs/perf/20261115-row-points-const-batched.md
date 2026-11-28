# `rootfind::row_points`: batching chosen at compile time (repairs the #3991 `ic-rf-pairs` slowdown)

Class: wall-time repair, no counted-cost or output change; not an improvement over the pre-#3991 baseline, so no leaderboard row.

**Finding.** The RunPod A/B of #3991 (merge 94c682e021 vs first parent 6745ba0a5f, `research/wallfix_runpod_20261114/`) measured `ic-rf-pairs` 1.3% slower (solve CPU time, B/A 1.011-1.015 in all three rounds, A/A 1.004). Callgrind (`ecbench audit-ir`, `ic-rf-pairs` sequence numbers 41, 51, 61 of the prime spec) attributes +0.90-0.95% executed instructions to `rootfind::row_points` (+3.7% exclusive) and `RootfindPairsOracle::decompose_relations` (+2.7%): #3991 gave `row_points` a runtime `batched: bool`, so every instantiation reaches both `add_many` and `add_many_batched`. The batched path never runs in this arm.

**Change.** `row_points<G, const BATCHED: bool>`; `RootfindMitmOracle` dispatches on its runtime flag outside the function; `RootfindPairsOracle` instantiates `false`. One file, +14/-39.

**Measured (`[pcc:unverified]`, instruction counts are deterministic).** Solve-marker Ir of `ic-rf-pairs`, binaries built from `origin/main` 7b3c057e1c (unpatched, sha256 7d46e5ee...c9cb) and with this change (sha256 eb78d3e8...fbf95), same rustc 1.100.0 and flags:

| seq | unpatched | patched | patched / unpatched | pre-#3991 (6745ba0a5f) |
|--:|--:|--:|--:|--:|
| 41 | 333,436,032 | 330,280,945 | 0.9905 | 330,279,252 |
| 51 | 348,812,538 | 345,511,722 | 0.9905 | 345,508,464 |
| 61 | 10,332,399,319 | 10,240,124,187 | 0.9911 | 10,240,146,100 |

Patched Ir is within 0.001% of the pre-#3991 binary's. **Outputs:** the prime spec (240 records, both `ic-rf-pairs` and `ic-rf-mitm-sgauss` arms) gives byte-identical normalised records for the two binaries (`docs/perf/20261110-wall-only-rust-fixes/norm-records.sh`), and the patched binary reproduces all 200 replays of the unpatched binary's session. No wall-time claim is made here: the instruction count is the evidence, and a wall-time confirmation needs L2 (isolab-eks, blocked by budget).
