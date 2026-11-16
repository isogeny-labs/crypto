# VERIFY: independent skeptic check of LEAD-C-quartic-n4v2-k26

2026-11-12. Checker: independent skeptic subagent. I edited only this file. Every statement here is
**[pcc:unverified]**. My scratch outputs are in
`/tmp/claude-0/-home-user/5d26ea47-4c79-5b59-bd9c-e47e6411a571/scratchpad/vq/` and are not committed.

## Bottom line

The engineering and the records hold up. The counts reproduce bit for bit, and the analysis output
and the binary hashes match what the producer reported. The weak point is the verdict on the
partial-range falsifier.

RESULT.md says that on k = 22..25 the lower bound is 1.0993, "0.0007 below the kill line, not
triggered". That depends on which calibration run it used:

* I re-ran the frozen calibration rule four times: the median of 3 `calib` records per size, with
  the same binaries and the same unit records.
* The DL exponent came out at 1.1001 to 1.1034 each time, with the bootstrap lower bound at
  1.1000 to 1.1034. All four are at or above the 1.10 kill line.
* The producer's 1.0993 is the lowest value seen across five calibrations.
* The quoted interval [1.0993, 1.0995] covers unit resampling only. Calibration noise is about
  10x wider than that interval.

The formal falsifier (k = 22..26) is still not evaluable without k = 26, so the label "partial" is
fair. But "not triggered on the partial range" should read "indeterminate". Under re-calibration
the partial range sits on or above the kill line.

## What I re-ran

| check | command (from the output dir, `RAYON_NUM_THREADS=1`) | result |
|:--|:--|:--|
| binary hashes | `sha256sum bin/* src/*.rs src_v1/*.rs ../../../quartic_n4v2_20261020/v2.rs` | match `raw/SHA256SUMS*.txt`; committed v2.rs sha 3a0223048d... |
| committed sources untouched | `git status --short research/quartic_n4v2_20261020 research/quartic_n4_20261019 src/cryptanalysis/plane_quartic_pic.rs` | clean (HEAD 6b7fc4520e) |
| key wide unit, k = 23 seed 1 (ell = 147571999864203698233 > 2^64) | `V2W_COMPACT=1 python3 runjob.py $S/k23s1 1200 ./bin/v2w ic 8388571 1 D 3.0 2.0 4 4` | 4 records, 294 non-timing fields, **0 differences** vs `raw/wide/c_ic_8388571_1_D.jsonl`; D/DL1/DL ok=true, k_found = planted k; 106.5 CPU-s, 0.84 GiB (reported 101.6 s, 0.84 GiB) |
| default-unchanged control | `bin/v2_orig` vs `bin/v2w` (default mode), `ic 16363 1 D 3.0 2.0 4 4` | 294 fields, 0 differences; also identical to `raw/controls_final/v2w_ic_16363_1_D.jsonl` |
| analysis | `python3 analyze.py` | output identical to the committed `analysis.md` |
| calibration noise | `bin/v2w calib q c` (k = 22..24) and `bin/v2w2 calib 33554383 c` (k = 25), c = 1..3, repeated 4 times; the DL k = 22..25 exponent recomputed with `analyze.meq_unit`/`boot_fit` on the unchanged unit records | see the table below |

DL exponent over k = 22..25 (M-eq at w = 0) under each calibration:

| calibration | e [95% boot] | single-record combinations (81) |
|:--|:--|:--|
| producer's (`raw/wide`, `raw/v2w2/k25`) | 1.0993 [1.0993, 1.0995] | n/a |
| fresh run 1 (`v2w2` for all sizes, load about 1) | 1.1015 [1.1014, 1.1016] | n/a |
| fresh run a | 1.1034 [1.1034, 1.1036] | 1.0994..1.1081 |
| fresh run b | 1.1001 [1.1000, 1.1002] | 1.0973..1.1025 |
| fresh run c | 1.1008 [1.1007, 1.1009] | 1.0912..1.1054 |

Where the variation comes from: `r_lmul` at k = 24 ranged from 3.04 to 3.27 between runs, and
`r_inv` from 26.3 to 28.9. On a shared host, the per-size calibration noise moves the exponent by
about ±0.005. That is wider than the 0.0007 margin the verdict rests on.

## Checks against the lane file (frozen thresholds)

* The thresholds were not moved. Kill lines: lower bound > 1.10 over k = 22..26; DL/rho-neg upper
  bound > 0.01 at k = 26; any unverified logarithm. The predictions 1.00 [0.95, 1.06] and
  0.00085 [0.0006, 0.0014] are quoted correctly, and both are reported as missed. Good.
* The fixed-`r_lmul` (1.011) and raw-count (1.018) exponents are post-hoc sensitivities. RESULT.md
  correctly keeps the frozen measure as primary and labels these as secondary.
* The logarithm verification is real. `ok` is `ec.mul(G, k_found) == Q` (src_v1/solver2.rs:679-681),
  and the records also show `k_found == k`, the planted value. I confirmed this myself on 3 records
  with ell > 2^64.
* DL/rho-neg is a ratio to a calibrated model, not to measured rho, and RESULT.md says so. The
  k = 26 value is a projection from an exponent that is itself calibration-sensitive. Under the
  fresh calibrations the projection moves slightly up (about 0.0016 to 0.0017), still far below the
  0.01 kill line. That falsifier is genuinely not close.
* The default is unchanged. All changes are a fork inside the output directory. `V2W_COMPACT` is
  opt-in. The orig-vs-fork identity reproduces on the unit I re-ran.

## Issues

1. The partial-range falsifier is mis-stated as "not triggered". It is calibration-limited: 4 of 4
   fresh calibrations put the lower bound at or above 1.10. The reported CI is too narrow by about
   an order of magnitude because it excludes calibration noise. RESULT.md does acknowledge this in
   Deviation 6.
2. The calibration runs used two binaries (`v2w` for k <= 24, `v2w2` for k = 25), and the k = 25
   calibration ran under load about 1.4-1.9 while k = 22-24 ran under load about 5-6. Calibrations
   taken at different loads add size-correlated bias to an exponent that is fitted over only 4
   sizes.
3. The headline remains "prediction missed" (1.099-1.103 against [0.95, 1.06]). This check
   strengthens that miss.

## Recommended verdict

Keep the label "partial" (k = 26 not run). Change the partial-range wording to: "on k = 22..25 the
frozen-measure exponent is 1.099-1.103 across five calibrations of this host. The lower bound sits
on the 1.10 kill line, so it is indeterminate, not cleared."
