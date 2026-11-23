# Errata after independent validation (2026-11-16)

Validator report: `validation/validator-report.md` (agent run a4c68413b89e783c3,
artifacts at commit 62307fead3). RESULTS.md is left byte-equal to the frozen
report; corrections are here.

Measurement: **validated** (freeze, 24 + 13 replayed runs identical, 9/9
independent certificates, G1, G2, every reading and every interval reproduced
exactly). The binary hash does not reproduce across build paths (one LLVM symbol
suffix and the build-id differ; `.text`, `.rodata`, `.data` identical).

## Headline correction (B13): w = 1 under Q3 is not established

RESULTS states arm w is below the cheapest rho at w = 0, 0.1 and 1 under Q3.
At **w = 1 that is insufficient_evidence**:

- 89.6% of negation rho's Q3 cost at w = 1 is its per-step 16-key
  short-cycle window scan (break-even price 0.0425 multiplication per compare).
- The plain walk carries the same window though it never fires (0 short
  cycles in 7 dumped runs). With `cycle_window = 0` it is the same walk
  (identical log, group operations, steps), so the window-free numbers are
  exact for that walk:
  - Q3, w = 1, plain walk without window, canonicalisation floored:
    **1.049 [0.780, 1.427]**; unfloored: **2.098 [1.560, 2.854]**.
- RESULTS' caveat "a leaner rho would narrow the margin" understated this:
  the margin disappears. The registered-walk numbers stand as computed, but
  the claim "below the cheapest rho at w = 1" does not, because a cheaper rho
  (the window-free plain walk) exists.

Corrected verdict for p = 8209: **below rho at w = 0 and w = 0.1 under every
reading tried; unresolved or above at w = 1** depending on the reading.

## Other corrections

- B14: "negation rho is the cheapest registered rho everywhere" holds only with
  plain rho's canonicalisation floored at one per step; the plain walk performs
  none. Unfloored, plain is cheaper at w = 1 in Q1 (0.719 [0.535, 0.967]) and
  Q2 (2.098 [1.561, 2.821], above rather than unresolved).
- B11: Q2 at w = 1 lower end is 0.971–1.007 across bootstrap seeds; whether it
  reads "unresolved" or "above" depends on the seed.
- B12: the Q3 "owner decision" attribution. The owner chose "All ops, both
  sides" in an AskUserQuestion answer in session
  https://claude.ai/code/session_01GmqZi7GDGUjCDahBRBx8c6 (2026-11-16); the
  validator could not see that record. The decision fixes which operations are
  priced; it does not fix which rho walk is the reference, which is B13.
- B15: the IC's word-level compares in `mat_vec` are not emitted; counting them
  moves Q3 at w = 1 to 0.191–0.255 against the registered walks.
- B16: Qcal prices rho at group operations only (per-step overhead about 4% of
  wall time untimed); LA additions are priced at the mod-p rate (protocol says mod-r).
- B17: the Qcal cell sits in the w = 0.1 column but is one calibrated reading;
  "window/Brent compares" are all window compares (Brent off).
