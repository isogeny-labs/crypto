# R71: complete rational-fiber residue algebras

The accepted `rational-fibers71-01` specializes **all 32 square columns**
of the retained rank-32 normal order at both omitted rational primes.
The complete 32-dimensional fiber at `z=0` has stable Frobenius image
of dimension 4 after five iterations; its reduced algebra is `F4 x F4`.
At `z=1`, the image has dimension 8 after four iterations and is
`F4^4`. The program obtains those profiles from fixed dimensions of
all 32 powers of Frobenius, integer totient/divisor inversion, and a
reconstruction of every fixed dimension from the resulting residue
degrees. It checks the source unit in both fibers and the stable-image
invariance. The full matrices, fixed dimensions and counts are saved.

These are **two** branches of residue degree two above `z=0` and
**four** branches of residue degree two above `z=1`. The 32-dimensional
fiber lengths include nilpotents, so the reduced dimensions 4 and 8
do not determine the separate ramification indices. The next exact
calculation must recover primitive idempotents in the full fiber
algebras, measure the dimensions of their local factors, and then
compute actual local uniformizers and valuations. The degree-five
`f_C` fiber and the infinity chart remain additional complete cohorts.

One native attempt passed exit 0, the exact terminal marker, and the
zero-error gate. Six executed source/protocol/driver files match their
frozen Git revision and five donor hashes match. This is an exact
finite-algebra result; the four-second UTC interval is a receipt, not
a controlled performance comparison.
