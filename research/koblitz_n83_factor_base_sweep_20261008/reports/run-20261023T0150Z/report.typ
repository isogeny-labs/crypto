#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:9.5pt)
= Native N83 arena reclamation: 1M validation
Campaign: 23 October 2026 UTC. Both public retained K2048/K4096 arms complete 1,000,000
conflicts, zero models/relations/rank and bounded Unknown. All 1,001 retained
search-counter signatures, 102 required storage snapshots per base, 59/61 memory
samples and exact card/relay/receipt/cleanup custody pass. n=1 per base; no
uncertainty estimate or matched speed/cold/rho/base-selection claim.

#image("storage-growth.svg",width:100%)
Requested capacities, logical bytes, actual worker RSS and process allocator
charges are distinct. No per-buffer physical residency attribution. Same
public inputs/query; historical-host timings and RSS are not pooled.

#pagebreak()
= Live clauses, detached tombstones and capacity
#image("arena-partition.svg",width:100%)
At level-zero reductions, compact in place only when at least one quarter
of used words is recoverable. Preserve clause ordinals, prefix boundaries,
live literal order, reduction schedule and watch rebuild order. Relocate live
reasons; clear stale unassigned reasons. Detached long clauses become harmless
three-word tautologies so incremental exports do not become UNSAT. Retain 50 percent
headroom, minimum 4,096 words. Allocator shrink may move memory.

32 SAT tests cover CNF/XOR/incremental/export/reasons and pinned search digests.
Final integrated native checks: 4,497 library pass, 119 ignored; 19 exporter,
2 independent model-checker, 13 CLI, 3 panel and 40 guard + 7 card tests.
The exact 1.5M/9,000 continuation guard passes 41 + 7; no higher-cap worker has run here.

#pagebreak()
= Reclamation observed; broader search remains open
#image("reclaimed-words.svg",width:100%)
#table(columns:(2fr,1fr,1fr),inset:5pt,
 [Terminal native observation],[K2048-r1],[K4096-r1],
 [Reclamation calls],[3],[1],
 [Arena words removed],[988381364],[776774054],
 [Owned buffer capacity, bytes],[1390447633],[3616528993],
 [Arena capacity, bytes],[1043002672],[3206401856],
 [Arena logical used, bytes],[947640748],[2069794744],
 [Worker RSS, bytes],[1465413632],[2858786816],
 [Worker HWM, bytes],[2435203072],[4159254528],
 [Largest sampled kernel peak, bytes],[2436382720],[4169007104])
Largest sampled kernel peak is not necessarily the full-run peak; unsampled tails
23059.29533500038/22055.481703999918 ms remain explicit. All sampled 8 GiB/swap 0
limits and event counters pass. Minimum safe memory remains unmeasured.

Frozen worker 4fcda42f, guard v1 563d2168 and clean source 37631c5c. Full hashes,
native JSON, source/build histories, exact inputs/cards, pressure refusals,
original test/GPU launch failure and two packaging failures are linked from
REPORT.md and verification/native-arena-reclamation-20261023/PUBLICATION.md.
Four immutable S3 archives pass freshGET/payload/native replay/repack as applicable.
No task build/test/compression overlapped either measured worker.

Snapshots: conflict 1, then every 1,000 through 1M; zero is separate solve entry.
Storage: entry, each 10K, return. Original 1M and 450K evidence stays immutable.
No scalar/rho certificate or admitted leaderboard/benchmark row follows.
Next: separately frozen 1.5M/9,000 arms, within the original 30,000-second four-arm
allowance. No timed retry/carryover/extension. Inherited CMS comparison is
separate; no new external timing comparison or fastest claim.
