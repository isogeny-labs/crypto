#set page(width: 210mm, height: auto, margin: 10mm)
#set text(font: "New Computer Modern", size: 9pt)
#let data = json("../../xor-cache-v2/final/audit.json")
#let matched = data.comparisons.filter(c => c.status == "PASS_MATCHED_SEARCH_ENDPOINT")
#let max-visits = if matched.len() > 0 { calc.max(..matched.map(c => c.off_repivot_visits)) } else { 1 }
#text(size: 14pt, weight: "bold")[N83 m=4: satisfied XOR-row cache]
#v(2mm)
Two public targets · two bases · A1/B1/B2/A2 per cell · 10,000-conflict cap
#v(3mm)
#grid(columns: (3mm, 66mm, 3mm, 66mm), column-gutter: 2mm,
  rect(width: 3mm, height: 3mm, fill: rgb("526c8c"), stroke: none), [Cache off: checked row visits],
  rect(width: 3mm, height: 3mm, fill: rgb("2b8c7c"), stroke: none), [Cache on: checked row visits])
#v(4mm)
#for c in data.comparisons {
  text(weight: "bold")[#c.arm · public target #c.target_index]
  v(1mm)
  if c.status == "PASS_MATCHED_SEARCH_ENDPOINT" {
    for (visits, color) in ((c.off_repivot_visits, "526c8c"), (c.on_repivot_visits, "2b8c7c")) {
      grid(columns: (135mm, 40mm), column-gutter: 3mm,
        rect(width: visits / max-visits * 135mm, height: 5mm, fill: rgb(color), stroke: none),
        [#calc.round(visits / 1000000, digits: 3) million])
      v(1mm)
    }
    text(size: 8pt)[#calc.round(100 * c.row_visit_reduction_fraction, digits: 3)% fewer checked visits; exact matching decisions, conflicts, restarts, XOR passes, row operations and row scans.]
  } else {
    text(fill: rgb("9c432a"))[Censored: endpoints differ; no paired work or time ratio.]
  }
  v(4mm)
}
#text(size: 8pt)[Each bar is a deterministic work counter, repeated twice with identical values within setting. The counter includes visits to already-decided rows. It is not the total XOR work: row operations and row scans are unchanged. Source: xor-cache-v2/final/audit.json. Worker revision 1de47a9d4186.]
#v(3mm)
#text(weight: "bold")[Recorded worker intervals — shared-host L0 observations]
#v(2mm)
#table(columns: (1.4fr, 1fr, 1fr, 1fr), inset: 4pt,
 [*Base / target*], [*Off mean (s)*], [*On mean (s)*], [*Off / on*],
 ..data.comparisons.map(c => if c.status == "PASS_MATCHED_SEARCH_ENDPOINT" {
   ([#c.arm / #c.target_index], [#calc.round(c.off_mean_worker_s,digits:3)], [#calc.round(c.on_mean_worker_s,digits:3)], [#calc.round(c.worker_ratio_off_over_on,digits:4)])
 } else { ([#c.arm / #c.target_index], [censored], [censored], [—]) }).flatten())
#v(2mm)
#text(size: 8pt)[Two observations per setting; one fixed target per cell. These are worker intervals ending before final receipt serialization. No isolated timing interval, complete IC speedup, external-fastest comparison or best-base selection is established. Natural search caps retain censored yield/rank.]
