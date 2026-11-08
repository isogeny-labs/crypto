#set page(width: 17.8cm, height: 6.6cm, margin: 5mm)
#set text(font: "Libertinus Serif", size: 9pt)
#let data = json("plot-data.json")
*Native explicit group operations: minimum unweighted D+A*
#linebreak()
Blue: direct. Orange: fused relation. Structural-v2; 90 rows; clocks disabled.
#v(3pt)
#for row in data [
 #let direct = row.group_count_minima.find(x => x.arm == "window")
 #let fused = row.group_count_minima.find(x => x.arm == "fused")
 #let d = direct.doubles + direct.adds
 #let f = fused.doubles + fused.adds
 #grid(columns: (1.4cm, 1fr, 1.8cm), gutter: 4pt,
  [#row.curve],
  stack(spacing: 2pt,
   rect(width: d / 650 * 11cm, height: 3mm, fill: rgb("286c9c"), stroke: none),
   rect(width: f / 650 * 11cm, height: 3mm, fill: rgb("d98235"), stroke: none)),
  [#d / #f])
 #v(4pt)
]
#text(size: 7pt)[Source-specific counts; map, setup and backend costs remain separate. No runtime optimum or latency result is inferred.]
