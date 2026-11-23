# Execution record

The previous source a78cc8906 and release benchmark executable were frozen
before edits. The local CPU/compiler manifests and original-source hashes
are adjacent. Unisolated local diagnostics inspect exact work and root
outputs only; their wall times establish no native result.

Local Conductor was initially unreachable and was restored using its
documented startup flow. Scope checks returned clear and task T-1 was
claimed before code edits. No credential contents are retained here.

New native tests verify direct F4/F5 packing against ordinary product rows,
complete roots and output caps against literal enumeration, degree/width
fallbacks, budget exhaustion, custom-node-oracle preservation and scoped
policy restoration. The three initial matrix-runtime tests pass. The new
kernel-policy test additionally checks nested/thread-local/unwind behavior.

The first release integration test completes32 independent DLP recoveries
and32 exact replays across Koblitz and generic binary curves. All are
verified. The local eleven-arm n11/d4 plumbing session then completes22
verified recoveries and22 exact replays, audit OK. These L0 timings are not
performance evidence; `local-smoke.audit.json` binds its sealed raw session.

Full suite: the first command was interrupted while waiting/building,
before tests, to restart with the required single-worker optimization
configuration. Its partial log is retained. The subsequent network-enabled
full library command enables the previous four switches and one test/Rayon
worker; its outcome is recorded only when complete.

Strict root Clippy with -Dwarnings fails on the unchanged
src/symmetric/modes/pmac.rs:112 mismatched_bit_width_type lint. That file
matches both a78cc8906 and origin/main. A separate diagnostic retains all
other warnings as errors while allowing that one existing lint; it passes.
This does not convert the original strict invocation into a passing check
or waive a CI gate. No unrelated PMAC implementation is changed.

Source0acfc55b7f freezes the implementation and protocols before native
timing. Two on-demand c7i.2xlarge jobs launched with full checkouts and a
retained649KiB patch on pushed a78cc8906:

| Scope | Job | Launch UTC | Deadline UTC |
|---|---|---|---|
| Matched native solver rerun and paired ablations | 20261019-165506-boolean-recursive-native-188e81 | 2026-10-19 16:55:06 | 2026-10-19 18:55:07 |
| Cold complete DLP recovery through ecbench | 20261019-165520-boolean-recursive-ecbenc-9d61aa | 2026-10-19 16:55:20 | 2026-10-19 22:55:21 |

Both automatically terminate and upload statuses/resource/source/binary
receipts. The first scheduled check is about50minutes after launch. No
native outcome is inferred from a launch. The later protocol prose change
from "higher-degree systems" to "nonquadratic branches" only clarifies the
already frozen implementation's behavior after specialization; it changes
neither sources, caps, dispatch, workloads nor admission.

The initial full release library suite passed4411 tests,0 failed,118 ignored;
all17 ecbench integration tests passed. Standalone generalized SDK13 tests
and strict Clippy passed. The complete30-cell first instruction batch passed.

A further representation-dispatch refinement was frozen separately in
LOOKUP_PROTOCOL.md. All20 profiles completed, with identical roots and all
checked deterministic counters: median complete-call Ir fell9.2485% for
F4 and9.1778% for F5 (five pairs each). It passed the frozen >2% gate.
Source71c4c99d85 freezes this implementation and new runners before final
native timing. The earlier two jobs remain intermediate measurements.

| Final scope | Job | Launch UTC | Deadline UTC |
|---|---|---|---|
| Full18-arm native comparison and paired controls | 20261019-174306-boolean-final-native-c19491 | 2026-10-19 17:43:06 | 2026-10-19 20:43:08 |
| Complete cold ecbench recovery and full replays | 20261019-174315-boolean-final-ecbench-25cc20 | 2026-10-19 17:43:15 | 2026-10-19 23:43:16 |

Both are dedicated on-demand c7i.2xlarge with automatic termination,
2266KiB source patches on a78cc8906814, and exact frozen specs. First final
job check is about18:35UTC. Initial jobs were checked/fetched at17:42UTC:
both still running; no raw out tree had uploaded. No outcome is inferred.

The first final-source integration command accidentally inherited the four
KIC optimization variables intended for the library suite. Its run reports
12 passed/5 failed: ecbench replay refuses unrecorded environment overrides,
as designed. That failed log is retained; no runtime source changed.
The corrected command removes those four variables and uses the exact
identity-bound policies exercised by the integration specs. Its outcome is
recorded separately. Neither dedicated ecbench runner sets those variables.

Final-source integration rerun with clean custody passed all17 tests,0
failed (including32 policy-aware DLP recoveries/replays). Final-source strict
root Clippy still fails only on unchanged PMAC; the separately scoped
all-other-warnings diagnostic passes. Rustfmt of changed runtime/analysis
files passes; analysis admission/accounting unit tests pass. The four local
reports' seven PDF pages were opened and visually inspected; hash receipt
is records/inspected-local-pdfs.sha256 and benchmark copies match.

Owner commitde185b4ded (Windows checkpoint portability) was merged without
altering the measured groebner.rs/kernel/plugin source hashes. Its Unix
checkpoint file-open behavior remains mode0600; these benchmark calls do
not use checkpoint persistence. Final timed hosts retain the pre-merge
source contract71c4c99d85 rather than being relabelled with the merge head.

The final-source generalized-rootfinder standalone package passed13 tests
and strict all-target Clippy. A focused checkpoint recovery/corruption test
run follows the owner merge; no measured solver source has changed.

At17:49UTC, PR2231 headde185b4ded had passed native-solver and Linux
integration release checks, with other checks queued/running. Older
correctness/replay jobs fail because separate research workflows overwrite
current manifests/locks with stale frozen lockfiles and then use--locked;
Cover transfer assessment fails its unrelated covers-row-count assertion.
These failures do not qualify as successful CI and no merge is attempted.

Final measured source71c4c99d85 passed the complete optimized release library
suite:4412 passed,0 failed,118 ignored,997.37seconds. The post-owner-merge
full suite runs separately after checkpoint compilation; this later
validation does not change the timed hosts' frozen source contract.

Archived-session site indexing now reads only checksum-bound session/spec/
records members, directly from the committed archive (including sparse Git
blobs); stale local extractions cannot override them. Six sparse/archive
UI tests pass, including corruption refusal. Two UI partial-price controls,
nine workflow-path unit tests and four workflow-parser tests pass. Browser
JavaScript syntax was checked with the existing portable Node22 executable.
CI now discovers archived sessions for integrity and weekly full replay,
and rejects any later rewrite of an already committed sealed archive.

An administrative integration check indexed the actual22-record sealed smoke
session directly from a temporary checksum-bound Git archive, preserving its
canonical archive/member links. No solver or measurement was run by that
site check. Owner checkpoint tests pass3/3 (including recovery/corruption).

Timing-boundary clarification, without changing the frozen spec or clocks:
the existing ecbench pipeline timer excludes adapter construction and report
metadata surrounding run_pipeline. For measured Koblitz views the adapter
borrows the instance. The separately recorded whole-child clock also captures
that surrounding work, workload construction and startup. Parent verification/
independent replay is outside both; internal pipeline verification is inside.
Only the pipeline comparison has the harness's native paired/A-A admission;
process-clock medians remain supplementary full-child observations.

Post-owner-merge complete release library validation passed4412 tests,
0 failed,118 ignored in905.10seconds. The current-branch ecbench integration
and touched-example/bin tests are rerun after that merge as a separate clean-
custody validation. No performance input/source/limit has changed.

The first post-merge integration invocation reached11 passing tests and6
failures because shared /tmp filled up (all six failures report errno28,
No space left on device). Its partial/error log is preserved. The previous
clean final-optimization suite had passed17/17. The rerun uses a private
TMPDIR on the workspace filesystem with1.7TiB free; no unrelated /tmp files
are deleted. This local scratch issue does not affect the separate VM disks.
The exec session for that storage-failed invocation ultimately reported143;
its attempted /tmp exit file was not created. That observed session status
is recorded rather than inferring a different exit from Cargo's error text.
All later fetches, renders and replay scratch use the workspace disk.

The current-branch rerun on workspace TMPDIR passed all17 integration tests,
0 failed, and the touched ecbench/example build tests. Both instruction
summaries also replay byte-for-byte from runs/profiles and
runs/lookup-profiles using the exact serde_json artifact from the root Cargo
graph, retaining arbitrary_precision/raw_value features. The storage-failed
invocation remains separately recorded.

At the scheduled18:33UTC check, both initial jobs had completed with exit0
and terminated: native at17:52:07UTC; ecbench at18:28:06UTC. Both final-source
jobs remained running. Initial outputs are fetched to the workspace disk
and analyzed separately. New completion activity resets the next final-job
check to about19:25UTC; no intermediate timings are relabelled as final.

The first local scheduling screen exited2 before creating a session because
the shared /tmp/crypto-bench.lock was not writable. Its command/error and
manifests remain in runs/adaptive-screen. The retry uses a private workspace
lock, --cpus none and --allow-busy: it is L0 correctness/work evidence only,
never native timing. It verifies/replays all16 pipeline records and completes
the original-width checks. F4 n23 target0 solver work falls from the failed
policy's3000248688 to62421379 word XORs, versus baseline79417986, with135075
calls and identical public target/query/rank/answer contracts. Small-system
native timing confirmation is still pending.

FULL_TABLE_PROTOCOL.md separately freezes a serial full-table construction/
sort path under the explicit one-worker resource contract. The initial full
Semaev-table rows mostly failed L2 on short-call preemptions; no threshold
is relaxed and improved admission is not presumed. Exact tuple/bucket/filter/
lookup and budget equality is tested against a four-worker pool.
The first full-table equality test used unsupported constructor(0,11), which
returnedNone before any table code ran. That failed test/log is retained.
The test now uses supported(0,19),(1,11),(1,23) bases; every-entry, bucket,
filter, witness and budget-boundary equality requirements are unchanged.
No performance workload/spec or admission rule changed.

Versioned adaptive screening passes:16 verified/replayed pipeline records,
exact baseline/failed-policy query/rank/answer/seed checks, and identical
original-width roots plus every checked deterministic counter. The legacy
prepared/quadratic-tail policy names retain their original scheduling for
current-binary replay. Corrected adaptive-prepared/adaptive-quadratic-tail
have distinct method identities; full adaptive specs retain all workloads,
rounds, budgets and reference/A-A arms. The pre-version L0 screen is preserved
with its exact guarded source and an explicit frozen-source replay recipe.
The full-table serial/parallel equality test passes on all three supported
bases and budget boundaries. The first unsupported-fixture failure remains.

Final corrected source9bed104b0a is frozen before timing. Two new dedicated
on-demand c7i.2xlarge jobs ship the same6374KiB patch on a78cc8906814:

| Final scope | Job | Launch UTC | Deadline UTC |
|---|---|---|---|
| Adaptive full18-arm native panel, paired holdouts and matrix/table four-worker controls | 20261019-190236-boolean-adaptive-native-d28a77 | 2026-10-19 19:02:36 | 2026-10-19 22:02:38 |
| Versioned adaptive complete cold ecbench recovery and full replay | 20261019-190253-boolean-adaptive-ecbench-cbeb18 | 2026-10-19 19:02:53 | 2026-10-20 01:02:55 |

The native launcher first received Unsupported for c7i.2xlarge in
us-east-1e, then succeeded in the next configured zone. No failed instance
was started, quota/IAM was not changed, and the resource type/budgets stayed
fixed. The next combined job check is about19:55UTC. Source71c4c99d85 jobs
are iteration2, not final. All three iterations and their failures remain.

Final-source strict root Clippy still reports the unchanged PMAC bit-width
lint (exit101). Its separately retained scoped diagnostic passes with only
that one existing lint allowed; this does not waive the strict failure.

Current corrected source passes all17 end-to-end integration tests, including
48 independently checked policy/generalized recoveries and48 exact replays
across Koblitz and generic binary curves. The standalone generalized SDK
passes13 tests and strict all-target Clippy. The complete library run remains
separate. An eight-replay legacy audit passes, but its fixed-stride sample
covers only reference/full-table methods; therefore a full old n17 session
replay is run to actually cover legacy F4/F5 identities too.

Corrected-source full release library result:4414 passed,0 failed,118 ignored
(1031.36seconds, local correctness validation only). Current-binary legacy n17
session replay passes:528 records verified,440 measured replays identical,
0 problems, including legacy F4/F5 and both table backends. Frozen-source
replay of the pre-version local screen also passes on both sessions; the
current source was restored and its full manifest rechecked afterward.

Correction to earlier screen prose/derived metadata: each local screen has
16 verified executions total, including8 warmups, and8 measured replays.
The harness's --replay-all excludes warmups. The earlier statement/key
"16 verified/replayed" incorrectly equated the two counts. Raw records and
audits are unchanged; the Rust summary and reports now state them separately.
Likewise each full ecbench curve has528 verified records but440 measured
replays, with88 warmups. Integration's48 records have no warmups, so all48
are replayed there. No measurement or admission criterion changed.

All28 pages of the current nine intermediate/local run PDFs were opened
and visually inspected after the replay-count correction. Graph labels,
missing-clock annotations, tables and repeated headers are legible; the
corresponding PDF hashes are in records/inspected-intermediate-pdfs.sha256.
Benchmark copies are produced from the same renderer. Later final/iteration2
reports require their own all-page inspection; no older PDF substitutes.

Evidence-index scope expansion was checked with Conductor and returned clear
for both existing rootfinding problem records and benchmarks/README.md. The
problem histories now distinguish bounded native/cold-wall access from their
unchanged complete-pricing, external/hard-family and independent-host gates.
An auxiliary browser preflight was initially invoked through the wrong Python
module path and failed import before executing tests; its log is retained in
records/archive-and-cost-final-preflight.log. The repository's direct script
invocation then passed all6 archive/input tests. No implementation changed.

Scheduled19:55UTC check: source71c4c99d85 native and ecbench jobs completed
with exit0 (18:40:33 and19:12:28UTC). Final adaptive ecbench completed
with exit0 at19:50:35UTC; both528-record sessions verify and all440 measured
replays per curve reproduce. Every matched F4/F5 query/rank/answer check
agrees. Candidate/baseline solve ratios are0.5622568354/0.5822453003 atn17,
and0.6308733689/0.6532104329 atn23. All four harness gates admit effects
outside A/A noise; n17 F5 has39 L2 pairs and one below-level pair, while
the other three have40. Full-table arm admission is34/40 and39/40; its
remaining below-level observations are retained. No all-L2-complete claim.
The final native job remained running. Next scheduled combined check is
about20:45UTC following this new activity.

The lookup-only iteration2 native run has all32 paired cells faster and
all72 cost cells with five admitted observations. Its n23 four-worker F4
matrix control is slower (baseline/candidate0.9632545963); the other three
controls are neutral. That negative result stays visible. Iteration2 full
ecbench still regresses: F4/F5 B/A2.16146/2.11551 atn17 and6.39625/6.08043
atn23. It is not relabelled as the corrected-source experiment.

Owner fixes3f1dd9ec3,16f61207f andf236a8c2aa were fetched by exact commit
and merged without conflicts as6f0b8518f. They refresh a frozen experiment
lockfile, the Cairn status test and the cover catalog assessment; every
measured runtime source still matches the9bed manifest. The cover verifier
first failed only because sandboxed Node could not spawn read-only Git
commands (EPERM); its subprocess-enabled rerun passes. Locked metadata for
the refreshed experiment passes. The merged Cairn integration test is run
separately; no old source capsule or sealed record was edited.

The merged backlog integration initially used the installed Cairn12.1.0,
which is outside its documented supported range, and the sandbox refused
a fake-script subprocess. With pinned Cairn9.1.0 and subprocess access,
13 of15 tests pass, including the owner's changed concurrent-claim test.
The remaining two fail on the existing quintic card's unresolved {artifact}
placeholder. Its card blob7789a2821deb79218e4144b1993ddb64bfa14c72 is identical
at baselinea78 and current HEAD; neither that card nor backlog code was
changed by this task. These strict failures are retained and reported, not
patched around or counted as passes.

All52 pages of the12 currently generated run PDFs have been visually
inspected. The ecbench comparison tables now show below-L2 pairs explicitly;
all24 pages of those three PDFs were reopened after that formatting change.
Every graph/table remains legible and the corresponding hashes are in
records/inspected-returned-pdfs.sha256. Final native and aggregate PDFs are
still pending and will receive their own all-page inspection.

Native ecbench SQL for all11 sealed sessions imports into a fresh local
SQLite index with integrity_check=ok:3222 executions,93 arms,48 saved
comparisons and zero claims. The database stays outside Git; the native SQL
exports and sessions are the source. records/ecbench-index-import.json
retains the administrative import receipt. AUDIT_CERTIFICATES.sha256 binds
the retained full/screen replay certificates; it is separate from the raw
archive's own checksum. No Python solver/statistical computation is used.

Final native job completed exit0 at20:02:08UTC and terminated. All32 paired
changes pass the native faster gate; every one of72 cost cells has five
admitted observations. All four matrix and two full-table four-worker
controls are neutral; all40 rejected attempts remain. n23 planted complete
roots: F4 baseline365.456843ms →32.499407ms, paired11.22091206; F5
371.048495ms →32.265593ms,11.50084547. Decomposition ratios3.30033588/
3.36897781. Full cold calls in the separate18-arm table retain their own
samples and the different full-DLP basis is never mixed with them.

The leaderboard regenerates all three twins with a separate latest native/
ecbench section. Every pre-existing machine value, including A–C and all
fits/qualified diagnostics, compares equal to the pre-continuation snapshot;
records/leaderboard-existing-values.json records that check. Both workflow
path filters cover all220 builder input files. No full-price row is promoted.

All65 pages of the14 final PDFs were visually inspected after final rendering.
Hashes are in records/inspected-final-pdfs.sha256; earlier12 run-PDF hashes
also verify unchanged. Archive extraction/repacking is byte-identical and
all11 Rust summaries reproduce byte for byte. All11 archived sessions pass
current-binary integrity audits with replay0, supplementing (not replacing)
the original full replay receipts. The browser indexes all11 archive members
and passes currentness,6 archive/input tests,2 cost-admission controls and
JavaScript syntax; all4 workflow-parser tests pass. An earlier auxiliary
PDF checksum command ran from the wrong working directory and failed path
resolution; its output is retained separately and the root-directory rerun
passes every PDF hash. No PDF bytes were changed to pass it.

Scheduled PR check at20:48UTC still shows head6f0b8518f open, mergeable,
40 checks queued, two skipped and two neutral, no reviews. It reports draft
again after the earlier ready transition; final publication will restore
ready status under the standing owner instruction. No required/applicable
CI suite is claimed complete and no merge is attempted.

The touched performance example compiles and its release test harness passes
(zero declared unit tests), satisfying the example check without inventing
a test count. Benchmark-copy validation matches all44 plot files and14
PDFs against the study originals. Current source/protocol flags and all
strict failures remain documented. All six measured VMs have finished;
there is no remaining benchmark job or source optimization queued.

The first staged whitespace scan also classified the benchmark-copy PDFs as
text and reported the renderer's SVG/PDF whitespace. The benchmark folder
now mirrors the study's generated-file attributes (PDF binary, generated
SVG whitespace preserved); no renderer bytes changed. The complete staged
whitespace scan then passes.
