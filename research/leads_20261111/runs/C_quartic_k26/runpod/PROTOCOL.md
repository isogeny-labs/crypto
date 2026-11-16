# Frozen protocol: LEAD-C-quartic-n4v2-k26, clean run on one dedicated RunPod pod

Written 2026-11-12 before any pod data was taken. Not edited after the first record arrived.

## Fixed inputs (not moved)

* Lead, prediction and kill rule: `research/leads_20261111/lane_C_prime_ext_cover.md`, Lead 4, verbatim:
  * Kill if the DL exponent over k = 22..26 has its lower 95% bound above 1.10, or DL / rho-neg at
    k = 26 has its upper bound above 0.01, or any unverified logarithm.
  * Prediction: exponent k = 17..26 of 1.00 [0.95, 1.06] (M-eq, w = 0); DL / rho-neg at k = 26 of
    0.00085 [0.0006, 0.0014]; DL wall at k = 26 about 190-400 s.
* Binary: one binary for every k and every calibration, `v2w2`, built with
  `rustc --edition 2021 -O src/v2w.rs` from `research/leads_20261111/runs/C_quartic_k26/src`
  (Git commit 1a0957d6c0 for these sources), sha256
  `cf55da044d09afae946ca09b5057663f7f6e8e9237e13779feed090d37e4245e` (bit-identical to the
  `bin/v2w2` of the local run).
* Sizes: k = 22..26, q = 4194271, 8388571, 16777183, 33554383, 67108819; seeds 1, 2, 3.
* Unit command (the committed grid's): `V2W_COMPACT=1 v2w2 ic <q> <seed> D 3.0 2.0 4 4`, run
  through `runjob.py` (wait4 CPU, peak RSS), `RAYON_NUM_THREADS=1`, timeout 3600 s, no RSS cap.
* Calibration procedure (frozen, the same as the local run): `v2w2 calib <q> c` for c = 1, 2, 3; the
  per-size constants (r_inv, r_lmul) are the medians of the 3 records. One "calibration round" is
  this procedure for all five sizes.

## Host and schedule

* One RunPod CPU pod, flavor cpu5c, 16 vCPU, named `bdr-quartic-k26-clean`, running nothing but
  this script. Every job runs strictly sequentially (one process at a time).
* Order (interleaved): CAL1; seed 1 units k = 22, 23, 24, 25, 26; CAL2; seed 2 units k = 26, 25,
  24, 23, 22; CAL3; seed 3 units k = 22..26; CAL4. Within calibration round r the size order is
  rotated by r. Four calibration rounds (at least 3 required).
* After the main sequence, and only then, optional measured negation-rho runs may be taken in
  parallel on the same pod. They are a cross-check of the rho model, not part of the decision.

## Analysis (frozen)

* Cost per unit: DL arm M-eq = sum over phases of mul + r_inv inv + r_lmul lmul (`analyze.meq_unit`),
  all attempts charged; w = 0.1, 1 add w * GAE * (hash + cmp); GAE = gae_mul + r_inv gae_inv.
* A unit counts only if `ok` and `k_found == k`.
* Exponent: OLS of ln(mean cost per size) on ln q over k = 22..26 (and k = 22..25, k = 17..26 with
  the local k = 17..21 records only as a secondary row, labelled mixed-host).
* Point estimate: per-size constants = median of all 12 calibration records of that size.
* Per-round estimates: e_r with round r's medians, r = 1..4; reported as a range.
* Primary 95% interval (decision interval): two-level bootstrap, B = 4000, seed 7: units resampled
  within size, and for every size independently one calibration round drawn uniformly (its median
  of 3). This includes calibration noise of the frozen procedure.
* Kill decision on the exponent: killed iff the lower end of the primary interval is above 1.10.
* DL / rho-neg: the calibrated rho-neg model of `analyze.py` section 4 (committed pooled
  steps/sqrt(ell) 0.9175, priced work per step and unpriced ops per step from the committed k = 22
  supplement, this pod's GAE), band = calibration constant over [0.75, 1.15] x seed range x
  calibration-round range. w = 0, 0.1, 1. Kill iff the k = 26 upper bound at w = 0 exceeds 0.01
  (reported at all three w).
* Identity control: non-timing fields of the pod's k = 22..25 records must equal the local records
  (`raw/wide`, `raw/v2w2/k25`), using `compare.py`'s timing-field rule.

## Addendum A (written 2026-11-12 ~18:15Z, while the main sequence ran, before any pod record was read)

* After `MAIN DONE`, the same pod runs `pod_low.sh` sequentially: k = 17..21 (q = 131071, 262111,
  524287, 1048507, 2097091), the same unit command and the same interleaved pattern (CAL1, seed 1
  ascending, CAL2, seed 2 descending, CAL3, seed 3 ascending, CAL4) with the same calibration
  procedure. This gives the prediction row k = 17..26 on one host and one binary. It does not enter
  the kill decision, which is on k = 22..26 only.
* Then, in parallel (16 processes), measured negation-rho: `v2w2 rho <q> 1 <i> <i+1> neg` for
  i = 0..7 at k = 22 and k = 23. Used only to check the model's pooled steps/sqrt(ell) constant
  (0.9175, admissibility range [0.75, 1.15]); the decision uses the model as frozen. If the measured
  pooled constant falls outside [0.75, 1.15], that is reported, not used to re-fit.
