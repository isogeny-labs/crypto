# Independent clock/preparation v5 source review

Read-only 2026-10-30. Reviewed runner-clock-v5-increment.patch SHA42c0752b099b309099018e3f88756f2af1bbde08c0340891b3895521458fefea atop v4 plus fixtures. No source changes, compilation, tests, solvers or cloud execution.

## Binding gap

session-preparation.json publication returns an exact-byte SHA, but the caller discards it. Neither resource journal nor final session currently binds this exact preparation SHA. Equal session/runner/clock identifiers associate the objects but do not cryptographically bind origin/namespace metadata to the journal. Before admitting external alignment, retain an immutable manifest binding both exact preparation and final-journal hashes, or require the authenticated external observation to bind both explicitly. A hash of journal alone cannot authenticate the auxiliary raw-clock origin. This does not create false closure today because no external exit is supplied; the metadata's alignment flag is conditional capability only.

## Clock arithmetic and identity

Linux origin uses CLOCK_MONOTONIC immediately before reading boot/time namespace metadata, before spec parsing. Conversion rejects negative seconds/nanoseconds, nanoseconds outside range and u64 overflow. Every offset checks boot ID, time namespace link and raw offsets text, subtracts the origin with checked arithmetic, and rejects regression against the last successful observation. Equal readings are accepted. Identity mismatch and unavailable proc metadata fail closed; they do not create zero times. An unsupported kernel/proc configuration therefore cannot silently receive alignment qualification.

Same verified clock/boot/namespace and an authenticated external receipt remain requirements; CRIU restore alignment is expressly unsupported. This code does not independently prove that a later observer met them. CLOCK_MONOTONIC semantics must remain named explicitly; no conversion into active CPU/worker or physical UTC elapsed time is warranted.

## Scope and compatibility

optsNone does not construct/read this diagnostic clock. Historical scientific method/timeout paths stay unchanged. With diagnostics, origin excludes CLI/process startup and input acquisition but includes parsing/lock wait/planning/host/preparation up to execute entry. Lock wait is a subset, not an additive charge. execute_entry records offset before directory creation and subsequent preparation-sidecar publication; that work lies after the labeled preparation interval, not inside it. Per-execution marks still have their explicit smaller scope. No summed nested cost total is emitted.

Preparation sidecar binds launch/spec/plan/host hashes and session/runner/clock identifiers, has active metrics null and final tail pending. Early failures before sidecar publication may produce no preparation artifact; they are unobserved, not zero. Existing disabled preplan diagnostic gate means this remains prospective wiring, not an executed instrument. The raw underflow/regression fixture is source-only; Linux proc identity failure and cross-namespace checks still need software validation if claimed tested.

No other concrete delta defect identified. Compilation, fixture results, external custody, whole-process accounting, restore behavior and scientific admission remain separate.
