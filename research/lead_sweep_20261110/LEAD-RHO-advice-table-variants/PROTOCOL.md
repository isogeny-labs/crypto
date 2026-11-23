# LEAD-RHO-advice-table-variants

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-24). Family: generic.
- Title: "Variants of the advice table: batch-reselected, batch-grown, basin-instrumented and transported tables".
- Next-step excerpt (truncated in leads.json): "Replicate EV-ECDLP-f6b3f5's re-scoped support (T_sel in {0.65T, 0.75T}) on an elliptic-curve wal..."
- Atlas survey notes: "T/2 reselected table -0.064 [-0.080,-0.049] at 2^24, contradicted"; atlas files class it constants-only (cannot change exponent, Shoup).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 65-67, 118, 245);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (grep context only).
- EV-ECDLP-f6b3f5 itself was not found/read; the mechanism below is my own reading (assumption, [pcc:unverified]).

## 2. Hypothesis
In a toy walk model, an advice table of T points whose first T_sel in {0.65T, 0.75T} entries are re-selected as the most-visited points of a pilot walk (cost about T steps), the rest uniform, does NOT reduce the mean steps-to-hit of fresh walks by more than 2% against a uniform random table of the same size T, on an r-adding walk in a cyclic group (additive model of E(F_p)). The recorded -0.064 reselection result at T/2 (read as a loss) is expected to persist at 0.65T/0.75T.

## 3. Preregistered prediction and falsification criterion
- Statistic: per-replicate ratio R = mean steps(reselected table) / mean steps(uniform table), paired on identical start points; 8 replicates; t-interval (t=2.365) on mean R. Gain g = 1 - mean R.
- Prediction: g <= 0.02 for both fractions (reselection gives no gain, likely R > 1).
- consistent_with_prediction: both g <= 0.02 and both lower-gain bounds (1 - upper CI) <= 0.03, controls valid.
- falsified: for either fraction, 1 - upper CI > 0.03 (reselection helps by a resolvable margin; would reopen the lead as a constant-factor gain, not an exponent change).
- inconclusive: otherwise, or controls fail.
- Null / negative control: same pilot cost, but T_sel pilot-visited points chosen at random (no top-visit selection), arm "null". Reported as a ratio; separates "selection" from "pilot points are non-uniform".
- Positive control: random-mapping walk (x -> f(x), heavy in-degree basins), where top-visited selection must help; require 1 - upper CI > 0.03 at 0.75 else detector invalid.
- A/A: uniform table vs an independent uniform table (same walk, same starts); its CI must lie in [0.9, 1.1].

## 4. Method
- Group Z_N, N = 262139 (prime), T = 1024, walk x -> x + s[h(x)] mod N, 16 random jumps, h = multiplicative hash bits. Fresh walks: 2500 uniform starts per replicate; stop on table hit, cap 40*N/T.
- Pilot: T/64 walks of length 64 (about T steps), visit counts; reselect = top T_sel by count; fill remaining with uniform points.
- Seeds: 20261110000 + 10*rep (+5 for control map), rep 0..7; tables, starts and jumps derived from them.
- Cost: about 12 arm-groups x 8 reps x 2 tables x 2500 walks x ~256 steps, under 2 minutes, a few MB RAM (control map list of N ints).
- Limits: stdlib only, single core. Smoke: `python3 -I run.py --smoke` (N=4093).

## 5. Classification
runnable_local. A stdlib toy decides whether top-visited reselection beats uniform tables in an abstract walk model; real elliptic-curve walks are group-isomorphic to this model for the r-adding rule.

## 6. What a run cannot establish
- It does not replicate EV-ECDLP-f6b3f5 (not read); the reselection rule is my interpretation.
- Not an actual elliptic curve, not 2^24 scale, only one table-size/walk-length point; batch-grown, basin-instrumented and transported variants are not tested.
- Even a gain is a constant factor and cannot change the sqrt(p) exponent (Shoup); no section 5a pricing of native work is done. Results are labelled [pcc:unverified].
