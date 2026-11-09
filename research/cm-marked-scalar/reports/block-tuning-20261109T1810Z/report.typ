#set page(paper:"a4",margin:16mm)
#set text(font:"Libertinus Serif",size:10pt)
#set par(leading:0.5em)
#align(center)[#text(size:18pt,weight:"bold")[Complete block-aware parameter census] #linebreak() P-192 primary; P-224/P-256/P-384/P-521 retained #linebreak() CM-BLOCK-PLAN-CENSUS-V1 · one scan · 2026-11-09 · PR 3674]
= Recorded outcome
The producer-run census completed 27,090 parameter rows. The standalone GMP replay returned 80 minimum sets, all 400 selected original-$mu$ point/count cases passed, and four altered-data controls rejected at their intended checks. A fresh extracted replay and deterministic archive repack were byte-identical. These are producer-side unverified assertions pending signed independent PCC standing. [pcc:unverified]

#table(columns:(1fr,1fr,1fr,1fr,1fr),inset:5pt,align:(left,right,right,right,right),[Curve],[V3 default block],[Tuned relation],[Tuned binary],[Prior direct],
[P-192],[231],[230],[229],[228],
[P-224],[265],[265],[260],[260],
[P-256],[303],[303],[300],[300],
[P-384],[452],[451],[450],[449],
[P-521],[612],[611],[606],[606])

Values are full D+A counts including prefix/table construction. Tuning recovers one call relative to V3 on P-192/P-384/P-521, but the relation minimum remains above both matched binary and prior optimized direct/binary references on every curve. Weights 2, 3 and 4 retain losses over this emitted program family. V3 is historical fixed-default context, not the decisive reference. No CPU, allocator, field-operation or latency price follows from D+A. [pcc:unverified]

#image("gap.svg",width:100%)

= Frozen domain and controls
Exactly five curves, seven radices (the certified $ell$ plus binary 1,2,4,8,16,32), 129 offsets from -64 through 64, and six widths 2 through 7 give 27,090 rows. Each row pairs the original single-column retained-bank schedule and the bounded 16-column folded program. Original $mu$, curve/subgroup and complete bank work stay fixed. Prior direct/binary reference plans retain their stronger earlier translation/window optimization; their donor census was not repeated. [pcc:unverified]

The independent GMP model includes no producer/planner headers. It reconstructs splits/digits/prefix/table counts and solves a suffix recurrence over every 16-column possibility, without the producer's early-exit pruning. Its small exhaustive check returned 90 cases; producer counters matched 630 paired actual point/counter boundary cases before data. The internal planner still minimizes unweighted D+A,N,D,steps; weights 2–4 select among those emitted programs and do not claim global weighted chain optimality. [pcc:unverified]

#pagebreak()
= Selected primary relation vectors
#table(columns:(1fr,1fr,1.5fr,1fr,1fr),inset:5pt,align:(left,right,right,right,right),[Curve],[offset / width],[D,A,N,T],[Bank / normalized],[DP offers],
[P-192],[+1 / 4],[190,40,18,6],[10 / 4],[676],
[P-224],[+3 / 5],[220,45,17,10],[14 / 8],[829],
[P-256],[+1 / 5],[251,52,22,10],[14 / 8],[928],
[P-384],[-2 / 6],[379,72,26,18],[23 / 16],[1553],
[P-521],[-1 / 6],[520,91,33,18],[21 / 16],[2139])

The fixed relation radices are 13,11,11,19,7 respectively. Logical table entries T, physical retained bank objects, normalized entries and DP offers are different events. Offers do not count every CPU instruction or heap allocation. All vectors and all 20 curve/weight records are in plans.json; 80 minimum sets count both methods and both schedules. [pcc:unverified]

= Cost interpretation and e2e obligation
This experiment selects parameters offline. Search/plan generation remains a cost, rather than a free first-use result. A future latency comparison must charge its actual input-dependent recoding/planning/allocation, point-bank setup/normalization, verification and destruction. No clock ran in this census, and no zero-time or e2e-speed inference is made from its event vectors. The previous V3 refusal is unchanged. The original full e2e objective stays open; the declared strict group-count prediction is not met within this finite family. [pcc:unverified]

The separately qualified full cold worker retains all costs inside its boundary. A later version4 amendment uses the user's existing-host quiet-window choice without substituting these tuned installed plans into its deterministic primary. Its separate admission record must be read independently; this census supplies no timing evidence.

= Evidence, publication and custody
Source/qualification 850a36dd03da, frozen record 24b03006a877 and guarded launch b700cb0bb8fa were hosted with PR updated before data. Protocol digest: ae0ddaa50b9d411171f3464998c50df18fec2836dfb862ff82e1678eb0fbb2d2. Eight inputs, nine sources and qualified binary hashes are bound. Raw publication 84cb43d91459 preceded this report. [pcc:unverified]

Raw scan is 1,523,811 bytes. The deterministic archive is 110,312 bytes, SHA256 11f5de7252cc1a9d2f26dd878b8eb8c5b93780e843e9ee79de9415bb71c559c8. The artifact bot externalized it in be6490725cd3; committed metadata supplies the content-addressed locator. The local exact backup was restored and checksum-checked without an agent S3 GET. Fresh extraction, native replay and repack match; no scan was repeated. [pcc:unverified]

The native-backend receipt accounts for 17 invocations, including the failed first build, under a 20-invocation cap; each was bounded by 120 seconds/2 GiB/one process/thread. Four write-once faults changed a counter, removed a row, duplicated a row, or changed an arm, with checksums updated. Rejections matched the expected model/design/completeness errors. The first replay build's two mixed-type auto errors remain preserved; final builds were warning-free. No new cloud job, role/cap/budget change or generator/controller restart. No Rust implementation changed; no root Cargo result is claimed. [pcc:unverified]

The schema-valid export's verification fields describe producer-run local checks, not signed PCC standing. Formal PCC registration was blocked by another task's write-exclusive namespace. The evidence-hashed submission draft is not a registered claim. Sources checked: unavailable KB tools/wrapper as retained, existing block/residual contracts and cited Bernstein–Lange/Grabner–Heuberger–Prodinger background. No novelty, global-chain optimality or expanded global-quotient claim. [pcc:unverified]

#text(size:8pt)[Evidence: ../../block-tuning-v1/README.md, plans.json, export.json, controls.json, selected-qualification.json, native-use.json, ARTIFACT_LOCATOR.md and manifests; ../../protocol/block-plan-census-v1/; ../../qualification/block-tune-build-v1/v2/.]
