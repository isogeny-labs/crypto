#set page(width: 600pt, height: 272pt, margin: 12pt)
#set text(size: 10pt)
#let data = json("../../evidence/summary.json").rows
#let names = ("koblitz", "binary")
#let cell(m, f, t) = {
 let r = data.find(r => r.m == m and r.family == f and r.target_kind == t)
 let certified = r.last_fall_degree != none
 block(width:100%, height:40pt, fill:if certified { rgb("d6eee5") } else {rgb("eef1f5")}, inset:5pt)[
   #strong(if r.linear_rank == none { "—" } else { str(r.linear_rank) }) linear
   #linebreak()
   LFD #if certified { str(r.last_fall_degree) } else { "?" }
 ]
}
*Fixed ell27 degree-two closure: linear rank and certified LFD*
#v(8pt)
#grid(columns:(68pt, ..range(8).map(_ => 1fr)), gutter:3pt,
 [family / target], ..range(2,10).map(m => [*m = #m*]),
 ..names.map(f => ("planted","natural").map(t => (
   [#f #linebreak() #t], ..range(2,10).map(m => cell(m,f,t))
 )).flatten()).flatten())
#v(7pt)
Green: global LFD2 certificate. Gray: LFD unresolved; shown linear rank is measured in V2.
#linebreak()
32 inputs, n=83, one target per cell. Exact integer values; no statistical interval.
#linebreak()
Run n83-fixed-fall-catalog-20261028-r1. Prime systems are outside Boolean V2 closure.
