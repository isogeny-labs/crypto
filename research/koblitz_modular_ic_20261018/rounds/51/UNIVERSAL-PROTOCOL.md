# Universal certificate transport, frozen before data

Reload the archived R22 independent b,c QR circuit and its printed two-generator
triangular basis and full 3x2/2x3 certificates. Recompute the circuit only; do
not recompute its Groebner basis or search for new lift matrices. Require both
complete original matrix products and a changed-entry rejection control.
Verify the compact monic degree-16 lambda polynomial and the degree-14 lambda,
linear-nu equation with coefficient (1+c)^2 b^3(b+c(1+c)^3).

Compute a common polynomial denominator of every coefficient of both matrices
and the basis by native Singular gcd/lcm. Clear it and require every coefficient
integral. Reinterpret all entries in F2[n,l,b,c] and require the two full
polynomial products QR F = D GB and (D GB) B = D^2 QR. Export all actual arrays
and D. This proves equality only on D nonzero. Later specialization must check
that condition; no primality or marked-23 field claim follows from this run.

Manual existing local authorization, seed51, single Singular/FLINT thread,
4GiB virtual memory and 1200s. Strict exit0, no question-mark error lines,
UNIVERSAL51_ALL_CHECKS_PASS. Preserve any rejected attempt and all outputs.
