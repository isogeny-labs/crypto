#set page(paper: "a4", margin: (x:15mm,y:13mm))
#set text(font:"DejaVu Sans",size:9pt)
#set par(leading:0.55em)
= Discriminant screening reduces graph-IC relation work
*Runpod49 · 17 November 2026 · four cold Rust workers*

Both grounded-pair collectors recover the expected elliptic scalar after raising the declared split-history cap to524288. The screened collector removes198848 provably ineligible quartics while preserving every source, graph projection, sparse kernel, descent and affine solve. Its observed work subtotal falls 3.185% against unscreened pairs; both remain above the original IC and quadratic-table baselines in this unit. [pcc:unverified]

#table(columns:(1.4fr,1.6fr,0.7fr),inset:5pt,stroke:0.4pt,
 [*Arm*],[*Observed work subtotal*],[*Inner s*],
 [Original IC],[4,525,556,132,414],[18.428],
[Quadratic table],[5,230,830,441,710],[14.895],
[Grounded pairs],[6,441,136,952,022],[31.220],
[Screened pairs],[6,235,988,188,572],[29.181],

)
#image("observed-work.svg",width:100%)

All subtotals are incomplete native bitops, not complete solve costs. Inner elapsed time is a practicality observation: one fixed-order cold call per arm on the same host/CPU/binary, no warmup, retry or exclusion. Full ratios and primary rho comparison remain unreported. [pcc:unverified]

== Exact geometry and cheaper algebraic refusals
Both pairs stop at B185344 with108058 validated sources,522 emitted rows and certified homogeneous rank508. Sources comprise250 full,9535 one-class and98273 two-class relations. The current graph discards93640 ungrounded two-class sources. This motivates a separately bounded signed-forest retention study on recorded data. [pcc:unverified]

Screen298778 quartic calls; refuse198848 by zero/nonsquare discriminant character; retain99930 calls to the original bounded splitter. Both pairs have identical retained random polynomials and uniform events; the necessary screen never substitutes for full factor/principal checks. Their3415 descent residuals and final affine scalar are identical. [pcc:unverified]

== Frozen check fails; separate replay preserves it
The frozen qualifier exits1at “paired exact geometry rank_checks”: it erroneously requires measured checkpoint times to agree (42529515 versus42358498ns). Every other checkpoint field matches. A separately sourced postexecution Rust replay excludes only that timer from pair equality, validates timer encoding, and independently checks all original arithmetic, source/graph/rank/flow/RNG/affine records. Original records and failed qualification remain unchanged. Replay controls150pass; no scientific call is repeated. [pcc:unverified]

== Execution and evidence scope
Execution source bd69a76db876fa1da120de0edf1033489327d937, same152scientific paths as7e574194, published before allocation. Runpod CPU5c4CPU/8GiB0.14USD/hour; owned pod89omx6a1a0ryra created09:28:06Z, checked live guardianPID134219, sole dispatch09:31:01Z. Archive collected/hash-checked/private-scanned before DELETE204/independentGET404at09:41:25Z. Large originals and full study remain hash-bound LOCAL; compact study and separate replay are published. [pcc:unverified]

PCC-20261117-208002/bba5e6 and replay/report claims remain UNVERIFIED producer evidence. Complete native cost, fresh targets, scaling and signed independent admission remain open. Swan1962, Corollary1/§4 (credits Stickelberger), supports the necessary finite-field discriminant test; no novelty claim. [pcc:unverified]
