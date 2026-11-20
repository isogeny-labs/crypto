# PROTOCOL — next leads after levers_20261116 (frozen 2026-11-20, before any v7 data)

Follows `levers_20261116`. Its incumbent `comb` is dlpf:0.5 with `HUMBERT_REL_FAST=1` and
`HUMBERT_LA_THREADS=4`. Three leads were built and checked by independent verifier agents
(workflow `humbert-next-leads`, run `wf_51f8d293-995`), all accepted:

| lead | flag(s) | contract |
|:--|:--|:--|
| setup | `HUMBERT_SETUP_FAST=1` | same RNG stream, same instance, every non-timing field byte-identical |
| factor base | none (arm parameter alpha) | counted sweep (60 runs, 1 LA thread) and a fitted cost model. It predicts the dlpf optimum at alpha ≈ 0.3–0.35 with 4 LA threads (6–17 % below alpha = 0.5), and lp's optimum near 0.4–0.5 |
| matched rho | `HUMBERT_RHO_ONLY=1 HUMBERT_RHO_THREADS=4 HUMBERT_RHO_BATCH=64 HUMBERT_RHO_FASTMUL=1 HUMBERT_RHO_NEG=1` | parallel distinguished-point rho with 64 batched affine walks per thread (one shared inversion per batch), Barrett K4 arithmetic and the negation map. Every step and group operation is counted |

The verifier's rho findings are reporting-only: the `fastmul` field reports the flag, there is a
1024 bound on the fruitless-cycle detector, and up to threads·63·batch post-solve steps are
counted. None affects a row here (q < 2^15, abandoned_walks = 0 in every verifier run).

Binary: `bin/humbert-v7`, sha256 `a4b55b7191c88f1f2212e2f4f7461b078c90a57a402cf6de3c0020b709af33d8`
(release build of the crate with the setup and rho levers merged; `cargo test --release`: 19 passed).

## Design

- Instances: q ∈ {16001, 24001, 32003} × seeds 1–5 (twist +1 for odd seeds, −1 for even). These are the v5/v6 instances.
- IC arms (all `HUMBERT_LA_PRECHECK=1 HUMBERT_LA_SOLVER=l1 HUMBERT_SKIP_RHO=1 HUMBERT_REL_FAST=1 HUMBERT_LA_THREADS=4`):
  - `comb6`: dlpf:0.5, the v6 incumbent.
  - `comb7`: the same plus `HUMBERT_SETUP_FAST=1`.
  - `comb7_aa`: an exact repeat of comb7 (A/A).
  - `d25`, `d30`, `d35`: dlpf:0.25, 0.3, 0.35 with SETUP_FAST.
  - `lp40`, `lp50`: lp:0.4 and lp:0.5 with SETUP_FAST.
- Rho arms (rho only, same instance):
  - `rho_m`: the matched rho above.
  - `rho_s`: the crate's scalar rho, the reference used in v5 and v6.
- Order: arm i = (seed + k) mod 10 at position k. Serial runs, one q per launch (`run_v7.sh`).
- Host: a 4-vCPU shared VM (a different container from v6), L0. Wall times are exploratory and
  unverified assertions. `load1` is logged.

## Rho reference time

The rho solve time is a random variable, so one solve per instance is a noisy reference. The
primary reference is the expected time: rho_expected_steps_negation (rho_m) or rho_expected_steps
(rho_s), divided by that run's measured steps per second. The actual solve time is reported
alongside.

## Predictions (frozen)

- **C0 (counted).** comb6 reproduces the v6 `comb` counted fields (rel_candidates, relations, la_cols,
  la_rows, la_nnz, la_mulmods, x_true, x_recovered, E_order, r, D_order) on 15/15 instances. comb7 and
  comb7_aa equal comb6 on 15/15.
- **S1 (wall).** Median paired t_setup comb7/comb6 ≤ 0.20 at every q.
- **S2 (wall).** Median paired t_ic comb7/comb6 < 1 at every q, by more than the comb7_aa/comb7 range.
- **F1 (wall).** At q = 32003 the best of d25/d30/d35 has a median paired t_ic ratio to comb7 below 1.
  Two-sided reporting for every alpha and q.
- **F2 (wall, two-sided).** lp40/lp50 and lp50/comb7 per q.
- **M1 (throughput).** rho_m steps per second / rho_s steps per second ≥ 8 at every q.
- **M2 (counted).** Every rho_m run is solved, with abandoned_walks = 0. Report steps_over_expected.
- **G (distance remaining, no direction predicted).** The best IC arm's median t_ic over rho_m's
  expected time, per q. This is the gap to the matched rho. The gap to the scalar rho is reported as
  before, for continuity only.
- **§5a.** IC does no group operations; rho does no field operations outside its group operations.
  Arms are ranked on paired wall time on one host. The 0 / 0.1 / 1 GAE band applied to IC's native
  mulmods is reported, and no ranking is made in counted units where the order flips across the band.

Every run must solve: e2e_ok for IC, rho_solved for rho. Failures are reported, never dropped. A
killed launch is resumed with `SEEDS=` and recorded in a dated addendum.
