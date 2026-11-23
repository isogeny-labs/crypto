# Restore suffix-only residual construction

## What was done

Round24 completed all 180 executions with exact certificates and showed higher
complete medians for early residual rejection in every matched cell. Restore
crates/hashed-ecdl-cpu/src/lib.rs byte-for-byte from the measured suffix-only
parent 4939261c9fcc48bf86f17ac36fa458f52a155b2e. This removes the optional x-only
prefilter, scratch rejection flags and the filtered subtraction path. The exact
suffix lookup bitmap remains. The removed path and its specialized test remain
available in frozen candidate b19d9b3beae66488c54ca50ec28b065e5683eccb and round24.

## Decisions made

The relevant references are residual-filter/README.md, the existing affine
subtraction equations and oracle-filter/README.md (Bloom's primary paper),
followed by round24/results/summary.csv and the inspected reports/round24.
The comparison preserves witness order, counters and certificate bytes. The
complete-cost increases are 1.00/10.34/10.72 percent at batch64 and
1.00/3.35/4.01 percent at batch1. The data do not isolate which added overhead
caused the regression. Use the suffix-only parent for the next candidate.

## Validation

The restored standalone CPU crate passes 47 release library tests, including
archived certificate equality, exhaustive small-group affine subtraction and
wide-field witness checks. The early-rejection-only test is removed with its
implementation. Example targets compile and test successfully (zero registered
tests). Strict all-target release Clippy and formatting pass. No root or CUDA
build was performed. Source equality with the measured parent is checked by
SHA-256 and cmp; logs are retained here. These checks are software validation,
not additional performance samples.

## Open threads

Continue CPU complete-cost optimization from this source under the owner's
continuing budget authorization. Preserve all fixtures, A/A controls, both
replays and frozen comparisons. Hosted checks on the final PR head remain
required. Round24's immutable candidate and raw results are unchanged.

## Errors

No software validation failure occurred. Native filesystem operations are
slow on the migrated volume; existing operations were polled without retries.
The optional Node-based PR-text edit was unavailable, so the same scoped edit
used awk and sed. No experiment was repeated.
