# Source review corrections

The independent source reviewer identified two eligibility-boundary issues in
the original external packet (source SHA256
`a99de4b3d839be55d117bedff53c9546fdb5d8f46be5a637113982b9a1a3b453`).
That packet remains unchanged in the sibling directory without the `-v2` suffix.

1. Platform/event semantics admission originally gated the training label after
   the whole-attempt projection. Version 2 moves this admission before that
   projection. Unadmitted events retain the raw diagnostic tuple only.
2. Successful attachment/reset before exec, readback after task exit and equal
   enabled/running times did not explicitly establish continuous coverage.
   Version 2 requires `continuous_exec_to_exit_coverage_verified`, derived from
   the complete reviewed task/event lifecycle with no early disable/reset/gap.
   Endpoint observations are not a substitute.

The focused regression preserves raw values but rejects whole-attempt and
instruction-to-solve values for either missing fact. Existing gate-mutation
tests also include both conditions. All 15 synthetic policy tests pass. This
is a source/policy review and mock test result; no hardware observations or
production integration were performed.
