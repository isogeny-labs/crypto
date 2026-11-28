# Root library v3: failed compile-stage attempt

The worker invoked the release-library compile command and stopped with AssertionError: supervisor permit expired. Its compiler child was reaped with exit -9. No test inventory command, any of the four exact controls, or full-suite test command was invoked. Partial Cargo artifact messages do not establish a successful library build.

The final retained permit expires at 2026-11-09 18:55:16.053503 UTC, later than the completion record at 18:55:06.630786 UTC. This does not contradict the worker having observed an earlier expired permit, but it does not identify which permit bytes were consumed at failure. The receipts do not distinguish stale read, supervisor renewal lag or another timing mechanism. This archive preserves the observed error without assigning an unsupported cause.

The compile command reports 630.008571627 seconds; worker 960.188331559 seconds; attempt 991.712660803 seconds; caller 991.841880024 seconds. These durations overlap and must not be added. The full 1800-second reservation is charged, bringing campaign charges to 12660 seconds. Native limits were 2 CPU, 16 GiB memory, zero swap and 512 tasks. Pre-input verification and fresh-cache binding 45f46580 were recorded before compilation.

Completion status is failed and completion sealing passed. The immutable supervisor record retains its pre-seal pending_seal/pending fields. Terminal output manifest sizes and hashes and durable log prefixes were checked. Worker, supervisor, retained permit and stopped-unit receipts agree on invocation 15d1304b26834459b62bd6adf6039d07. Owned cleanup is recorded, terminal MainPID is zero with empty ControlGroup, and the cgroup is absent at archival review.

The native publication receipt binds df85767861d8558043bf64c596a291c159674ca7 and admission e06e. All native and caller files are archived unchanged, with hashes checked before and after packing. No compiler or test was rerun. Structured results retain failure, seal, missing-stage and resource fields; PDF/SVG/CSV provide the same scoped observation.
