# Native N83 arena reclamation validation, 23 October 2026 UTC

Both candidate arms completed **1,000,000 conflicts**, with zero models,
relations and rank, and bounded `Unknown`. Each matches all **1,001 retained
search-counter snapshots** and passes native audit, complete storage validation,
memory-profile validation and director card/relay/receipt custody. K2048 records
three arena reclamations removing988381364words; K4096 records one removing
776774054words. This validates reclamation on these two native workloads;
no useful relation, UNSAT, recovered scalar, selected base or speed gain follows.

Nominal start is K2048's retained container start2026-10-23T01:50:06.278910105Z.
The [protocol](../../verification/native-arena-reclamation-20261023/PROTOCOL.md)
was committed before timing. The later1.5M continuation is separately staged;
this report covers only the two completed1M validation arms.

## Request and acceptance

| Requested requirement | Actual status |
|---|---|
| Fix PR2036 CI | Original failures retained and fixes committed; full local validation passes; published37631 has15 successful checks and no failures at04:15UTC, with remaining checks running/queued. CI remains pending. |
| Implement arena reclamation | In-place compaction, ordinal preservation, reason relocation, harmless tombstones and watch rebuilding implemented;32SAT tests including pinned search digests and CNF/XOR/incremental equivalence pass. |
| Validate native N83 | Both1M arms accepted; all1001 original counter signatures,102 storage snapshots each,59/61 memory samples, exact custody and cleanup pass. |
| Continue higher conflict caps | Preregistered1.5M/9000-second arms are planned. New exact-pair guard passes41unit+7card tests; no higher-cap worker has run at this freeze. |
| Store all raw evidence | Readiness/integration and both completed1M bundles freshly verified in setec-astronomy-artifacts; [publication](../../verification/native-arena-reclamation-20261023/PUBLICATION.md). |
| Finish factor-base selection/full cold recovery | Open: zero useful relations/rank, no verified unknown-scalar result, no complete cold/rho comparison. |

## Recorded quantities

All values below are emitted/validated by native Rust; n=1 per base, no
uncertainty estimate. Capacities, logical words, process allocator charges and
resident memory are different quantities. The kernel peak below is the largest
sampled peak; the unsampled tail can contain a larger full-run peak.

| Observation | K2048-r1 | K4096-r1 |
|---|---:|---:|
| Conflicts / models / relations / rank |1000000 /0 /0 /0|1000000 /0 /0 /0|
| Matched retained counter snapshots |1001|1001|
| Required storage snapshots |102|102|
| Memory samples |59|61|
| Reclamation calls |3|1|
| Cumulative arena words removed |988381364|776774054|
| Terminal total owned buffer capacity, bytes |1390447633|3616528993|
| Terminal arena capacity, bytes |1043002672|3206401856|
| Terminal arena logical used, bytes |947640748|2069794744|
| Terminal worker RSS, bytes |1465413632|2858786816|
| Worker HWM, bytes |2435203072|4159254528|
| Largest sampled cgroup kernel peak, bytes |2436382720|4169007104|
| Unsampled tail, ms |23059.29533500038|22055.481703999918|

[Storage/RSS growth](storage-growth.svg), [logical arena partitions](arena-partition.svg)
and [cumulative reclamation](reclaimed-words.svg) plot the native JSON in data/.
Every chart has a same-source150dpiPNG; [PDF](report.pdf) is inspected before
publication. No historical-host timings/RSS are pooled. Reclaimed-word counters
measure removed arena storage; they do not measure physical pages attributable
to one buffer or establish a minimum safe memory limit.

Counter clarification: the1001 conflict snapshots are conflict1 and each
multiple of1000 through1M. Solve entry separately records conflict0. The102
storage snapshots are solve entry, each10K through1M, and solve return.
The earlier450K report's phrase “including zero” was a prose error: its retained
trace also starts at conflict1. Its raw bytes and reported451-count are unchanged;
the [explicit correction](../run-20261021T1617Z/CLARIFICATION.md) preserves that history.

## Implementation and correctness

Compaction runs at a level-zero reduction boundary when at least one quarter
of arena-used words are reclaimable. It retains all clause ordinals, original
and learnt prefix boundaries, reduction triggers, candidate/live-literal order
and watch rebuild order. Detached long clauses become three-word tautologies;
empty tombstones would incorrectly make incremental exports UNSAT. Live reasons
relocate; stale unassigned reasons clear. Copying is in place, then capacity
shrinks with50percent headroom/minimum4096words; allocator shrink may move memory.
Cumulative reclamation counters accompany all storage snapshots.

The new native verifier requires a complete retained1M reference, identical
input inventories, full audits, all1001 five-counter signatures, complete
storage coverage and monotone positive terminal reclamation counters. Later
continuation counters will be explicitly unpaired. Changed/missing/duplicate
prefix counters and malformed reclamation counters are rejected in tests.

Full integrated native checks pass4497library tests/0fail/119ignored,
19exporter,2independent model-checker,13CLI,3unknown-scalar-panel and40guard
unit+7card tests. Subsequent continuation-only guard passes41+7. Worker/library,
exporter, dependencies and adapters are byte-identical to the validated9d858211
build after current-main integration; empty source diff is retained.

The initial development test compile failure and integrated library failure are
preserved. The latter's GPU mock receipt reports pre-execETXTBSY, PID0 and no
program execution. The bounded fix waits only before a successful launch, within
the original deadline; native held-writer/deadline/missing-path tests pass.
No executed program is retried. The original transient writer is unidentified.

## Source, execution and limits

Worker SHA256 `4fcda42fe79c90912c4ba4e8f702945dc0cc2914d93d3c1941dd3966d481d060`,
built at9d85821127abba88001b5a2ad8a3c8bf0e4c10e5; both cards froze clean
HEAD37631c5cda4a370745e5c229fe73e90aba52b2bf. Guard-v1 SHA256
`563d2168849f7096c217c6bcbba18ed915e2c3c44bc50f422de226631b0dbad0`.
Each receipt records exactly one attempt, exit0, no timeout, unchanged binds and
clean tracked source before/after. Original1M reference runs remain immutable.

Public retained model `icv1-f2m83-tm6151469093347-debefd74`, seed2026100801,
m5,identitymask0; existing K2048/K4096 panels/query/fixtures, pinned imageID
541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63.
Migrated32logical-CPU Xeon6975P-C host, complete physical core14/30, worker14,
one CPU quota,8GiB/zero extra swap, networknone/read-only/capdropALL,
no-new-privileges/64PIDs. Native reservation, synchronized systemd swap barrier,
drift detection, existing global lock and cleanup verifier remain enforced.
No task build/test/compression overlapped either measured worker. No automatic
L2 or matched-clock admission is inferred from these diagnostics.

K2048's first compile readiness refusedPSI16.94; a separately authorized quiet
attempt compiled. K4096 compilation beganPSI0 but refused dispatch afterPSI36.45;
a later independent readiness check passedPSI0 without changing/recompiling the
card. Both final dispatch checks passed<=5. Neither refusal launched a worker.
Two K2048 packaging failures (root-owned source, execute-only bound system binary)
are retained; privileged copying/read normalization affects staged copies only.
Both final archives pass freshGET/native audit/memory/reclamation/payload/repack.

The inherited external CryptoMiniSat5.17 comparison remains separately scoped
and was faster under its reported algorithm CPU timer; no new external timing
comparison or fastest claim is made here. These zero-relation diagnostics do
not qualify for VALIDATION_GATE admission. Documentary benchmark copies add no
benchmarks/README or leaderboard row and no scalar/replay/rho certificate.

Next: execute the separately frozen1.5M K2048 and K4096 cards only after both
accepted1M results, exact-pair guard tests and resource readiness. The total
preregistered four-arm worker allowance is30000seconds, without carryover,
automatic timed retries or extensions. Full cold selection and minimum-memory
obligations remain open in the [memory problem](../../../../problems/n83-native-800k-memory-limit.md).
