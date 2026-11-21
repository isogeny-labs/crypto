# Errata to RESULTS.md (from the independent validation)

`RESULTS.md` stays byte-equal to commit 2190b15c4c. The validator's report is
`validation/validator-report.md`, disposition **validated-with-errata**, findings F1–F14. These
corrections take precedence. The verdict is unchanged: K1 and K2 do not fire. Every number is
`[pcc:unverified]`.

- **E1 (F1, F2): freeze binding.** The order "protocol, then grid" is evidenced by file mtimes
  (hashes written at 02:46:18Z, run started at 02:46:27Z) and the session transcript only. The
  protocol was not committed before the run, there is no resume checkpoint, and the development
  records were left in the scratchpad. An abandoned G+T walk-step variant, tried on development
  data only, was not disclosed.
- **E2 (F3): certificates.** Rows carry only `certified` and `ok` booleans, so no independent
  certificate check is possible. The validator's replays reproduce both flags, and the solver
  never reads `t_secret`. The ladder study must record recovered logarithms.
- **E3 (F9): GAE band.** At w = 0 (Pic operations only), NEW and OLD tie: 1.032 [0.880, 1.197].
  The pooled 0.391 holds only in FPM mid, and in GAE for w >= 0.01. At p = 1031 the ratio is 1.002.
- **E4 (F10): table labels.** The `decomp_trials` column holds arithmetic means, not geometric
  means (geometric means: 287 to 4230). The 6.9e8 "of which" figure mixes an arithmetic mean with a
  geometric-mean total.
- **E5 (F11): why the slope is below 1.** It is not fixed walk-start costs; those are only 138–311
  Pic operations.
  - The cause is one Pic addition per decomposition trial, about 3.6k–4k F_p multiplications each.
    These grow as p^0.735 and are 52% of FPM mid at p = 1031 but 15% at 65537.
  - The rest of the cost has exponent 1.022.
  - So 0.897 is a finite-range mixture, consistent with Diem's O~(q). It must not be extrapolated.
- **E6 (F11): what NEW/OLD measures.** It measures the removal of instrument defects, not an
  improvement over the lab's index-calculus baseline.
- **E7 (F12): "singular by construction" is wrong.** On the grid, OLD succeeds at its first attempt
  in 20 of 56 runs. The development curves (0 of 36) were unrepresentative.
- **E8 (F13): where the retry cut comes from.** It comes mainly from the linked policy (NEW/KICK
  attempts 0.635). The T kick alone is not significant (0.870 [0.737, 1.028]). The proposed
  rank-aware stop is an untested hypothesis.
- **E9 (F8): bootstrap intervals.** The percentile bootstrap over 4 curves per prime is mildly
  anti-conservative. The t-intervals (FPM mid [0.792, 1.002]) are also far from 1.40.
