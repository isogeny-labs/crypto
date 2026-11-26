# Development qualification plan

These are fixed Rust correctness and integration checks, not the Prime31 campaign or timing admission. No new benchmark/census is authorized by this file.

## Required root-library gate

The inherited duplicate-name parse defect was repaired upstream. Attempt the required `cargo test --release --offline --locked --lib` once from this worktree, using one build job, a fresh recorded target directory, 180 seconds wall/CPU limit and 2 GiB per-process address-space limit. Retain command, source manifest hashes, stdout/stderr, exit code and interruption status. Do not broaden or repeat unchanged failure. This check is independent of the new standalone native package.

## Focused native gate

After all owned source edits are finished, inventory every compiled source and exact historical selection. Build optimized test executables and CLI binaries offline using one coordinated build (two jobs, bounded outer wall and memory). Copy test executables to an immutable directory, record hashes, execute each fixed contract once with an explicit timeout and preserve failures. Check pre/post hashes and summarize actual test counts. Repeat only after a concrete source correction, retaining the earlier failure.

The tests cover legacy input bytes, exact signed witnesses/statuses, full-V versus target-liftable domains, malformed input/denominator/limit handling, augmented-only versus dual-matrix recovery at every prefix, source-span provenance, current baseline control boundaries and independent transcript tampering. Additional fiber-aware extraction and specialized baseline contracts belong to this same coordinated gate once their code is ready.

Build envelope: `cargo test --release --offline --locked --no-run --message-format=json` in the standalone native package, two build jobs, incremental compilation disabled, 600 seconds wall time, 4 GiB per-process address-space bound. Reuse the prior dependency cache; record the exact new command and retain its output independently. No arithmetic test is executed by this build command. Copy every reported test executable and the four CLI executables before any calls. Each fixed contract receives one thread, at most 120 seconds wall time and 4 GiB per-process address-space. Compiler or test failures permit only a documented concrete correction and a new preserved attempt.

## Root-library attempts observed

Attempt 1 stopped before compilation because tracked path dependencies were not materialized. Attempt 2 used the same source after materializing those dependencies, compiled registry dependencies, and reached its 180-second wall cap with exit 124 before any library test ran. Both raw attempts are retained; neither is a passing root-library gate. A later continuation may reuse the now-populated dependency cache under a separately recorded finite bound. [pcc:unverified]

Attempt 3 continues the materialized dependency build from that cache under a 600-second wall/CPU bound, one build job and the same 2 GiB per-process address-space bound. The longer finite compile allowance addresses the observed dependency-build wall cap; it does not change any experiment cap. Root source hashes and raw output are recorded again. No benchmark is part of this library-test command.

## Scope

Test elapsed times are diagnostics only. A passing fixed correctness suite does not provide natural Prime31 yield, full solve cost, negative-search completeness outside its fixture, new asymptotics, statistical timing admission or signed PCC standing. The admitted matched cold experiment remains a separate requirement.

Attempt 3 was interrupted with observed exit 130 during dependency compilation after current-main reconciliation changed six root-library source files. Its raw log and source-change list remain evidence, not a passing gate. Attempt 4 uses the resulting stable source at b6f269c1fd57d2654fc5c8435c1f1415dae1c72c, the populated cache and the same 600-second/2 GiB/one-job bound. No standalone compiled input changed during that reconciliation. [pcc:unverified]

Attempt 4 has no terminal result. The executor subsequently reported `Unknown process id 84025`; neither `result.json` nor a finished timestamp was present. Its last retained log was dependency compilation. No exit code, timeout, out-of-memory event or source-level compiler failure is inferred. The root-library gate remains unresolved, and the unchanged command was not retried. `evidence/root-library-gate.json` indexes the four retained attempts and the separate path-addressed source-tree inventory. [pcc:unverified]

## Completed standalone qualifications

| Candidate | Source commit | Fixed contracts | Status |
| --- | --- | ---: | --- |
| Prepared fiber domains and controls | `6dd952d6b5a848f69a65afa5e00ef59e708628a5` | 103 | All passed; original receipt preserved |
| Reuse the validated tower paths | `559302c7df0cb411ce345b98b2650f33ee6e137d` | 104 | All passed; sibling v2 receipt |

Both records cover 24 fixed test executables, four CLIs and 104 compiled source files. The candidate-2 recount matches every logged case against frozen test source, all actual executable hashes against the pre/post manifests, and all inventoried source bytes against its commit. The new shifted-Dickson fixture checks both the degree-two and degree-four presentations. These are fixed-fixture observations with unsigned PCC standing, not performance measurements. [pcc:unverified]

The proof subjects are `DEVELOPMENT_RECEIPT.json` plus `evidence/qualification-review.json` (claim PCC-20261123-dfa99a), and `DEVELOPMENT_RECEIPT_v2.json` plus `evidence/qualification-review-v2.json` (claim PCC-20261123-3b68e0). Raw native records are in the two deterministic `native-qualification-attempt*.tar.gz` archives. Binary bodies remain only in the transient qualifier directories; no S3 executable archive or checkpoint upload is claimed.

The existing 138 prospective inputs remain unselected and unrun. Their parser receipt belongs to candidate 1; the second candidate preserves the staged-input contracts without regenerating those input bytes. No campaign was launched. [pcc:unverified]
