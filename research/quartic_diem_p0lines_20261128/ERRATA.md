# Errata: research/quartic_diem_p0lines_20261128

Found by the independent validator on c271f320e40 (`validation/validator-report.json`; verdict validated with errata, gate `valid`). RESULTS.md and the report are left as written; cite the corrected values below. Every number is `[pcc:unverified]`.

## Errata

- **E1. Double rounding.** Locations: `RESULTS.md:89`, `reports/run-20261128T0345Z/report.typ:49` (and `report.pdf`).
  - **Printed:** the p0 IC / ideal negation rho at p = 131101, w = 1, as 1.91 [1.82, **2.00**].
  - **Correct:** the upper end is 1.9945176. Cite **1.91 [1.82, 1.99]**.
  - **Verdict:** unchanged.
- **E2. "The share of unrescued failures grows with p" is overstated.** Locations: `RESULTS.md:76`, `report.typ:43`.
  - **Counts:** 1/9, 4/7 and 5/9 unrescued at 16411, 65537 and 131101.
  - **Pattern:** the share is higher at the two larger primes than at 16411, but it does not grow from 65537 (57%) to 131101 (56%).
  - **Sample size:** n ≤ 9 per prime.
- **E3. Imprecise summary** (minor). Location: `report.typ:40`.
  - **Printed:** "unrescued failures (10): about 1.0".
  - **Measured:** the geometric mean over the 10 pairs is 0.97. Per prime it is 1.002, 1.004 and 0.933 (16411, 65537, 131101). Single pairs range from 0.75 to 1.06.

Every other displayed number matches on a round-once check: 175 numbers, 2 errors, both E1.

## Mechanism addendum (validator's replay of the uncounted diagnostic on grid curves)

- **Diagnostic does not disturb runs.** `--quartic-diag` changes no recorded field (156 of 156 rows).
- **The base-point shift is gone.** It is not free in any of the 89 `p0` attempt cores, which supports the claim that the P0 lines remove that cause.
- **Causes of the 11 failed `p0` attempts:**
  - **9 attempts** have columns that occur only in decomposition rows. These are the frontier columns that RESULTS names, all at p = 65537 and 131101.
  - **2 attempts** have a third cause that RESULTS does not name: only one or two decomposition rows reach the core (`ab_rank` = 1). They are 4099 curve 205 seed 6, and 16411 curve 158 seed 4 (the unrescued 16411 pair).

## Process findings

- **P1. Late hash commit.** `binary.sha256` and `source.sha256` were committed 15 minutes after the run started. The binary itself is bound to the freeze commit 7f67f384ece by cargo's build records: the compile started 22 s after the freeze, and 1149 of the 1188 sources it read are byte-equal to the freeze. This is mtime evidence, weaker than a rebuild.
- **P2. Main merged during the run.** The worktree merged main 3 s after `run_main.sh` started, and again at 04:00:32. The running binary and the frozen scripts were unaffected.
- **P3. Oversized raw files.** The raw `.jsonl` files (553 KB and 193 KB) are over the 64 KiB data limit. They could not be moved to the artifact store, because both the repository's and this session's AWS keys are rejected.
- **P4. Silent missing keys in `analyze.py`.** It reads the C0 fields and `p0_lines` with `.get`, so a missing key would pass silently, and it does not check completeness. No effect here: the validator read every key with `[]`, and 86 of 86 shared keys match.
- **P5. Incomplete seed manifest.** `seeds.json` omits three streams: the descent RNG, the sampled-factor-base seeds, and the BSGS order seed (inherited from the parent).
- **P6. Calibrated weight not recorded.** The §5a calibrated weight is not recorded. It cannot change this verdict.
- **P7. Mechanism replay.** The mechanism addendum above comes from a replay of the uncounted diagnostic.
- **P8. Branch moved during validation.** The branch moved during validation, so this validation covers c271f320e40 only.
- **P9. Local host instead of EKS.** The runs used the local host instead of EKS, as PROTOCOL.md §7 discloses.
