#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Joined IC through a computed calibration base
F127 · genus 3 · 15 November 2026
#image("flow.svg",width:100%)

The joined path recovers k=183049 and verifies [k]D1=B on the original source curve and recovered destination classes. It computes a new calibration base and matrix, lifts both original public inputs, and collects point logs afresh. No old original-generator/target image, alignment matrix, point-log table or expected scalar is loaded. The quotient kernel and normalization remain supplied setup inputs; this is a correctness integration, without a timing ratio.

== Fresh calibration and both inverse solves
The degree18fiber is computed again with all exact restriction/saturation and124regular-value controls. Use the first supported point(54,56). Its doubled source class has u=(14,25,16,1), v=(103,116,48), with target image twice((9,13)-(89,44)). The first12multiples fit a rank63system; the matrix is invertible of rank8. Four withheld multiples and corruption rejection pass. Both original inputs use the first supported offset1.

A rebuilt private msolve0.11 passes71upstream tests. Positive control and both original-input systems give degree12parametrizations satisfying all12equations exactly. Each actual polynomial factors into two cubics and three quadratics; all five factors reconstruct two distinct divisor representations and one common class per input. Adjacent public coordinates choose signs using the new calibration image. All ten corrupted-adjacency checks reject.

== Fresh relations, descent and verification
The48pointbase examines1128pairs,878unique lines and314split relations. Sparse elimination removes81large-prime columns. The remaining232×47core is dense, rank46/nullity1; reverse tracing gives all129point logs. All314relation residuals pass.

Twenty conic pairs are examined per original input. D1:3accepted,16nonsplit,1projection/infinity exclusion. B:3accepted,17nonsplit. Every accepted conic passes exact intersection/root checks,127resultant evaluations and an independent Jacobian divisor identity. The centered formula, public sign and new calibration log137451 give original logs34511and829912. Their ratio is183049; actual source and destination scalar multiplications pass, while k+1fails on source. These are repeated checks of one registered pair, not independent curve samples.

== Frozen gates and proof scope
Prep: sourcecc24c005a260/checkpoint225944. Solve: df1c2f9a5687/230343. Class recovery:8c030a9a4df1/230658. Descent:4aea4bb4b468/230937. All dated2026-11-15; each executed once, exit0, empty stderr. Protocols, source, locked dependencies, binary hashes and raw outputs are adjacent. No scientific rerun or discarded input.

Existing checked Lean degree and scalar-centering theorems support their stated conditional identities, not the whole implementation or geometric isogeny premises. See prior proof maps linked from RESULTS.md. Classification: engineering integration; no novelty or general complexity claim. Tian2020§4.2.2–4.2.3 is the primary comparator: https://arxiv.org/html/2007.03172.

Next replace copied kernel/normalization with fresh cold construction, then register and measure all charged stages. Fully sparse final solving, larger-field evidence, genus2/4 and the full10×goal remain open. Existing performance graphs remain unchanged. Evidence: RESULTS.md, RUN.txt, SOLVE.txt, RECOVER.txt, DESCENT.txt and PROTOCOL files, in research/hec_fiberalign13_20261115.
