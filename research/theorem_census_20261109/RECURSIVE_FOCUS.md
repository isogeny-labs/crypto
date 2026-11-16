# A surviving input's degree through the actual resultant

Lean proves an exact degree theorem for the actual fixed-size resultant
over every domain. Let f be a nonzero polynomial in an elimination variable,
whose coefficients are polynomials in a surviving old variable; assume its
elimination degree is at most the declared size D. If the four curve
coefficients are constant in the surviving variable, its degree d doubles
to2d in the resultant. The coefficient at2d is exactly the same size-D
resultant on f's leading coefficient in the surviving variable.
[pcc:unverified]

Applying this to the actual canonical recursive family proves the
third-newest input has degree2^(m−2)at every aritym≥5. Together with the
prior newest-pair theorem, this covers the three newest input degrees at
these arities. All four arity-four input degrees were already proved.
It does not prove every remaining older coordinate's degree.
[pcc:unverified]

## Actual variables and proof map

For f in S[old][E], resultantStep returns S[old][Z][T]. The actual
three-variable ring homomorphism rotate maps it to S[Z][T][old], so its
ordinary outer degree reads the surviving old variable's degree. It is a
composition of actual bivariate swaps and a polynomial map, not a degree
sequence supplied as a hypothesis. [pcc:unverified]

| Declaration | Obligation |
|:--|:--|
| rotate_scalar_polynomial / rotate_Z / rotate_T / rotate_scalar | Actual variable/constant transport |
| coeff_swap / old_coeff_degree_le | Swap coefficient indices; bound every coefficient's old degree |
| rotated_first_coeff / rotated_first_top | Actual lifted f coefficient and its old leading coefficient |
| rotated_second | Actual lifted S3 quadratic independent of the old variable |
| focusColumnBound_sum / rotated_column_degrees | D quadratic columns have old-degree0; two f columns have old-degree≤d; sum2d |
| old_resultant_degree_le / old_resultant_coeff_top | Determinant upper degree and exact coefficient at2d |
| old_leading_degree / old_leading_nonzero | Old leading polynomial still has elimination degree≤D and is nonzero |
| old_resultant_degree | Previous general resultant theorem makes top coefficient nonzero, so degree=2d |
| recursive_third_newest_degree | Instantiate with actual preceding canonical penultimate input |
| focus_degree_audit_certificate | Fully quantified generic theorem plus actual canonical corollary |

The Sylvester sizes D,2 are fixed throughout, including if the leading
polynomial has elimination degree smaller than D. No generic leading-term
assumption, irreducibility or unproved geometric recursion is used.
The point here is a formal polynomial identity/degree, not a claim that it
survives evaluation at selected finite-field coordinates. [pcc:unverified]

The canonical corollary at stage n+2 has arityn+5. The preceding actual
polynomial has elimination degree2^(n+2), is nonzero, and its penultimate
input has degree2^(n+2); the new sharp degree is2^(n+3). General transport of
arbitrary older inputs through successive coefficient-ring permutations is
still needed for a full every-input theorem. [pcc:unverified]

## Reproducibility and scope

Whole census gate passes357declarations, zero failures/zero trusted-native
exceptions. Full warning-bearing receipts are retained losslessly with
unpack/hash equality checks. Lean4.35.0/Mathlib56a00585 are pinned.
Source15 compiles with no warnings;
all reported declarations use only propext/Classical.choice/Quot.sound.
No custom axiom, hole, native decision exception or linter waiver. Fourteen
earlier attempts, including accepted intermediate components, are preserved
losslessly with exact source, diagnostics, exit status and unpack hashes.
Nested ring inference was repaired with explicit hom domains/codomains.
The earlier compatibility option is used only for the canonical nested-ring
instantiation, not to relax the kernel. [pcc:unverified]

Retained primary read scopes include Kosters–Yeo2015§2/Proposition2.1and
related Semaev notes. Reviewed Mathlib code includes Bivariate.swap,
Resultant.Basic and Matrix.Polynomial. Original Semaev2004remains unread
403. No novelty claim or source promotion. No native experiment, timing
or paid compute. Exact bindings/gate and producer-local packet status are
recorded in recursive-focus-sources.json and recursive-focus-verification.json;
signed standing remains distinct. [pcc:unverified]

Source commit `a55402b8` precedes packet `PCC-20261110-2c396d`; its full
producer-local check passes at03:01:24UTC on host762f5fc120c9. Signed
standing remains unverified. See [actual bindings](recursive-focus-verification.json),
[inspected two-page report](reports/proof-20261110-recursive-focus/report.pdf)
and [editable report](reports/proof-20261110-recursive-focus/report.md).
[pcc:unverified]
