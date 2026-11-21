# Errata to RESULTS.md (from the independent validation)

`RESULTS.md` is left byte-equal to the validated commit b76bc83fbe. The validator's report is
`validation/validator-report.md`, with disposition **validated-with-errata**. These corrections take
precedence over `RESULTS.md`. Every number is `[pcc:unverified]`.

## E1: the cross-check is in-sample and the "within 10%" claim is wrong (F2)

The GAE re-pricing at the measured weights is 0.0056 / 0.0060 at p = 16411 and 0.0043 at p = 32771.
It differs from R1 by +6% / +7% at p = 16411 and by **+11.1% (x) / +10.7% (x\*)** at p = 32771, not
"within 10%". It also re-prices the same rows the weights were fitted on, so it is not an independent
check. The gap is the negative OLS intercept. Strike "so the counted operations account for the wall
time". What does hold is the in-sample fit quality: R² = 0.942 at p = 16411 and 0.986 at p = 32771.

## E2: the split between w_rel and w_la is fragile; their sum and the decision are not (F3, F4)

Relation and linear-algebra counts move in opposite directions with cc (correlation −0.94 / −0.93),
so the split of non-group time between the two classes rests on one linear model.

- **Leave-one-cc-out:** w_rel ranges from −0.00004 to 0.0033 at p = 16411, outside its reported
  interval [0.0019, 0.0032]. w_la ranges from 0.0032 to 0.0050. The reported intervals are
  conditional on the model and understate this.
- **Robust bound:** charging all non-group wall time to one class gives w_la ≤ 0.0097 and
  w_rel ≤ 0.042 at both primes, far below 1. The **conservative** decision stands without the split.
- **Intercepts:** the OLS intercept is −0.15 s at p = 16411 and **−0.68 s at p = 32771** (8% of mean
  wall). The explanation in RESULTS ("collinearity between relation and linear-algebra counts, which
  both fall with p and rise with cc in opposite directions") is garbled. Correct version: the
  intercept is not identified separately from the two count regressors. Those regressors are nearly
  collinear within a prime because raising cc trades relation work for linear-algebra work. The
  intercept has no physical meaning.

## E3: statements that are post hoc but were not labelled (F5)

These are post hoc, outside the frozen analysis:

- "Where the time goes";
- the "The §5a band is very wide" and "Tuning flips" bullets of "What this changes";
- the sensitivity paragraph under Caveats;
- the intercept explanation.

## E4: scope of the verdict and the over-charge factors (F6, F11)

- **Scope.** The calibration covers p ∈ {16411, 32771}, prime-order curves, cc 1.0–2.8. Extending
  "the w = 1 headlines of the validated genus-4 studies are conservative" to other studies (for
  example p = 8209, or cc 0.5–0.7) is an extrapolation. It is plausible because the weights barely
  move with p (P5) and the robust bound in E2 has a wide margin, but it was not measured.
- **Over-charge factors.** "The w = 0.1 column overstates native cost by about 25×" holds for w_la.
  For w_rel the factor is 39–41×.
- **"Tuning flips"** holds at p = 16411 only. At p = 32771, x\* and x are tied (wall ratio 0.999).

## E5: sensitivity paragraph (F7)

- The group-operation share at p = 16411 is 0.85 s, not 0.86 s.
- The arithmetic ignores the intercept.
- Faster Montgomery F_p arithmetic would make relation-phase F_p operations cheaper too, so it need
  not raise w_rel. It would raise w_la, which is mod-r work outside the F_p routine.
- The conclusion (about 0.04 of the bound if t_G fell tenfold with linear algebra unchanged) is an
  order-of-magnitude illustration, not a measurement.

## E6: wording on certification, intervals and the reference (F8, F9, F10)

- **Certification.** "Certified and correct" is the binary's own check. No row records the recovered
  logarithm, so it was not re-verified independently. The count replay reproduced the ok/certified
  flags. Do not cite these rows as independently certified solves.
- **R1 intervals.** They come from a bootstrap over only 4 clusters, so they are coarse and too
  narrow. They exclude t_G's uncertainty, and p = 32771 has a single rho run.
- **The reference.** `sqrt(pi r / 4)` is the heuristic expected step count of an ideal negation-map
  rho, not a lower bound. t_G includes rho's per-step bookkeeping (hash and coefficient adds), about
  1% of a step.

## E7: process (F1, F12)

- **Bench record.** It was left open: `tools/benchrec.py close` aborts on a malformed record already
  on main. It has no raw-data S3 hashes, because there is no AWS identity here. The
  `benchmarks/20261119-genus4-cost-calibration/` directory holds the report, the figures and the
  validator's report.
- **No gate row.** It has **no `VALIDATION.json` and no row in `benchmarks/README.md`**. The
  validation gate requires a replay certificate with `scalars_verified: true`. Here the scalars
  cannot be independently verified (F8):
  - no row records the recovered logarithm;
  - the target k = 0 keys used here are not among the 60 keys the prime-order validator certified
    (k = 1–4 there).

  The study is a price calibration, not a speed row, and none of its readings depend on the
  correctness of the solves. Its findings enter other rows only as the cost convention, through E2's
  robust bound.
- **Host.** Local, not isolab-eks; no S3; no PMU. All three were disclosed in the protocol.
