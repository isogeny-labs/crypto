# Runtime chart construction: correctness gate

Source freeze 0ec950e6b995; checkpoint 20261116T001855Z. All three registered models executed once, sequentially, and exited zero. Raw COHORT.txt and run/case{0,1,2}.{stdout,stderr,status,txt} retain all cases. This is engineering evidence over F127 within one isomorphism class; no performance comparison was collected.

| Translation | Selected O | Tangent residual | Negation pairs | Graph overlaps | Base divisor |
|---|---|---|---:|---:|---|
| (0,0) | (0,4) | x²+34x+69 | 63 | 117 | irreducible quadratic |
| (1,1) | (0,36) | x²+70x+56 | 63 | 121 | split quadratic |
| (7,11) | (0,33) | x²+112x+48 | 64 | 123 | irreducible quadratic |

Each candidate discovers 129 rational points, regular origin, tangent residual, canonical line and control points from its runtime quartic. No archived anchor participates in candidate selection. The independent legacy comparator intentionally retains its archived chart. It is transported by each certified affine isomorphism.

Each model passes RR dimension eight, multiplication rank 18, diagonal rank 112/nullity eight, exact coefficient identities, corrupt-section rejection, 480 quotient determinant and 480 ordinary split tensor controls. Each also passes 120 base-divisor tensor comparisons against independent derivative limits. Cases zero and two exercise non-split cancellation in F127²; case one exercises split roots.

All supported scalar pairs agree under negation and overlapping graph charts. Irreducible-cubic, mixed and fully split graph divisors occur in each case. Each independently fitted basis change has coefficient rank 63 and matrix rank eight; twelve training and four withheld pairs pass, and corrupted coordinates/matrices are rejected. The unsupported scalar/chart cases remain printed in raw output.

## Mathematical construction and boundary

For a regular point O, the tangent cuts 2O+D, so K is linearly equivalent to 2O+D. Cubic forms vanishing on D represent H⁰(3K−D), isomorphic to H⁰(2K+2O), of dimension eight. Exact diagonal sections and centering are checked by coefficients and Jacobian arithmetic. This adapts the pullback construction in [Tian 2020, §4.2.3](https://arxiv.org/html/2007.03172); no new theorem or universal chart-existence result is claimed. Prior conditional Lean lemmas do not formalize this complete construction.

The three inputs are coordinate representations of one curve, not independent curves. The implementation remains fixed to F127 and a bounded chart search. The full cold DLP driver still uses its prior chart; integration, wider fields, scalable relation linear algebra, genus two/four, and the tenfold mathematical speedup remain outstanding. The previous failed cold performance comparison remains unchanged.
