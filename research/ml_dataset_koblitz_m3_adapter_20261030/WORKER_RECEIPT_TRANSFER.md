# Worker receipt transfer

A private WorkerSolveOutput carries existing SolverData plus moved, already stored root Artifacts. The shared worker preserves this output outside the telemetry closure, so failure in final telemetry storage cannot erase successful solver receipts. Stored records are validated and adopted before pending-path validation; all pending names are validated before any pending write. A failure in validation or writing therefore retains already acknowledged roots.

Existing reserved names are unchanged, and duplicates are rejected across stored items, pending payloads and the current registry. Receipt transport capacity is admitted through an original-budget callback. The worker reserves its registry capacity before solver entry as separate worker setup; this is not a claim of complete adapter/worker memory or instruction accounting. No public SolverData or types.rs change is needed.

Controls use real stored files and cover reserved/duplicate identities, later pending write failure, and reserved/duplicate pending paths after a valid stored receipt. They require prior receipts to survive and no pending files to be created on validation failure. Existing verified-original worker behavior continues through an empty stored channel until native runtime dispatch is wired.

Reviewed sources are the existing worker registry writer, immutable storage, telemetry::measured finalization, and WORKER_HANDOFF_CONTRACT.md. This shared runtime change is validated with the broad ml_dataset suite; runtime Koblitz activation and central replay remain separate gates. No cloud job is launched.
