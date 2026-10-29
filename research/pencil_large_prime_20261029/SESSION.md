# Session record — 2026-10-29 UTC / 2026-10-28 Pacific

## 2026-10-29 T-796 continuation

### What was done

Added an observed System allocator with nonallocating atomic hooks. Recorded
per-phase, cold-setup, whole-candidate and rho allocation observations and
timing envelopes. Twelve library and three producer tests pass; producer
clippy passes. Independent review checked unsafe forwarding and identified
the phase-only coverage gap; the separate outer envelope addresses it.

### Decisions made

Requested layout bytes are not RSS or allocator internal copying. Preserve
baseline live-state and optimizer limitations explicitly. Do not assign an
unfrozen price to allocations or call the remaining accounting complete.
Read the official Rust GlobalAlloc contract before implementation.

### Open threads

Native index accounting, frozen protocol/export, and the upper gate remain.
EKS job-list access still fails on the configured AssumeRole call. No identity,
permissions or external workload changed; no experiment was submitted.

### Errors

Expanded JSON metadata exceeded the default compile-time macro recursion
limit; increased it to 256. An initial envelope declaration matched a test
instead of the producer loop; compilation exposed it and placement was fixed.
The final feature suite and clippy passed.

## 2026-10-29 T-795 continuation

### What was done

Instrumented symbolic arithmetic and traversal semantic events. Added solver
setup and symbolic phase deltas with an attribution invariant. Replaced full
basis scans with exact pivot lookup. Eleven library and three producer tests
pass; clippy passes. Independent review found no concrete counter or lookup
bug and identified overlap boundaries documented in README.md.

### Decisions made

Preserve the original distinction between base-field price-band operations
and scalar native work. Keep inversion multiplications and explicit events;
do not classify Miller--Rabin powers as inversions. Residual diagnostics
overlap these counters. Full accounting remains incomplete, not priced.

### Open threads

Finish native/index/allocation accounting and freeze the experiment contract.
Job inventory remains unavailable: the configured spot-job-submitter context
requests the existing remote-launcher role but its AssumeRole call is denied.
No identity, permissions, infrastructure or job state was changed.

### Errors

The EKS read-only job-list request failed during credential acquisition.
No experiment was submitted or restarted. Diem remains deadline-expired.

## 2026-10-29 T-793 continuation

### What was done

Forked the frozen pencil producer into the optional producer feature and
connected the symbolic collector. Preserved geometric relation checks and
same-process rho/source checks. Ten library and two producer release tests
pass; producer-feature clippy passes with warnings denied. No cohort ran.

### Decisions made

Require an explicit identity-column partition parameter; do not choose it
from measurements. Use new schema IDs and incomplete-cost status, with null
ratios. Remove dead legacy modes and peeling storage. The original source
and its stopping rule remain unchanged. The migration receipt was read with
existing sudo access; no external job or generator was restarted.

### Open threads

Complete graph and allocation accounting, freeze a new protocol and resource
contract, reconcile live EKS jobs, then execute the admitted upper gate and
remaining cohort. Preserve the subgroup-proof limitation in the report.

### Errors

Review caught a missing constant-only inconsistency assertion and copied
dual-norm overstatement; both corrected and a regression added. First build
exceeded the JSON macro recursion limit; removing inapplicable old solver
fields resolved it. Clippy found dead imports and a long legacy tuple type;
removed the imports and named the type. Final checks pass.

## 2026-10-29 T-792 continuation

### What was done

Added a composed relation adapter with fixed partition, shared field identity,
explicit three-large admission rejection and terminal residual caps retaining
the unincorporated constraint. Ten release tests pass, including 25,600
independent composed row-space checks. Clippy passes with warnings denied.

### Decisions made

Keep geometric verification in the existing producer. The adapter consumes
checked equations only. No partition is chosen based on observed results;
freeze it before measurements. The prior source and primary large-prime
paper remain the mathematical basis recorded in README.md.

### Open threads

The current saved four-lead review summary records complete frozen cohorts
for factor-base, projective-law and uncached Kummer, with the stated proof
limitations. Pencil remains 543/1260, with 717 open. Producer integration,
full cost instrumentation and a frozen upper-gate amendment remain required.
No cloud job or experimental cohort was launched.

### Errors

None in native tests or clippy. Existing shared daily-log reservation is
preserved; this scoped session record carries the update.

## 2026-10-29 T-791 continuation

### What was done

Added native residual row-space retention with explicit coefficient caps.
Extended all 25,600 prefix comparisons to the retained basis and added two
cap regression tests. Seven tests and clippy with warnings denied passed.
Refreshed the PDF, diagram, standardized table and checksums.
A separate read-only agent found no concrete algebra or cap bug. It identified
the missing formal composition proof and the requirement that the eventual
adapter bind identical moduli and small-column definitions. The shared daily
log remains exclusively held by T-785, so this scoped record retains the update.

### Decisions made

Caps report rejected inputs explicitly; no dropped equation counts as retained.
Counters are partial logical accounting, not full cost evidence. Rechecked
the primary double-large-prime paper, sections 2.3 and 3.1, and retained its
genus and relation-model scope boundaries. No experiment was launched.

### Open threads

Collector integration, full accounting, frozen admission and cohort execution
remain outstanding. This module has no new Lean proof.

### Errors

The migration handoff receipt is unreadable with current filesystem access;
no migration-dependent jobs were restarted. Diem is deadline-expired and idle.

## What was done

- Reconciled the existing Markowitz primary and replay receipts. Both completed
  with the same stored-coefficient cap; the ready T-193 task was stale relative
  to those artifacts. No generator, replay or EKS job was started.
- Read the frozen pencil protocols, existing SignedGraph and general
  large-prime eliminators, and the primary 2005 double-large-prime paper.
- Under T-789, implemented a standalone symbolic finite-field row reducer
  in Rust. It has no curve or scalar-recovery interface.
- Five native tests passed, including 25,600 independent dense-RREF prefix
  comparisons and six nonsaturated 32-variable component fixtures. Clippy
  passed with warnings denied. Exact logs are retained.
- Compiled four field identities in Lean 4.34.1 with mathlib revision
  d13f23b723b8a846827a245b89c10fc7d3f11612. Axiom output contains only
  propext, Classical.choice and Quot.sound. The full forest/Rust theorem is open.
- A separate read-only agent reviewed the Rust code and identified the
  caller-retains-constraints obligation and the saturated-rank test weakness.
  Documented the former and added nonsaturated and wide-field chain tests.
- Rendered and inspected a two-page PDF and its embedded editable diagram.
  Added a standardized check table and proof map. No performance claim is made.

## Decisions made

- The user's mathematical, public known-answer research scope governs this
  implementation step. Historical capped runs and their protocols are preserved.
- A signed cycle with a nonzero root coefficient creates an anchor, not an
  unconditional small-only relation. The API handles both cases explicitly.
- Existing counters cannot price symbolic payload operations. The new cost
  ratio remains unmeasured, and the 717 missing cohort outcomes remain missing.
- The canonical cross-lead plots remain unchanged because no cohort row or
  cost measurement changed.

## Open threads

- Account for symbolic payload operations and storage; freeze the small-base
  partition and admission policy; implement a verified pencil adapter and
  downstream small-row elimination before the full cohort can run.
- Full inductive forest correctness and Rust-to-field correspondence are open.
- Append this record to the shared daily log after T-785 releases its exclusive
  reservation on docs/session-logs/2026-10-29.md. That path was left untouched.

## Errors

- The first Lean command imported all of Mathlib and stalled in filesystem
  I/O. It was deliberately terminated (exit 143), then replaced with the
  two necessary tactic imports. The final direct compile passed.
- The first clippy check found two style lints, both fixed before the retained
  successful run. Native test behavior remained unchanged.
- T-789's first CLI lease expired; checked for conflicting owners and reclaimed
  the same task. A bounded lease keeper maintains the second claim until handoff.

## 2026-10-29 T-797: sparse map observations

### What was done
Added transparent tracked sparse maps and four semantic counters, preserving
numeric key order and coefficient JSON. Reviewed against official BTreeMap
documentation. Independent read-only review found lost iterator size hints;
forwarding and regression checks now preserve collection allocation sizing.

### Decisions made
This is mathematical implementation and instrumentation, not a real-world
key-recovery result. No new cohort or comparative result is asserted. Map
comparison counts are implementation-dependent; equality and tree internals
are not fully counted. Output serialization remains outside the meter.

### Open threads
Frozen pricing contract, experimental admission and EKS ownership reconciliation
remain open. The pencil cohort still has 717 missing candidate completions.
Shared dated session log remains reserved to another task; record kept here.

### Errors
Initial wrapper omitted iterator size hints; corrected following peer review.
Diem remains deadline-expired at 50/52 records and was not restarted.

An overlapping local build produced a missing-object linker diagnostic; all
builds were allowed to finish and default tests were rerun sequentially.

## 2026-10-29 T-798: phase and scalar ledger reconciliation

### What was done
Added field/native/symbolic phase sum checks and explicit interval remainder.
Moved walk-label generation into generation timing. Independent source review
found candidate expanded inversion versus reference atomic inversion counters.
Added a checked native scalar normalization and tests against both actual
inverse implementations; recorded formulas and open gates in COST_MODEL_DRAFT.json.

### Decisions made
Retain raw counts and immutable old evidence. Do not apply historical prices
unchanged to this producer. Atomic scalar inverse weights remain unfrozen;
no new ratio or full-cost claim follows from the normalization. Sources reviewed:
Ring/modpow in three_leads.rs, frozen price_sparse.rs, and producer/library pow.

### Open threads
EKS retained-job listing again failed AssumeRole authorization on the existing
job-submitter connection. No role changes or cohort submission. Full protocol,
logical/setup coverage, valuation and remaining 717 candidate completions remain.

### Errors
Historical scalar event sums use asymmetric inversion internals. Recorded the
specific discrepancy without changing saved results or claiming a revised ratio.

Independent read-only review found no formula/path/phase-ordering defect.
Prime validity remains a caller obligation. Future analyzer aggregation must
use checked sums; existing producer phase aggregation is not overflow-checked.

## 2026-10-29 T-801: execution route reconciliation

### What was done
Read isolab-eks skill, ACCESS.md, HARNESS.md, recovery reference and live MCP
schema. Installed overview succeeded. Existing documented host connection
confirmed create Jobs permission and listed 9 terminal isolab Jobs, 7 succeeded
and 2 failed. Read-only agent audited migration and saved Diem/freeze receipts.

### Decisions made
Correct prior interpretation: research in migration prose is not a namespace
assignment. The documented namespace is isolab; retained access audit maps the
existing host role to isolab:submitters. No credentials, permissions or configs
changed. Preserve Diem deadline and unmet freeze prerequisites. No Job submitted.

### Open threads
Standalone pencil CLI needs native ecbench per-target integration for the
qualified resumable contract. Source paths are src/cryptanalysis/ecbench/
three_leads.rs and methods.rs. Full protocol, image/binary identity, complete
valuation and cohort upper gate remain. Historical task EKS IDs were not found.

### Errors
Earlier turns tested namespace research and overgeneralized launcher AssumeRole
denial to all approved EKS access. Corrected using live documented connection;
other namespaces and unrelated historical experiment outcomes remain unclaimed.

## 2026-10-29 T-802: reusable solver extraction

### What was done
Moved exact arithmetic and six solver tests to native_solver.rs; solve_one
accepts instance/target/seed/config/optional observer. Driver retains archive,
known-answer checks, rho and allocator installation. Default 14 and producer
21 library tests and both Clippy configurations pass.

### Decisions made
Version 2 repeats cold setup per target, includes target-validity work, and
has a changed live-memory baseline before rho. Preserve 16 walks and rho cap
semantics. No hidden environment node ordering. Source aliases permit later
root-engine inclusion without passing duplicate backend types. No cohort run.

### Open threads
Root ecbench registration, serialization-only kernel feature, executable
protocol, complete prices, qualified recovery and 717 missing completions.

### Errors
Independent review identified hidden environment ordering, insufficient source
metadata checks and overbroad allocation-parity wording. Corrected all three.
Guard restores the arithmetic flag on unwind; panic still invalidates a run.

## 2026-10-29 15:00 UTC: resumed adapter integration

### What was done
Verified PR 2843 merged and Diem remains deadline-expired with 50/52 records.
Reclaimed T-807 for root manifests and T-808 for adapter/study files after
Conductor cleared the scopes. Added reporting-only kernel dependency.
Independent source review confirmed deterministic ledger digest coverage;
narrowed excluded keys and added a future-field regression.

### Decisions made
Keep public synthetic mathematical workloads and all original cohort gates.
Digest counter words are identifiers; group-only GAE remains a lower bound.
No cohort, cloud job, or expired Diem generator was launched.

### Open threads
Root compile and replay regression pending sparse materialization. Producer
feature test must be retried after its dependency is materialized. Existing
full-cost protocol, qualified execution and missing cohort outcomes remain.

### Errors
A producer test started while sparse checkout was materializing dependencies
and failed to find five-leads-study/Cargo.toml. It did not execute tests.
The previous source lease had expired; the new claim replaced its scopes, so
a separate task reserves the adapter files.

## 2026-10-29 15:11 UTC: producer tests and online boundary audit

### What was done
The resumed producer-feature library test command passed all 21 tests.
Independent read-only review identified the exact online-export dependency:
source.valid(target) precedes target-blind base construction and candidate
clock; current phase sums omit overhead; matched three-leads rho has no
online phase hooks.

### Decisions made
Do not relabel existing elapsed sums as an eligible online window. Preserve
all five exclusive IC phases and matched rho solve/check boundary.

### Open threads
Root build still needs isogeny_algos materialization after current sparse
checkout completes. Full online integration must address target validation,
setup ordering, solver setup, overhead and final fresh source verification.

### Errors
A concurrent git restore encountered the live sparse-checkout index lock.
The lock was preserved and the owning process was left running.

## 2026-10-29 15:25 UTC: continuous online clock and rho boundary

### What was done
Moved target validation after target-blind setup and connected the shared
solver to the existing exclusive clock. Added five-phase export sum check
and truthful cap/absence metadata. Added backward-compatible rho target hook:
plain/negation begin at first target multiplication, Jacobian at target
transfer after destination/generator setup. Producer tests: 28 pass, one
pre-existing shared-clock test ignored. Producer library Clippy passes.

### Decisions made
Query/control owns between-scope bookkeeping. Extra adapter verification
is outside pencil online, charged in cold group counts. Standalone clock
hooks remain inert without an owning session. No timing comparison claim.

### Open threads
Root cargo check is live in tool session 84181, log /tmp/pencil-root-check.log.
Root library tests, digest regression, report refresh and new PR remain.
T-810 reserves src/cryptanalysis/three_leads.rs for the rho boundary.

### Errors
The new clock test initially indexed setup in the online-only map; corrected
to check its absence and its presence in the whole-session map. The shared
clock includes one root-only Callgrind entry, intentionally unused in the
standalone crate; a scoped dead-code expectation records that fact.

## 2026-10-29 15:29 UTC: root compile and zero-coefficient boundary

### What was done
Root release library check passed (2m23s). Full root library tests started
in tool session 91415, log /tmp/pencil-root-tests.log, using the existing
/home/ec2-user/crypto/target cache. Producer suite after hook correction:
28 pass, one ignored; producer all-targets Clippy with denied warnings passes.

### Decisions made
Independent review found first rho target coefficient can be zero. The hook
now defers until a nonzero target multiple, including start and restarts.
Added root regression with a traced small group exercising leading zeros.
No actual timing experiment, cohort, or cloud job was launched.

### Open threads
Root tests remain running; new zero-coefficient regression awaits that build.
Update PDF/check table/checksums and prepare follow-up PR after validation.

### Errors
No new observed test failure after correcting the online-map test assertion.

## 2026-10-29 15:34 UTC: review artifacts refreshed

### What was done
Updated checks.csv, timing-boundaries.csv and SVG/PDF diagram. Regenerated
REPORT.pdf and visually inspected all three pages: graphs, labels and table
are legible, pending root regressions and absent new cohort results explicit.

### Decisions made
Diagram widths are illustrative, not measured durations. Historical 543/1260
coverage and unknown new candidate/rho cost remain unchanged.

### Open threads
Root library test build remains live (cargo PID 1271485, session 91415).
Reconcile that process before any retry; do not launch duplicate tests.
T-810 lease expired during read-only build waiting; reacquire before any
further edits to src/cryptanalysis/three_leads.rs. T-807/T-808 heartbeat
keeper session 35568 is bounded to 30 minutes from 15:10 UTC.

### Errors
Renderer warned about embedded PDF 1.7 versus output setting 1.5; rendered
output opens and all pages were inspected. No clipping observed.

## 2026-10-29 15:44 UTC: partial root outcomes and checkout repair

### What was done
Root replay-digest and zero-coefficient regressions passed; retained filtered
output in root-regressions.log. Full suite still live with seven failures.
Read-only audit traced five Certicom failures to missing gpu/certicom and
one bounds failure to the absent prime-toy-reference-e1 source session.
Materialization is live in session 78154; includes isogeny_algos preservation.
GPU-stage failure remains unclassified pending final panic diagnostics.
Updated and visually inspected all three PDF pages again.

### Decisions made
No duplicate full-suite or focused reruns while session 91415 is active.
Do not infer a pre-existing GPU failure solely from unchanged source.

### Open threads
Await full test and materialization completion, inspect final failures,
then focused prerequisite verification and publication checks.

### Errors
T-808 reclaim without explicit scopes produced no reservation; release
transitioned it to cancelled and reopening was refused. T-812 now reserves
the study directory explicitly. No source edits outside active scope follow.

## 2026-10-29 15:47 UTC: completed root validation and focused recheck

### What was done
Full root suite completed: 5146 pass, 7 fail, 123 ignored. Final panics
confirm five missing Certicom emulator-source failures, one missing bounds
source-session failure, and one GPU-stage test-lock conflict. Materialized
the tracked prerequisites. Same test binary serial recheck passed all 8
selected tests, including all 7 original failures. Both complete logs retained.

### Decisions made
No production-source change for those failures. Do not relabel the initial
full command as passing. Prepare reviewable follow-up PR with these limits.

### Open threads
Qualified full-cost protocol, execution admission and outstanding cohort
remain. No new cohort or cloud run has occurred in this implementation work.

### Errors
A checksum check was initially invoked from repository root instead of the
study directory; rerunning from the manifest directory verified all entries.

## 2026-10-29 15:49 UTC: delivery preparation

### What was done
Conductor reports the root manifest and adapter/engine paths clear for
publication. No open PR exists on this branch; PR 2843 previously merged.
Prepared source-manifest.sha256 for exact current integration source bytes.
Final PDF states initial full-run failures and passing same-binary rechecks.

### Decisions made
Publish a regular follow-up PR with complete validation limitations and
unchanged unknown full-cost ratio; no hypothesis promotion or new cohort claim.

### Open threads
PR checks/review and the original qualified execution/cohort objective remain.

### Errors
None in the focused eight-test recheck; initial failures retained in full.

## 2026-10-29 PR2860 formatting follow-up

Reconciled artifact-store bot head 772e8d0e8ffebf641450c1d66a44528cc27a0723.
The original root test log remains available by its artifact manifest digest.
CI rustfmt identified three formatting-only hunks in ecbench/methods.rs.
Applied rustfmt and refreshed the affected source-manifest digests.
Added the existing dependency materializer to the fmt job so its sparse
checkout can resolve the shared pencil native_solver module.
Validation: all four changed root Rust files pass rustfmt --check; all
source-manifest entries verify. No algorithm or measured evidence changed.
The local Node test command could not start because node is absent from PATH;
this is not a passing workflow test. Remote CI remains authoritative for it.
No experiments or generators were launched. Cohort completion remains open.

Follow-up: `bun test ./.github/scripts/test-ci-scope.mjs` passed all 12
tests. This checks the Node-compatible helper under Bun, not the hosted Node
runtime. Commit 95cf825152 was pushed; fresh checks are pending.
Read-only execution review identified common cold-call timing and separate
parent audit overhead as the remaining accounting boundary work. Per-arm
process RSS is already available. EXECUTION_NEXT_STEP.json preserves these
findings and the unchanged upper gate; it is not an executable protocol.
A fresh isolab Job listing had no active Jobs; no submission was made.

## 2026-10-29 common cold-cost observations (T-820)

Implemented optional child/record method_call_wall_ns around methods::solve,
preserving the legacy solve timer. It includes method setup, internal checks
and report construction. Parent mathematical verification has its own
parent_audit_verification_ns observation and does not inherit child L2.
Older records deserialize with the fields absent; absent fields are omitted
on serialization. Runner censoring still removes accepted results and audit
measurements for watchdog-expired output. TIMING_CONTRACT.json defines scopes.

Validation in progress: rustfmt and scoped diff checks pass. The initial
release test invocation used a wrong exact module filter and ran zero tests;
this is not a regression pass. Corrected build/test is live in exec session
82538, log /tmp/pencil-common-cold-test-recheck.log. Run the runner watchdog
regression and required root library suite after focused validation and final
source edits. No experimental data, timing ratio or cohort was produced.

Read-only archive verification matched SHA256
550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339,
1260 distinct workload IDs,21 curves with60 targets each, and all1260
archived algorithm seeds. Planner currently includes warmups in seed index;
T-821 implements an opt-in measured-index policy preserving legacy plans.
Native workload ID/coordinate parity still requires the compiled plan check.

Corrected focused cold-boundary test completed:1 passed,0 failed. Retained
as cold-boundary-test.log. T-821 implementation is complete: opt-in
measurement.algorithm_seed_round=measured_index preserves the first archived
seed despite warmups; default absolute is omitted from serialization.
Checked references: docs/ecbench/README.md seed pairing contract and
https://serde.rs/field-attrs.html. Added legacy identity and paired seed
regressions; these have not yet been reported as passed.
Combined required root suite is live in exec session8835, log
/tmp/pencil-cold-seed-root-tests.log. Do not start a duplicate suite.
Current code changes remain uncommitted pending validation. No new cohort.

## 2026-10-29 paired diagnostics and archive parity preparation

PR2860 merged at f680bd28038f0cc9a15fe351f506967303b03355; last read still
had the ecbench harness check running. Current uncommitted work is on
codex/pencil-timing-export-20261029.

Full release library suite finished:5154 passed,1 failed,123 ignored. Both
seed-policy regressions and the watchdog regression passed. GPU capsule test
failed on its lock; same compiled binary serial recheck passed1/1. Preserve
initial failure, not a clean full-suite label. The later addition of optional
time.online_clock_diagnostics is compiled by the native build but needs its
updated regression rerun. Root suite handle8835 is terminal(exit101); recheck
86880 is terminal(exit0). Native ecbench build is still live in handle13328,
log /tmp/pencil-native-ecbench-build.log; do not duplicate it.

Standalone paired_export uses common cold and explicit-clean online clocks,
retains absent/failing pairs and independent memory/audit observations, and
supports candidate/rho plus shared rho/rho_AA in three arms.8 tests passed;
paired_export/cargo-test.log is retained. All qualified ratios stay null until
reviewed A/A evidence for these exact windows exists. No cohort was processed.

prepare_native_parity.py reproducibly extracts seven original specifications
and a205402-byte projection of1260 archived identities/coordinates/seeds,
without planted answers. Preparation reproduced expected-workloads.jsonl
byte-for-byte. Its check mode invokes only ecbench plan --doc; native parity
is pending the build. These archived specs are validation inputs, not the
new executable protocol. No cloud jobs or experiments launched.

Native build13328 completed(exit0). Run-free native parity29938 completed
(exit0), comparing all1260 archived workload IDs,targets,target seeds/indices
and first algorithm seeds exactly. Retained native_parity/result.json, all
seven raw plans, and a payload hash manifest. No solver was run. Binary hash
0da4de11053815be21549e3d0af14efcf303fa65af2bc7f38403e87ba5bacfc1.
This is a local dirty-source validation build, not the pinned experiment image.
Updated ecbench regression suite remains live:11411,
/tmp/pencil-final-ecbench-tests.log. It covers the later online diagnostics
field and runs ecbench tests serially. Do not duplicate.

PR2860 lab parity was canceled after checkout exhausted its10-minute limit;
no parity script ran. Post-merge main run40909710263 at
17805c0db9bd92e8b1cfd10c5c10130c36aad3ce passed all three parity scripts.
Harness unit tests passed and integration tests were still running at last
read. No unsupported source repair or retry performed.

## 2026-10-29 timing and input validation completion

### What was done
Final ecbench module suite: 287 passed, 0 failed, 1 ignored. Plan-aware paired exporter: 12 passed. All 1,260 archived inputs and first measured seeds match the native prepared plans. These plans contain 22,680 executions (18,900 measured, 3,780 warmups); none was executed by planning. Logs and native plans are retained.

### Decisions made
Research scope is public synthetic mathematics. Known-answer checks are correctness controls; no deployed-key recovery claims. Standard A/A direction is rho_AA/rho with the frozen operational tolerance [0.90,1.10], not a confidence interval.

### Open threads
Commit/source/image identity, complete frozen protocol, staged EKS gate, full cohort, independent review and measured reports remain.

### Errors
The earlier full-library run had one GPU test-lock failure (5,154 passed, 1 failed, 123 ignored); its same-binary serial recheck passed. Preserve both logs.
