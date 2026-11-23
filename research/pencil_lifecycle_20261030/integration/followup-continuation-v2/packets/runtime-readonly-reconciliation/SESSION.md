What was done: inspected saved worker/completed receipts and local losetup manual; reproduced only metadata-query output difference; implemented separate strict read-only reconciler and 15 controls.

Decisions made: explicit --list; reject empty output; preserve failed launcher receipt; require empty entire loop inventory for this exact single attempt.

Open threads: independent source review then separate read-only observation, no qualification admission.

Errors: original JSONDecodeError belongs to immutable supervisor attempt; this helper has not been invoked live.
