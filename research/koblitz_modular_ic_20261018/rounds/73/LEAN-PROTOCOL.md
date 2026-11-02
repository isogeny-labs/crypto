# R73 conditional degree-budget Lean checks, frozen before compilation

Compile `lean/FiveFiber73.lean` with pinned Lean 4.34.1 and mathlib
`d13f23b723b8a846827a245b89c10fc7d3f11612`, CPU0, one Lean job,
`-j1 -M4096`, separate 1200-second compile and axiom audit, no VM cap.
Copy and bind source/driver/audit bytes to Git before execution.
Acceptance requires compile and audit exit 0, zero failing and zero
trusted declarations, and all three named authored theorems.

The arithmetic theorem uses the exact R73 relative residue degrees
`1,1,1,1,2,14` and the R65 discriminant budget 12. Its weighted
excess equation is an **explicit premise**: the geometric
discriminant/different identity and lower bound, the normality
hypothesis, and the native residue-degree entries are not kernel-proved
by this Lean file. The conclusion is specifically that the degree-14
branch has ramification index one. The third theorem checks the
separate R72/R65 tame exponent arithmetic at `z=1`.

No EKS, Diem, freezer, Conductor claim, merge, or retarget. Preserve
failed compile/audit attempts and freeze repairs before a fresh run.

## Version 2 import repair, frozen before attempt 02

`degree-five-lean73-01` failed compilation before theorem checking:
the pinned mathlib build lacks `Mathlib.Tactic.Omega.olean`. The pinned
Lean core has `Init.Omega.olean`; import that module instead. The
three theorem statements, proof tactic, audit and resource gates are
unchanged. Retain the failed first compile and use a fresh attempt ID.
