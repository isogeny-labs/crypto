# Export coverage boundary

Schema, flow and referenced-file hash validation passes. The protocol-aware
canonical checker retains five coverage findings because it assumes the
rectangular product 3 arms × 4 fields × 2 seeds = 24 measured rows.
The frozen v5 stopping rules explicitly prescribe six matched 52-bit cells
plus two candidate-only cells per larger field, at most 12 rows. After the
second 58-bit target timed out, both 60-bit cells were left unattempted.
The actual record has ten executions: nine verified recoveries and one
timeout. The frozen native analyzer verifies that exact nonrectangular
coverage and the complete-rung stopping rule. No rows are excluded.

These findings are retained in protocol-check.log; this is not a full
canonical protocol-checker pass or scientific validation-gate approval.
The raw archive, source and binary hashes, complete certificate/log member
manifest, replay outputs and deletion receipt are preserved alongside the
export. All timing fields remain L0 and wall_valid=false.
