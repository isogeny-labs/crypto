#set page(paper:"a4",margin:15mm)
#set text(font:"New Computer Modern",size:10pt)
= N83: full-size common-public panel control gate
_2026-10-20 UTC · full-m4-v1 · completed controls; natural panel tracked separately_

Four constructed controls—retained U and exported D18, cache off/on—passed independent general-field replay. The two native pair checks require identical complete SAT-model and sparse-row digests, the same search counters, and exact conservation of checked visits plus cached skips. Both bases have 16,627 signed-Frobenius columns and 2,760,082 points. This supplies the prerequisite for the separately frozen eighteen-attempt public panel.

#align(center,image("controls.svg",width:100%))

== Frozen inputs and execution

Curve `icv1-f2m83-tm6151469093347-debefd74`, a=0, subgroup 2417851639230796216685689, cofactor 4, polynomial z^83+z^45+z^2+z+1. U object BLAKE3 `1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`; D object BLAKE3 `e0c24980207cbb15fa57ec76bcfc6a628a763a80834cba1117ca83ea04678682`. The sets differ; the existing same-set preparation experiment remains separate.

Worker/replayer source `d9ef42301cc3d22f437e1f89304e673836afb6a3`. Build/test completion, exact binaries, scripts, input hashes and prior cache-audit hash are fixed in `FULL_M4_HOST_CARD.md`. Full release-library validation applies to the unchanged solver tree; final example tests cover policy boundaries, general-field receipt rejection and full shell-card ordering. The gate admits cache off only for constructed full-base controls.

The public panel uses all eight frozen corpus points with no scalar sidecar. Target 0 order U,D,D,U; odd indices 1–7 D,U and even indices U,D. Every natural arm uses cache on, four free summands, one model and 10,000 conflicts, with 1,790 s internal / 1,800 s outer caps. The maximum natural allocation is nine hours. Returned relations need separate replay before useful-rank aggregation. All caps and failures are retained.

== Accounting and limits

Authorized SSH Spot host, Intel Xeon 6975P-C, CPU 31, one CPU quota, hard 4 GiB, zero swap, no worker network and read-only root/inputs. Other workloads share the machine, so these are L0 observations. Each worker interval includes import/index, corpus validation, model/root/search and relation/rank checking, ending before final receipt serialization. Container and card boundaries retain surrounding overhead. Construction/export, independent replay and final output/custody remain separate charges.

The controls certify their constructed witnesses. Natural rank, complete cold/reused IC cost, same-point rho, external-fastest comparison and best-base selection remain open. No qualified cost row changes. The run's live state is timestamped in `full-m4-v1/STATUS.md`; a later Spot interruption must not be mistaken for ongoing execution. Sources: `FULL_M4_PROTOCOL.md`, host card, `full-m4-v1/controls/`, four raw data receipts, and this directory's `RUN_REPORT.md`.
