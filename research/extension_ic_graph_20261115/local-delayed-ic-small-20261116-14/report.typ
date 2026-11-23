#import "../local-report-template.typ": render
#let s=json("summary.json")
#let plan=json("runtime-plan.json")
#for (i,oracle) in ("PreparedSymmetricPairOnly","F4Groebner","F4KrylovDelayed").enumerate() {
 if i>0 {pagebreak()}
 render(summary:s+(strata:s.strata.filter(r=>r.oracle==oracle),),plan:plan,
 figure:image("ratios-"+oracle+".svg",width:100%),title:"Full cold IC: "+oracle,
 run:"local-delayed-ic-small-20261116-14",date:"16 November 2026",duration:"199.383",
 findings:[All scalars and dense/delayed relation streams match. Seven of twelve
 strata pass local timing controls; five A/A rejections remain diagnostic.
 Direct delayed/dense graph wall is 0.857 at p127/seed109 (interval 0.819–0.904),
 the sole fully controlled direct comparison. The final page retains all four
 direct comparisons. Prepared S3 remains much cheaper on these inputs.
 [pcc:unverified]])
}
#pagebreak()
#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:10pt)
= Direct cold pipeline comparison
Source #raw(s.source_revision.slice(0,9)); identical factor bases, residual
counts, group work, recovered scalars and admitted relation fingerprints.
Field counts may differ. Every base preparation, sampling restart, graph
elimination and scalar replay is inside each cold interval. [pcc:unverified]

#let fmt(v)=str(calc.round(v,digits:3))
#table(columns:(1fr,1fr,1.4fr,1fr),inset:5pt,
 [p / seed],[Delayed / dense],[95% interval],[Both controls],
 ..s.oracle_comparisons.map(r=>(
 [#r.p / #r.seed],[#fmt(r.candidate_graph_wall_over_reference.geometric_mean)],
 [#fmt(r.candidate_graph_wall_over_reference.bootstrap_95.at(0))–#fmt(r.candidate_graph_wall_over_reference.bootstrap_95.at(1))],
 [#if r.eligible_local_controlled_comparison {[Pass]}else{[Rejected]}],
 )).flatten())

The p127/seed109 comparison supplies a local full-pipeline engineering gain.
The other three ratios remain ineligible through retained A/A failures.
Intervals describe repeated timing of fixed curves; independent replay,
strongest external references and complete primitive pricing remain absent.
No general crossover or admitted ranking is inferred. [pcc:unverified]
