# Coordinator decision — XP-20261029-srenew-deferred-guard-v3

- **Decision:** approve with required edits E1–E5 (2026-10-29, `coordinator` role, before any execution of these specs).
- **Edits applied:** E1 (§6.1 condition 5), E2 (exit-42 clause), E3 (§2 trigger for 32 bits), E4 (§8 disclosure) and E5 (§10 counted-unit and replay rules). Each was applied verbatim in the commit that adds this file, and that commit is the freeze (§9).
- **Specs checked:** each spec differs from the first study's only in `label`, `description`, the two curve seeds, `target_seed` and `measurement.seed`.
- **Reasons:**
  - This is new data on fresh curves under a rule fixed before execution, and the first study is not re-graded.
  - The waiver covers exactly the grading-v3 1 ms wall-time minimum, on rho-reference executions only.
  - Counts are replay-certified, and wall time is not a claim.
