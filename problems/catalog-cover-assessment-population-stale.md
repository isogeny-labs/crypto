# Cover-transfer assessment generator pins an older catalog population

- **Status:** open
- **Kind:** implementation gap; metadata validation gap
- **Affected work:** [assessment generator](../scripts/build_cover_transfer_assessments.mjs), [weighted Weil study validation](../research/weil_weighted_decomposition_20261019/validation/README.md)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-19

## Problem

The catalog-wide assessment freshness check cannot pass because its
generator hard-codes 2,100 cover rows, while base commit
8e4c99759fd8d9594ed207d793b07b59ebaf19f7 already contains 2,572 and the
weighted Weil study registers 12 more. The check fails with
`unexpected covers row count` before assessing the new catalog.

## Evidence and uncertainty

- **Observed:** the frozen base registry and covers.json each have 2,572
  rows; their registry SHA-256 binding agrees. The current native cover
  refresh verified all 2,584 models with zero invalid or unsupported rows.
- **Observed:** the unchanged generator asserts `covers.curves.length ===
  2100` at line 1124 and pins other older profile/membership counts.
- **Derived:** the 2,100-row assertion conflicts with both the base and
  current catalog populations. This predates the 12 new experimental models.
- **Missing capability:** an updated assessment population and regenerated
  metadata with all typed-map/subgroup evidence bindings retained.
- **Not established:** mathematical failure of any cover or DLP transfer;
  this is a metadata population mismatch, not evidence about DLP cost.

## What would unblock it

- [ ] Update population guards and membership bindings against the current
  native catalog while preserving the historical 2,100-row report.
- [ ] Regenerate current assessment metadata, pass its schema and typed
  evidence verifier, and keep geometry claims separate from transfer claims.

## Next discriminating step

Inventory the new native catalog models against the frozen assessment
membership set, review their profile assignments, then regenerate and
replay all assessment checks. Stop on an unclassified or invalid binding
instead of dropping rows or weakening a mathematical assertion.

## Resolution and history

Unresolved. The weighted experiment's arithmetic checks and native cover
catalog refresh passed; they do not refresh the separate assessment ledger.
