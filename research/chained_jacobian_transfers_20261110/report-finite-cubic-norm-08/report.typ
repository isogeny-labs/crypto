#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(justify: false, leading: 0.6em, spacing: 0.7em)
#set heading(numbering: none)
= Universal finite cubic norm at223
#text("Every nonzero Fin223 coefficient triple has nonzero norm and a derived two-sided coefficient inverse. Both unchanged original Lean4.35/4.34.1 gates audit37 declarations/0 failing/0 trusted each, with twenty new complete original-extractor bindings121d9f. Claims remain unsigned. [pcc:unverified]")
== Exact representation
#text("For x=(a,b,c), write N=a³+3b³+9c³−9abc and P=(a²−3bc,3c²−ab,b²−ac). The source preserves the full Constructor01 prefix and four byte-identical full04 schoolbook/cofactor/norm/scale definitions. No ambient field/domain or norm-unit assumption is supplied. [pcc:unverified]")
#image("zero-norm-contradiction.svg", width: 100%)
== Structural proof of N=0 iff x=0
#text("Only223 base elements are checked per finite certificate: t=0 or t^221*t=1; t³=0 implies t=0. Constructor01 already excludes u³=3. No223³ coefficient population is enumerated. [pcc:unverified]")
#text("Set p1=3c²−ab,p2=b²−ac. A generic ring identity gives p1³−3p2³=N(3c³−b³). If N=0 and p2!=0, its explicit base inverse u yields (u*p1)³=3, impossible. Thus p2=0, and cube-zero gives p1=0. These relations imply b³=3c³. If c!=0, its base inverse v yields (v*b)³=3, again impossible. Thus c=b=0, and N=a³=0 forces a=0. The converse is direct substitution. [pcc:unverified]")
#text("Ratio lemmas use free base-unit witnesses in low-degree generic ring identities; the221st-power certificate is instantiated afterward. This avoids expanding a variable221st power in polynomial automation. The accepted repair changes no statement, hypothesis, resource, tactic limit or gate. [pcc:unverified]")
== Genuine controls
#text("The zero tuple has norm0, the unit axis norm1/product1. For reducible d=1, nonzero(222,1,0) has norm0. The theorem stays specific to noncube3/q223; other cubic parameters are not silently admitted. [pcc:unverified]")
#pagebreak()
= A derived coefficient inverse
#text("For every x!=0, norm_zero_iff implies N!=0 and the base certificate derives N^221*N=1. The exact schoolbook product gives x*P=(N,0,0). Scalar transport and commutativity then give both x*(N^221*P)=(1,0,0) and (N^221*P)*x=(1,0,0). The earlier supplied norm-unit condition is derived here. [pcc:unverified]")
#table(columns: (36%,64%), inset: 6pt,
 [Central declarations], [norm_zero_iff; nonzero_input_nonzero_norm; derived_norm_unit; every_nonzero_coefficient_inverse. Complete hypotheses/bindings in final04.],
 [Exact audit coverage], [13 old constructor+20 new named+4 new generated equations=37,0 failing/0 trusted in both original gates.],
 [Resources/foundations], [One worker/120s/original memory/default tactic limits; propext,Classical.choice,Quot.sound only. No native decision/custom axiom.],
 [Proof boundary], [Finite coefficient-operation identities. Full cubic field typeclass, Rust norm/Fp.inv/compiler, Jacobian/group maps, counters/cost remain separate.],
)
== Rejected attempts retained
#text("Firste5bbb1 simpa leaves9*a*0*0. Second7faa57 adds mul_zero but leavesa³−0. Both full sources/logs/printed downstream sorryAx remain rejected; Consumer not attempted after Root rejection. Final uses original grind only for that one low-degree normalization. No hypothesis or gate weakened. [pcc:unverified]")
== Sources and reproducibility
#text("Primary preregistratione9d4fe checked complete Constructor01/full04/Word07 and official Arkworks algebra v0.5.0 cubic-extension inverse. Arkworks already uses this established cofactor and assumes the base inverse; no novelty claim. Its cited paper was not reviewed. KB status/exact/negative/adjacent/theorem wrappers/conductor127, locked atlas unread, no absence inference. [pcc:unverified]")
#text("Source SHA256 5f9ad6da74d02cc262e04a4255b31305db48f035d0976f99cfedfa76e31a873d. Exact constructor prefixe81532c9 and four full04 definitions are pinned. Complete source/callers/builds/audits/pins/times/twenty complete bindings remain in finite-cubic-norm-final-04; additive exact-source CI replays both unchanged original gates. [pcc:unverified]")
#text("Canonical chain/cubic/F101 diagrams are unchanged: this adds arithmetic structure, no geometric edge. Active Root278/Consumer12cc ownCI20:55/original authenticated whole store replay still gate normal merge. Original full scientific/runtime/ABI/admission/matched-cost obligations remain fixed and open under paid STOP. [pcc:unverified]")
