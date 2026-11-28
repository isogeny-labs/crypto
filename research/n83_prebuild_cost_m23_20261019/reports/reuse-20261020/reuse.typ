#set page(width: 210mm, height: auto, margin: 10mm)
#set text(font: "New Computer Modern", size: 9pt)
#let data = json("accounting.json")
#let value(c, q) = c.projections.find(p => p.reuse_queries == q).setup_plus_worker_and_fresh_replay_containers_seconds
#text(size: 14pt, weight: "bold")[N83 D18: derived finite-query reuse budgets]
#v(2mm)
One-time preparation + worker and fresh replay container intervals
#v(2mm)
#grid(columns: (3mm, 60mm, 3mm, 60mm), column-gutter: 2mm,
  rect(width:3mm, height:3mm, fill:rgb("526c8c"), stroke:none), [Prebuilt],
  rect(width:3mm, height:3mm, fill:rgb("2b8c7c"), stroke:none), [On demand])
#v(3mm)
#for q in (1, 8, 64) {
  let maximum = calc.max(..data.cells.map(c => value(c, q)))
  text(weight: "bold")[#q query budget · panel scale 0–#calc.round(maximum, digits: 3) seconds]
  v(2mm)
  for c in data.cells {
    grid(columns: (33mm, 112mm, 30mm), column-gutter: 3mm,
      [m=#c.m · #c.mode],
      rect(width: value(c,q)/maximum*112mm,height:5mm,
        fill:rgb(if c.mode == "prebuilt" {"526c8c"} else {"2b8c7c"}),stroke:none),
      [#calc.round(value(c,q),digits:3) s])
    v(2mm)
  }
  v(4mm)
}
#text(size:8pt)[Panel scales differ and are printed above each group. Every value is a projection from nine complete probes per mode and m, on eight public points with target 0 repeated. The 64-query panel was not executed. Prebuilt adds 87.989088451 s for census/export/full-object replay; on-demand preparation occurs inside each worker interval.]
#v(2mm)
#text(size:8pt)[Accounting addition, not an algorithm improvement. All source observations are shared-host L0. Docker launch/teardown outside the containers, remote custody/storage, earlier retry history, later rank/linear algebra/individual logarithm work and matched rho are excluded. Source: accounting.json; native derivation: examples/n83_prebuild_reuse_accounting.rs.]
