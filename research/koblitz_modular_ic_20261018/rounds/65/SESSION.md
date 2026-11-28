# R65 session

## Objective

Compute the complete first descending Frobenius step from the trace dual
of the rank-32 affine level-77 source order.

## Protocol and inputs

The frozen version-1 protocol binds the R61 trace Gram and inverse, R52
full product table, R53 discriminant, and R64 field bridge. Version 2
binds the accepted first-run payloads before reconstructing the lattice.
Both runs use the local 1200-second/4-GiB single-math-job gate and the
strict native acceptance rule in `PROTOCOL.md`.

## Results and failures

`first-frobenius65-01`: accepted; quotient 160, binary remainder rank 80,
survivors 80. `lattice65-01`: accepted; rank-32 lattice with quotient
length 80, all 32 basis squares checked. No rejected run occurred.
Altered-kernel and altered-inverse negative controls were rejected by
their respective accepted runs. See `results.json` for exact lines,
durations, source revisions, and byte pins.

## Next action

Freeze and execute the second complete square-preimage step using the
saved first-lattice basis. Continue until equality, then compute branches
and divisors for the proper level-77 map search.
