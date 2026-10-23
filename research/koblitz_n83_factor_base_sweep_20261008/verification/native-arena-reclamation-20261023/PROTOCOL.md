# Native N83 arena reclamation and continued sweep

Preregistered before new timed SAT work, 2026-10-23 UTC. User authorizes
fixing PR2036 CI, implementing and validating arena reclamation, continuing
the native K2048/K4096 sweep, and uploading every new raw record to
`setec-astronomy-artifacts`. Prior completed 1M and ownership profiles remain
immutable. Migration reconciliation found no interrupted N83 worker to repeat.

| Requirement | Acceptance evidence |
| --- | --- |
| Fix CI | Saved original logs, focused regression checks, committed fixes, fresh exact-head workflow outcomes; queued checks remain pending |
| Reclaim arena | In-place implementation, live/stale reason relocation, unchanged reduction schedule, CNF/XOR/incremental/export tests and pinned search digests |
| Validate native N83 | Separate K2048 and K4096 1M-conflict candidate arms; full audit, memory and ownership checks, every emitted search-counter snapshot matches the original retained 1M arm |
| Continue sweep | Only after validation, fresh 1.5M-conflict candidate arms for each base, independently compiled cards and assessed executions |
| Preserve evidence | All attempted executions/build failures, source/ELF/inputs/cards/custody/raw streams/profiles; immutable S3 archives with fresh GET and payload verification |

The implementation retains all clause ordinals, original/learnt prefix
boundaries, reduction triggers, candidate order and live literal order.
Detached long clauses become three-word tautologies; all retained reasons
relocate and stale reasons of unassigned variables are cleared. Watch lists
rebuild in the original ordinal order. Compaction requires reclaimable words
at least one quarter of arena use and leaves 50% headroom (minimum4096words).
This targets retained detached storage, not XOR semantics or relation search.

First compile/test all changed native code. Use the original public panels,
fixtures and model `icv1-f2m83-tm6151469093347-debefd74`, seed2026100801,
m5, identitymask0. Retain the original image by exact digest; restore it after
migration and verify imageID541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63.
Reinspect host topology; reserve complete physical core14/30, run14, one CPU
quota,8GiB memory and zero extra swap, networknone/read-only/capdropALL,
no-new-privileges/64PIDs. Use the native reservation, synchronized systemd
swap barrier, drift detection, global benchmark lock and cleanup verifier.
CPU PSIavg10 must be <=5.00 before card compilation/dispatch. No overlapping
task builds/tests/compression or automatic timed retries.

Validation arms: one K2048 and one K4096 candidate, each1M conflicts,
6000worker-seconds,7200command-seconds including at most600seconds queued
for the benchmark lock. These replays validate a changed implementation;
they do not replace the already completed original 1M evidence. Each requires
its own director-compiled card, verbatim root relay, mechanical execution and
director assessment. Freeze clean tracked HEAD, worker/guard ELFs and all
binds through assessment. Any failed/refused arm remains evidence and must
be assessed before a separately authorized corrective card.

If both pass, continuation arms: one K2048 and one K4096 candidate, each
1.5M conflicts,9000worker-seconds,10200command-seconds, same resource policy.
The guard must explicitly admit this new bounded pair, with boundary tests;
it must not raise witness/external solver limits. Total new worker allowance
is30000seconds for the four decided arms, separate from all prior campaigns.
No budget carryover or automatic extension. Preserve missing costs as null.

All mandatory storage snapshots and exact owner/arena partitions must validate.
Native reclamation counters report calls and words removed; trace signatures
check search-counter agreement, not all internal states. Validate the full
retained1M prefix for continuation arms, explicitly identify later counters as
uncompared, and reject changed/missing prefix counters. Require complete
audits and mandatory storage coverage even beyond the matched prefix.

No matched timing comparison is preregistered here. n=1 per arm gives no
uncertainty estimate. Historical and migrated host clocks/RSS are not pooled;
requested capacities, logical words, allocator bytes and RSS/HWM remain
distinct. Counters/partitions can demonstrate physical storage reclamation;
no speed, fastest, full-cold, rho or per-buffer residency claim follows.
The inherited external CryptoMiniSat comparison remains separately scoped.
Bounded unknown with zero relations is not UNSAT or a base-selection verdict.

Record native summaries, source-linked quantitative SVG/same-source150dpiPNG,
an inspected PDF and documentary benchmark copies. Respect VALIDATION_GATE:
these diagnostics do not earn an admitted leaderboard/benchmark row or scalar
certificate. Preserve canonical graphs unless admitted conclusions change.
Full release library, touched examples and native guard checks precede final
publication. Update the existing PR without force or unrelated changes.
