# Amendment 2: falsify a candidate density formula at q=8

14 November 2026, before the q=8 run. The q=2 and q=4 direct counts leave
respectively 7 and 189 nonzero trace-zero elements outside the union of the
two published W1 loci. These equal `(q-1)(q^3-1)` in both fields. This is a
candidate formula, not a theorem or a prediction of attack cost.

Run the native direct-product control at `q=8`, `K=GF(2^21)`, for both cubics.
Use the first ascending irreducible degree-21 binary polynomial. Count the
union and compare its complement within the nonzero trace-zero subspace to
`(q-1)(q^3-1)=3577`. This arm enumerates direct products to falsify the
candidate formula; it does not benchmark projective membership over the full
field or claim an `O(q^2)` measured end-to-end cost. Record source and binary
hashes, host, elapsed time, and raw stdout in a new write-once run directory.
Stop after ten minutes, a failed assertion, or memory exhaustion. Do not
replace either earlier receipt.
