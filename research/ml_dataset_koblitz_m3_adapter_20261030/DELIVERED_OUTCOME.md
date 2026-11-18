# Outcome claims require verified delivery

The adapter now composes canonical mathematical/terminal replay with the independent parent-delivery verifier. `verify_delivered_outcome` requires a complete delivery receipt before accepting `solver_completed`. An omitted outer `live_persistence_errors` field is not evidence of successful delivery. Partial outcomes retain independently authenticated witness facts.

The six existing F4/F5 positive/negative/capped fixtures exercise both complete delivery and a rehashed, honestly incomplete parent record whose final root acknowledgement is absent. Its original admitted charges and capture acknowledgement are retained, and parent failure is explicit. Completed outer claims without an error field must fail; honest partial claims retain their checked witness. This is outcome-verifier composition, not worker/CLI activation. The shared worker still needs acknowledgement transfer to avoid repeating already charged root writes, and original-input setup admission and central dispatch remain required.

Validation source and terminal results are preserved separately; no cloud generation is launched by this increment.
