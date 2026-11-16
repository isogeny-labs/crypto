# Errata (2026-11-16, from the independent validator report `validation/validator-report.md`)

RESULTS.md is left byte-equal; these corrections apply to it.

- D1 and the F1 fallback counts are confirmed exactly (268 at p = 131 in 8 of 20 runs, 13 at 2053, 2 at 4099, 0 from 8209).
- F1's "60 × 37, rank 17" diagnosis came from an uncommitted debug dump (`core131.json`, a temporary build); the validator
  confirmed it from that file (rank 17, 18 distinct rows, target column undetermined). It is not reproducible from the
  committed files alone.
- The column "U/S LA incl. additions" (0.954–0.985) is the ratio of **total** native work; the linear algebra alone,
  counting additions, is **0.944–0.963**. The prose "0.95–0.97" omits 0.985 at p = 8209.
- Calibrated reading (w_cal) and pricing disclosure: as the Lucas study's ERRATA (validator X2, X3).
