# M31 natural stream: three exact MST strategies (2026-11-25)

Input: the group-admitted M31 ten-query stream from PR #5076, rebuilt locally
with the original `prime31_b677_capture` and `prime31_lp_group_admission`
examples (PR #5076 head, `cargo build --locked --release`) from the frozen
producer file at `577d6c2c`. All four identities match the CI record
(`INPUT_SHA256SUMS`; `ed57ac0d…` is the admitted 613-row stream). The 256-
and 512-row prefixes are the retained files in
`evidence/native-m31-capture-20261124/raw/m31/prefixes/`.

Command: `lp_mst_strategies --input <stream> --modulus 2147446831 --reps 101`
(`examples/lp_mst_strategies.rs`, built by the `native_gain_dsu` harness),
with the `lp_graph_eval` MST defaults. Host: shared 4-vCPU x86_64 cloud
container, unpinned. [pcc:unverified]

| rows | arm | identical output | replay failures | differences formed | wall median / exhaustive |
|---:|---|:-:|---:|---:|---:|
| 256 | exhaustive | — | 0 | 139 | 1.00 |
| 256 | overlap-sensitive | yes | 0 | 137 | 1.21 |
| 256 | shared-key | yes | 0 | 82 | 1.08 |
| 512 | exhaustive | — | 0 | 779 | 1.00 |
| 512 | overlap-sensitive | yes | 0 | 757 | 1.14 |
| 512 | shared-key | yes | 0 | 319 | 1.04 |
| 613 | exhaustive | — | 0 | 1328 | 1.00 |
| 613 | overlap-sensitive | yes | 0 | 1280 | 1.13 |
| 613 | shared-key | yes | 0 | 449 | 1.03 |

Deterministic instruction counts for the MST stage on 613 rows (callgrind,
`CALLGRIND.txt`): exhaustive 29.43 M, overlap-sensitive 32.75 M (1.113x),
shared-key 29.21 M (0.992x). Wall ratios for shared-key ranged 1.008 to
1.026 across runs and are noise at this size.

The exhaustive arm on 613 rows reproduces the CI-retained `graph-613.json`
MST stats exactly (1328 edge evaluations, 143 trees, largest column degree
15, 3033 output nonzeros, 452 complete relations).

**Outcome.** All three arms give identical relations, witnesses, report and
legacy stats. Every admitted row carries the target column `u32::MAX`
(613/613 rows, 10 distinct coefficients), so 96 % of same-column pairs
overlap: the overlap-sensitive arm saves nothing and costs 11 % more
instructions. The shared-key arm computes every pair's exact key from
shared-key counts and forms only the 449 selected rows (0.34x), but the
stage cost moves by 0.8 % only, because spanning-tree work is about 8 % of
this stage; the per-pivot rebuild of the column map and row vector in the
outer elimination loop dominates.
