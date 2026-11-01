#set page(width: 190mm, height: auto, margin: 8mm)
#set text(font: "New Computer Modern", size: 9pt)
#let data = json("construction.json")
#assert(data.status == "PASS" and data.search_performed == false)
#let names = (per_representative: "Original", shared_bits: "Shared bits")
#text(size: 15pt, weight: "bold")[D18: actual SAT construction counts]
#v(2mm)
N83 · 16,627 fixed representatives · both encodings constructed for m=2,3,4
#v(2mm)
#data.shared_bits.len() common root bits; accepted domain and choice wires unchanged.
#v(4mm)
#grid(columns: (9mm, 28mm, 94mm, 31mm), column-gutter: 2mm,
  [*m*], [*Encoding*], [*Clauses: scale 0–6,000,000*], [*Actual clauses*])
#v(2mm)
#for r in data.rows {
  grid(columns: (9mm, 28mm, 94mm, 31mm), column-gutter: 2mm,
    [#r.m], [#names.at(r.root_encoding)],
    rect(width: r.total_clauses / 6000000 * 94mm, height: 5mm,
      fill: rgb(if r.root_encoding == "shared_bits" { "2b8c7c" } else { "526c8c" }), stroke: none),
    [#r.total_clauses])
  v(2mm)
}
#v(3mm)
#table(columns: (1fr, 2fr, 2fr), inset: 4pt,
  [*m*], [*Variables, both encodings*], [*XOR rows, both encodings*],
  ..data.rows.filter(r => r.root_encoding == "shared_bits").map(r => (
    [#r.m], [#r.total_variables], [#r.total_xor_rows]
  )).flatten())
#v(3mm)
One deterministic construction per cell. Exact integer counts; no sampling interval.
Source: `construction.json`, native source digests recorded in that receipt.
No solver search or runtime comparison is included in this construction check.
