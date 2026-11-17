#set page(width:210mm,height:auto,margin:9mm)
#set text(font:"New Computer Modern",size:9pt)
#let records = (("U","off"),("U","on"),("D","off"),("D","on")).map(p => json("data/"+p.at(0)+"-"+p.at(1)+".json"))
#let ceiling = calc.ceil(calc.max(..records.map(r => r.process_wall_ms))/1000)*1000
#text(size:14pt,weight:"bold")[N83 full-size bases: constructed controls]
#v(2mm)
16,627 columns each · four controls and independent replays · shared L0 host
#v(4mm)
#for r in records {
 grid(columns:(25mm,132mm,30mm),column-gutter:1mm,
  [*#r.arm #if r.xor_decided_cache {[on]} else {[off]}*],
  rect(width:r.process_wall_ms/ceiling*132mm,height:7mm,stroke:none,fill:if r.xor_decided_cache {rgb("168578")} else {rgb("386cb0")}),
  [#calc.round(r.process_wall_ms/1000,digits:3) s])
 v(3mm)
}
#grid(columns:(25mm,132mm),column-gutter:1mm,[],[
 #box(width:132mm,height:8mm)[
  #line(length:132mm,stroke:0.5pt)
  #for fraction in (0,0.25,0.5,0.75,1) {place(top+left,dx:fraction*132mm,dy:1mm,text(str(fraction*ceiling/1000)))}
 ]
])
#text(size:8pt)[Worker seconds, one observation per bar. U is the retained base; D is the exported dimension-18 base. Both contain 2,760,082 points. Controls share a constructed target within each base, with exact off/on SAT-model and row equality. These observations do not determine natural yield or a timing winner. Source: four raw worker receipts in data/, full-m4-v1 controls, worker source d9ef42301cc3.]
