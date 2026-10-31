#set page(paper:"a4",margin:(x:17mm,y:16mm),numbering:"1",header:align(right,text(size:8pt)[Fresh ECDLP experiments · 2026-10-19]))
#set text(font:"DejaVu Sans",size:9.1pt)
#set par(leading:.55em,spacing:.8em)
#show heading.where(level:1):set text(size:18pt)
#show heading.where(level:2):set text(size:12pt)
#show table.cell:set text(size:8pt)
#show link:set text(fill:rgb("235789"))
#include "report-body.typ"
#pagebreak()
#heading(level:1)[Recorded observations]
#figure(image("figures/genus_cost_bands.svg",width:100%),caption:[#text("Price sensitivity, not a confidence interval. Cold work includes setup, transfer, roots, elimination and source verification. Each point uses 180 matched targets; both cohorts contain 1,260 targets.")])
#figure(image("figures/binary_coverage.svg",width:100%),caption:[#text("Primary and supplementary complete finite sumsets. Descended-b controls demonstrate why the trace kernel alone does not establish a 3/8 limit.")])
#figure(image("figures/coverage_controls.svg",width:100%),caption:[#text("The two PKM nulls answer different questions. Odd-cell bars are native curve-resampling intervals. Standardizations and intervals do not prove universal random equivalence.")])
#figure(image("figures/signed_cycle_stopping.svg",width:100%),caption:[#text("The signed collector distinguishes graph cycles from useful target dependencies. Repeated split divisor classes are discarded in the primary arm.")])
#pagebreak()
#include "claims-body.typ"
