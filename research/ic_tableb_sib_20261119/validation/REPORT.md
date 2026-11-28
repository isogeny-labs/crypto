# Validator report: Table B round 3 (sibling-pool wwalk) and its ablation, 2026-11-19

Independent validation of `research/ic_tableb_sib_20261119` and `research/ic_tableb_sib_20261119/ablation`. Every statement here is `[pcc:unverified]`. The main study's files are in this directory; the ablation's are in `ablation/`. `python3 scripts/validation_gate.py evidence <dir>` says `valid` for both.

## What was checked
- **Replay:** `ecbench verify --replay-all` on all six sessions (three per study). 720 of 720 records verified per session, and all 480 candidate runs per session reproduced identically.
- **Scalars:** own Python prime-field arithmetic confirmed `[recovered]G == target` for every record in all six sessions.
- **Recompute:** 207 headline numbers for the main study and 180 for the ablation (S per arm, paired ratios in counted GAE at w = 0, 0.1, 1, the recorded-S ratios, the `bitop/v1` ratios and their intervals). They agree with `ecbench band` and `ecbench compare` output to at most 2.6e-15 relative difference.
- **Band orders:** the validator's S reproduces every printed order, including `sib8-p256` below `tw2l` at every weight and size, and `sib8-p512` unresolved against `tw2l` at P24.

## Rows
| row | arm | gate result | against matched rho-neg |
|---|---|---|---|
| Table B round 3 | `sib8-p256` | passes | order holds only at P20 (rho lower); at P22 and P24 the band order is unresolved, ratio 0.93 to 0.96 at w = 0 and 1.6 to 1.7 at w = 1 |
| Table B round 3 | `sib8-p512` | passes | rho lower at every weight and size |
| Table B baseline | `tw2l` | passes | rho lower |
| Ablation (addendum A) | `tw2l-sib`, `tw2l-sg`, `sib8-p256` | passes as supporting evidence | as in the table above |

`VALIDATION.json` carries the `sib8-p256` P24 band, where `order_holds_across_band` is false: the gate accepts this, and a row may not be ranked against rho there. The bands for every arm and curve are in `rho-references.json`.

## README discrepancies (rounding only)
- `P22 sib8-p256 / rho-neg bitop` is printed 1.088; the value is 1.0875.
- `P24 sib8-p256 / tw2l w = 1` is printed 0.932; the value is 0.93150 (0.9314999562).
- The ablation README prints 0.817 and 0.672 for values 0.8165 and 0.6715.
- The first table's w = 0 column is the recorded-S ratio, not the band w = 0, as in round 2. Both are reproduced.

## Named gaps
Same as round 2: same binary and host, no pcc claim or keyring signature, wall-time columns (end-to-end wall / tw2l) not reproduced, `bitop/v1` per-run prices taken from the records, no L2 admission.
