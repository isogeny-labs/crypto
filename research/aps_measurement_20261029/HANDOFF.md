# APS measurement instrument handoff

## 2026-10-30 — Native adapter and controlled-input support

### What was done (commands, files, features)

Continued merged PR2803/PR2867 with native `ic.aps_v1`. Added opt-in core
relation visiting/recovery phase observers, preserving existing recovery's
no-observer path. Added explicit bounded parameters, plan-time orbit/refusal
checks, closed setup/five-phase online clocks, retained abort/retry/partial
records, canonical actual-system exports and separate ordinary-residue witness
checks. Late verified witnesses are retained while denied timed-success status.
Partial F4/root counts reach the PDP native ledger; all missing costs remain
unpriced. No sixfold search quotient is claimed merely from orbit expansion.

Thirteen new development cases cover recovery/paired roots, corruption,
pre-transcript aborts, wrapping seeds, planning, visitor errors, late witnesses,
incomplete bitop pricing, numeric method identity and plan-only control specs.
No measurement session, performance/EKS job, funding request or source checkpoint
upload was launched. Draft specs/protocol are not a frozen held-out experiment.
The source-linked diagram and one-page PDF were rendered and visually inspected.
PDF font subsetting retained extracted text exactly after correcting diagram
labels that exposed a renderer ligature-ordering issue; build script keeps that
assertion and the source figure. Final PDF is below store thresholds.

### Decisions made (and why)

Instrument the existing algorithm before claiming an improvement. Charge every
attempt and all intervening control work. Keep mathematical witness validity
separate from budget success and from root completeness/cost attestations.
Preserve the fixed-fixture replayer's original scope and existing caps. Core
native operations remain incompletely counted, and the standard bitop ledger
must remain incomplete. Per-attempt degree/matrix diagnostics are preserved;
unknown solver-only wall/memory figures are not invented as zero measurements.

Use scoped APS handoff and normalized verdicts/source pins. Shared full daily
log/problems index, detailed dependency/build logs and host metadata remain
unapproved for publication. No benchmark or canonical progress chart gets a
speed value from development tests. No new theorem or stronger Lean claim.

### Open threads (unfinished, pick up next time)

Final validation is active: adapter tests, existing APS/F4, full release library,
fixture example, ecbench integration and strict Rust1.98 lint. Only the initial
source boundary has an observed verdict so far:8passed/1failed; its source pins
and normalized failure are retained in validation/. The failed callback test's
first base had no roots. Its rejection assertion remains; the input now uses
the shipped positive third base. Do not claim final passes before actual exits.

After validation, review and publish only scoped source/docs/evidence, open a
regular PR and enable guarded MERGE auto-merge. Respect gate/size/artifacts,
reviews/protections and authoring check-in cadence; no foreground CI wait.
T-9 remains the continuation; T-8's cancellation during lease recovery is
preserved, not rewritten as success. Complete-cost panel remains open.

Practical measurements still require complete claim-appropriate accounting,
native experiment.export/v1 and metric/full-set validation, a reviewed held-out
four-size ladder and frozen protocol/analysis/seeds/controls/budget, mandatory
successful source checkpoint, and maintained EKS L2/A-A/replay admission. Current
control specs are development plans; they need the frozen experiment's full
reference/control set and are not substitutes for the requested panel.

### Errors (what went wrong, how it was handled)

Initial sparse add required a profile. Bulk maintained sparse restoration and
CI materialization attempted large lazy downloads, exhausted the disk and
exited before tests. No assertions/tooling were changed. Removed disposable
debug cache and orphan temporary Git packs after verifying no active Git
writer; indexed objects, release binaries, source and evidence were preserved.
Restored required inputs by matching tracked Git hashes to local copies and
fetching only pinned missing files through GitHub. Restored a disk-truncated
new source file from its intact staged blob, reapplied changes and formatted.
Interrupted sparse-view absences were marked skipped, never staged as deletions.

Automatic approval review rejected terminating the broad sparse-checkout process
group because it could interrupt the coding agent/writes. That action was not
retried or bypassed; both restore commands exited naturally. Node metadata setup
also hit ENOBUFS on an overbroad file listing and sandbox EPERM; narrowed the
path list/buffer and used the supported process-capable permission. No hidden
configuration/test bypass. All raw setup/build failures remain local.

The stale Conductor lease first required normal local API outage recovery.
Default native release cancelled T-8; terminal reopen was refused, so T-9 was
created as an unchanged continuation. After disk-full interrupted its heartbeat,
T-9 was explicitly released to ready and reclaimed with a new wrapped session;
no new cancellation, false completion or database bypass. The service has no
scheduler/workers/GitHub/notification/model polling. Shell-session checkpointing
reported unsupported harness; no source upload was attempted.

The mandatory source/worktree S3 checkpoint had previously been rejected as
unapproved source-code egress. Generic continuation/measurement requests are
not specific approval. Do not retry, reroute, shrink or substitute another
upload mechanism. Rendering first used an incorrect directory; its failure log
was retained and the source build corrected. A first font-subset text comparison
failed on diagram ligature extraction; labels were fixed and the unchanged exact
text-equality check then passed. No validation failure is erased.

## 2026-10-30 — Comparison-bias review correction

### What was done (commands, files, features)

Cancelled the next build with exit130 before tests after source review found
that the adapter revalidated its full base per streamed root. Preserved its
source pins, private compiler log and local source snapshot. Added a private-
field, immutably borrowed RecoveryContext, validated once for both stopping
policies; per-candidate checks remain. Added a reuse/refusal control. Fourteen
adapter tests are now planned. Added rho A/A alongside APS A/A in both static
control specs; all seeds remain paired. No test result is claimed for the
interrupted build.

### Decisions made (and why)

Match existing find_relation validation hoisting instead of charging streaming
an artificial repeated base check. Unexpected construction/solver/check errors
stop the execution; sampling/orbit failures can retry with all work retained.
Preserve prior validation boundaries rather than presenting cancellation as pass.

### Open threads (unfinished, pick up next time)

Rerun affected and required validation on the corrected frozen inputs. Native
pricing/export/protocol, held-out scaling and all upload/launch gates remain
open. Hybrid/cover timing support remains outside the current adapter.

### Errors (what went wrong, how it was handled)

Managed-environment reconnect ended the terminal-backed coordination wrapper;
resumed T-9 through native ready/reclaim at epoch3 with a nonterminal wrapper.
The old tool-session identifier no longer resolved after reconnect, so checked
actual processes and exit files. Sent SIGINT only to the verified owned Cargo
process, not a process group. Its compiler then exited naturally; a guarded
cleanup command sent no signal because the process had already become a zombie.
The prior broad process-group termination rejection was not retried or bypassed.

## 2026-10-30 — Public integration and validation-input recovery

### What was done (commands, files, features)

The frozen adapter boundary passed 14 tests; existing APS and F4 subsets passed
34 and 67 respectively. Added three public-dispatch integration controls for all
six supported encodings, matched certified-orbit pruning, and planning refusals.
The existing ecbench CI integration step now includes the new target. Its event
triggers, permissions, checks and assertions were preserved; actionlint passed.

### Decisions made (and why)

Keep integration controls separate from performance sessions. No development
duration is a practical speed/scaling result. Preserve the complete library run
and all failures; do not substitute focused passes for a full-suite verdict.

### Open threads (unfinished, pick up next time)

The full library run and remaining example/integration/lint checks are pending
at this entry. Main has advanced separately; any integration requires reviewing
both sides and validating the touched result. Final receipts supersede these
intermediate observations only when actual commands exit.

### Errors (what went wrong, how it was handled)

The library run reported five device-test failures and one historical-bounds
failure because tracked emulator sources and a recorded session were absent in
the sparse view. Verified and restored 18 exact Git blobs from the existing
local checkout; no source, historical record or assertion was changed. The six
device controls and historical-bounds control then passed separately. Preserve
the original full-run result and distinguish those targeted reruns. Two early
diagnostic invocations used an incorrect binary path and then an unavailable
proc link; their failures remain private. The correct binary reproduced the
missing-input failures before restoration.

## 2026-10-30 — Lint correction and coordination boundary

### What was done (commands, files, features)

Cargo's public APS integration passed 3/0, the fixture example passed 10/0,
and existing ecbench integration passed 22/0. Strict Rust 1.98 lint exited 101
on one manual-divisibility idiom in the new witness checker. Preserved its
original source pins and log digest. Replaced the expression with the equivalent
standard integer predicate; corrected lint and affected adapter/integration
checks are running on the separately pinned inputs.

### Decisions made (and why)

Do not suppress the warning or weaken acceptance/assertions. The active broad
library binary predates this lint-only change: its eventual verdict belongs to
the pre-lint boundary. Do not call it a corrected-head full-suite pass.

### Open threads (unfinished, pick up next time)

Collect the actual corrected checks and original full-run verdict, finalize
normalized receipts and publish the scoped PR under the unchanged protections.
The full measurement task and complete-cost panel remain open.

### Errors (what went wrong, how it was handled)

A request to refresh the private resume file conflicted with T-5's existing
exclusive .resume territory. Left that file unchanged. The live T-9 session's
native scope API reaffirmed permission for its existing study/adapter scopes;
no competing scope was overridden. T-9 is labelled blocked_conflict after the
denied expansion, with its own lease still renewing. Preserve this coordination
state; do not turn partial instrumentation into completed measurement work.

## 2026-10-30 — Audit policy and online-boundary review

### What was done (commands, files, features)

Classified this exact cooperative-deadline method as nondeterministic in the
native audit registry. Added a fourth public integration control: a one-round
L0 correctness scratch session checks honest APS records and replays only the
rho control. Static APS audit is not an independent APS count replay. Existing
method classifications, replay assertions and frozen exemptions are unchanged.

Review also moved live-target Montgomery conversion after the target_query
boundary; all target-dependent conversion/validation is now inside online time.
Exposed missing algebra reports as unknown diagnostics and made the existing
post-run aggregation/serialization exclusion explicit. Rebuilt the source-linked
one-page report, retained exact text equality during font subsetting and visually
inspected its final page. No benchmark figure or quantitative claim was created.

### Decisions made (and why)

Preserve each validation boundary. The predicate correction passed both strict
lint versions, 14 adapter tests and the three directly executed public controls.
Its Cargo driver was cancelled during collateral binary builds for audit review.
The audit correction passed both strict lint versions; its next build was
cancelled before tests after the online-boundary issue was found. Current final
inputs include both corrections and are pinned separately. No stale pass is a
current-source full-suite result.

### Open threads (unfinished, pick up next time)

Final validation script validate-query-correction.sh runs strict Rust1.98/1.99,
adapter, audit regressions, APS/F4, four public controls and example tests.
Collect actual exits, finalize normalized receipts and publish the scoped PR.
The original broad pre-lint suite remains active on one unchanged isogeny test;
its six missing-input failures have separate passing reruns. Preserve its final
verdict when available, without substituting it for a later source boundary.
Native unified export/independent replay/accounting and protocol/launch gates
remain open. No checkpoint upload, performance/EKS run or research completion.

### Errors (what went wrong, how it was handled)

The prior audit registry defaulted unrecognized wall-budget methods to Always;
APS reports false under its differently named budget parameter. Corrected only
this new method's classification and added a real harness control, without
removing checks. Interrupted only verified owned Cargo/compiler PIDs to avoid
validating stale source; no process-group signal or rejected broad-kill retry.
All cancelled builds, original failures, raw logs and source pins are retained.

## 2026-10-30 — Final scoped validation

### What was done (commands, files, features)

Final frozen inputs passed adapter14/0, audit regressions43/0, APS34/0,
F4 67/0, public integration4/0 and fixture example10/0. Both strict Rust1.98.0
and1.99.1 lint runs passed; formatting and workflow actionlint passed. The new
native correctness control accepts honest, incomplete APS records and count-
replays only rho. No native APS cost replay is claimed. The updated APS-only
problem record preserves the original panel and exit criteria.

### Decisions made (and why)

Publish the scoped implementation and normalized receipts as measurement
support. Preserve all original failures and cancelled boundaries. The broad
pre-lint library run finished5397/6/123 with exit101; all six failures were
confirmed missing-input errors and passed in targeted reruns after restoring
18 exact tracked blobs. Do not combine these into an invented passing full
run or call it a final-source full suite. No test remains in flight.

### Open threads (unfinished, pick up next time)

Open the regular scoped PR, enable guarded MERGE auto-merge, and follow the
actual head/reviews/required checks at repository cadence. Main advanced while
validating: preserve its changes, ask Copilot once on an actual conflict, and
validate any integration result. The experiment, native unified export,
independent metric/full-set replay, complete accounting, frozen held-out ladder,
checkpoint approval and EKS admission remain open. T-9 is not complete.

### Errors (what went wrong, how it was handled)

The broad suite's original final output confirms missing tracked emulator
sources in all five device failures and a missing historical session in the
bounds failure. Their originals/digests remain intact. The prior build, lint,
fixture, scope and cancellation errors retain their exact boundaries. Shared
full logs/index and raw dependency/host data remain unpublished; no denied
upload or performance/EKS run was retried.
