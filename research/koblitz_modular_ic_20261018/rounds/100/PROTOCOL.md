# R100: factor the actual R99 norm numerator Frobenius root

Bind to R99's exact `norm99.sing` by SHA-256. Parse every term and check
that the degree-1968, 73-term numerator has all exponents divisible by 16.
Construct the unique degree-123 root and verify in Singular over F2[x]
that its sixteenth power is the actual numerator. Factor the root, check
every factor and exponent, the full product, total degree 123, and a
changed-product rejection. Export the exact factors. Freshly replay in a
separate Singular process and compare exported bytes.

One local pinned Singular process, seed100, one thread, 2 GiB virtual
memory, 120 seconds. Freeze this source and protocol in Git before the
run; retain any failure. No statistical or timing comparison. R99's raw
archive and exact norm output are committed source provenance. The default
EKS image lacks the pinned CAS, so use the existing local-CAS exception.

This result determines the finite zero support of the displayed lambda
monogenic discriminant norm on `D(x)`. It does not give normalized branch
indices. Sources: R51 donor; R97--R99 notes and exact outputs;
[Singular manual](https://www.singular.uni-kl.de/index.php/singular.pdf)
for `factorize`; and [Stacks 53.2](https://stacks.math.columbia.edu/tag/0BXX)
for normal projective models.
