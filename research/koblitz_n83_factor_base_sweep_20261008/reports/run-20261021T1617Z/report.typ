#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:9.5pt)
= Native N83 SAT storage ownership
Diagnostic campaign2026-10-21UTC. Public retained K2048/K4096 panels,
450000conflicts/2400worker-seconds each,8GiB/zero-extra-swap. A separate
allocation diagnostic after both original1M arms completed; it does not
replace or extend that sweep's search cap. n=1 per base, no uncertainty estimate.
No instrumented clock is admitted as a matched timing comparison.

#image("storage-growth.svg",width:100%)
Requested Vec buffer capacity, logical arena words and actual SAT-worker RSS
are distinct. Process glibc in-use-plus-mmap includes non-solver storage and
uncommitted capacity; no per-buffer page attribution or allocation-stack trace.
Snapshot schedule: solve entry/return and each10000conflicts. Native checker
validates all24 components, sums, arena partitions, worker PID/RSS/HWM units,
full trace boundaries and retained conflict-counter signatures.

#pagebreak()
= Retained learned-clause storage
#image("arena-partition.svg",width:100%)
The appendix/native JSON records exact byte counts. K2048 crosses its first
clause reduction; K4096 remains before its first retained reduction threshold.
An absence of detached K4096 clauses at450K says nothing about larger caps.
Requested capacities are not resident bytes; allocator retention is not itself
a leak. No reclamation candidate has run in this diagnostic campaign.

#pagebreak()
= Owners, execution and limits
#image("terminal-owners.svg",width:100%)
Frozen worker SHA256:
#text(size:7pt)[c4b2062df1bd14e13c2c3d9adae28bdb552be103a0c6dd3f2dbc272ce3b1337d].
Worker source674d2075958c5cb388cbfcb271e8ebcd6dea646c adds read-only telemetry;
SAT algorithm/branching/forgetting unchanged. K2048 uses guard-v21; K4096 uses
queued guard-v22 after a retained infrastructure refusal. Same worker bytes,
public inputs,image,core14/30,workerCPU14,1CPUquota,8GiB/no extra swap,
strict quiet/eviction/restoration,networknone,read-only/capdropALL/64PIDs.
Host ip-172-31-26-135.us-east-2.compute.internal, Xeon6975P-C,32logicalCPU.
Missing per-worker context-switch/fault counters and fixed kernel threads
prevent automatic L2. No task build/test/compression ran during its workers.

K2048-p21 shell redirection failed before native execution: missing parent,
no receipt/worker/duration; retained director impeded assessment. K2048-p22
completed450K/zero. K4096-p21 command exited1 after10.985894322seconds,
nonwaiting lock refused before setup; all14 outputs missing. Guard-v22 adds
explicit600-second bounded interruptible lock queue before CPU planning,
with hashed policy/elapsed record, same lock/permissions and fixed worker cap.
Original refusals remain unadmitted. Exact relays, receipts, raw SAT traces,
cgroup samples,startup records and cleanup are retained outside this report.

== Evidence and next step
Study research/koblitz_n83_factor_base_sweep_20261008; proof
verification/native-allocation-profile-20261021. REPORT.md and PUBLICATION.md
link actual per-arm values, source/ELF/relay identities, native summaries,
all failures/tests and S3 PUT/HEAD/freshGET/payload/replay/repack proofs.
Graphs render native JSON from data/ with same-source150dpiPNG. Report PDF
visually inspected before publication; benchmark copies have identical bytes.
Original1M and older failed attempts retain their separately verified archives.

Canonical docs/ic-leaderboard.html, docs/index-calculus-scoreboard.html and
docs/ic/progress-timeline.json remain unchanged: no new eligible cold/rho/
identity/component timing promotion. Full cold accounting, natural relations,
rank/recovery and broader PR2036 CI/readiness remain open. Existing CMS5.17.0
known-witness external comparison remains faster; no fastest-tool claim or
new matched natural external comparison. Next: separately preregister and test
arena reclamation preserving clause/reason references and retained search
signatures, then measure before extending caps. This report grants no speedup,
UNSAT, recovery or minimum-memory claim.

#pagebreak()
= Exact terminal observations
#table(columns:(2fr,1.2fr,1.2fr),inset:5pt,
 [Bytes unless stated],[K2048-p22],[K4096-p22],
 [Requested solver Vec capacity],[4611260673],[4673328420],
 [Arena requested capacity],[4294967296],[4294967296],
 [Arena capacity / all owned],[0.9314084803637385],[0.9190381907719638],
 [Arena logical used],[2263844356],[2399593924],
 [Original clauses],[14108924],[24717564],
 [Active learned clauses],[461943344],[2374876360],
 [Detached learned clauses],[1787792088],[0],
 [Worker RSS at return],[2763960320],[3151409152],
 [Worker HWM at return],[2764464128],[3151409152],
 [Process allocator in-use+mmap],[4830762864],[5109477536],
 [XOR reason literals],[2136400],[2108032],
 [Matched conflict snapshots],[451],[451],
 [Storage snapshots],[47],[47],
 [Cgroup samples],[29],[29],
 [Greatest sampled cgroup charge],[2746028032],[3022114816],
 [Last sample to outer end(ms)],[27008.30878400011],[38622.77860700013],
 [Command wall(seconds)],[1721.016573635],[1731.917490259])

Native capacities and fractions are derived from exact recorded owner sums and
arena partitions; no Python research computation. Conflict snapshots include
zero. Both native terminals450000conflicts/models0/relations0/rank0,exit0/noOOM/
no wall censor. All sampled8GiB/swap0; sampled kernel peaks are lower bounds on
full-run peaks. Worker RSS may exceed the last cgroup sample. Original1M runs
remain complete in their own immutable evidence; these450K diagnostics do not
replace or extend them.

Actual mechanical relay SHA256:
#text(size:7pt)[K2048 c2b64ea540c876bd5336eafee62272e5555a16f6d41a2191d83b4e540b6e0ca2]
#text(size:7pt)[K4096 6d32f93a890c5a4aad92214fd3da123840438bba66f3d0a34694b3e6586c202f]
Director assessments grant execution compliance; root independently verifies
full audits, profiles, storage invariants and all retained conflict signatures.
Both read-only live Docker captures equal final raw logs byte-for-byte.
Amended conservative native-card envelope7200worker-seconds includes full2400
for K4096's refused command; actual two-worker query allowance4800seconds.
Earlier shell failure has missing measured duration, not fabricated zero.
This remains separate from incomplete historical campaign accounting.

Full release validation at code source7e8ae96c:4471library pass/0fail/119ignored;
19exporter pass;2independent model-checker pass;38native guard unit+7card pass.
Initial telemetry-specific readiness31SAT tests and old guard35+7 remain
retained. Python is used only for plotting/rendering/PDF inspection. Full native
positive/negative replay from freshly downloaded final S3 bytes is established
by external PUBLICATION.md receipts, not an assertion inside this PDF.
