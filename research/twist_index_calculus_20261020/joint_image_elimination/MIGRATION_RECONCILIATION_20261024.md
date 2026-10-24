# Reconciliation after the 01:36 UTC app recovery

Observed again on 2026-10-24 at 01:41–01:43 UTC, before resuming work.

- The approved migration receipt remains
  `/srv/experiments/agent-migration/handoff-receipt.json`.
- `/run/spot-legacy/diem.json` reports `deadline-expired`, 50 durable records
  of 52, `pending=false`, and no launcher PID. Its original deadline is
  1792521741.993407. No generator was launched or restarted.
- EKS overview lists only terminal jobs. The study's capability probe
  `twist-joint-capability-20261024-v1` failed its scheduling deadline. No
  command ran in that probe; it is not a scientific failure. No EKS job was
  resubmitted and no role or node-pool policy was changed.
- Process inspection finds no native study runner, proof gate, msolve, or
  study orchestration process. Docker lists no running study container.
  Missing exit receipts are recorded as interruptions, not successful or
  failed mathematical results.
- The old source-side waiting freeze lane belongs to the retained migration
  history and has not been restarted by this study.

## Retained work and exact continuation

| Lane | Saved evidence | Reconciliation | Continuation |
|---|---|---|---|
| Historical 189 cases and 21 doubling controls | `../endomorphism_image_base/` | Complete, independently audited and priced | Do not rerun |
| Joint construction v1 | `runs/algebra-v1/`, `results/algebra-v1-validation.json` | 297 constructions and 694 system files validated | Reuse validation |
| Native global S5 regression | `runs/regression-v1/metadata/*.exit` | Six solves terminal: five allocation failures and one cooperative incomplete result | Do not rerun |
| Native source-block unions | Same metadata; three `*-orbit-union.started.txt` files | No exit receipt, solve directory, or surviving process | New recovery record, existing frozen runner and inputs only |
| Polynomial Lean gate v2 | `runs/lean-gate-v2/` | Interrupted; no gate verdict retained | Fresh receipt, unchanged theorem source, 900 s watchdog |
| Repair verification v7 | Scratch launcher log; no `runs/verification-v7/` directory | Guarded verification never produced its first run artifact | Execute once in a fresh v8 directory |
| Uniform-mass Lean gate v1 | `runs/uniform-lean-v1/` | Empty stdout/stderr, no verdict or process | Fresh receipt; preserve v1 |
| External regression v1 | Metadata plus first input/command | No basis, exit receipt, or surviving container | Fresh continuation; never infer a complete basis from an exit code |

All new scientific phases retain one thread, an 8 GiB address-space limit,
the repository's busy reservation, and at most the original 1,200 s phase
budget. Compilation remains separate. Recovery watchdogs may be shorter;
they do not replace the original outstanding experiments or establish a
negative mathematical result. Original scripts and partial artifacts remain
unchanged. No completed scientific phase is repeated.

## Additional recovered evidence

The temporary directory from the first actual Lean build survived. Its
build log records `failed to create thread: Resource temporarily unavailable`.
It is copied to `runs/lean-gate-v2/recovered-build.log`. A missing outer
receipt therefore does not mean that this attempt performed no compilation.
The fresh checker explicitly supplies `-j1`; its later memory failure and
the uniform proof's actual arithmetic error have their own receipts.

Continuation processes now have distinct managed owners so an app recovery
does not silently abandon them. Inspect each owner and its run directory
before resubmitting any phase:

- `twist-joint-orbit-regression-recovery-v2.service`: three previously
  interrupted unions only, using the original frozen runner.
- `twist-joint-external-regression-v2.service`: the existing 21-input
  external regression inventory, with corrected v8 variant paths.
- `twist-joint-variant-regression-v1.service`: six already authorized
  implicit/full-symmetry native regressions, using the frozen v8 runner.
- `twist-joint-uniform-zero-v2.service`: the original 21 zero cases and
  three population cases. V1 was stopped only while waiting to compile,
  after source inspection caught an invalid zero-query JSON encoding. Its
  prebuild receipt and source hashes are preserved; no oracle case ran.
- `twist-joint-proof-uniform-recovery-v3.service`,
  `twist-joint-proof-coordinates-v4.service` and
  `twist-joint-proof-quartic-v4.service`: separate bounded proof checks.

Earlier proof-owner units are terminal. These are study-specific transient
owners, unrelated to Diem's managed owner. Scientific watchdogs remain
inside the busy reservation, so time spent queued does not consume a
solver's execution limit.

## 05:17 UTC continuation reconciliation

Diem still reports the expired original deadline, 50/52 records,
pending false and no launcher PID. The approved handoff receipt was read
with read-only elevated access. The legacy freeze-followup was not
restarted; no EKS job or permission was changed. The historical 189-case
and doubling comparison remains complete and was not repeated.

The following managed owners were inspected before any new execution:
external-regression-v2, external-audit-v3, uniform-zero-v2,
variant-population-v1, variant-validation-v1 and raw-verify-v1. Their
existing work continues. The two first external pure-L0 physical-set
audits actually pass on p37/seed0/query2; their summaries explicitly
leave reverse ideal containment unverified.

Newly reconciled terminal results are the independent first-discriminant
IDC replay (IDC1h09d838b7dca9895f), stronger square-target native search,
and seven-theorem square/root bridge. None was repeated. Two distinct
new owners, `twist-joint-square-zero-verify-v1.service` and
`twist-joint-proof-square-ideal-v1.service`, now check the stronger
certificate and composed theorem under the original resource limits.

The existing variant validator had not yet frozen its runner at the time
of inspection. Its registered native source is held unchanged until the
actual build/copy receipt exists. The proposed pruning audit is prepared
in a separate, unregistered module. It requires the stronger checks
before execution and reads the saved relation oracle without regenerating
it. The raw checkpoint contains only its 22 literal selected run trees;
all later work remains outside that immutable checkpoint.

## 05:50 UTC managed gates and archive check

The variant reporter's build, immutable runner copy and recorded runner
checksum all pass. Only then were the pruning module and CLI registered
in the native crate. The new 77-test gate has not executed yet.

The combined square-ideal check actually passes all 10 audited theorems,
with only standard Lean foundations and zero failing/trusted-native
declarations. Independent stronger IDC replay remains under its original
owner. `twist-joint-fp-pruning-audit-v1.service` waits outside the busy
reservation for that owner, requires the replay and proof to pass, and
only then compiles and audits the saved 21-case tape. It does not deploy
a filter or repeat a historical collector/oracle. Its replay retains the
already point-verified matrix solution as oracle-only provenance for a
possible higher-arity diagnostic.

`twist-joint-external-audit-completion-v1.service` waits for both original
external owners to become terminal. It enumerates all 21 expected input
positions, including missing/not-run cases; reuses matching existing
native audits and verifies only new terminal bases with the frozen v3
runner. No msolve or BSGS job is duplicated by this follow-up.

The terminal raw checkpoint passes fresh extraction, its whole-archive
checksum, inside-manifest comparison and native `cmp` for all 4,035
regular files. Canonical publication is queued. The user-authorized
existing store credential is scoped to that short-lived publication
process; no global credential/profile, EKS role or other service access
was changed. The archive remains a 22-directory checkpoint, not a
completed full-study archive.

## 07:23 UTC terminal receipts and pending work

The stronger independent replay now actually passes as
IDC1h65c11f01d0b1e67a, with zero integer remainder, 32 extra points and
both mutation controls rejected. The full 297-position variant sweep
also passes its separate native consistency validator: 294 new and
three reused exports, no missing, invalid or bounded-failure records.
That result does not certify generic S5 ideals, complete roots or cost.

Canonical checkpoint publication passes in
`results/publication/terminal-store-publish-v2.json`. The local archive,
native cache/remote check and S3 whole-object SHA-256 checksum agree on
542,528,576 bytes and `f4ddf5ce185d0866ff02e7a4e4085065e59756cf86798aaff85f7a8398eaa39c`.
The first wrapper fails locally before input/auth/upload due to an
unavailable Python3.9 helper; that failure is retained. The portable
second wrapper succeeds without repeating any successful upload.

The pruning source compiles successfully; its 77-test execution, runner
freeze and 21-case audit are pending behind the shared lock. Four
external physical-set audits pass on the pure fixture; the separate
geometric-root audit hits its 1,200 s cap and stays censored. The native
historical-miss binding check is queued under its own managed owner.

The v6 document render passes source-hash stability checks but page
inspection is pending. A v7 square-diagram render explicitly adds the
H4=0 premise and identifies the separate group/basis bridge; its final
bytes need fresh page review. No new conditional proof is represented
as a measured filter gain. Other managed owners and their original
budgets remain intact; no original collector, BSGS, Diem generator or
EKS job was duplicated.

## 10:00–10:07 UTC reconciliation after the repeated migration steering

Diem's native state was inspected first, before any other action. It remains
`deadline-expired`, 50/52, with no pending work or launcher; its managed owner
is inactive. The approved handoff receipt was read. The EKS service overview
again lists only `twist-joint-capability-20261024-v1`, terminal with
`DeadlineExceeded`. Direct kubectl has no configured context and was not
used to change configuration or assume another role. No Diem generator,
legacy waiting-freeze followup or EKS job was restarted. The legacy source
lane remains quiesced under its original authorization.

Every scientific lane from the 07:23 snapshot is now terminal. All 21 pruning
audits, all 21 four-factor zero fibers and three full p37 populations, all 21
five-factor censuses and all 63 independent public-point arm comparisons
pass within their stated scope. All 21 doubled-triangle point/set/row checks
pass. All 21 external positions are reconciled: 11 physical-set passes,
nine external timeouts and one native-root timeout. The original historical
miss is bound by the corrected checker, with its absent runtime seal still
explicit. No original collector or BSGS run was repeated for these checks.

One external-audit followup did accidentally repeat an already terminal
native-root timeout before detection. Its duplicate child was stopped and
its partial outputs, before/after stop receipts and whole-service CPU counter
132461232000 ns are retained in `external-audit-completion-v1`. The corrected
v2 reuses failures only after exact argv/hash binding and audits newly
available bases. A service stop's success is not a native audit pass.

The main-branch merge retains all 15,797 curve identities and parent aliases;
the 325 differing shared metadata records are one-sided main changes.
The semantic addendum preserves the changed modulus-provenance descriptor.
All ten post-merge software phases exit zero; the release library passes
4,488 tests with 119 existing ignores, examples pass 1+3, and the separate
native point-replay gate passes 81. These are software gates, not new solver
prices or published-head CI. Exact sources and commands remain in the raw
metadata. Unrelated dirty source files are excluded from publication.

The 43-directory level-6 archive attempt was deliberately stopped at 09:52
without a completed archive or upload. Its 99 MiB partial and stop/hash
receipts remain in scratch. Only infrastructure compression changed:
committed level-1 scripts retain one worker, the shared busy lock, exact
fresh extraction/cmp of every file and byte-identical repacking. The v3
owner was confirmed absent before its single launch at about 10:04 UTC.
All packing and extraction staging is on the durable workspace volume.
The old 22-directory canonical f4ddf5ce checkpoint remains published until
this replacement passes verification and actual remote publication.
