# Quotient partition amendment

Frozen before the second cohort run. The first exploratory partition used
the Legendre class of `K0*K1`, a scale-invariant quotient function. Its raw
3,780 transition rows are retained under `results/legendre_diagnostic/`.
It chose branch 1 in 3,778 rows and branch 0 in two. Cross-product Legendre
classes appear constant per model, so this partition is unsuitable for a
collector. This is a measured diagnostic, not a corrected result.

The next run uses the least significant bit of the canonical field residue
`K0/K2`. If `K2=0`, it uses the first nonzero denominator among `K1,K3`;
if all are zero, branch 0. This is invariant to projective scaling but costs
one field inversion per transition. Preserve every branch and failed row.
