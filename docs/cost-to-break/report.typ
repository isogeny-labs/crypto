// Cost-to-break table v1: report. Lays out table-v1.json and chart-v1.svg;
// it computes nothing. Build: typst compile report.typ report.pdf
#let t = json("table-v1.json")
#let inp = t.inputs
#set page(paper: "a4", flipped: true, margin: (x: 1.5cm, y: 1.4cm),
  footer: context [#set text(8pt, fill: luma(110)); Cost-to-break table v#t.table_version, #t.as_of · derived table, not a benchmark · page #counter(page).display()])
#set text(font: ("DejaVu Sans", "Liberation Sans"), size: 9.5pt)
#set par(justify: false)

= Cost to break, table v#t.table_version (#t.as_of)

#text(fill: luma(70))[Action plan 2026-11-21, row R2. Drafted for the Principal Cryptographer's review. #t.label]

#image("chart-v1.svg", width: 92%)

#pagebreak()
== The table

#{
  set text(size: 8pt)
  table(
    columns: (auto, 1.5fr, 1.7fr, 1.3fr, 1fr, auto, auto, auto),
    stroke: (x: none, y: 0.4pt + luma(200)),
    inset: 4pt,
    fill: (x, y) => if y == 0 { luma(235) },
    ..t.display_header.map(h => strong(h)),
    ..t.rows.map(r => r.display_cells.map(c => c.replace("`", ""))).flatten(),
  )
}

== Checks recomputed by the generator

#for c in t.checks [
  - *#c.id*: #c.what. #if "difference_bits" in c [Derived 2^#c.derived_log2 against published 2^#c.published_log2.] #if "ratio_gpu_hours" in c [Ratios derived/repository: #c.ratio_gpu_hours (GPU-hours), #c.ratio_usd_spot (spot dollars), #c.ratio_usd_on_demand (on-demand dollars).] #if "ratio_predicted_to_reported" in c [Predicted/reported ratio #(c.ratio_predicted_to_reported)\; #(c.caveat).]
]

== Method and assumptions

*Elliptic curves.* Expected parallel-rho work $E = c sqrt(pi n \/ (2A))$ group operations, with $n$ the prime group order, $A$ the automorphisms the walk folds by (2 for the negation map, 6 for secp256k1, $2 dot 131$ for ECC2K-130) and $c$ the walk constant (1.0 idealised; 1.069 for ECC2K-130). Time is $E$ divided by a rho rate measured in this repository on an RTX PRO 6000 Blackwell Server Edition. No rate is carried across field types; Curve25519 has no measured rate and uses the P-256 rate as a labelled proxy.

#{
  set text(size: 8.5pt)
  table(
    columns: (auto, auto, 1fr),
    stroke: (x: none, y: 0.4pt + luma(200)),
    inset: 4pt,
    strong[Input], strong[Rate], strong[Where it was measured, and conditions],
    ..inp.rates.pairs().map(((k, r)) => (raw(k), str(calc.round(r.value / 1e9, digits: 3)) + " G/s", [#r.where. #r.conditions])).flatten(),
  )
}

*Price.* Low \$#(inp.prices.rtx6000_aws_g7e.low_usd_per_gpu_hour) per GPU-hour (#inp.prices.rtx6000_aws_g7e.low_basis); high \$#(inp.prices.rtx6000_aws_g7e.high_usd_per_gpu_hour) (#inp.prices.rtx6000_aws_g7e.high_basis). Source: #inp.prices.rtx6000_aws_g7e.source. RSA rows use the RSA-260 record's own \$#(inp.prices.rsa260_anchor.low_usd_per_gpu_hour) per GPU-hour.

*RSA.* #(inp.gnfs.heuristic)\; scaled from #(inp.gnfs.anchors.rsa260.source). Order of magnitude only.

*ML-KEM.* Cited from the CRYSTALS-Kyber specification v3.02, Table 4 and §5.2 (ML-KEM keeps these parameter sets); not converted to GPU time. The lab's `mlwe margins` estimator is excluded until its outside review (action plan R5).

*Quantum.* RSA and all the elliptic curves fall to Shor's algorithm on a large fault-tolerant quantum computer, which does not exist today; no quantum algorithm is known that breaks ML-KEM at these parameters beyond the quantum core-SVP figure. Cost columns are classical.

== Limits

- Prime-field rates are single sweeps on rented GPUs without a recorded isolation level, not ecbench sessions with replay certificates; turning them into those is follow-up work that needs the owners' go-ahead.
- 256-bit curve rows show the size of the gap (about $10^21$ GPU-years), not a precise figure.
- RSA rows rest on one self-reported record and a heuristic with its $o(1)$ dropped.
- Prices are list prices on one date and one provider, not billing receipts.
- Nothing here is a pcc-verified claim; every figure is an unverified assertion until reviewed.

== Sources

#{
  set text(size: 8pt)
  for r in inp.rows [
    - *#r.name*: #if "group_order_source" in r [order: #(r.group_order_source)\; ] #if "nist_source" in r [#(r.nist_source)\; ] #if "lattice_source" in r [#(r.lattice_source)\; ] quantum: #(r.quantum_source).
  ]
  [- *GNFS checks*: #(inp.gnfs.checks.rsa896.source)\; #(inp.gnfs.checks.rsa250_cpu.source).]
}
