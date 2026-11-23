# Native live-panel snapshot

This is an interim invocation of the frozen Rust reducer, not a new
collector execution. Its expected exit 2 means the full panel is still
partial. The snapshot records **29 candidate exits**; unexecuted or
A/A-pending cases are explicitly unverified with missing values, not cheap
successful solves. The managed panel continues after this immutable snapshot.

All four p37 seed0 and seed1 configurations have verified first/A/A and
independent physical/RREF audits in this snapshot. Seed2 zero-phi also
verifies. Other rows must retain the snapshot's unverified status.

On p37 seed1, zero-plus-phi reduces target queries from two to one:
82 zero systems plus 72 target systems gives 154 total, versus 144 in
plain phi. Zero-plus-doubling uses 216 plus 144, or 360 total. This is an
observed rank/online effect with an increased total F4-system count against
the plain baseline; it is not an admitted complete-price comparison.

The exact native `summary.json` and `cases.csv` are copied from raw
`runs/phase2-panel-v1/progress-v1/`. Later case completion does not rewrite
this snapshot. Full terminal reduction, all seven curves, calibrated cost
and archive publication remain pending.
