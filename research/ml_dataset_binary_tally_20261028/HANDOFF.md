# Binary batched-square tally correction

Recorded 2026-10-28 at 13:37 UTC. This is software accounting work, not a
dataset run or a measured performance result.

## What was done (commands, files, features)

- Created branch `codex/ml-binary-square-tally-20261028` in
  `/home/ec2-user/crypto-ml-binary-square-tally-20261028` from
  `f5c4d4ed96f78332348c661dc752788d20281276`. Conductor task T-740 reserved
  `src/binary_ecc/f2m.rs`, `src/binary_ecc/operation_tally.rs`, and this directory.
- Added the existing `square` tally tick to each executed iteration of
  `F2mElement::square_k_times`. Its zero-iteration return records no work.
  No field arithmetic, serialization, historical artifact, or counter scope
  changed. `operation_tally.rs` required no source change.
- Added `square_k_times_tally_matches_repeated_squaring`, comparing both output
  and complete count maps against separate `square` calls, including zero
  iterations and zero/one/general inputs across degrees 8, 16, 113, 127, 163.
- Added `flt_inverse_tally_includes_all_squarings`, checking the complete
  `n-1` squaring count and one informational inversion call for nonzero
  inputs across those odd/even and single/multiword fields. The inverse
  identity is checked outside the counting window.
- Scoped formatting and whitespace validation passed:
  `rustfmt +stable --edition 2021 --config skip_children=true src/binary_ecc/f2m.rs`
  and `git diff --check`. Final source SHA-256 before commit:
  `0871085f0301912b94392c78a15559911e7207fcfa4d556ee7f25822233b94c1`.

## Decisions made (and why)

- Source review followed `src/binary_ecc/f2m.rs`'s existing Itoh--Tsujii
  inversion schedule, `src/binary_ecc/operation_tally.rs`'s informational
  inversion contract, and the calling-thread/replay/exclusion scope exported
  by `src/ml_dataset/solvers.rs`. The missing batch ticks understated the
  inversion squaring count even though the existing outcome said those
  operations were included.
- Count executed loop iterations through the existing ledger API rather than
  changing field algorithms or adding a separate cost model. Inversion calls
  remain informational; their primitive operations must not be priced twice.
- The parent will centralize Cargo validation after integration. No Cargo
  command, experiment, cloud job, PR push, or historical-artifact rewrite
  was performed by this implementation task. No budget or approval changed.

## Open threads (unfinished, pick up next time)

- Implementation is committed for review but native tests have not been run.
  Run both named regressions, relevant binary-field/dataset suites, strict
  scoped lint, and the repository's required full release library suite on
  the integrated source before pushing.
- Retain final validation output, source identities, and any failed attempts
  in the parent's canonical S3/Git validation packet. Append the resulting
  session-log entry during coordinated parent delivery.
- Binary S3 intermediate-state instrumentation and independent root-set
  certificates remain a separate planned increment. This correction provides
  no complete-machine-cost claim, new root coverage, or fall-degree result.

## Errors (what went wrong, how it was handled)

- Review found that the batched-square loop bypassed the operation tally.
  Existing numerical output was unaffected; the patch only restores the
  declared primitive count. No command failure occurred during this patch.
