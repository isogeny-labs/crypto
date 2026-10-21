#set page(width: 210mm, height: 105mm, margin: 8mm)
#set text(font: "New Computer Modern", size: 10pt)
#let data = json("controls-data.json")
#text(size: 14pt, weight: "bold")[N83 m=4 constructed-control worker wall]
#v(2mm)
One observation per base · shared L0 host · controls use different constructed targets
#v(5mm)
#for (i, arm) in data.arms.enumerate() {
  grid(columns: (22mm, 140mm, 25mm), column-gutter: 1mm,
    text(weight: "bold", arm.arm),
    rect(width: arm.process_wall_ms / 800 * 140mm, height: 8mm,
      fill: if i == 0 { rgb("386cb0") } else { rgb("168578") }, stroke: none),
    [#calc.round(arm.process_wall_ms, digits: 3) ms])
  v(5mm)
}
#grid(columns: (22mm, 140mm), column-gutter: 1mm, [], [
  #box(width: 140mm, height: 9mm)[
    #line(length: 140mm, stroke: 0.5pt)
    #for tick in (0, 200, 400, 600, 800) {
      place(top + left, dx: tick / 800 * 140mm, dy: 2mm,
        text(if tick == 800 { "800 ms" } else { str(tick) }))
    }
  ]
])
#v(3mm)
#text(size: 8pt)[Source: small-m4-v1-controls, 2026-10-19; controls-data.json and archived worker receipts. Both controls passed independent general-field replay. No natural-query timing or base ranking is shown.]
