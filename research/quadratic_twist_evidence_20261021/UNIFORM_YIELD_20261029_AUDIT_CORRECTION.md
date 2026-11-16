# Pre-rerun membership-division correction — 2026-10-29

The first frozen uniform execution at7c463e834 completed the producer's96arms,
896targets and210comparisons, but its independent audit rejected b3 before a
receipt or corruption tests. Producer-reported joint counts0/0 are unverified.
Exact attempt remains /workspace/pr2381-uniform-20261029/attempt-01; no payload,
source, failed log or assertion is overwritten. Scientific runner exited101.

Inspection identifies a specific implementation defect: the prepared Rust
query calls legacy divide_base, which always divides by the polynomial for the
FIRST TWO roots. The b3 assertion that cubic membership leaves S3 unchanged
correctly rejects this quadratic reduction. It is not removed or weakened.

New native/uniform-yield-audit-v2 preserves the old source and adds ordinary
monic polynomial division by the actual base_poly(xs) in each variable, using
every supplied root. Distinct roots, monic leading terms and resulting degree
bounds are checked. For b3, S3's variable degrees below3 and exact unchanged
remainder are both asserted. Native field/group/RREF/witness/state/rational/
recovery logic, all comparisons/frontiers and13 corruption modes are unchanged.
Producer/GMP and every prior source/protocol/proof remain byte-identical.

New run-uniform-yield-v2.sh changes only its versioned source/input binding and
auditor crate path. Freeze and commit compiled corrected sources before rerun.
Execute the FULL original protocol on a fresh attempt-02, including independent
audit, all rehashed controls and backend tests. Compare all97 producer case
files byte-for-byte with attempt-01; do not introduce a favorable subset.
Only a successful independent replay/control contract can verify results.

The original96/896/210 scope, iid sampler, b2/b3 inputs, all policies/losses,
actual source reductions,600-second command caps/3600-second envelope/two
workers and no-paid-compute constraints stand. Include a faithful Lean attempt
and report/graph/inspected PDF; retain unresolved probability/regularity/cost,
catalog, full-library and coordination gates explicitly. Checkpoint uploads
remain within the user's approved branch prefix, using the newly authorized
credential only to mint in-memory scoped tokens; never record its value.
