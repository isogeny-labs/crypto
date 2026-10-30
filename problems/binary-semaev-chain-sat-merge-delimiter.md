# Inherited native compilation error blocks PKM integration validation

- **Status:** upstream repair observed; integration/validation blocked
- **Kind:** inherited software build failure and merge conflict
- **Affected work:** [PKM G1–G6 continuation](../research/prime_ecdlp_three_leads_20261029/continuation-20261030/README.md)
- **Next action owner:** PR integration lane; native chain-SAT maintainer
- **Last updated:** 2026-10-30

## Problem and evidence

Fresh native validation of mergea030c16305 stops before tests: Rust reports an
unclosed delimiter in `src/cryptanalysis/binary_semaev_chain_sat.rs` at1703,
with the outer guard at27–28 unclosed. Cargo and the supervisor exit101.
Zero tests ran; clippy, format, CLI, backlog and library phases were not reached.

The complete file is byte-identical at local mergea030, PR head0cc5 and main
parentd4b3: Git blob `cc1d72055ee13acf4941e13162186ffead445904`, SHA256
`554203d74681f1c8eed9b9e3ac6c644551bfd48be5dae28a1053494bb5cb3da0`.
This matches earlier PR comment6526695631. It is outside the new PKM source
diff and is not patched around or waived. The separate Lean gate passes18
declarations/zero trusted; that does not validate native compilation.

[Failure receipt](../research/prime_ecdlp_three_leads_20261029/continuation-20261030/validation/integration-0cc5/native-failure/receipt.json)
retains exact logs, source objects, comparison hashes, terminal files and the
eight not-reached phases. The earlier23 native passes apply to faade source
only; stale executables cannot validate this new integration.

A GitHub current-main excerpt now lacks the obsolete outer guard, under file
blob `4e2111e29f64c5fece73f2c52dd9aff9c6e7f69f`. Its exact main commit/full
build was not established here. PR2949 now reports conflicts at0cc5; adopting
that repair requires reviewed integration and fresh affected checks. A source
preview is not a compilation pass.

## Resolution criteria

- Resolve current main into the existing PR history through Copilot-first and
  the documented fallback, preserving both append-only logs/index rows and all
  frozen evidence. No rebase/force-push or side selection of frozen records.
- Bind the repaired source and run the affected release examples, strict lint,
  CLI/backlog/library checks. Repeat the Lean gate only if its actual source,
  dependency or gate objects change.
- Preserve all failed/capped logs and not-reached phases. No assertion, ignore
  or gate weakening. Full-suite runtime limits remain tracked separately in
  [the JV validation problem](release-suite-jv-isogeny-walk-budget.md).

This is a software-validation blocker. It establishes no negative mathematical
conclusion about symmetric PKM. All G1–G6 scientific requirements remain open.
