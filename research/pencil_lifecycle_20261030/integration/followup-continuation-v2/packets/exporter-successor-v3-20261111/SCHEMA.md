# v3 retained exporter schema

Request ecbench.pencil-attempt-export-request/v3 uses the same pinned plans and observed attempts; supplied session process/controller pins are optional. A declared native session pin is still verified. Primary sessions may be absent altogether and are recorded as unclosed. Retry sessions must have checked native plan bindings; their explicit parent refers to an enrolled primary slot.

Output ecbench.pencil-attempt-cohort/v3 retains all expected primary slots and6300 measured triplets even when observations are absent. Missing slots are marked not_observed with null evidence/cost and false eligibility. Supplied retries are retained as additional observations, never substitutes. Partial sessions use running/interrupted status and consistent records_written/last_seq prefix counts.

finite_algorithm_cost may retain a valid nine-category raw arithmetic projection even when external custody/diagnostics are incomplete. candidate_reference_eligible is separately false in those cases, with explicit reasons. Null mapper outputs remain null and ineligible. Owner authentication remains false; structural closure is not a signature.
