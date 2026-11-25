# M31 natural stream: exhaustive vs overlap-sensitive MST (2026-11-25)

Input: the group-admitted M31 ten-query stream from PR #5076, rebuilt locally
with the original `prime31_b677_capture` and `prime31_lp_group_admission`
examples (PR #5076 head, `cargo build --locked --release`) from the frozen
producer file at `577d6c2c`. All four identities match the CI record
(`INPUT_SHA256SUMS`; `ed57ac0d…` is the admitted 613-row stream). The 256-
and 512-row prefixes are the retained files in
`evidence/native-m31-capture-20261124/raw/m31/prefixes/`.

Command: `lp_mst_strategies --input <stream> --modulus 2147446831 --reps 101`
(`examples/lp_mst_strategies.rs`, built by the `native_gain_dsu` harness).
Options are the `lp_graph_eval` MST defaults. Host: 4-vCPU x86_64 cloud
container, shared, unpinned. [pcc:unverified]

| rows | identical output | replay failures | overlapping pairs | logical edges | differences formed | implicit selections | wall median ratio (overlap / exhaustive) |
|---:|:-:|---:|---:|---:|---:|---:|---:|
| 256 | yes | 0 | 135 | 139 | 137 | 2 | 1.21 |
| 512 | yes | 0 | 753 | 779 | 757 | 4 | 1.13 |
| 613 | yes | 0 | 1273 | 1328 | 1280 | 7 | 1.13 |

The exhaustive arm on 613 rows reproduces the CI-retained `graph-613.json`
MST stats exactly (1328 edge evaluations, 143 trees, largest column degree
15, 3033 output nonzeros, 452 complete relations).

**Outcome:** outputs, witnesses and legacy stats are identical, as the proof
requires, but there is no saving on this stream: 96 % of pairs overlap and
the strategy forms 0.96x the differences, while its overlap-index overhead
makes the MST stage about 1.13x slower. Cause: every admitted row carries the
target column `u32::MAX` (613/613 rows, 10 distinct coefficients, one per
query), so two rows of the same LP column almost always share it.

Wall times are informational (same process, interleaved A/B, input clone
included in both arms), not a benchmark row.
