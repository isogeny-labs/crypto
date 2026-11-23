# R67 session

## Objective

Compute the complete third descending Frobenius square-preimage lattice.

## Protocol and inputs

The frozen source binds both accepted R66 second-lattice bases, its
custody receipts, and every R52 source square column. One fresh local
run uses the 1200-second/4-GiB single-math-job gate and strict native
acceptance.

## Results and failures

One accepted terminal attempt, no rejected attempt. Quotient 32,
binary rank 24, survivors 8, and rank-32 third lattice with quotient
length 24. All 32 basis squares, inclusions, inverses, and altered-input
controls pass. See `results.json` for the exact line and byte pins.

## Next action

Freeze and execute the complete eight-class square test, then compute
proper branches and pole divisors after lattice stabilization.
