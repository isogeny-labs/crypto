# Addendum A: ablation of the L3-SIB gain (2026-11-19)

This addendum was frozen before any of its data; a 2-target P20 smoke run checked only that the arms run. Every statement here is `[pcc:unverified]`. It uses the same binary, curves and 48 targets as `../PROTOCOL.md`, with a new measurement seed.

## Question
`sib8-p256` differs from `tw2l` in three parameters: `sib=8,pool=256`, `targets=stride-generator`, and no `jumps=16`. Which of them carries the 0.44–0.48× (w = 0) and 0.84–0.93× (w = 1) gain?

## Arms

| arm | role | definition |
|---|---|---|
| `rho-neg` | reference | — |
| `tw2l` | baseline | as before |
| `tw2l-sib` | candidate | `tw2l` plus `sib=8,pool=256`; nothing else changed |
| `tw2l-sg` | candidate | `tw2l` plus `targets=stride-generator`; nothing else changed |
| `sib8-p256` | candidate | as in round 3 |

The run uses 2 rounds, 1 warm-up and alternate order, at L0.

## Predictions (registered)
- **A-a:** every run verifies.
- **A-b:** `tw2l-sib / tw2l` is at most 0.60 at w = 0 at every size; the siblings carry most of the gain.
- **A-c:** `tw2l-sg / tw2l` is in [0.85, 1.05] at w = 0.
- **A-d:** `sib8-p256 / tw2l-sib` is in [0.85, 1.15] at w = 0; dropping `jumps=16` and changing targets matter little once siblings are on.

## Decision
- `tw2l-sib` is reported as an improvement over `tw2l` only if `ecbench band` orders it lower at every weight at every size, and likewise for `tw2l-sg`.
- If `tw2l-sib` is below `sib8-p256` at every weight at every size, it becomes the incumbent instead.
- Distance to rho is reported as the distance remaining.
