# Native analysis receipt

The frozen `d=7` screen completed all 24 planned D/R/P rows with zero
censoring or exclusions. All 12 planted arm rows evaluated exactly and
group-verified. Two consecutive native analyzer executions were byte-identical
for JSON, CSV, SVG, and export outputs.

## Primary result

- P/D paired geometric-mean `field_ops`: `2371.7222842982674`.
- P/D descriptive paired 95% interval:
  `[2371.7202856805307, 2371.723833659716]`.
- P/R paired geometric mean: `2.0732899318008795`; interval
  `[2.0732872203343446, 2.0732926432709604]`.
- R/D paired geometric mean: `1143.9414468376672`.
- P maximum productive degree at most 4: 4/4 random targets.
- Maximum per-pair P/D peak-RSS ratio: `14.451239669421488`.

P fails the frozen P/D work and memory gates. The decision is
`stop_before_d9_d19_and_m4`; those stages were not opened.

## Controls

- Planted failures: 0/12 qualification rows.
- Exact target pairing: pass.
- Self-test and every retained quadratic identity: pass.
- D/predecessor reproduction mismatches: 0/8.
- D/R/P random censoring: 0/0/0.
- P productive degree at most 4: 4/4.

The pair trace is algebraically exact, and quadratization lowers maximum
productive degree from D's 5 to 4. It does not compensate for the expanded
state: R has 18 variables versus D's 15 and already costs 1,143.94 times D;
P retains the same `d=7` variable count and costs another 2.0733 times R.

This finite small-curve solver result does not include relation probability,
relation collection, filtering, sparse linear algebra, individual logarithms,
or target recovery and is not evidence of total `r^o(1)` work.
