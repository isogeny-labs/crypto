#import "../delayed-component-report.typ": render
#let s=json("summary.json")
#let flow=json("local-evidence.json")
#for (i,p) in (67,127,1039).enumerate() {
 if i>0 {pagebreak()}
 render(s,flow,p,image("ratios-p"+str(p)+".svg",width:100%,height:62mm,fit:"contain"),[
  Fresh seeds 101/103/107: every output matches the dense qualifier. 33 of 36 cases
  pass controls; 32 of those 33 have upper intervals below 1, one near-tie crosses 1.
  Three A/A rejections remain diagnostic. Means across all cases span 0.734–0.968.
  All 12 largest-field comparisons pass and improve the complete coordinate phase.
  Full cold IC remains to be measured separately. Outer process: 21.469 seconds.
  [pcc:unverified]
 ])
}
