#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(justify:false,leading:0.6em)
#text(size:18pt,weight:"bold")[Critical-pair source certificates]
#v(4pt)
#text(size:12pt)[qualification001 — fixed native software qualification]
#v(8pt)
Eight empty auxiliary ideals complete with checked source derivations. All four positive targets hit the unchanged reduction cap. The six small controls and preserved natural Kummer D4 case also match.

*Frozen source:* 7f7ce5112e. *Schedule:* Minimum-LCM-degree critical pairs and coprime-leading-term skips; creation-order reducers. No chain filter.

#image("completion.svg",width:100%)

The plotted values copy the complete twelve-target E/F19 census: every
nonzero rational target is retained. Counts are targets, with no sampling
interval, timings, fitted exponent or growing-family inference.

#table(columns:(1fr,2.5fr),inset:6pt,
 [Control],[Recorded outcome],
 [Six small exact ideals],[All six complete and match the existing comparator],
 [Natural Kummer norm, D4],[Complete; quotient1. 579,063 recorded reduction-loop steps;10 exported polynomials;918 retained DAG nodes;155,119 certificate bytes.],
 [Auxiliary quartics, D6],[8 certified empty; 4 unknown;0 completed positive models],
 [OpenSSL replay],[12 target statuses retained; completed sets checked where present],
)
#v(6pt)
The four positive targets each stop at 1,000,001 recorded reduction-loop steps. Calls3/4 retain328 working basis polynomials and calls11/12 retain338. These steps include visited irreducible terms; they are not the previous PC producer's reduction count.

#pagebreak()
#text(size:16pt,weight:"bold")[Verification and evidence scope]
#v(6pt)
#image("../../pipeline.svg",width:60%)

Every accepted basis has source-ideal derivations and passes the separately
compiled unchanged checker before quotient extraction. The checker verifies
bounded-degree DAG identities, Buchberger completion and original-generator
inclusion. Producer and checker share arithmetic/order primitives; this is
native symbolic checking, without a formal program proof or signed standing.

The same eight-variable projected auxiliary equations lift to the original
fourteen-variable equations. The reference compares complete tiny-field
sets. Unknown cases provide no accepted witness or complete-set conclusion.
The natural Kummer and small controls retain their prior exact domains.

Protection caps:20s CPU/wall per producer plus checker,2GiB, one thread,
D≤6,1M reduction steps,2M charged slots,20k attempted rows,40k DAG nodes,
128 exported basis polynomials and1MiB exported proof. These are fixed caps,
not measured durations. No cap, field, target or degree was retuned.

The sole round uses a separate immutable execution wrapper. All19 build and
nine pre-call wrapper/source/binary bindings match at completion. Raw files,
exits, failures, inputs, proofs and status tables are in the separate whole
archive: *314,744 bytes*, SHA256
#raw("bedb93ed6cdfd9c259d1a85e804d5454228289ef2920439ad773b917855021fa",block:true)
Normalized unpack/repack is byte-identical. Artifact-autofix run43720392567
uploads both whole objects and replays both complete remote bytes. Bot0683ddd111
retains both SHA256/length pointers; hydrated checks have zero problems.
No archive is split or old evidence rewritten.

Sources checked (KB unavailable): existing PC/indexed producer, separate
checker, initial-elimination and small-positive-target reviews; Berthomieu,
Eder and Safey El Din, msolve, arXiv:2104.03572v2, Theorem3 and section3.1.
Author-hosted F99a formats returned503. No novelty or exhaustive review claim.
The sufficient growing-family degree and useful positive yield remain open.

Read PROTOCOL.md, PUBLICATION.md and README.md beside the implementation.
This report makes no performance, general-field or complete ECDLP claim.
All new scoped statements remain PCC unverified.
