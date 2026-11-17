#import "../krylov-component-report.typ": render
#let s=json("summary.json")
#let flow=json("local-evidence.json")
#for (i,p) in (67,127,1039).enumerate() {
 if i>0 { pagebreak() }
 render(s,flow,p,[
  #image("ratios-p"+str(p)+".svg",width:100%,height:62mm,fit:"contain")
  All 36 systems pass exact-output and local timing controls. Ratios range
  0.998–1.390 across fields. Two point estimates just below1 have intervals
  crossing1; no upper interval is below1. Every first measured sparse call
  succeeds on its first projection with no dense fallback. This scalar sparse
  implementation supplies no accepted complete-phase gain. [pcc:unverified]
 ])
}
