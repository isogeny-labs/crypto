# Binding the refreshed formula inventory — protocol

2026-10-23, before executing regeneration. The strict independent assessment
verifier rejected exactly one changed source digest after the native Jacobian
refresh. That failure is preserved in
`validation/group-map-20261023/cover-assessment.log` and `.exit`.

Expand scope to `scripts/build_cover_transfer_assessments.mjs` and
`docs/curves/cover-transfer-assessments.json`. Regenerate source bindings and
correct only the formula inventory's freshness description: it now covers
the exact 4,493-row registry, with 2,572 prior per-model objects unchanged,
but still supplies no executable transfer certificate or speed measurement.
Bind the actual native refresh receipt. Do not modify the independent
verifier, its pinned semantic claims/open-obligation sections, model/formula,
source/partition/byte checks, or any transfer status. Preserve both catalog
provenance dependencies at a11984a57344 and 228cdc0d50f2.

Check exact before/after equality of all other assessment sections and all
profile/map/subgroup/membership/typed-binding statuses. The historical pinned
open-obligation ledger remains unchanged; the supported source-freshness claim
does not mark all mathematical or operational obligations complete. Run the
generator's strict --check and the independent verifier; retain the initial
failure and final receipts. Older dated reports remain immutable. The new
dated report records the correction and no transfer/solver-cost promotion.
