# Native analysis receipt

The frozen 72-row panel completed with zero censoring or exclusions. Two
consecutive native analyzer executions were byte-identical for
`analysis.json`, `paired.csv`, `figure.svg`, and `experiment.export.json`.

## Primary result

- B/D paired geometric-mean `field_ops`: `1.3292420765657798`.
- Descriptive paired 95% bootstrap interval, 10,000 resamples, frozen seed
  `20261125`: `[1.3292409734930948, 1.3292431924473607]`.
- B maximum productive degree at most 4: 8/8 random pairs.
- Maximum per-pair B/D peak RSS: `2.2876417616866953`.
- Maximum per-pair B/D membership-build operations:
  `0.9863561514937662`.

B fails the frozen work-ratio and memory gates independently. The decision is
`stop_before_scaling_and_m4`.

## Controls

- Planted failures: 0/24 rows.
- Exact paired targets: pass.
- D predecessor reproduction mismatches: 0/12.
- A predecessor reproduction mismatches: 0/12.
- U negative-control degree drops below D: 0/8.
- D and B random censoring: 0/8 each.

The U arm's `0.9761405089` work ratio is diagnostic only. The frozen protocol
forbids promoting a favorable control post hoc; it also left productive
degree at 5. Any study of auxiliary-variable ordering or non-beneficial cuts
requires a separate frozen protocol and fresh validation targets.

## Claim boundary

This is a finite small-curve solver-stage result labeled
`[pcc:unverified]`. It is not an end-to-end relation-collection measurement
and not evidence of a classical subexponential Curve25519 ECDLP algorithm.
