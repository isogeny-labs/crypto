# Native analysis receipt

The frozen 60-row panel and 12-row independent reproduction completed with
zero censoring or exclusions. Two consecutive native analyzer executions were
byte-identical for JSON, CSV, SVG, and export outputs.

## Primary result

- F/D paired geometric-mean `field_ops`: `1.293439072522688`.
- F/D descriptive paired 95% interval:
  `[1.2934369446978975, 1.2934413624698864]`.
- F/T paired geometric mean: `0.9730660832231747`; interval
  `[0.9730637571481997, 0.9730682962946151]`.
- F maximum productive degree at most 4: 8/8 random targets.
- Maximum per-pair F/D peak RSS: `2.376556667164402`.
- Maximum per-pair F/D membership-build operations:
  `0.9863561514937662`; permutation overhead versus T is exactly zero.

F fails the frozen F/D work, F/T work, and memory gates. The decision is
`stop_before_scaling_and_m4`.

## Controls

- Planted failures: 0/20 qualification rows.
- Exact target pairing: pass.
- Inverse-permutation ideal and assignment identity: pass.
- Quotient-layout input-count or verdict mismatches: 0/48.
- T/canonical-B reproduction mismatches: 0/12.
- D/T/F random censoring: 0/0/0.

Local interleaving I was the best quotient layout at `1.1864764325` times D,
but it was a diagnostic arm and remains slower and substantially larger than
D. The frozen protocol forbids promoting it post hoc.

This finite small-curve solver result does not include relation probability,
relation collection, filtering, sparse linear algebra, individual logarithms,
or target recovery and is not evidence of total `r^o(1)` work.
