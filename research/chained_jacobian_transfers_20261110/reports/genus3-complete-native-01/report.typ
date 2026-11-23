#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#set par(leading:0.6em)
#let ps=json("paragraphs.json")
#text(size:19pt,weight:"bold")[Completed prime-field genus-three descent]
#parbreak()
Native component result · 2026-11-13 · local acceptance / unsigned
#parbreak()
#image("flow.svg",width:100%)
#image("counts.svg",width:100%)
#for p in ps.slice(0,2) { text(p);parbreak() }
#table(columns: (1fr,1fr,1fr,1fr),inset:5pt,[q],[K-rational branches],[Geometric branches],[Origin multiplicities],[5],[0],[4],[1,1,1,3],[7],[4],[4],[1,1,1,3],[11],[4],[4],[1,1,1,3])
#pagebreak()
#text(size:17pt,weight:"bold")[Preserved failures and corrected coverage]
#parbreak()
#for p in ps.slice(2,6) { text(p);parbreak() }
#text(size:12pt,weight:"bold")[Three different mathematical scopes]
#parbreak()
The runtime fibre has degree2; its rational trace divisor has degree6; the descended product isogeny has degree8. Finite native enumeration checks fixed encoded groups. The Lean files prove the exact algebra and conditional group implications listed in the proof map. The written geometry identifies those hypotheses with the actual covers. [pcc:unverified]
#pagebreak()
#text(size:17pt,weight:"bold")[Execution, publication and reproducibility]
#parbreak()
#for p in ps.slice(6) { text(p);parbreak() }
