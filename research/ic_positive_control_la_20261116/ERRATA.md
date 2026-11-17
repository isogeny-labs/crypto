# Errata (2026-11-16, from the independent validator report `validation/validator-report.md`)

RESULTS.md is left byte-equal; these corrections apply to it.

- "696 runs" is **576**; "348 N and S runs" is **288**.
- The column "S/M incl. additions" (0.388–0.453) is the ratio of **total** native work, not of the linear algebra. The
  linear algebra alone, counting additions, is **0.334–0.339**.
- "S/R at w = 0.1 is 2.6–5.6 against 1.9–2.8 at the neighbouring primes" is labelled as training-curve data; it comes from
  the test rows.
- `train.sh` ran the unhashed `target/release` binary while `ic_control.rs` was being edited for the next study. The
  validator replayed the p = 4099 training rows exactly; p = 8209 and 16411 training rows were not replayed. cc* = 2.0 is
  unaffected at 4099.
- Calibrated reading (w_cal) and pricing disclosure: as the Lucas study's ERRATA (validator X2, X3).
