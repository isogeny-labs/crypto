# Division-free sign normalization — T-1547

## What was done
Read current shared-seed source, latest low-product implementation and continuation
history. Located final ratio inversion used only to decide a calibrated sign.
Consulted EFD projective-coordinate references for standard homogeneous comparison.

## Decisions
Extract private RawParts; raw-value API still divides and preserves all metadata.
Sign-only direct and window APIs compare N*calibration_inverse to D and -D.
Require canonical nonzero numerator/denominator/calibration. Public zero point
remains sign0 via existing control flow. Use prior low-product engine unchanged.
No novelty claim for cross multiplication. No comparative panel in this step.

## Open threads
Exact finite predicate/scaling tests, saved source/raw/window tests, and complete
recovery equivalence; publish proof, raw receipts and inspected report.

## Errors
No build/test attempt yet. No cloud job.

## Checked construction
One optimized native build and one30-second-capped invocation pass all106 methods,
none filtered.1944 nonzero ratio cases and3888 rescalings pass,6 invalid tuples
reject. All8 complete recovery receipts match prior low-product implementation.
Raw records and legacy/direct/prepared window checks pass. All24 prebuild source
and data hashes pass; recovery routing remains byte-identical. No cost panel.

Report rendering first failed on inline multiplication markup; preserved error log
and corrected prose formatting. Native build emitted3 inherited warnings.
