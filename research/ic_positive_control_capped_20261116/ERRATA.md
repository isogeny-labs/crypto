# Errata (2026-11-16, from the independent validator report `validation/validator-report.md`)

RESULTS.md is left byte-equal; these corrections apply to it.

- **Undisclosed identity:** at p = 16411, 32771 and 131101 every W row equals the unit-T study's U row field for field (8/8,
  8/8, 4/4; also 4 of 12 at 4099). There the restart cap is the default 4096 and the deterministic fallback never fires
  (cube-root primes), so W/R = 1.77, 1.72, 1.78 and the "about 10% at the cube-root primes" are U's results. The capped
  walk and the fallback act only at p ≤ 4099 and at the fallback primes (2053, 8209, 65537).
- "LA incl. additions" is the ratio of total native work, as in the other studies' errata.
- The extrapolated w = 0.1 crossover of the fitted trend is about **2^21.7**, not "2^20–2^21".
- Calibrated reading (w_cal) and pricing disclosure: as the Lucas study's ERRATA (validator X2, X3). Under the validator's
  recomputation W at 131101 reads 0.137 / 0.238 / 1.118 at w = 0 / w_cal / 10·w_cal.
