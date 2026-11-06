# Initial native integration failure

The initial row-0 diagnostic exited with status 1 before producing a JSON
result. Its actual stderr is retained in `initial-native-attempt.stderr`;
the instrumented failure is in `native-integration-failure.stderr`.

The C++ integration treated the exponent column returned by PARI's
`FpX_factor` as a column of GEN integers. It is a `t_VECSMALL` of machine
integers. Applying `itos` to its first exponent caused the segmentation
fault caught by PARI. The implementation now checks the column's type and
reads its representation correctly. It uses the public elliptic-curve
division-polynomial entry point before finite-field factorization.

This was an integration error in the new diagnostic, not evidence of a
PARI arithmetic defect. The failed attempts establish no mathematical
result; the six completed per-curve JSON records come from the corrected
source. A native result remains a computational check, not Lean
formalization or an independent-host replay.
