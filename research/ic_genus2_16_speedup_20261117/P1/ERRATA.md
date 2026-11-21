# P1/P2 errata (from the independent validation, 2026-11-20)

Source: `../VALIDATION_P1P2.md`, written by an independent validator subagent using its own code
(`../validation/`). Overall verdict: **validated-with-errata**.

From now on, errata live here. Validated files are no longer edited in place (adopted lesson
L-8d86e8fc352c). E1-E7 were first folded into `RESULTS.md`, `DESIGN.md` and
`../P2/RESULTS_crossover.md` in commit `eddade9186`. That broke the validated hash of
`RESULTS.md`; this file is now the record.

## Certificates: now verified

- **Re-runs.** Every instance behind the P1 and P2-crossover tables was re-run with binary g16f
  (sha256 `573993fd…`, source patch `../P4_followup/certs/instrument_g16f.patch`), which emits G,
  T and the recovered t.
- **Validator's own check.** For all 236 solved records, [t]G = T, [r]G = 0 with r prime, and
  G != 0. The arms agree on t for each target. All 165 tampered negative controls are rejected.
- **Counted fields** are identical to the committed records: 8448 fields, 0 differences.
- **Limit.** The 30 arm-F solves (`../P2/RESULTS_F.md`, binary g16e) remain flag-only.
- **Limit.** A planted log cannot rule out a solver that read the planted t.

## Errata

| Id | Severity | What changes |
|---|---|---|
| E1 | minor | FPM_1 excludes additions and DSU work (`rel_add`, `la_add`); `RESULTS.md` had said DSU work is charged. |
| E2 | minor | §5a band (0, 0.1, 1 per unpriced op): every arm-vs-rho ordering holds. One ranking does not: at g=6 p=211 the best arm over `c` goes 1.29x, 1.24x, 0.995x, so that cell is a **tie**, not a gain. |
| E3 | superseded by E3b | g=10 p=11 seed 27 was admitted but not run. |
| **E3b** | **material** | With curve 27 included (6 pairs, curves 7/12/27), the g=10 p=11 row is **D/rho 1.03** (not 0.725), **gain over `c` >= 118x** (not >= 201x), and E/rho >= 52.5. On curve 27, D is at 1.93x and 2.21x of rho and E exhausts. The statement "at g=10 p=11, D solves at 0.73x of rho" is withdrawn: index calculus with the degree-2 base is at rho there, not below it. |
| E4 | minor | `order` is an exponent multiple in 14 of 32 instances (shared-r rule). r is prime and exactly divides #J in all 32. |
| E5 | minor | Walk steps per relation at g=6 are about 900-1700. p=211 is about 900. |
| E6 | minor | The quoted slopes are two-point endpoint slopes. The g=7 slope from p=53 to 101 is -0.554. |
| E7 | minor | The P2 crossover cells used a 2e7 trial cap, against 5e6 in P1. This is immaterial at g=7 p=101. |
| E8 | minor | Within a cell, the two seeds per curve share one target (`target_seed` = `curve_seed`). A "pair" is two walks on one target, not two targets. |

## Corrected P1 table row

| g | p | pairs | lab `c` / rho | D / rho | E / rho | best over `c` |
|---|---|---|---|---|---|---|
| 10 | 11 | 6 (curves 7, 12, 27) | >= 121 | **1.03** | >= 52.5 | **>= 118x (D)** |

The other rows of `RESULTS.md` stand as re-derived by the validator, within 0.38%.
