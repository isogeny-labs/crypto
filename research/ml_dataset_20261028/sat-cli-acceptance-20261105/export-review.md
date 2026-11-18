# Export adapter review and correction

Reviewed the original V2 adapter against its driver, protocol draft, pinned canonical protocol/export tool and schemas, and native supervisor/cell sealing code. Main source was not edited and no native Generate or cloud operation was executed.

## Blockers corrected

1. A PASS analysis was not bound back to the current manifest digest, even though the analyzer records manifest_sha256. Current manifests could change after analysis and receive new export file hashes while retaining PASS labels. The corrected adapter joins each current manifest to its analyzed digest, cell index, twist and exact native outcome.
2. The original PASS predicate checked only exit zero, initial_checked true and solver_completed false. It omitted partial status, exactly one of two patterns, and scalar_recovered_without_witness false. These fields are now checked independently, including integer types for pattern counts, and the analyzer identity and aggregate labels are checked.
3. trace_budget_exhausted with an exit-zero worker was mapped to succeeded. Native supervisor code can set this after process exit when final output exceeds its cap. The corrected adapter exports cancelled/not_completed for that censored outcome; timeout remains timeout, actual worker failure failed, and missing exit status infra_error. Native outcome remains retained verbatim in observations.

## Validation and limits

Five synthetic Python software contract groups passed using the retained Python environment. These include exact positive structure, changed manifest bytes, seven invalid outcome variants even with refreshed hashes, four incorrect analysis joins, and five status mappings validated against the actual pinned canonical run-row schema. These are adapter software tests, not generated curve data or native acceptance results. The full adapter has not been run on a completed acceptance attempt.

The canonical schema requires attempts >= 1 and has no unattempted row status. The adapter therefore includes actual attempt.json rows only; canonical flow.planned denotes exported rows, while its explicit observation retains protocol planned=2 and unattempted=2-minus-attempted. Shortfall findings remain visible. Do not describe canonical flow.planned alone as the registered population for an interrupted attempt.

File references are absolute original paths. Hash verification works with the current canonical tool, but a relocated archive needs reconstruction of the original root or an explicit derived export with rewritten paths; do not claim path-independent restore validation. The adapter preserves semantic findings and sets canonical_complete_pass false when findings exist; successful Python exit alone is not canonical approval.

The input remains draft/frozen instrument-specific: freeze a new protocol version binding the corrected adapter SHA before data. Original V2 artifacts are preserved. No further code blocker was identified in the scoped adapter review. This does not independently certify future generated evidence.
