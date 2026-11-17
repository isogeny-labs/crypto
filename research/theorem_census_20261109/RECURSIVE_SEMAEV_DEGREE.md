# Canonical recursive polynomials: every arity, last input

Lean defines the actual canonical resultant family and proves its last-input
degree is 2^(m−2) for every arity m≥3, over every field. Each coordinate
remains an independent polynomial variable. The successor leading coefficient
is the square of the preceding polynomial, and an explicit equality identifies
the arity-four member with the previously checked canonical S4.
[pcc:unverified]

## Verification

The whole tracked census gate passes 308 declarations, 0 failing and
0 trusted-native exceptions, including the explicit combined binding.
Packet PCC-20261109-3bc113 passes its full local check at18:58:23Z, with
source commit196d2fac preceding issue. Standing remains unsigned.
See `recursive-semaev-degree-verification.json` for exact hashes.
[pcc:unverified]

Supplement IDC1h0872017c01480c3d accepts 32 derived and 11 extra points only
for the fixed D=8 scalar monic-square resultant rewrite: 10 variables,
degree bound18, actual ten-by-ten determinant AST. The proof-file pointer
matches. It supplies no all-arity or other-characteristic inference; those
mathematical conclusions rest on Lean. See the
[report](reports/proof-20261109-recursive-semaev-degree/report.pdf).
[pcc:unverified]

Let R0=K[x1][x2] and R(n+1)=R(n)[Z]. Stage n is a polynomial in one more
independent input T, with coefficient ring R(n), so its arity is n+3.
The base is the actual invariant-form S3 quadratic. Each successor is the
actual fixed-size Sylvester resultant of the preceding polynomial with S3
in the elimination variable and two fresh coordinates. Its declared first
size is 2^(n+1), and its second size is2. [pcc:unverified]

The checked induction establishes the following formulas for the defined
polynomials, without assuming a numerical degree recurrence. [pcc:unverified]

$$\deg_T P_n=2^{n+1},\qquad
[T^{2^{n+2}}]P_{n+1}=P_n(Z)^2.$$

The base leading coefficient is (x1−x2)²≠0. Every coefficient ring is a
domain, so the already-checked general resultant theorem gives the next
degree and keeps the polynomial nonzero. Lean proves the integer exponent
identity converting stage n to arity m=n+3. [pcc:unverified]

## Source and exact scope

`lean/RecursiveSemaevDegree.lean` contains the coefficient-ring construction,
constant coefficient embeddings, actual recursive polynomial definition,
S3 evaluation formula, canonical S4 bridge, leading-coefficient formula and
all-arity last-input degree theorem. `CommRingCat` keeps each ring's carrier
and actual operations together during the recursive type construction.
No finite-field interpolation is involved. [pcc:unverified]

The statement concerns one distinguished input of the canonical family.
It does not establish symmetry, the remaining input degrees, irreducibility,
or the geometric zero-sum equivalence beyond S4. Those are still separate
obligations in the original audit. [pcc:unverified]

Generic degree also cannot be applied unchanged after specializing the
other coordinates. The earlier actual smooth F5 example S4(0,1,4,T)=1
has degree0. The preceding general-step counterexample likewise proves
that resetting a declared Sylvester size after specialization changes
the operator. Neither counterexample refutes the qualified generic degree
theorem. See [S4 degree](S4_DEGREE.md) and
[general step](RESULTANT_DEGREE_STEP.md). [pcc:unverified]

Primary read scopes remain Semaev2015§2/pp2–3 and Kosters–Yeo2015§2/p3,
with exact hashes in `recursive-semaev-degree-sources.json`. Original
Semaev2004 remains unread after403/404responses; related sections do not
replace that source or establish a full-paper audit. The original fourteen
requirements and untouched source backlog remain open at their documented
scopes. No new native curve population, statistical study, benchmark,
frozen-data change, paid compute or separate-host run occurs here.
[pcc:unverified]
