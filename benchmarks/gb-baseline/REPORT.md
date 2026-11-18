# Gröbner engine baseline panel

Regenerated automatically by `.github/workflows/gb-baseline-rebench.yml` (`tools/gb_baseline.py`) whenever the F4 / Gröbner engine code, the harnesses or `panel.json` change. Do not edit by hand.

- Commit: `224e39ad0a09a55364c68438b240a8fe2aa17089`  
- Run: https://github.com/isogeny-labs/crypto/actions/runs/44235614366  
- Host: AMD EPYC 7763 64-Core Processor (4 CPUs), rustc 1.100.0 (0ef1bc386 2026-11-09)  
- Generated: 2026-11-18T05:28:09Z; per-cell timeout 600 s

## Panel

- **N31** — Koblitz K_0/2^31 (binary, Frobenius-invariant factor base 0); harness `examples/groebner_stage_bench.rs`, engine koblitz_groebner (matrix-F4 over GF(2), Weil-restricted Semaev)
- **M31** — p = 2^31 - 1 (Mersenne prime), Dickson torus tower (2^t | p + 1); harness `examples/pkm_tower_pilot.rs`, engine f4_tower_generic (F4 over F_p)
- m values: [2, 3, 4, 5, 6, 7, 8, 9]

## Cells

| curve | m | status | counted cost | unit | answer digest | wall s | vs baseline |
|:--|--:|:--|--:|:--|:--|--:|:--|
| M31 | 2 | ok | 5620 | multiply-adds (F_p elimination) | `7695ff52fe1defda` | 0.00228 | unchanged |
| M31 | 3 | ok | 1.845e+06 | multiply-adds (F_p elimination) | `8d038345ea40a3fa` | 0.0194 | unchanged |
| M31 | 4 | ok | 2.143e+08 | multiply-adds (F_p elimination) | `f7d48b0b09178c89` | 0.22 | unchanged |
| M31 | 5 | timeout | — | — | `—` | — | still timeout |
| M31 | 6 | timeout | — | — | `—` | — | still timeout |
| M31 | 7 | timeout | — | — | `—` | — | still timeout |
| M31 | 8 | timeout | — | — | `—` | — | still timeout |
| M31 | 9 | timeout | — | — | `—` | — | still timeout |
| N31 | 2 | ok | 796 | word XORs (GF(2) Macaulay elimination) | `10055e840b7b42c7` | 0.000635 | unchanged |
| N31 | 3 | ok | 7.039e+07 | word XORs (GF(2) Macaulay elimination) | `10055e840b7b42c7` | 0.187 | unchanged |
| N31 | 4 | ok | 0 | word XORs (GF(2) Macaulay elimination) | `20b0fa11cd19564e` | 2.18e-05 | cost unit changed |
| N31 | 5 | ok | 0 | word XORs (GF(2) Macaulay elimination) | `20b0fa11cd19564e` | 2.66e-05 | cost unit changed |
| N31 | 6 | ok | 0 | word XORs (GF(2) Macaulay elimination) | `20b0fa11cd19564e` | 4.77e-05 | cost unit changed |
| N31 | 7 | ok | 0 | word XORs (GF(2) Macaulay elimination) | `20b0fa11cd19564e` | 2.1e-05 | cost unit changed |
| N31 | 8 | ok | 0 | word XORs (GF(2) Macaulay elimination) | `20b0fa11cd19564e` | 2.3e-05 | cost unit changed |
| N31 | 9 | ok | 0 | word XORs (GF(2) Macaulay elimination) | `20b0fa11cd19564e` | 2.29e-05 | cost unit changed |

![counted cost vs m](cost_vs_m.png)

![change vs baseline](delta_vs_baseline.png)

![wall time](wall_vs_m.png)

## Reading this

Counted cost is the harness's own unit and is what decides faster/slower; wall time is descriptive (shared host). `ANSWER CHANGED` means the solver decided a target differently from the baseline — a correctness question before any speed one. `timeout` cells are censored, `skipped` cells are inadmissible for that m. Stage diagnostics of the solver axis only: no end-to-end ECDLP cost, no `S`, no leaderboard row; every number is `[pcc:unverified]`.
