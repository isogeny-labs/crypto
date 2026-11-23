#set page(width: 18cm, height: 10.5cm, margin: 5mm)
#set text(font: "Libertinus Serif", size: 9pt)
#let data = json("plot-data.json").curves
*Exact minimum D+A: centered and translated scalar recipes*
#linebreak()
Dark/light blue: centered/translated direct. Dark/light orange: centered/translated fused.
#v(4pt)
#for row in data [
 #let values = (row.centered_direct_score,row.direct_score,row.centered_fused_score,row.fused_score)
 #let colors = (rgb("406c9c"),rgb("7fa9c5"),rgb("d08239"),rgb("e8b57a"))
 #grid(columns: (1.4cm, 1fr, 1cm), gutter: 4pt,
  [#row.curve],
  stack(spacing: 1pt, ..values.enumerate().map(((i,value)) => rect(width: value / 650 * 12cm, height: 2mm, fill: colors.at(i), stroke: none))),
  stack(spacing: 1pt, ..values.map(value => box(height: 2mm)[#text(size: 8pt)[#value]])))
 #v(6pt)
]
#text(size: 7.5pt)[One frozen finite census, 30,780 recipes; no clock. All raw rows and four sensitivity weights retained. D+A is an unweighted API-call proxy, not latency.]
