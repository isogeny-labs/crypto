#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Fixed-model cold construction and verified IC
F127 · genus 3 · 15 November 2026
#image("flow.svg",width:100%)

Both separately frozen versions construct quotient data from the public source input and recover k=183049, verified on source and destination. No earlier kernel, normalization, matrix, point-log table, image or scalar is loaded. The section routines use a preconfigured target model/chart, validated against the fresh quotient. This is correctness within that registered configuration, without a timing or novelty claim.

== Source recovery and fresh setup
V1(source222ed0a121f3/checkpoint232606Z) uses direct counts. V2(sourcee694bfedd11a/checkpoint233622Z) replaces only that component with the admitted p-adic recurrence/exact lift, receiving actual f,p,ell. It uses380coefficient steps, residues(1,84,46),30lifts→3coarse→1endpoint survivor and three exact root brackets. It recovers(1,-43,-1224,2057786); implied129/16043counts match V1 independently. No source Fp²counting is called in V2.

Each constructs two27pointkernels, checks order3/Frobenius stability, rebuilds27normalizations, quotient geometry and Serresign/pointcount. Mandatory runtime setup supplies constructed HC/QF. Missing setup causes expected loader rejection before field work. Kernel/alpha/model/matrix/log outputs match byte-for-byte only in post-run checks; V1data are not V2inputs.

== Calibration, actual inverses and DLP verification
Fresh first degree18fiber point gives an invertible rank8matrix from rank63system,12training and4withheld pairs. Positive/bothactual msolve cells yield degree12parametrizations satisfying all12equations exactly. All factors agree on original classes; public adjacency supplies signs and all ten corrupted-adjacency checks reject.

Fresh48pointbase collection examines1128pairs/878unique lines and314split relations. Sparse elimination removes81LPcolumns. The232×47densecore has rank46/nullity1;129logs and314residuals pass. Each input gets3agreeing conic descents from20pairs. Logs34511and829912give183049; source/destination scalar multiplication passes, k+1fails on source.

== Scope, custody and next costs
Each version ran once after source/binary/publicinput freeze and S3checkpoint/gate. All positive stages exit0, stderr empty; expected loadercontrol exits101. No unexpected failure, rerun or dropped input. Darwin arm64/Apple M4 Pro,L0/unisolated. Versions are not pooled samples. Separate reports/PDFs sit in reports/run-20261115T2326Z and reports/run-20261115T2337Z; full raw/protocol/hashes are adjacent.

Source recurrence has O(p)coefficient steps with unit-inversion costs, not full-pipeline complexity. General chart discovery, field/section portability, scalable finalLA, larger-field comparisons and genus2/4remain. Full10×goal and old failed coldcost record are unchanged. Performance graphs stay unchanged: no comparable duration cohort was collected. Prior conditional Lean proofs do not formalize complete geometry/code. Primary comparator: Tian2020§4.2.2–4.2.3, https://arxiv.org/html/2007.03172. Classification: engineering integration.
