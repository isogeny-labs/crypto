# PROTOCOL — measuring the IC / matched-rho crossover at q < 2^17 (frozen 2026-11-21, before any v9 data)

Follows `slope_20261120`. There the best IC arm was at 3.89, 1.97, 2.00 and 1.37 of the SIMD rho's
expected time at q = 16001…65521. The slopes were 1.21 for IC and 2.00 for rho, and the
extrapolated crossover was q* ≈ 92 000. Workflow `rho-audit-and-q17-resume` (run `wf_d21df8b5-954`)
widened the crate to q < 2^17. An independent verifier found no defect:
- all 75 non-timing fields are byte-identical to v8 at q < 2^16;
- `cargo test --release` gives 42 passed, 1 ignored;
- the r < 2^62 rule is kept. Above 2^16 it means h ≥ q^4/2^62. It changed the instance for 13 of
  the verifier's 20 large-q seeds and lowers r, and so the rho time, more at larger q. Comparisons
  are therefore made per instance (IC and rho on the same instance, same r).

Binary: `bin/humbert-v9`, sha256 `26f1620a25510d27e61e261ffc9cbadd885bf9b76a3b4ca14b74418cd73115f2`.

**Disclosed before freezing.** One smoke pair ran at q = 131071, seed 1 (dlpf:0.35 with all IC
flags; SIMD rho): t_ic 26.4 s (LA 14.7 s, relations 10.6 s) against a rho expected time of 12.8 s,
a ratio of about 2.06. It is not part of the data. It is why the smaller bases d20 and d25 are arms.

## Design

- Instances: q ∈ {65521, 81001, 98009, 115001, 131071} × seeds 1–5 (twist +1 for odd seeds, −1
  for even). 65521 is the overlap with v8.
- IC arms, all with `HUMBERT_LA_PRECHECK=1 HUMBERT_LA_SOLVER=l1 HUMBERT_SKIP_RHO=1 HUMBERT_REL_FAST=1 HUMBERT_LA_THREADS=4 HUMBERT_SETUP_FAST=1 HUMBERT_LA_FASTMOD=1`:
  - `d20`, `d25`, `d35`, `d50`: dlpf at alpha 0.2, 0.25, 0.35 and 0.5;
  - `d35_aa`: an A/A repeat of d35.
- Rho arm `rho`: the SIMD matched rho (4 threads, batch 256, FASTMUL, negation map) with
  `HUMBERT_SETUP_FAST=1` (same instance) and `HUMBERT_RHO_CAP=2e10`. The default 2e9 cap cut off a
  verifier run at q = 115001.
- Order: arm i = (seed + k) mod 6 at position k. Serial runs, one q per launch.
- Host: the same 4-vCPU shared VM (L0). Wall times are exploratory and unverified assertions.
- Rho reference: (rho_expected_steps_negation + extra_group_ops) / measured steps per second on the
  same instance, with the actual solve time reported too, as in v7 and v8.

## Predictions (frozen)

- **C0 (counted).** At q = 65521, d35 and d50 equal the v8 `best8` and `d50_8` counted fields on 5/5
  instances, wherever the r < 2^62 rule leaves the instance unchanged. Any instance it changed is
  listed. d35_aa equals d35 on every instance.
- **X (the tested hypothesis, from the slope_20261120 extrapolation).** At q = 131071 the best IC
  arm's median per-instance ratio t_ic / rho expected time is below 1. The smoke pair above
  predicts the opposite. Both outcomes are reported, with the per-q medians and their ranges.
- **Y (two-sided).** Per q, which alpha is fastest, and the log–log slope of the best arm's median
  t_ic over q ∈ [65521, 131071] against the rho expected-time slope.
- **Z (counted).** Every run solves (IC: solved and e2e_ok; rho: rho_solved, abandoned_walks = 0),
  with no rho run at the cap.
- **§5a.** Same-host paired wall time only. The band is reported as before.
