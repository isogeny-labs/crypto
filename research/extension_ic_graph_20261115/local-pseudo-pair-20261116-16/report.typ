#import "../local-report-template.typ": render
#let s=json("summary.json")
#let plan=json("runtime-plan.json")
#let e=json("local-evidence.json")
#for (i,oracle) in ("PreparedSymmetricPairOnly","PseudoRemainderPairOnly").enumerate() {
 if i>0 {pagebreak()}
 render(summary:s+(strata:s.strata.filter(r=>r.oracle==oracle),),plan:plan,
 figure:image("ratios-"+oracle+".svg",width:100%),title:"Cold graph IC: "+oracle,
 run:"local-pseudo-pair-20261116-16",date:"16 November 2026",duration:str(e.outer_wall_seconds),
 findings:[All 540 executions complete with correct scalars and exact paired
 relation streams. Retained: 90 warmups and 450 measured executions; outer
 process wall 15.151 seconds, exit zero, no retries. Sixteen of eighteen
 individual strata pass controls. The final page shows all nine direct
 candidate/reference comparisons, including the two rejected comparisons.
 [pcc:unverified]])
}
#pagebreak()
#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:10pt)
= Pseudo-remainder S3 versus prepared S3
Source #raw(s.source_revision.slice(0,9)); identical factor bases, residual
counts, group work, recovered scalars and admitted relation fingerprints.
Only polynomial remainder arithmetic changes: extra products replace modular
inversions, with final monic normalization and shared root/exception handling.
Every base preparation, sampling restart, failed test, graph elimination and
scalar replay remains inside each cold interval. [pcc:unverified]

#let fmt(v)=str(calc.round(v,digits:3))
#table(columns:(1fr,1fr,1.5fr,1fr),inset:5pt,
 [p / seed],[Pseudo / prepared],[95% interval],[Both controls],
 ..s.oracle_comparisons.map(r=>(
 [#r.p / #r.seed],[#fmt(r.candidate_graph_wall_over_reference.geometric_mean)],
 [#fmt(r.candidate_graph_wall_over_reference.bootstrap_95.at(0))–#fmt(r.candidate_graph_wall_over_reference.bootstrap_95.at(1))],
 [#if r.eligible_local_controlled_comparison {[Pass]}else{[Rejected]}],
 )).flatten())

Seven direct comparisons pass controls. Three have upper intervals below one:
p523/seed139, p523/seed151 and p1039/seed151. Four controlled comparisons are
inconclusive; the remaining two comparisons are rejected by timing controls.
The largest-field candidate costs 2.389, 27.847 and 9.369 times the existing
local rho for seeds139,149,151. No local parity flag passes. [pcc:unverified]

All nine diagnostic targets recover at the first repeated decomposable
abscissa. This is consistent with generic walk coalescence and supplies no
independent relation-density or improved-exponent evidence. The new arithmetic
preserves that mechanism. These diagnostics are not a causal proof.
[pcc:unverified]

Intervals cover five repeated timings on each fixed curve, not independent
curve samples. The primary outcome is complete cold elapsed time; the legacy
multiplication model does not fully price divisions or collection work.
Synthetic public Fp^3 fixtures isolate the existing extension-field oracle;
the standard panel has no matching family. No signed independent verdict,
strongest external comparison, canonical encrypted-schema validation or
admitted ranking is asserted. All failures and raw/source/binary hashes remain
retained with explicit local custody. [pcc:unverified]
