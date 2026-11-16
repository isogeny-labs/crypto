# Vieta Rust port: continuation receipt

## Substantive continuation — 16 November, 16:35 UTC

User requested continuation after the closeout. PR4410 now carries opt-in
Karatsuba/Newton arithmetic and the completed matched continuation panel.
See continuation-20261116/RESULTS.md and report.pdf. Main192 calls all succeed;
supplement12 calls have4 lookup successes/8 polynomial30s deadlines. Wide scalar
replay matches196 successful calls, retaining8 failures [pcc:unverified].
Conditioned field-multiplication ratio schoolbook/Karatsuba is2.1469 at
word62/m16384 and1.8604 at P224/m2048. Successful queries invoke zero Newton
divisions. Seven unit groups/four CLI tests, clippy and formatting pass;
root focused build hit its explicit180s preparation cap, no full-suite pass.
All clocks belowL2. Measured qualifier/analysis are pinned to source9814b4bb5c;
no build/test/heavy analysis ran during the measured interval.

The sibling SPLIT_TOWER_BOUND.md gives a hand conditional extension to full
split rational-map towers, d_F<=2*sum(q_i−1)+2, and an interpolation-free
homogeneous evaluator construction. A separate Rust F17 symbolic example is
retained. It is not a general rational/isogeny solver implementation or formal
validation. Original pinned proof/data/protocols remain unchanged.

Theoretical statement PCC-20261116-480062 and data statement80ec09 are registered
and unverified. Completed reports, final local receipts and lessons are committed.
A retrospective registration correction is documented in
continuation-20261116/REGISTRATION_ERRATA.md; the original closed record is unchanged.
Main4b2d25c933 is merged with both journal block multisets and exact ledger prefix.
Remaining: push final head, update PR4410 and inspect its actual gates. No
measurement active. Retain original closeout notes below as history.

## Publication closed — 16 November, 07:30 UTC

Results PR4358 merged at03:20:47 UTC as a79fc35e35c1be023e8c862cb74128b356c4ef30.
See CLOSEOUT.md and scale-20261115T2053Z/closeout-20261116.json for actual
job outcomes. Required jobs failed after merge; this is not a green-CI or
independent-validation claim [pcc:unverified]. Scientific observations and the
hand argument retain pending standing. The results follow-up automation is
paused; no benchmark rerun or cloud experiment occurred. Older continuation
receipts below are historical.

## Current follow-up — 16 November, 03:14 UTC

PR4358 had no Copilot response after four hours; generic autofix lacked its API
key. The documented timeout fallback is merging main222394077d locally, with
exact main ledger prefix, two unchanged appended local bodies and complete
journal block multisets. See results-merge-20261116T0311Z.json in the scale
directory and today's session log. Sources/protocols/claims/data are unchanged.
Merge9c9801cd05 is pushed and PR4358 is mergeable again. Native formatting,
clippy -D warnings, five unit groups and three CLI tests pass. Ledger1190,
lessons170, relevant two censored records and progress checks pass locally;
claim-file and pinned-proof hashes agree [pcc:unverified]. Receipts/logs are
postmerge-checks-20261116T0314Z.json and postmerge-*-20261116.log in the scale
directory. Current-head merge gates started; auto-merge remains enabled.
Next bounded follow-up: inspect the then-current head and required reviews,
record merge if landed, otherwise diagnose actual failures. No benchmarks needed.

## Current state — 22:15 UTC

PR 4340 merged (merge 5181b305c62eb8032b7f0ea5e0b81a6d6a14eb31).
User authorized local isolated execution and Git-only checkpoints. The full
matrix plus a separate fixture repair is now complete; see RESULTS.md, ERRATA.md
and reports/local-scale/report.pdf. The older receipt below is historical.
No benchmark is active and no further run is needed to reproduce the report.
Both bench records are closed censored; no admitted canonical row or clock
speedup is asserted. New results branch is codex/vieta-local-run-20261115.
Native formatting, clippy and 5 unit/3 CLI checks pass [pcc:unverified].
All 2,184 successful rows replayed without mismatch [pcc:unverified]. Twelve
Vieta deadlines and twelve original missing three-term calls remain recorded.
The proposed last-fall-degree bound is a hand argument awaiting independent
or formal validation, not a running-time claim.

Results PR [4358](https://github.com/isogeny-labs/crypto/pull/4358) is regular,
mergeable and auto-merge enabled subject to gates. Shared metadata were
reconciled, statements registered (6a8b11, 74f8a5; both unverified), and reports
pushed. Check current-head CI/review and required gates on the bounded cadence;
do not rerun the finished benchmarks. Old follow-up automation was paused when
this active work resumed and is repurposed for the results PR at handoff.

## Historical receipt

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
