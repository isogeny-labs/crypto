# PR2949 integration: scoped checks passed, full library incomplete

The latest reviewed merge is `2acf618dd960011237d6628d32edc1ea5e018785`, with
parents `e0963790c613b6dcb3c018556fcb59fa0b48a4d8` and main
`954c0bb023f33657fe3cb6b95720f572655d0b12`. The earlier reviewed merge is
`4e063aa5bb048bd474c967d7c000aee741d51967`, with parents
`507316c7fc9a00afdc63f857e65b02fa3a109595` and
`af6ed463f556d26550d21a8c6c0191c172fd79ed`. Copilot request
[6516309991](https://github.com/isogeny-labs/crypto/pull/2949#issuecomment-6516309991)
had no response or visible resolver activity for over an hour; the documented
fallback explanation is
[6517345192](https://github.com/isogeny-labs/crypto/pull/2949#issuecomment-6517345192).
Auto-merge remains paused pending newer-main resolution and required checks.
Continuing the same unanswered-request fallback for the main refresh is
explained in [6519790949](https://github.com/isogeny-labs/crypto/pull/2949#issuecomment-6519790949).

Only the daily log and problem index conflicted. All 221/732 parent log
paragraphs and all 86/87 parent problem links are preserved. The prepared
publication handoff was appended after resolution. All 58 other original PR
blobs were verified unchanged through scoped validation. The active JV problem
now has appended current evidence; its original body and the other 57 blobs
remain unchanged. No frozen evidence conflict occurred. The historical
source maps were checked at their own recorded revisions; old Cargo hashes
do not validate current dependencies. The resolution review and preservation
records here retain those checks.
The refresh conflicted only in the daily log and preserves all 824/742
paragraphs of its two parents. All 58 original non-log/index PR blobs were
unchanged at that refresh. The main change adds an opt-in CLI contract feature
and its tests/documentation; that Cargo configuration has its own checks below.

Fresh scoped Lean validation at the earlier merge passes 13 declarations, zero trusted exceptions;
the final affine module builds and kernel-replays against its imports. Local
size and artifact checks passed on the merge commit. The later handoff commit
changes only validation metadata and the append-only log.
The refresh changes no Lean project, dependency or gate object; these match
the earlier checked versions exactly. No repeated Lean run is claimed.

The first native validation was interrupted during compilation, with no test
summary or exit status. Supervisor `14000` is absent, session `49270` is unknown,
and its Cargo/compiler processes are zombies with Linux wait status 9
(SIGKILL). The cause is unknown. Its exact 279-byte compiler log and interruption
record are preserved here; no later validation phase started.

The first detached retry reached a recorded terminal result. Its directory is
`/workspace/pr2949-integration-20261030/retry-20261030T0838Z/`, supervisor PID
`15115`, now a terminal zombie. All 20 example tests pass (12 new plus 8 legacy),
as do strict lint/formatting, CLI build, expected checkpoint refusal, and fresh
backlog validation. Library compilation exited 101 because two tracked APS
`include_str!` fixtures were missing from the sparse checkout; the library suite
did not start. Both exact Git blobs were restored without changing tracked
bytes. `native-retry1-result.json` preserves all outcomes. The native stdout's
exact bytes remain as the prefix of `retry1-native-tests.log`; its appended
capture note and reconstruction hashes are in `retry1-capture-packaging.json`.
The original receipt binds the exact capture at commit `2acf618dd`.

The refreshed validation has terminated, in
`/workspace/pr2949-integration-20261030/refresh-954c0bb02/`, supervisor PID
`16905`, SID/PGID `16905`, parent 1. It checks 16 committed source/fixture
objects and tracked worktree cleanliness before each phase. Its recipe,
bindings and launch snapshot are versioned in `main-refresh-review.json`.
Its supervisor and timeout wrapper exited 124 at the library execution cap.
All phase statuses and actual counts are in `final-receipt.json`; no duplicate
attempt was started. The sequence covers three
example suites, strict lint/formatting, compiled CLI refusal at the unchanged
checkpoint gate, fresh backlog validation, and a fresh library build/run.
Only the library execution, after compilation, has a 2400-second cap.
`RAYON_NUM_THREADS=8`, Cargo jobs 2, serial test execution. These are development
checks, not measured scientific panels.

The refreshed Cargo configuration now passes its own 20 example tests (12 new
plus 8 legacy), strict scoped lint/formatting, CLI build/refusal and fresh
backlog validation. The refusal message was checked as well as its expected
exit 1. Its library no-run build passes after exact fixture restoration.
`refreshed-scoped-checks.json` binds these completed phases, captures and six
binary hashes to the 16 unchanged source/fixture objects. Outputs retain exact
captured prefixes, with explicit footers/hashes where needed for final blank
lines. These are development checks, with no admitted timing or cost result.

The 5646-case full-library execution ran from 10:50:16 to 11:30:16 UTC and
hit its 2400-second execution cap: 2208 observed passes, zero observed failures,
47 ignored, the unchanged JV case active and 3390 cases not reached. The timeout
wrapper exited 124; Cargo's direct exit is unknown. There is no final suite
summary or full-library pass. All validation processes are terminal.

The 214615-byte raw log is NOT_UPLOADED, SHA256
`d41f2749c59315e4f1c11d5315c212e67489ab983662dd94633aabc39c0977ca`.
It remains outside Git; the final receipt binds a bounded annotated excerpt
and records missing durable upload without a fake store manifest. No raw bytes
were compressed or split to evade policy. The earlier 20-test receipt remains
separate and belongs to the previous Cargo configuration. The prior
5553-case cap remains historical: 2173 observed passes, zero observed failures,
47 ignored, one active and 3332 not reached; wrapper exit 124. Its raw log
remains NOT_UPLOADED. Keep new run logs separate, preserve failures/caps and
store large raw logs through the existing artifact policy when access exists.

The final integration receipt preserves these actual outcomes without
overwriting any earlier snapshot. All 16 source/fixture bindings and six binary
hashes were rechecked. The JV source is identical to the original `8baeee2d`
snapshot. This publication supplies scoped software validation with an explicit
full-library limit; it does not turn that limit into a pass.

Newer main `da7a63b11b486861bbc0ee6ac7695a9c756486f8` changes the
`hashed-ecdl-tensor` dependency and has not been integrated into the tested
snapshot. Its resolution needs review and fresh affected validation. These
receipts do not validate those changes. Inspect exact-current-head `gate`,
`size`, `artifacts` and review state before merging; re-enable auto-merge only
after resolution validation, with an expected-head guard and no bypass.

GitHub autofix run `41019218473` failed its `Require ANTHROPIC_API_KEY` step
for this PR and two others. That unavailable service credential does not
establish a native code failure. No credential or access configuration was
changed, and the required merge checks remain `gate`, `size`, `artifacts`.

The full G1–G6 research goal remains incomplete: G1–G3/G6 partial, G4/G5 unrun.
There are no admitted cost, scaling or speedup results. Checkpoint/artifact/lab
access, qualified native execution/resume, complete pricing, external panels
and the Rust/polynomial/cost proof bridges remain open. The existing panel card
stays blocked and unfunded. No scientific launch, funding, lease, IAM, quota or
pool change occurred.
