#import "../local-report-template.typ": render
#let s=json("summary.json")
#let plan=json("runtime-plan.json")
#let e=json("local-evidence.json")
#for (i,oracle) in ("DelayedPseudoRemainderPairOnly","PreparedSymmetricPairOnly","PseudoRemainderPairOnly").enumerate() {
 if i>0 {pagebreak()}
 render(summary:s+(strata:s.strata.filter(r=>r.oracle==oracle),),plan:plan,
 figure:image("ratios-"+oracle+".svg",width:100%),title:"Cold graph IC: "+oracle,
 run:"local-delayed-pseudo-pair-20261116-17",date:"16 November 2026",duration:str(e.outer_wall_seconds),
 findings:[All 810 executions complete: 135 warmups and 675 measured. Every scalar and
 cross-oracle relation stream matches; pseudo/delayed semantic products are
 identical. 26 of 27 individual strata pass controls; delayed p127/seed163 is
 rejected. The following pages retain every direct comparison. [pcc:unverified]])
}
#let fmt(v)=str(calc.round(v,digits:3))
#for (reference,candidate,title,finding) in (
 ("PseudoRemainderPairOnly","DelayedPseudoRemainderPairOnly","Incremental delayed convolution",
 [Eight comparisons pass controls. Two upper intervals are below 1:
 p523/seed157 costs 0.925 (interval 0.863–0.983), p1039/seed167 costs 0.844
 (interval 0.653–0.982). Six controlled comparisons are inconclusive;
 p127/seed163 is rejected. There is no uniform incremental gain. [pcc:unverified]]),
 ("PreparedSymmetricPairOnly","DelayedPseudoRemainderPairOnly","Combined arithmetic versus prepared S3",
 [All eight controlled comparisons have upper intervals below 1, with direct
 cold cost ratios 0.813–0.931. The ninth is rejected through timing controls.
 These are direct measurements in this trial; no cross-run ratio composition.
 [pcc:unverified]]),
 ("PreparedSymmetricPairOnly","PseudoRemainderPairOnly","Fresh pseudo-remainder control",
 [All nine comparisons pass controls; five upper intervals are below 1 and
 four are inconclusive. This fresh control retains the mixed nature of the
 earlier pseudo-remainder result. [pcc:unverified]]),
) {
 pagebreak()
 set page(paper:"a4",margin:16mm)
 set text(font:"DejaVu Sans",size:10pt)
 heading(level:1,title)
 [Source #raw(s.source_revision.slice(0,9)). Candidate #raw(candidate),
 reference #raw(reference). Same factor bases, residuals, group work,
 recovered scalars and relation fingerprints. Semantic product equality is
 additionally required for pseudo/delayed. [pcc:unverified]]
 parbreak()
 table(columns:(1fr,1fr,1.5fr,1fr),inset:5pt,
 [p / seed],[Candidate / reference],[95% interval],[Both controls],
 ..s.oracle_comparisons.filter(r=>r.reference==reference and r.candidate==candidate).map(r=>(
 [#r.p / #r.seed],[#fmt(r.candidate_graph_wall_over_reference.geometric_mean)],
 [#fmt(r.candidate_graph_wall_over_reference.bootstrap_95.at(0))–#fmt(r.candidate_graph_wall_over_reference.bootstrap_95.at(1))],
 [#if r.eligible_local_controlled_comparison {[Pass]}else{[Rejected]}],
 )).flatten())
 parbreak()
 finding
 parbreak()
 [All preparation, sampler setup/restarts, failed oracle attempts, graph
 elimination and scalar replay stay inside each cold interval. No retries.
 The delayed convolution checks modulus and coefficient ranges, accumulates
 at most 5 products with sum below $2^43$, reduces once per output, and falls back outside
 its arithmetic guard. Pseudo-remainder/root/exceptional recovery is shared.
 Every semantic multiplication/addition hook remains. [pcc:unverified]]
 parbreak()
 [The largest-field delayed candidate costs 3.166, 4.523 and 3.606 times the
 existing local rho for seeds 157, 163, 167. No local parity flag passes.
 All 9 mechanism diagnostics recover at first repeated decomposable abscissa,
 consistent with generic coalescence; these counters are not a causal proof
 or independent relation-density evidence. [pcc:unverified]]
 parbreak()
 [Intervals cover five repeated timings on each fixed curve, not independent
 curve samples. Cold elapsed time is primary; the legacy cost model is
 incomplete. Public synthetic Fp^3 fixtures isolate this extension family
 absent from the standard panel. No signed independent verdict, strongest
 external reference, canonical encrypted-schema validation or admitted
 ranking. Full raw/binaries/checkpoint remain in hashed local custody;
 local-evidence.json records exact flow and the raw path/hash. [pcc:unverified]]
}
