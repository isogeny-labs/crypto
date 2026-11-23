#set page(width: 210mm, height: auto, margin: 9mm)
#set text(font: "New Computer Modern", size: 9pt)
#let data = json("../../small-m4-v1/final/audit.json")
#text(size: 14pt, weight: "bold")[N83 m=4 public query: recorded capped work]
#v(2mm)
One public point · K1,D1,D2,K2 · shared L0 host · all at 100,000 conflicts
#v(3mm)
#grid(columns: (3mm, 65mm, 3mm, 65mm), column-gutter: 2mm,
  rect(width: 3mm, height: 3mm, fill: rgb("386cb0"), stroke: none), [XOR propagation elapsed],
  rect(width: 3mm, height: 3mm, fill: rgb("b7c5d6"), stroke: none), [Remaining worker elapsed])
#v(4mm)
#for arm in data.arms {
  grid(columns: (14mm, 142mm, 28mm), column-gutter: 2mm,
    text(weight: "bold", upper(arm.label)),
    stack(dir: ltr, spacing: 0pt, rect(width: arm.xor_s / 1800 * 142mm, height: 7mm, fill: rgb("386cb0"), stroke: none), rect(width: (arm.wall_s - arm.xor_s) / 1800 * 142mm, height: 7mm, fill: rgb("b7c5d6"), stroke: none)),
    [#calc.round(arm.wall_s, digits: 3) s])
  v(3mm)
}
#grid(columns: (14mm,142mm), column-gutter: 2mm, [], [
  #box(width: 142mm, height: 8mm)[
    #line(length: 142mm, stroke: 0.5pt)
    #for tick in (0,450,900,1350,1800) {
      place(top + left, dx: tick / 1800 * 142mm, dy: 1mm, text(str(tick)))
    }
  ]
])
#text(size: 8pt)[Seconds from worker start; one execution per bar. XOR uses elapsed intervals, not exclusive CPU time. All outcomes are UNKNOWN_search_cap; natural yield/rank remain unresolved. Source: small-m4-v1/final/audit.json, worker revision 611aad60e4, 2026-10-19.]
