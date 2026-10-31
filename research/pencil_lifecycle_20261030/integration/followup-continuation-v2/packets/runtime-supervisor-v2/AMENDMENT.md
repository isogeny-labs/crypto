# Source-only supervisor correction v2

Original v1 is preserved. This candidate repairs the independent review findings without executing the supervisor or Docker.

1. Fixture manifest binding now verifies every listed member byte hash, regular-file/link identity, no symlink components and exact file membership before Docker admission; revalidation occurs in fixture snapshots. This includes preparation.json and expected-report.json, not merely the manifest file itself.
2. Every launched client has a finally-path that terminates its exact process group, waits, escalates to kill if needed and reaps before container cleanup. SIGINT/SIGTERM are suppressed only during bounded reaping. If the client remains unreaped, container cleanup is forbidden and ownership retained to prevent a concurrent client/cleanup race.
3. Q0 compares actual directory/file kind against the expected member kind. Every expected file must be a file and must match its length and digest. Executable archives also check UID/GID0. Image-declared volumes and unapproved mount types are rejected; tmpfs values, IPCprivate, no added capabilities/device requests and default security-option contract are explicitly checked.
4. Inspection output uses an anonymous temporary file with guaranteed closure. Only redacted JSON reaches a named stdout artifact. On a malformed/timeout/exception path raw inspection bytes are not persisted under a filename. Receipts retain bounded byte count/hash when available.

Only AST parsing was performed; no software/native process checks or container actions were run. Independent source review of v2 remains pending. The proposed /opt allowlist extension remains explicit and subject to review. Exact final binding slots remain null. Configured-versus-effective pre-start PID enforcement remains unresolved, so the supervisor continues to report qualification_complete=false even after otherwise successful checks.

The outer fallback now records action state unknown rather than falsely asserting no Docker actions after a possible final receipt-write failure. /opt ancestor coverage is explicitly limited to /opt; it does not stat /usr/local/bin ancestors. Actual UID1000 execution remains a separate usability check.
