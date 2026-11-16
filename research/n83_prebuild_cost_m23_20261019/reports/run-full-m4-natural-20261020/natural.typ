#set page(width: 210mm, height: auto, margin: 10mm)
#set text(font: "New Computer Modern", size: 8.5pt)
#let data = json("../../full-m4-v1/final/audit.json")
#let label(r) = if r.status == "UNKNOWN_budget" { "wall cap" } else if r.status == "UNKNOWN_search_cap" { "conflict cap" } else { r.status }
#text(size: 14pt, weight: "bold")[N83 m=4: full-size U/D public searches]
#v(2mm)
16,627 columns / 2,760,082 points per base · eight common public points
#v(1mm)
Each bar is one retained attempt, in execution order. Shared host: L0.
#v(3mm)
#grid(columns: (3mm, 61mm, 3mm, 70mm), column-gutter: 2mm,
  rect(width: 3mm, height: 3mm, fill: rgb("526c8c"), stroke: none), [U: public-x-hash orbit base],
  rect(width: 3mm, height: 3mm, fill: rgb("2b8c7c"), stroke: none), [D: direct-subgroup D18 orbit base])
#v(4mm)
#grid(columns: (19mm, 90mm, 25mm, 43mm), column-gutter: 3mm,
  [*Attempt*], [*Worker interval: 0–1,800 seconds*], [*Seconds*], [*Endpoint*])
#v(2mm)
#for r in data.natural_attempts {
  let seconds = r.process_wall_ms / 1000
  grid(columns: (19mm, 90mm, 25mm, 43mm), column-gutter: 3mm,
    [#r.label],
    rect(width: seconds / 1800 * 90mm, height: 4.5mm,
      fill: rgb(if r.arm == "U" { "526c8c" } else { "2b8c7c" }), stroke: none),
    [#calc.round(seconds, digits: 3)],
    [#label(r)])
  v(1.5mm)
}
#v(3mm)
#text(size: 8pt)[Limits: 10,000 conflicts, 1,790 s internal / 1,800 s outer wall. The endpoints differ across attempts. These bars do not give a time ratio for equal work or a cost per relation. Every cap remains censored. Repeated target-0 attempts are displayed individually; other points have one attempt per base.]
#v(3mm)
#text(weight: "bold")[Latest durable completed-conflict counts]
#v(2mm)
#table(columns: (1fr, 1fr, 1fr, 1fr), inset: 3pt,
  [*Attempt*], [*Conflicts*], [*Decisions*], [*Unobserved tail (s)*],
  ..data.natural_attempts.map(r => (
    [#r.label], [#r.counters.completed_conflicts],
    [#r.counters.completed_decisions],
    [#calc.round(r.progress_audit.unobserved_tail_ms / 1000, digits: 3)]
  )).flatten())
#v(2mm)
#text(size: 8pt)[Wall-cap counters come from the last durable progress sample and are lower bounds on completed work. Final conflict-cap counters are bound to the returned receipt. Source: full-m4-v1/final/audit.json and its content-addressed archive. Worker revision d9ef42301cc3; protocol full-m4-common-public/v1.]
