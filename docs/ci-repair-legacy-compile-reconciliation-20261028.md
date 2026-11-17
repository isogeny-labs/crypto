# Reconcile the original compile repair without regressing later fixes

PR 2675 preserves early delimiter, import, assignment-mask and lint repairs.
Its original head is `120ab564845435d1aebe618d51704e9721c6a2f4`.
Later main integrations already contain every resulting source change and a
newer canonical artifact bootstrap. Its merge conflicts are confined to
23 generated workflow bootstrap blocks and the generator/control files.
No scientific evidence file conflicts.

The Copilot-first requests at comments 6452395282 and 6456907148 received no
response or branch update; the latest waited over 90 minutes. The documented
fallback uses a normal merge of main, preserving both original histories.

A structured YAML comparison proves that removing only generated pull steps
and their artifact-tool/configuration sparse patterns makes every one of the
23 original workflow changes equal to its pre-change base. Their conflicts
therefore retain current canonical bootstrap and all later workflow behavior.
The current control file retains all original named test cases, and all 28
current artifact controls pass. The shared artifact helper is unchanged.

The Rust source and regression-control changes merge automatically. Before
adding this report, an exhaustive staged-tree comparison finds no differences
from the merged main tree: every path, mode and blob OID is identical. The
matching complete tree OIDs, exact heads, resolved path hashes and validation
output are retained in `reports/legacy-compile-reconciliation-20261028/`.
This preserves the old repair and its history while retaining every later
source fix, regression test, workflow pin, session entry and research input.

Final exact-head gate, size, artifacts and review remain required. This
history reconciliation does not assert that all current main checks pass;
separate repair PR 2733 addresses the newly diagnosed evidence hydration,
historical-report path and maintained-example lint failures. No research
result, frozen lock, admission threshold or performance claim is changed.
