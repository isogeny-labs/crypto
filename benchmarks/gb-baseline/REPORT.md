# Gröbner engine baseline panel

Regenerated automatically by `.github/workflows/gb-baseline-rebench.yml` (`tools/gb_baseline.py`) whenever the F4 / Gröbner engine code, the harnesses or `panel.json` change. Do not edit by hand.

- Commit: `ef5eda5e665219a6be187e6152a6f0d38c1db5db`  
- Run: https://github.com/isogeny-labs/crypto/actions/runs/46010462955  
- Host: Intel(R) Xeon(R) 6973P-C (4 CPUs), rustc 1.100.0 (0ef1bc386 2026-11-09)  
- Generated: 2026-11-27T14:03:08Z; per-cell timeout 600 s

## Panel

- **N31** — Koblitz K_0/2^31 (binary, Frobenius-invariant factor base 0); harness `examples/groebner_stage_bench.rs`, engine koblitz_groebner (matrix-F4 over GF(2), Weil-restricted Semaev)
- **M31** — p = 2^31 - 1 (Mersenne prime), Dickson torus tower (2^t | p + 1); harness `examples/pkm_tower_pilot.rs`, engine f4_tower_generic (F4 over F_p)
- m values: [2, 3, 4, 5, 6, 7, 8, 9]

## Cells

| curve | m | status | counted cost | unit | answer digest | wall s | vs baseline |
|:--|--:|:--|--:|:--|:--|--:|:--|
| M31 | 2 | ok | 5620 | multiply-adds (F_p elimination) | `7695ff52fe1defda` | 0.00216 | unchanged |
| M31 | 3 | ok | 1.845e+06 | multiply-adds (F_p elimination) | `8d038345ea40a3fa` | 0.0188 | unchanged |
| M31 | 4 | ok | 2.143e+08 | multiply-adds (F_p elimination) | `f7d48b0b09178c89` | 0.196 | unchanged |
| M31 | 5 | error | — | — | `—` | — | still error |
| M31 | 6 | error | — | — | `—` | — | still error |
| M31 | 7 | error | — | — | `—` | — | still error |
| M31 | 8 | error | — | — | `—` | — | still error |
| M31 | 9 | error | — | — | `—` | — | still error |
| N31 | 2 | ok | 796 | word XORs (GF(2) Macaulay elimination) | `10055e840b7b42c7` | 0.000559 | unchanged |
| N31 | 3 | ok | 7.039e+07 | word XORs (GF(2) Macaulay elimination) | `10055e840b7b42c7` | 0.168 | unchanged |
| N31 | 4 | ok | 0 | word XORs (GF(2) Macaulay elimination) | `20b0fa11cd19564e` | 2.07e-05 | cost unit changed |
| N31 | 5 | ok | 0 | word XORs (GF(2) Macaulay elimination) | `20b0fa11cd19564e` | 2.03e-05 | cost unit changed |
| N31 | 6 | ok | 0 | word XORs (GF(2) Macaulay elimination) | `20b0fa11cd19564e` | 1.96e-05 | cost unit changed |
| N31 | 7 | ok | 0 | word XORs (GF(2) Macaulay elimination) | `20b0fa11cd19564e` | 2.19e-05 | cost unit changed |
| N31 | 8 | ok | 0 | word XORs (GF(2) Macaulay elimination) | `20b0fa11cd19564e` | 1.82e-05 | cost unit changed |
| N31 | 9 | ok | 0 | word XORs (GF(2) Macaulay elimination) | `20b0fa11cd19564e` | 1.84e-05 | cost unit changed |

![counted cost vs m](cost_vs_m.png)

![change vs baseline](delta_vs_baseline.png)

![wall time](wall_vs_m.png)

## Reading this

Counted cost is the harness's own unit and is what decides faster/slower; wall time is descriptive (shared host). `ANSWER CHANGED` means the solver decided a target differently from the baseline — a correctness question before any speed one. `timeout` cells are censored, `skipped` cells are inadmissible for that m. Stage diagnostics of the solver axis only: no end-to-end ECDLP cost, no `S`, no leaderboard row; every number is `[pcc:unverified]`.
