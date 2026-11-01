#set page(width: 190mm, height: auto, margin: 8mm)
#set text(font: "New Computer Modern", size: 10pt)
#let data = json("root-inventory.json")
#assert(data.status == "PASS_ROOT_INVENTORY")
#assert(data.sat_construction_performed == false)
#text(size: 16pt, weight: "bold")[D18 selected roots: shared coordinates]
#v(3mm)
Complete input census: #data.columns roots, each with 83 binary coordinates.
#v(3mm)
#grid(columns: (17mm, 56mm, 94mm), gutter: 1mm,
  rect(width: 100%, height: 13mm, fill: rgb("d4eee8"), stroke: none,
    align(center + horizon, [Bit 0\ *Zero*])),
  rect(width: 100%, height: 13mm, fill: rgb("e3eaf3"), stroke: none,
    align(center + horizon, [Bits 1–17\ *Varying*])),
  rect(width: 100%, height: 13mm, fill: rgb("d4eee8"), stroke: none,
    align(center + horizon, [Bits 18–82\ *Zero*])))
#v(2mm)
Coordinate groups are schematic, not proportional to their widths.
All #data.shared_bits.len() shared coordinates are zero. The inventory
does not compute affine rank or the dimension of the selected set's span.
#v(5mm)
#text(weight: "bold")[Derived root-implication reductions]
#v(2mm)
For m summands: m × #data.shared_bits.len() × (#data.columns − 1).
#v(3mm)
#grid(columns: (9mm, 112mm, 41mm), gutter: 2mm,
  [*m*], [*Clause reduction, scale 0–4,500,000*], [*Derived clauses*])
#v(2mm)
#for row in data.derived_clause_reductions {
  grid(columns: (9mm, 112mm, 41mm), gutter: 2mm,
    [#row.m],
    rect(width: row.derived_removed_root_implications / 4500000 * 112mm,
      height: 6mm, fill: rgb("2b8c7c"), stroke: none),
    [#row.derived_removed_root_implications])
  v(2mm)
}
#v(3mm)
One complete deterministic input census; exact integer counts, no sampling interval.
Run: native inventory v2, 2026-10-21 UTC. Source: `root-inventory.json`.
Full SAT construction and solver search are separate pending gates.
