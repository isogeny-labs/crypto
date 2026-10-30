# Requested ECDLP ML scope: actionable coverage ledger

Read-only audit, 2026-10-30. Source observed at factor-base branch HEAD
`e1ffaf5b81b605a8f117f2e5f15f756ec65905d4`; exact inspected bytes are in
`source-inventory.json`. No builds, tests, generator, cloud calls or tracked edits.
Historical pass counts in the documents are not new current-source validation.
The session log says standalone factor-base attempt02 is active; this audit did
not inspect or classify its eventual result. Verified cloud dataset cells remain
zero in the current status records; this is a documented count, not a fresh S3 audit.

## Immediate delivery boundary

The strict v2 plan and signed-Frobenius-union constructor/replayer exist, but
current CLI paths still use `DatasetPlan` v1: mod.rs:978,1487,2043; startup_gate.rs:1291;
report.rs:482; restore.rs:394 delegates to the v1 bound_run; export.rs:920 obtains
legacy cells. Worker construction at mod.rs:1274 still calls curves::generate,
not curves::generate_with_policy (curves.rs:755). Therefore policy APIs are not
an end-to-end v2 dataset feature yet. Generation, preflight/start, resume/verify,
status/coverage, publish/restore, budget binding and export must all normalize
one exact plan/cell/policy identity and reject policy substitution. Preserve v1
bytes/IDs and explicit v2-legacy origin. The parent's existing binding work is
part of this critical path; do not implement a second competing normalizer.

## What a neural network can currently be taught

| Target | Replay-backed implementation | Remaining boundary |
|---|---|---|
| Static PDP routing/domain structure | Public curve/G/Q/concrete domains and exact prime-chain equation graph; restricted outcomes/censoring | Requires actually attempted matched backend cells; no counterfactual runtime or unsupported-as-failure labels |
| Prime recursive branch choice | Causal pre-action branch context, bound source/native event replay | Eligible prime affine-chain nodes only, p<2^32,m3..15; not private F4 pivots, signatures or global optimality |
| Polynomial consequence proposal | v6 restricted target plus directly checked generator-multiple identity against pre-native source equations | Only already ideal-equality-certified native nodes; zero/unknown omitted from public examples; not minimal degree, complete closure, exact first/last fall degree or native-operation trace |
| Root proposals | Distinct-root and group-lift replay; bounded prime/binary root internals retained | m2 standalone; root certificate is not complete execution/cost verification |
| SAT decision/learning policy | v7 exact initial CNF/XOR/gate-map and monotonic replay APIs, checked learning and causal predecision records | Producer map patch remains unapplied in held solvers.rs; no actual nonzero PDP CLI decision+learning+later-decision witness or nonzero twist fixture yet. Standalone tests do not establish usable generated SAT training rows |
| Factor-base/orbit proposals | New explicit signed action, full orbit/stabilizer/permutation and restricted construction replay | Pending v2 pipeline/export. Post-construction domains may be features; chosen future base cannot be an input for policy selection. No measured policy ranking yet |
| Large-prime selection | Source group identities, residual elimination, rank and determined-target scalar replay | Restricted transcripts exist; full causal internal pivot/filter decision features and broad natural-target yield remain missing |
| Novel solver / hidden algebraic law | Exact checkers can reject invalid model proposals | No trained model, held-out efficacy result, novel SAT solver, universal proof or checked fall-degree counterexample established by this audit |

Keep seeds, known scalar, generator multiplier, membership logs, outcomes,
future learned clauses, target consequences and costs restricted. Conservative
whole-field-size split keeps generators/keys/twists/policies together; cross-size
embedding/isogeny links need explicit component merging. Unknown or capped checks
are not negative mathematical examples. Native target-selection bias must remain
visible in supervision.

## Original parameter and solver matrix (source capability, not solved coverage)

All rows retain both genuine twist positions, all 24 requested sizes n23..31 and
47..61, all m2..15, many curve seeds/generators/keys; unsupported/censored cells
remain explicit. Prime n61 is p=2305843009213693951. Default v1 plan requests
165,150,720 method cells; this is arithmetic, not observed corpus size. Koblitz,
general binary and prime are separate populations; twist orders/generators are
independent. Binary twists use trace-one shifts; prime twists use nonsquares.

| Adapter | Actual declared lane | Missing requested cells / artifacts |
|---|---|---|
| enumerate | All three families,m2..15; bounded signed finite-domain PDP | Generic enumeration; exponential work/caps, not scalar recovery |
| bsgs | All three families; certified u64 subgroup; m axis does not change scalar problem | Independent generic scalar baseline, bounded baby/probe table; large subgroups may fail table admission |
| sat | Binary/Koblitz,m2..15 under formula/work caps | Prime SAT absent; full gate producer not wired; proof-checked general UNSAT/minimization and watch/heap internals absent |
| weil | Binary/Koblitz,m2 exact selector Weil restriction | Encoding construction, solver_completed=false; higher m and broader basis/descent absent |
| f4 | Binary/Koblitz,m2 matrix+splitting; prime m2 reference and m3..15 affine chain at p<2^32; supervised wide-prime m2 through 2^61-1 | Wide prime m3..15 absent; exceptional infinity branches prevent general negative PDP claims; private pivots/zero rows/full critical pairs absent |
| f5 | Binary/Koblitz,m2 Boolean row criterion+splitting | Not general incremental F5; other families/arities and per-row signatures/rejected criteria absent |
| f6 | Binary/Koblitz,m4 F6WidePairIndex four-sum | Geometric generic decomposition, not general Groebner F6; remaining implemented F6 variants/arities need distinct adapters |
| root | All families,m2 native S3 root/lift | Higher-m standalone elimination absent; recursive prime chain extraction is separate; complete execution accounting absent |
| pkm | Prime,m2/3,p<2^31 Kummer tower F4 with grid extraction | n31 prime may fit strict bound; n47..61 absent; higher m absent; alternative rational Kummer base may not lie in target subgroup |
| double-large-prime | All families,m2..15; signed Frobenius when certified, identity/sign otherwise | Natural-target rank/yield may cap; full internal filter/pivot mutation traces absent |

Source: solvers.rs:326 capabilities and dispatch:534; do not equate a 360-cell
endpoint outcome test with 360 solved instances. Frobenius must preserve the
actual model/subgroup; prime rational Frobenius is identity. New closed orbit
bases do not authorize compressed columns without certified eigenvalue transport.

## Actual ECDLP scalar recovery

Recovery is NOT entirely absent: BSGS sets independent recovery only after
Q=[k]G replay (solvers.rs:790..881). relations.rs and relations_generic.rs retain
sampled aG+bQ source identities, elimination/rank evidence and determined-target
recovery; their scalar artifacts are checked against public G/Q. They explicitly
do not read the private witness. Known-answer construction and PDP/root lifts
alone are not recovered ECDLP instances.

The missing algebraic end-to-end path is a bounded relation collector that feeds
actual F4/F5/SAT/Weil/PKM PDP answers into checked relation rows, filtering/rank,
factor-base logs/target descent and scalar recovery, charging all failures/setup.
Current large-prime discovery uses generic pair/domain machinery; it must not be
relabeled algebraic solver recovery. First useful acceptance: public-only input,
known scalar removed from solver access, natural-target bounded positive and
insufficient-rank controls, exact source/row/matrix/scalar replay, both twists,
all attempted work retained. At requested large sizes, capped outcomes remain
valid data and cannot be promoted to solved coverage.

## Measurement and artifact gaps

Checked telemetry explicitly uses calling_thread and sets training_cost_eligible
and whole_worker_coverage_verified false (telemetry/checked/mod.rs:506,572..574).
Legacy phase PMU excludes startup/upload/preexisting threads. Supervisor output
explicitly says whole_worker_not_counted (mod.rs:1734). Native operation counts,
phase instructions, scoped verifier work, simulated instructions and whole-worker
retired instructions must remain distinct. A missing PMU is null with reason;
multiplexed scaled counts are estimates. No total instructions-to-solve label is
currently supported by these phase observations.

The worker-accounting proposal already separates lifecycle/resource receipts
(P0) from parent-owned exec-bound PMU/descendant admission (P1), and finalization,
verification, publication, recovery and inference/fallback costs. Capture retains
emitted bounded prefixes, not private states the native engine never emits.
Complete F4/F5 internals, SAT watch/minimization, Hasse/integer factorization and
filter mutation streams still need hooks; caps must be explicit, not truncation
reported as completeness.

## S3 pilot gates before enormous generation

1. Reconcile prior jobs, grants, usage, durable receipts and current source/binary;
   no duplicate worker or renewed budget by relaunch.
2. Obtain actual worker scoped read/write for approved data, budget/control and
   lab receipt prefixes; run conditional PUT+HEAD+fresh GET/hash as actual UID/role.
   Controller uploads and local S3 evidence archives do not prove worker access.
3. Admit actual us-east-2 worker shape and cumulative dollars/bytes/transport
   attempts, including prior/ambiguous outcomes and retries; freeze protocol,
   source/binary, finite shard/case/time/memory/output limits and owner binding.
4. Run preflight/start with exact plan/protocol and activated authoritative grant;
   pin image digest, reconcile Job/Pod UID and unique shard; preserve all failure
   artifacts. Preflight alone is not an access or execution proof.
5. Produce one completed durable cell, restore fresh bytes in isolated consumer
   storage, verify producer arithmetic plus pinned independent consumer, hashes,
   scalar/PDP status, leakage and counts. Only then scale finite admitted waves.

Historical USD200/20GiB/request limits in OPERATIONS are a retained proposal,
not a fresh available balance. No IAM/cloud/budget action was taken here.

## Highest-priority parallel implementation outside held shared files

Complete v2 binding/export remains the immediate feature-delivery dependency,
but its normalization is already assigned. The best nonduplicating independent
chunk is **P0 worker-accounting schemas, strict replay and collector core**, under
a new telemetry child module (subject to ownership check and frozen contract).
Reuse WORKER-ACCOUNTING-PROPOSAL.md rather than inventing new PMU semantics.
Implement bounded immutable launch-intent/post-spawn/terminal records with exact
attempt/source/binary identity, typed failures and null reasons; an injectable
reap/usage interface; corruption/timeout/spawn-failure/identity controls. Do not
race wait4 with existing Child::try_wait and do not call P0 whole-worker counting.
Prepare a small shared-file integration patch for SupervisedChild (mod.rs:1153),
not an unauthorized edit. Parent-owned exec handshake/PMU (P1) remains necessary
for the user's instruction-count goal and can follow that admission contract.
A standalone module alone does not complete worker accounting or CLI support.

The next scientific functionality after wiring is the public-only algebraic
PDP-to-relation collector described above; its bounded verifier can likewise be
built as a separate module, but needs a distinct preregistered contract and real
adapter/CLI integration. Neither work item substitutes for the original scope.

Sources checked first: DESIGN, OPERATIONS, REMAINING-COVERAGE, current factor-base
contract/status, SAT status and WORKER-ACCOUNTING-PROPOSAL; all are inventoried.
Linux primary interface checks confirm per-task perf scope/permissions and
child-specific wait resource records:
https://man7.org/linux/man-pages/man2/perf_event_open.2.html
https://man7.org/linux/man-pages/man2/wait4.2.html
These inform admission boundaries, not a new measured result. No broad research
survey or novelty claim was undertaken.
