#set page(width: 210mm, height: auto, margin: 8mm)
#set text(font: "New Computer Modern", size: 9pt)
#text(size: 14pt, weight: "bold")[N83 XOR cache: constructed control intervals]
#v(2mm)
One observation per cell · shared L0 host · all four passed independent replay
#v(4mm)
#for (base, setting) in (("K2048","off"),("K2048","on"),("D12","off"),("D12","on")) {
  let r = json("data/" + base + "-" + setting + ".json")
  grid(columns: (28mm,135mm,24mm), column-gutter: 1mm,
    [*#base #setting*],
    rect(width: r.process_wall_ms / 800 * 135mm, height: 7mm, stroke: none,
      fill: if setting=="off" {rgb("386cb0")} else {rgb("168578")}),
    [#calc.round(r.process_wall_ms,digits:3) ms])
  v(3mm)
}
#grid(columns: (28mm,135mm), column-gutter: 1mm, [], [
 #box(width:135mm,height:8mm)[
  #line(length:135mm,stroke:0.5pt)
  #for t in (0,200,400,600,800) {place(top+left,dx:t/800*135mm,dy:1mm,text(str(t)))}
 ]
])
#text(size:8pt)[Worker milliseconds, including import/model/root and verification. Same target within each base; different bases use different constructed targets. No admitted speedup. Source: xor-cache-v2 controls, start 2026-10-20 01:16:34 UTC; four archived worker receipts.]
