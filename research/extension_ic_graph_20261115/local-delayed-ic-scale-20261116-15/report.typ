#import "../local-report-template.typ": render
#let s=json("summary.json")
#let plan=json("runtime-plan.json")
#for (i,oracle) in ("PreparedSymmetricPairOnly","F4KrylovDelayed").enumerate() {
 if i>0 {pagebreak()}
 render(summary:s+(strata:s.strata.filter(r=>r.oracle==oracle),),plan:plan,
 figure:image("ratios-"+oracle+".svg",width:100%),title:"Censored cold IC: "+oracle,
 run:"local-delayed-ic-scale-20261116-15",date:"16 November 2026",duration:"900.005",
 flow:[Timeout at 900.005 seconds (exit124): 224 starts, 223 completed executions
 (40 warmup, 183 measured), one unfinished arm and 16 unattempted executions of 240
 planned. The unfinished arm is p1039/seed113, delayed graph, measured round 1.
 Partial-arm time is unknown and not imputed. [pcc:unverified]],
 findings:[All four prepared-pair comparisons pass controls at 1.24–2.13 times
 the existing rho. One complete delayed-F4 comparison passes, two complete
 delayed comparisons fail A/A, and the last has only one of five measured
 rounds. Its displayed 645.22 ratio is a one-round diagnostic with no usable
 interval. No eligible full-pipeline interval reaches 1. The candidate remains
 much costlier than the pair baseline. Every rejected/incomplete case stays.
 [pcc:unverified]])
}
