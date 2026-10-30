# Restore policy lint correction 01

Clean-clippy-01 failed with collapsible_if at restore.rs114. Preserve that
terminal101 result and both successful source guards. Correct only the nested
policy-artifact condition, from `if A { if B { reject } }` to `if A && B { reject }`.
A remains path-is-policy OR role-is-policy. B remains path mismatch OR role
mismatch OR non-JSON encoding OR oversized bytes. Preserve short-circuit order,
the error string, all limits, and all subsequent checks. Add no lint allowance.

Work on a copied proposal file and produce a contextual patch against immutable
source package01 commit1d858fb9e4b8035b38f93db4423880627b09211f. Do not edit the
validated package or another task's source. Check Rust formatting and patch
application. This is a behavior-preserving lint repair; existing restore and
dataset tests supply the applicable semantic regression checks.

Prepare a new guarded snapshot and clean Git source package using the existing
helpers, with that actual base commit and this one-file overlay recorded.
Use a separate /var/tmp source destination to avoid repeated shared-filesystem
Git object writes. Keep preparation logs and receipts in the durable workspace,
and archive the complete new Git bundle plus source manifests with fresh S3
readback before building. Preserve the previous packages and failures.

Then build the actual CLI/controller from the new clean commit and run the
applicable clean readiness checks prospectively. No old binary is relabeled with
the new commit, and no old check is claimed to have run against a new binary.
No generation or cloud worker is part of this repair.
