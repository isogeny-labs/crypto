# PROTOCOL — slope test against a stronger matched rho (frozen 2026-11-20, before any v8 data)

Follows `next_leads_20261120`. In that study the best IC arm was at 1.68, 1.19 and 0.77 of the
expected time of a 4-thread rho at q = 16001, 24001 and 32003. Three levers were then built and
checked by independent verifier agents (workflow `humbert-slope-levers`, run `wf_6b058b58-659`);
all were accepted:

| lever | flag | contract |
|:--|:--|:--|
| wide | none: the crate now accepts q < 2^16 | every q < 2^15 code path unchanged (4-run all-field contract check); BEST solved 25/25 at q ∈ {40009, 48017, 56003, 65003, 65521} |
| rho | `HUMBERT_RHO_SIMD=1` (with `HUMBERT_RHO_BATCH=256`) | lane-parallel (AVX-512/AVX2/portable) K4 kernel, same r-adding walk with negation map, FASTMUL valid for q < 2^16; same log and step semantics |
| la | `HUMBERT_LA_FASTMOD=1` | Montgomery mod-r arithmetic with lazy accumulation in the serial and threaded Lanczos/Wiedemann; identical x, la_mulmods and every non-timing field |

`main.rs` now reports `rho_par.batch` and `rho_par.fastmul` from what actually ran (batch_used,
fast_used) and adds `rho_par.simd`. These are reporting changes only.

Binary: `bin/humbert-v8`, sha256 `08d43a4e223c2be3a74da51401eeb810dc259dcc23e38dfaf716d26086c543d7`.
`cargo test --release`: 31 passed, 1 ignored.

## Design

- Instances: q ∈ {16001, 32003, 48017, 65521} × seeds 1–5 (twist +1 for odd seeds, −1 for even).
  This spans a factor of 4.1 in q, against 2 in the previous study.
- IC arms, all with `HUMBERT_LA_PRECHECK=1 HUMBERT_LA_SOLVER=l1 HUMBERT_SKIP_RHO=1 HUMBERT_REL_FAST=1 HUMBERT_LA_THREADS=4 HUMBERT_SETUP_FAST=1`:
  - `best7`: dlpf:0.35, the v7 best arm.
  - `best8`: best7 + LA_FASTMOD.
  - `best8_aa`: A/A of best8.
  - `d25_8`: dlpf:0.25 + FASTMOD.
  - `d50_8`: dlpf:0.5 + FASTMOD.
  - `lp50_8`: lp:0.5 + FASTMOD.
- Rho arms, rho only on the same instance:
  - `rho_m7`: the v7 matched rho (4 threads, batch 64, FASTMUL, negation map).
  - `rho_m8`: rho_m7 with SIMD and batch 256.
- Order: arm i = (seed + k) mod 8 at position k. Serial runs, one q per launch (`run_v8.sh`).
- Host: the same 4-vCPU shared VM as v7, L0. Wall times are exploratory and unverified
  assertions. `load1` is logged.
- Rho reference time: as in v7, the primary reference is (rho_expected_steps_negation +
  extra_group_ops) / measured steps per second. The actual solve time is reported alongside.

## Predictions (frozen)

- **C0 (counted).** best7 equals the v7 `d35` counted fields (rel_candidates, relations, la_cols,
  la_rows, la_nnz, la_mulmods, x_true, x_recovered, E_order, r, D_order) on 10/10 instances at
  q ∈ {16001, 32003}. best8 and best8_aa equal best7 on every instance.
- **L2 (wall).** Median paired t_LA best8/best7 ≤ 0.40 at every q.
- **M3 (throughput).** Median paired rho_m8/rho_m7 steps per second ≥ 2.5 at every q.
- **M4 (counted).** Every rho run is solved with abandoned_walks = 0. Report steps_over_expected.
- **G (distance remaining).** Per q, the best IC arm's median per-instance t_ic over rho_m8's
  expected time. No direction is predicted, but one direction is predicted for the slopes: the
  log–log slope of the best IC arm's median t_ic over q ∈ [16001, 65521] is below the slope of
  rho_m8's median expected time. A crossover q* is extrapolated from the two fits and labelled
  exploratory.
- **F3 (wall, two-sided).** d25_8, d50_8 and lp50_8 against best8 per q: does the factor-base
  optimum move with cheap LA?
- **§5a.** Ranking on paired same-host wall time only. The 0 / 0.1 / 1 GAE band is reported for
  IC native mulmods against rho group operations, with no counted-unit ranking where it flips.

Every run must solve. Failures are reported, never dropped. A killed launch is resumed with
`SEEDS=` and recorded in a dated addendum.
