# Additive diagnostic correction review

Reviewer /root/four_lead_state; 2026-10-30. Static follow-up only, no tests or experiments rerun, no repository/cloud mutation. This follows progress-diagnostic-independent-review.md without replacing it.

Reviewed research/pencil_progress_20261030/src/progress.rs SHA5415467accaf0e8b6426a8834838ac6fc7e7a463520b02dfcbc8eb7d1338cd89. Native solver remains SHA4d88d814a04da81ec723aa9e91e9f742e329780cdffdd118d7a9906096d0de5c; its prior static math/RNG/counter assessment is unchanged.

The identified corrections are present:

- Unix directory/file creation explicitly requests0700/0600 at creation using DirBuilderExt and OpenOptionsExt; umask may only further restrict. Trusted parent remains a prerequisite. The private_unix_modes fixture checks group/other permission bits on directory and committed file.
- published/published_bytes advance immediately after successful hard_link; committed/bytes advance only after unlink and parent-directory sync succeed. The injected unlink and directory-sync fixtures assert published1/committed0, complete visible JSON, expected temporary retention, error, disabled future writes and no retry. This closes the previously ambiguous publication accounting for handled errors. It does not establish crash persistence or survive loss of in-memory totals.
- Sequence>=256 is rejected. Existing byte/attempt bounds and non-clobber behavior remain.
- README explicitly places cold_setup_complete outside BOTH setup and candidate clocks and solver_returning after candidate elapsed. It states writer timing excludes JSON construction and Snapshot cloning. No total-overhead or clock-qualification claim is made.

The additive software-validation-review.json binds nine release diagnostic fixtures passing, twelve solver tests filtered, and strict Clippy success. Retained tests-review-release.stdout SHA44689edae5d5bd9425848ecde152869fc68cf48847fbb6da3c56974e5bd59341. Parent separately reports the same nine-fixture rerun; this reviewer did not execute it.

No remaining concrete defect was found in these requested corrections within the isolated Unix diagnostic scope. Production obligations remain: explicit bounded parent association/ingestion and counter/boundary schema validation; exact real execution identity provenance; optional adapter plumbing; checkpoint/output custody; full outer wall/observer overhead accounting; write/file-sync/process-interruption fault coverage; timeout/late-output integration fixtures; prospective source/binary/protocol binding and independent admission. Missing final IO totals are unknown, never zero. Non-Unix mode privacy is not established by this Unix-specific implementation.

This is a corrected software-source review, not production admission, scientific execution, solver parity, timing equivalence, timeout repair or cohort completion. The existing planning expiry22:08:40 UTC is unchanged.
