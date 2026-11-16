# Vieta Rust port: continuation receipt

2026-11-15 20:49 UTC. PR https://github.com/isogeny-labs/crypto/pull/4340
is open, ready for review. Source commit `5306b099f`; pending statement
`PCC-20261115-0b04ff` in `47b244609`. No performance run has launched.

The source, protocol, report and evidence are in this directory. Final standalone
checks: 5 unit groups plus 3 CLI/replay tests pass; clippy `-D warnings`, formatting
and draft protocol validation pass [pcc:unverified]. Correctness-only cases reach
m=4096 at 62 bits and m=1024 on P-224. `report.pdf` was rendered and inspected.
The existing root release test binary also passes the five Vieta test groups;
its full 6611-test suite is still running in `/tmp/vieta-root-test4.log`, process
22532 when last inspected. It has failures outside this module: missing stored
targetwalk records, sidecar publication checks, and initially missing Certicom
emulator source. The exact Certicom arithmetic test passes after sparse hydration;
the targetwalk records remain absent after hydration (artifact-store input).

PR head initially conflicts in shared files. The standing AGENTS workflow was
followed: Copilot was asked in comment 6901110087 to merge main without rebasing
or force-pushing, preserve all journal blocks and claim digests, and keep main's
PCC ledger as the prefix while appending the unchanged local claim body with
the official append_entry helper. Do not edit those conflicted files while its
request is pending. This separate receipt avoids a simultaneous journal edit.
No conflict resolution or signed independent standing is claimed.

Auto-merge was requested. A bounded follow-up was scheduled for approximately
21:37 UTC under AGENTS.md's 50-minute cadence. Review any resolution and rerun
affected checks; merge only after current-head gate/size/artifacts and required
review gates pass. If Copilot cannot act/times out, follow the documented fallback,
preserving all ledger bodies and immutable evidence. Record the subsequent
session entry and autonomy/lessons episode when appropriate.

The larger performance request remains open. The prospective matrix is
4 fields × 5 coset sizes (32,128,512,2048,8192), 2 warmups plus 10 retained target
calls, with two populations and four arms. Separate three-term controls exist.
The protocol is deliberately a draft: no preflight clock, source/image binding
or backend admission is invented. EKS requires missing checkpoint credentials,
live shared-budget/capacity visibility and component image/runner integration.
An asynchronous user question about isolated local measurements is pending;
silence is not an override. Local execution also needs an explicit resolution
of the S3 checkpoint requirement. Do not silently replace the requested scale
experiment with development checks or the existing ecbench fixture.

No canonical frontier, leaderboard, dashboard or admitted benchmark row changed.
No exponent, last-fall-degree bound, fastest-backend or end-to-end speed claim.
