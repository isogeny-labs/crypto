# Protocol: sibling-points panel (frozen 2026-11-21, before any data)

Stage 1 panel of the sibling-points mechanism (`../ideas/SIBLING-POINTS-20261110.md`), after Stage 0 (`../siblings/STAGE0.md`, per-parent pools keep the Poisson count) and two single-curve smokes (`../siblings/SMOKE.md`) [pcc:unverified]. Nothing below changes after the first session runs; failed predictions stay failed.

## 1. Question

Does `sib=8,pool=64` lower the cost per stored point of the target walks by the predicted factor on five fresh curves at every size, on the main track (C = 2) and the boundary track (C = 1), in the harness's exact unit and in the GAE band? The 0.75x goal is not in reach of this mechanism for any K >= 2 (note, section 3); the panel measures the gain it does give.

## 2. Design (frozen)

- Binary: `target/release/ecbench` built from branch `claude/sibling-stage1` at 763df27931 (the merged #3847 code), hash in `data/binary.sha256`; the k = 24 SNC panel session replays under it (SMOKE.md).
- Curves: the five fresh sibling curves per size of `../c1/confirm-curves.json`, k = 20, 24, 28, 32.
- Targets: 12 public targets per curve, 8 measured rounds plus 1 warm-up, `order = alternate`; arms paired by workload and round (ecbench seeds runs by workload and round, so no A/A arm).
- Arms: `rho-neg`; `inc` (the ledger incumbent at the size: box at k = 20, `tw2l-x20-j32` at 24, `wwalkGg-x09-sr` at 28, `tw2l-x20-j16` at 32); `tw2cj-lazy` (`wwalk:x=2.0,jumps=16,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`, size-2 base); `tw2cj-lazy-sib8` (the same plus `sib=8,pool=64`); `c1-lazy` (`wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,lazy=1,seed=run`, size-1 base); `c1-lazy-sib8` (plus `sib=8,pool=64`). All pipeline arms use `targets=stride-generator`.
- Units: (a) `bitop/v1` per sqrt(r), from `work.bitops` on every record (the exact field-work price; a sibling second point is sqr + 2 add); (b) GAE S with the §5a band at w = 0, 0.1, 1 (`ecbench band --ranks`), where the second point is a native unit priced at w. Both are reported; which one the ratchet reads is the owner's ruling (note, section 4). Rho-neg is priced identically in both.
- Audit: `ecbench verify --replay 8` per size; every record must carry the audit status `verified` [pcc:unverified].
- Statistic: `scripts/bitops_pairs.py` (paired by workload and round, ratio of sums, percentile bootstrap over workload blocks, 4000 resamples, seed 20261121) for the bit-op unit and for GAE at w = 0; `ecbench compare --weight w --resamples 4000 --seed 20261121` for the GAE band.
- Pre-registered pairs per size: (`tw2cj-lazy`, `tw2cj-lazy-sib8`); (`c1-lazy`, `c1-lazy-sib8`); (`inc`, `tw2cj-lazy-sib8`); (`rho-neg`, each IC arm).
- Lanes: the four sizes run as independent lanes under one bench record; an independent validator re-derives every ratio from `records.jsonl` with its own code; a red-team reads protocol and results before the write-up. Subagent models: the lanes that compute run on Sonnet; validation, red-team and the write-up inherit the session model (Mythos 5.1; Mythos 5.2 is not selectable for subagents in this runtime).

## 3. Predictions (before data) [pcc:unverified]

Bit-ops per sqrt(r), paired ratios:

| pair | k = 20 | k = 24 | k = 28 | k = 32 |
|:--|:--|:--|:--|:--|
| `tw2cj-lazy-sib8` / `tw2cj-lazy` | 0.74 [0.66, 0.84] | 0.70 [0.62, 0.80] | 0.68 [0.60, 0.78] | 0.68 [0.60, 0.78] |
| `c1-lazy-sib8` / `c1-lazy` | 0.74 [0.66, 0.84] | 0.70 [0.62, 0.80] | 0.68 [0.60, 0.78] | 0.68 [0.60, 0.78] |
| `tw2cj-lazy-sib8` / `rho-neg` | 1.40 [1.25, 1.55] | 1.30 [1.15, 1.45] | 1.30 [1.15, 1.45] | 1.30 [1.15, 1.45] |
| `c1-lazy-sib8` / `rho-neg` | 1.10 [0.95, 1.25] | 1.00 [0.88, 1.12] | 1.00 [0.88, 1.12] | 1.00 [0.88, 1.12] |

Stored points per sqrt(r) of a `sib8` arm within 10 percent of its no-sibling arm. GAE band: `sib8` below its no-sibling arm at w = 0 and 0.1, above it at w = 1 (second point priced as a whole addition), so the band verdict is predicted unresolved at every size.

## 4. Falsification (frozen)

- The mechanism is dead on the main track if the upper end of `tw2cj-lazy-sib8` / `tw2cj-lazy` in bit-ops is at or above 0.80 at two of k = 24, 28, 32; dead on the boundary track by the same rule for the C = 1 pair.
- Any record not `verified`, any failed replay, or stored points above 1.10 x the no-sibling arm at two sizes kills the panel's claim.
- A prediction outside its interval stays recorded as missed; no interval is widened after data.

## 5. Reporting

`README.md` with the pair table in both units against the predictions, per-arm means, audit results, validator and red-team verdicts, an SVG and PDF, a bench record, a pcc statement claim, a row in `../README.md`, the session log. No ledger change in this PR: the ratchet waits for the owner's unit ruling.

## 6. Amendments

None yet.
