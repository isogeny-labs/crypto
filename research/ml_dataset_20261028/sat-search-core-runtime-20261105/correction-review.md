# Strict empty variant correction and review correction

Reviewed patch `872615337718a88e41605d9bcb5781a48755f2dcc11c7e7615c95608b5b960ad`, proposed source `14e6b2775453f46e15e56317cf72fb1accc8b62f49c027159204940c99986806`. All four manifest artifacts verified. Parent reports actual isolated runtime317pass/1fail/4ignored, with unknown fields accepted for tagged unit Decision. I did not run tests.

This corrects my earlier source review's overbroad statement that every nested event/reason rejects unknown fields. Derive-level deny_unknown_fields on a tagged unit variant did not establish that behavior; the existing runtime negative test correctly exposed it. Prior report and failing run remain immutable.

Accepted source correction: local strict wire Decision and XorMatrixReset now use empty struct variants, retaining deny_unknown_fields on their tagged enums. Conversions explicitly map Decision {} and XorMatrixReset {} to unchanged native unit variants. Their JSON remains the same kind-only object. Other variants in SolverEvent, SolverEventReason, SolverEventConstraint and Payload already have named fields; no other tagged unit variant remains in these wire enums. NativeOutcome uses ordinary externally represented enum strings, a different encoding with no object fields to silently accept.

The original failing test remains unchanged. Added controls cover valid kind-only objects and rejection of added fields for both empty variants, plus conversion to the actual native variants. This meaningfully checks wire compatibility and the discovered failure. Accept for composition and focused rerun, followed by required broader checks. Runtime behavior of the corrected source is not claimed until those tests execute. No main/shared edits, build or runtime in this review.
