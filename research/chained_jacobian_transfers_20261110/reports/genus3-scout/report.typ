#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(justify:false,leading:0.65em)
#text(size:19pt,weight:"bold")[Complete encoded genus-3 descent]
#parbreak()
Source review and frozen proposal · 2026-11-13 · unsigned · no new native run
#parbreak()
#image("flow.svg",width:100%)
#parbreak()
#for paragraph in json("paragraphs.json").slice(0,2) {
  text(paragraph)
  parbreak()
}
#pagebreak()
#text(size:17pt,weight:"bold")[Sources, controls and exact remaining work]
#parbreak()
#for paragraph in json("paragraphs.json").slice(2) {
  text(paragraph)
  parbreak()
}
