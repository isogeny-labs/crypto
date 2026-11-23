#import "../delayed-component-report.typ": render
#let s=json("summary.json")
#let flow=json("local-evidence.json")
#for (i,p) in (67,127,1039).enumerate() {
 if i>0 {pagebreak()}
 render(s,flow,p,image("ratios-p"+str(p)+".svg",width:100%,height:62mm,fit:"contain"),[
  All outputs and semantic multiplication/dimension controls match.35 of 36 cases
  pass local timing controls and all 35 upper intervals are below 1. One p67 case
  fails A/A and remains diagnostic. Means across all cases span 0.634–0.787.
  All 12 largest-field cases pass the frozen conditional threshold; the retained
  interstage receipt authorized fresh stage 13. No ratios are combined across
  stages. Outer process: 22.846 seconds. [pcc:unverified]
 ])
}
