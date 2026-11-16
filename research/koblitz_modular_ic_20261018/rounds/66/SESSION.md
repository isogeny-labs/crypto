# R66 session

## Objective

Compute the complete second descending Frobenius square-preimage lattice.

## Protocol and inputs

The frozen source binds both accepted R65 first-lattice bases, its custody
receipts, and all R52 source-square columns. The one fresh local run uses
the 1200-second/4-GiB single-math-job gate and strict native acceptance.

## Results and failures

One accepted terminal attempt, no rejected attempt. Quotient 80,
binary rank 48, survivors 32, and rank-32 second lattice with quotient
length 48. All 32 basis squares, inclusions, inverses, and altered-input
controls pass. See `results.json` for exact line and historical byte pins.

## Next action

Freeze the complete third 32-class square test, then compute proper
normalization branches and pole divisors after lattice stabilization.
