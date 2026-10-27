# Round18 proof scope

| module | named results | kernel-checked scope |
|---|---:|---|
|KoblitzRound18|11|characteristic-2 square/symmetric-character uniqueness; Frobenius normalization; quartic and square/cube depth implications; commuting periods; elliptic-map identities|
|KoblitzRound18Tate|3|universal integer coordinate residual; characteristic-2 consequence; normalized Tate-form implication with explicit coefficient hypotheses|
|KoblitzRound18Companion|12|four scalar Q25 separators; actual Q24 units and quartic gates|
|KoblitzRound18Boundary|12|complete Q26 finite models, prior constraints, cubic zeros and nonzero status|
|KoblitzRound18Squares|24|twenty scalar square/cube separators and four actual Q26 units|
|KoblitzRound18FinalBoundary|12|complete Q28 finite models, all prior constraints, five cubic/five square zeros and nonzero status|
|KoblitzRound18Theta|8|actual character and inversion-discrepancy exponent congruences|
|KoblitzRound18Frobenius|2|actual modulo81 row obstruction; no matrix lift through B|
|KoblitzRound18Refinement|4|actual integer BC=CB=105I and rotation; actual two torsion matrices of exact order168|
|KoblitzRound18Hecke|3|all11-residue inverse characters, actual common rational system, no scalar separation of equal systems|
|KoblitzRound18MapCounterexamples|3|102 exact nonzero polynomial-basis counterexample residues, total and per-sign counts|

Total:94 named results. Generated equation lemmas are additional entries in
the full transitive audit. Only propext, Classical.choice and Quot.sound are
permitted; no sorry, custom axiom or trusted native result is accepted.
The build and audit receipts are in runs.tar.xz; VALIDATION.md records the
completed audit count.

The counterexample checker uses binary polynomial convolution and long
division, independent of the native field multiplication algorithm. Its
finite theorem verifies those actual residue calculations. It does not
formalize field irreducibility, all source point constructions or the
33,554,432-case native search exhaustion.

The symmetry lemmas concern actual scalar/character implications. Matching
those scalars to the canonical mixed pullback action uses the written
symmetric line-bundle class and divisor argument in THETA.md. Full abelian,
Miller, modular and scheme geometry is not encoded by axioms or disguised
as a finite decision problem.

The integral no-lift theorem is over ZMod81 and is a genuine exact obstruction
to any integral rotation lift of this B. Identifying every geometric group
endomorphism with an integral CM matrix is a written maximal-order bridge.
The scalar refinement matrices are proved in Lean; native point controls
verify their CM action. The full8-dimensional Frobenius order840 is native;
its small order168 matrices and the commuting-period implication are formal.

Source normalization and restriction-of-scalars geometry remain written/native
bridges. The literal finite boundary certificates prove their stated complete
constraint checks, not whole saturated ideal equality. Three new source-bound
IDC records cover the square/cube residual, Tate-coordinate residual and
cleared elliptic-map identity. Their ids are added after issuance.

Completed transitive project audit:651 theorems,0 failing,0 trusted native results.

| identity | source-bound certificate |
|---|---|
|Tate-coordinate residual|[IDC1h0c6e29317a135107](identity-Tate-diamond-coordinate.json)|
|Cleared elliptic-map identity|[IDC1hee24ebe64bfee106](identity-elliptic-map-cleared.json)|
|Square/cube source residual|[IDC1hb0f2e139dc231b5a](identity-square-cube-source.json)|
