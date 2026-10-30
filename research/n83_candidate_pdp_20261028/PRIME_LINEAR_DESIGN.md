# Degree-five syzygy exclusion by covering projections

Implementation-stage contract, 2026-10-30. Production is not yet frozen or run.
Retain all 42 prime m3–9 original/baseline/spike40 planted/natural sources and
point-manifest identities. This is a lower-bound step within the full n83
three-family m2–9 FFD/LFD and elite-base goal, not a replacement objective.

## Sources and argument

Read Caminata–Gorla, https://arxiv.org/html/2112.05579, Definition 1.3 and
Section 4, before implementation. Highest-form nontrivial syzygy degree and
strict filtered falls are distinct. PRIME_FIRST_FALL_DESIGN.md explains the
quotient by the regular sequence of selector squares. PRIME_TERMINAL_DESIGN.md
and the complete-source validator fix the original chain and normalization.
No claim of a new literature theorem or scientific review is made.

Normalize every nonzero selector weight to one in the highest forms. Work in
A = Fp[z_0,...,z_(27m-1),U_0,...,U_(m-3)]/(z_i^2).
Let F_0,...,F_(m-2) be the quartic highest forms of the original S3 chain.
The complete degree-five map sends (L_0,...,L_(m-2)) to sum L_j F_j,
where each L_j is an arbitrary homogeneous linear form in ALL variables.

Set 22 selectors in each block to zero, retaining five and all U variables.
This is a graded homomorphism respecting the square ideal. Every projected
quartic is the same normalized five-selector template after renaming. If the
template degree-five map is injective, each projected L_j must be zero.
Use six subsets [0..4], [5..9], [10..14], [15..19], [20..24], [22..26]
in every block. Their union covers every selector; all U are always retained.
Thus the full original degree-five map is injective. These are coefficient
restrictions proving an implication for the original sources, not smaller
factor bases substituted for the requested dataset.

Injectivity at degree five also excludes constant relations at degree four:
multiply a hypothetical constant relation by a free U variable (m>=3).
Below four only selector squares occur. Their syzygies are Koszul because
they form a regular sequence. At degrees four and five, reducing a syzygy
modulo the square ideal leaves precisely the constant/linear F-coordinates.
If those vanish, the remaining square syzygy is Koszul. No mixed
square/quartic Koszul generator occurs before degree six. Characteristic-p
trivial syzygies occur above these degrees. Therefore successful injectivity
certifies FFD>=6, not equality. A rank-deficient projection proves NOTHING
about a syzygy in the original 27-selector system; retain it as inconclusive.

## Exact matrix and certificate

Rows are degree-five monomials with squarefree selector support and unrestricted
U exponents. Columns are (quartic index, multiplier variable), generator-major.
Expand (X-Y)^2 Z^2 - 2(X+Y)XYZ + X^2 Y^2 for each nonterminal relation;
the terminal highest form is U^2 X^2. Reduce selector squares only after
ordinary multiplication. Store every nonzero coefficient canonically mod p.
Sparse Gaussian elimination chooses rows in increasing monomial code order.
Retain original pivot rows and pivot values. A nonzero square minor covering
every column is a compact injectivity certificate. Fresh replay regenerates
the polynomial coefficient matrix and all pivots, not just its reported rank.

Production will bind all 42 full source checks to seven template certificates,
the invertible weights and the six covering subsets. Reused templates are
explicitly shared evidence, not 42 independent rank computations. All failures,
rank-deficient templates, controls, logs and partial results must survive in
Git/S3. Neither LFD nor elite-base improvement follows from this test.

## Development controls and production gate

Small-prime development tests compare sparse elimination with an independent
dense implementation. Three-selector terminal annihilators must be detected;
five-selector controls must not silently treat a kernel as injective. Check
selector-square reduction, unrestricted U powers, altered coefficients,
missing/duplicated columns and invalid coverage. Complete-source donor retains
the previous full byte and planted-witness checks. Development results are
not n83 production measurements.

Before production: freeze the native binary, full 42 input identities, exact
limits and an experiment.protocol/v1 record; upload source and verify recovery;
use the EKS default or a specifically recorded existing-host fallback. Do not
change a frozen earlier protocol. Publish raw matrices/minors, file structure,
fresh and restored replays, export and inspected run report.
