# Preserve SAT capture-negative evidence and add a budgeted positive fixture

Frozen after combined-dataset-03 terminal101 (476pass/1fail/2ignored), before edits.
The new positive worker/export test hit StreamByteLimit at occurrence0 with the
existing1MiB output allowance. capture_limits derives only128KiB stream and128KiB
payload allowance from1MiB; no map was registered for positive export admission.
Both source guards passed. Do not treat this failure as a completed SAT sample.

Preserve the original1MiB occurrence0 as a named negative native-worker regression:
require explicit StreamByteLimit failure, false solver completion/recovery, no
promoted gate-map artifact and retained unregistered partial bytes. Add a separate
positive four-occurrence test using the existing native CLI fixture64MiB output
allowance (8MiB stream/payload), bound consistently in run.json and capture limits
before capture/execution identities are created. Same seed19/n7/m2/work16/base4,
both policy origins/twists and actual worker/collector/replay/export remain.

This changes only test helpers/assertions, not production caps, solver work,
semantics or acceptance criteria. Existing enumerate helper defaults stay1MiB.
Keep original failed source/log/receipt immutable. Candidate06 composes the test
correction with candidate05 donor repairs; candidate05 remains unexecuted because
it shares the observed incorrect positive-fixture allowance. No scientific trial,
cloud spend or full acceptance claim follows.
