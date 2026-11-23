# Retrospective native development execution receipt

2026-10-30 UTC. This document is written after execution. It is not a
prospectively frozen scientific experiment protocol. The earlier planning
notes were written before Cargo execution, then updated with the later tiny
CLI addendum; the combined notes are retained separately without claiming
that their final form preceded the34-test result.

Archived source revision6b6f42a2de5f1ddbbc2cb4bbf0a3fe072248d248,
crate tree712d83360e07bf9fe321c11d3367653949bd9af7. The24 isolated files
match their recorded Git blob IDs. No shared checkout/index/source/lock or
branch/PR metadata was changed by this validation task.

Cargo/rustc were absent from PATH and the inspected standard locations.
Verified the official rustup-init SHA256 and installed official minimal
Rust1.78.0 under the audit prefix with --no-modify-path. No shell startup or
system identity/configuration changes. Versions, installer output and hashes
are retained.

Executed the crate's existing cargo test --locked --release with2 build
jobs and1 test thread. Exit0:34 library tests passed,0failed,0ignored;
binary tests0 and doctests0 also passed. No source/lock edits.

A later bundled tiny CLI solve was launched to check the binary/serialization
entrypoint, whose Rust unit count is0. Its exact public fixture is p5,a2,b1,
r7,cofactor1,P(0,1),Q(3,2),slots2,fixture_seed1,stepcap2000000000.
The solve finished exit0 and produced the retained receipt reporting verified
scalar4. No separate verify process was run, and no further CLI tests were
run after the parent requested finalization.

The initial git archive stalled in a promisor-object fetch and was interrupted
(exit130). Fallback downloaded the24 pinned public raw files and checked every
Git blob ID. This is a source-retrieval issue, not a test or mathematical
failure.

No scientific timings, benchmark, asymptotic-sized recursive run, challenge
solve or end-to-end speedup is claimed. The root library suite was not run:
its manifest/lock/src are off disk here and it needs1028 source files plus
four path dependencies. This task assessed that scope and did not materialize
or build it without parent coordination. Cargo is now available via the
explicit audit-prefix environment.
