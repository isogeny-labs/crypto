# Preconditioned abscissa differences

All 48 inputs had a higher median full-solver speed than the delayed-reduction arm in both runs. All 6,912 timed solves verified and both integrated arms returned identical report tuples.

| Genus | Prior / candidate, run 1 | Prior / candidate, run 2 | Original / candidate, run 1 | Original / candidate, run 2 |
|---|---:|---:|---:|---:|
| 2 | 1.062–1.285 | 1.028–1.217 | 3.707–4.735 | 3.665–4.947 |
| 3 | 1.048–1.132 | 1.030–1.096 | 3.063–4.239 | 3.046–4.135 |
| 4 | 1.044–1.139 | 1.065–1.137 | 4.300–9.289 | 4.339–9.289 |

The scan reuses each abscissa difference for both signs and across successive anchors. Storing that difference in Montgomery representation replaces two reductions per ordinary rolling product with one. The table is built inside each timed solve and results include all other stages. No fixture reached the tenfold goal. Timing on the shared host includes compiler specialization effects. Run ruby summarize.rb to regenerate RATIOS.tsv.
