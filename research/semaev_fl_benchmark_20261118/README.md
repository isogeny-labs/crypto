# Semaev Faugere-Lachartre benchmark

This study measures the Boolean F4 Faugere-Lachartre split merged in PR #4562
against its exact same-binary full-width control.

- `spec.json` is the native-L2 ecbench request.
- `protocol.json` freezes the question, arms, outcomes, gates, and claim
  boundary before the admitted run.
- `ANALYSIS_PLAN.md` freezes the paired analysis.
- `examples/semaev_fl_bench.rs` is the fixed-system stage harness used for
  direct input/basis identity and operation-counter checks.
- `REPORT.md` summarizes the local diagnostic with its explicit admission
  boundary; `local/` retains raw rows, hashes, analysis, chart, and PDF.
- `VALIDATION.md` records passed source gates and the trusted-infrastructure
  blockers that prevented native-L2 promotion.

No result in this directory implies a general ECDLP speedup or a sub-rho
attack. Local probe data are diagnostic until reproduced in the admitted
environment.
