# Semaev Faugere-Lachartre benchmark

This study measures the Boolean F4 Faugere-Lachartre split merged in PR #4562
against its exact same-binary full-width control.

- `spec.json` is the frozen native-L2 request used for the recorded plan.
- `spec.canonical-v1.json` is its unmeasured successor under the canonical
  `semaev-groebner-f2m17-v1` field policy. It does not replace or amend the
  frozen request.
- `protocol.json` freezes the question, arms, outcomes, gates, and claim
  boundary before the admitted run.
- `ANALYSIS_PLAN.md` freezes the paired analysis.
- `examples/semaev_fl_bench.rs` is the fixed-system stage harness used for
  direct input/basis identity and operation-counter checks.
- `REPORT.md` summarizes the local diagnostic with its explicit admission
  boundary; `local/` retains raw rows, hashes, analysis, chart, and PDF.
- `VALIDATION.md` records passed source gates and the trusted-infrastructure
  blockers that prevented native-L2 promotion.

The local diagnostic's `m=2` row uses `GF(2^17)` and its `m=3` row uses
`GF(2^15)`. They remain immutable separate observations, not one field-matched
series. New runs derive both cases from the canonical `GF(2^17)` profile in
[`../../docs/ic/GROEBNER_FIELD_POLICY.md`](../../docs/ic/GROEBNER_FIELD_POLICY.md).

No result in this directory implies a general ECDLP speedup or a sub-rho
attack. Local probe data are diagnostic until reproduced in the admitted
environment.
