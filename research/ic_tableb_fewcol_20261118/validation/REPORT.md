# Validator report: Table B round 2 (STACK and FEWCOL), 2026-11-19

Independent validation of `research/ic_tableb_fewcol_20261118` for the validation gate. Every statement here is `[pcc:unverified]`. The machine-readable files are `VALIDATION.json`, `replay-certificate.json`, `validator-report.json` and `rho-references.json`. `python3 scripts/validation_gate.py evidence research/ic_tableb_fewcol_20261118/validation` says `valid`.

## What was checked
- **Replay:** `ecbench verify --replay-all` on all three sessions. 720 of 720 records verified, and all 480 candidate runs per session reproduced identically. The receipts are `replay-receipt-P*.json`.
- **Scalars:** a separate Python implementation of prime-field curve arithmetic confirmed `[recovered]G == target` for every record (48 distinct targets per curve, no failures).
- **Recompute:** `recompute.py` prices every measured run from `records.jsonl` and recomputes 135 headline numbers (S per arm at w = 0, 0.1, 1; every paired ratio; the intervals). They agree with the full-precision output of `ecbench band` and `ecbench compare` to a relative difference of at most 2.4e-15. The gate's tolerance is 1e-9.
- **Band orders:** the validator's own S reproduces every order `ecbench band` printed and every order the README states (STACK and FEWCOL below the k-sum baseline at every weight and size; `rho-neg` below `tw2l` at every weight and size).

## Rows
| row | arm | gate result | distance to matched rho (w = 0 / 1) |
|---|---|---|---|
| Table B incumbent update | `tw2l` | passes (no band order above rho: rho is lower at every weight) | 2.14 / 2.03 at P24 |
| Table B STACK | `ksum5-ps` | passes | 4.68 / 6.98 at P24 |

## README discrepancies (none changes a result)
- The README column headed w = 0 in the first table is the recorded-S ratio of `ecbench compare` (for example 0.475 at P20), not the band's w = 0 (0.481). The columns for w = 0.1 and w = 1, and the S tables, are band values. The labels mix two readings of w = 0. Both are reproduced.
- `P24 S ksum5-ps w = 1` is printed as 13.49. The value is 13.4847, which rounds to 13.48 (double rounding from 13.485).
- The "before" distances "4.6 to 5.3 times (w = 0)" and "6.3 to 6.7 times (w = 1)" for `ksum5-sib` against `rho-neg` do not come from this session. This session's own values are 4.5 to 5.1 at w = 0 (band) and 5.7 to 7.2 at w = 1. They probably quote the earlier sibling study with other seeds. The conclusion (tw2l at 1.8 to 2.2 times) is unaffected.
- The `ksum5-ps` distance "4.0 to 4.7 times" holds for the band at w = 0 (4.04 to 4.68); read as recorded S it is 4.10 to 4.77.
- The adds in the README (2,424 to 1,738 and so on) are the `oracle_setup` phase adds, which the validator confirmed exactly (all 288 paired runs made identical trials and relations).

## Named gaps
See `GAPS` in the certificate: same binary and host (no second host, no rebuild), no pcc claim or keyring signature, wall-time columns not reproduced, no L2 admission. These are recorded, not repaired.
