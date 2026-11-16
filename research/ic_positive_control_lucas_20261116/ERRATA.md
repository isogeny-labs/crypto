# Errata (2026-11-16, from the independent validator report `validation/validator-report.md`)

RESULTS.md is left byte-equal; these corrections apply to it.

- "G3 … 696 runs": `run.json` has **576** rows (C, L, M and R at 11 primes is 4 arms × 144; 696 was a miscount).
- LA share of arm L at p = 32771 is **0.556** (tables.md: 0.56), not 0.54.
- The freeze commit contained `uncertainty.py` (a one-stage bootstrap) while PROTOCOL §5 specifies two-stage; the results
  commit deleted it without saying so. The reported intervals come from `analyze.py`'s two-stage bootstrap, which the
  validator reproduced.
- The bench record says `commit_dirty: true` and no binary hash was recorded; the validator's rebuild of the freeze commit
  (sha256 `6f822439…`) replays every p ≤ 2053 row exactly and is the binding.
- G2's "same recovered log" clause is not checkable from `run.json` (no logs stored); it holds on all replayed cells.
- Not reported, required by the owner rule of 2026-11-12: the calibrated reading (w = 0, w_cal, 10·w_cal). The validator's
  recomputation is above 1 at 10·w_cal everywhere, so no ranking changes.
- Pricing disclosure (validator X2): `run_rho` does not include rho's counted cycle-window compares (11.5 per step at
  p = 131 to 17.3 at 4099) in its native work, and the IC's additions are unpriced in the band (multiplications only).
  Which counters the band prices is an open decision; see `validation/out/pricing_sensitivity.json`.
