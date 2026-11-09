# Primary-source audit and actual torsion witnesses

The new Lean modules establish concrete torsion boundaries in Mathlib's
actual elliptic point groups. The final local repository gate audits74
declarations,0failing,0trusted-native exceptions; the earlier41-declaration
receipt remains a dated snapshot. Counts include generated equation lemmas.
All new results remain unsigned producer assertions. [pcc:unverified]

## FST2010: a direct r=2 source discrepancy

On E/F7: y²=x³+x, Lean computes #E(F7)=8 and trace0, meeting the finite-field
supersingularity definition in FST §2. The curve satisfies Algorithm3.3(1):
7≡3 mod4 and −1 is a nonsquare. The actual point(0,0) has order2.
[pcc:unverified]

In the algebraic closure, Lean constructs α²=−1 and the actual nonsingular
point(α,0), killed by2 but outside the image of E(F7). Independently, it
proves for **any field extension** that the field adjoined by all square
roots of unity is the bottom intermediate field. Thus the actual extension
F7(μ2)/F7 has degree1. This uses `IntermediateField.adjoin` and
`Module.finrank`, not an assumed order-of-q identification.
[pcc:unverified]

The printed paragraph after Algorithm3.3 on p239 calls this embedding
degree2 for the subgroup of order2. Under the paper's own Definition2.1
that literal assertion is false: nonrational 2-torsion does not make the
cyclotomic field larger. The PDF page was visually checked, not inferred
only from extracted text. This is a boundary correction; it does not refute
the large-prime construction analysis. [pcc:unverified]

Declarations in `lean/SourceFSTBoundary.lean`:

- `roots2_adjoin_eq_bot`: general field statement.
- `roots2_extension_degree`: actual degree1 of F7(μ2).
- `two_torsion_not_all_rational`: actual geometric/rational point distinction.
- `fst_algorithm3_3_r_two_counterexample`: complete concrete witness.

`TorsionBoundaries.literal_supersingular_k_two_false` separately refutes the
universal r-prime reading of the corpus node using E/F5:y²=x³+1, #E=6,
trace0, and embedding degree1 for r=2. The source definition of
supersingularity is copied explicitly as gcd(trace,q)≠1; no equivalence to
geometric p-rank is asserted as a new theorem. [pcc:unverified]

## BK1998: preserve the exclusion

The original Theorem1 includes r not dividing q−1; its proof agrees with
the previously formalized triangular-matrix core. Theorem2's printed
probability factor and variable-prime sample space match the transcription.
The paper already warns that r² dividing #E does not remove the exclusion.
[pcc:unverified]

Our closed witness is E/F7:y²=x³+3x+2. Lean computes #E(F7)=9 and Δ≠0.
Let α be a root of X³+5X²+3X+2 in the algebraic closure; Lean proves this
cubic has no F7 root. A square-root lift gives an actual affine point P.
The repeated-S3 group theorem proves3P=O, and the x-coordinate proves P
is outside the rational-point image. Thus 3|7−1 and9|#E coexist with
nonrational3-torsion. [pcc:unverified]

Declaration: `TorsionBoundaryAudit.bk_excluded_boundary_counterexample`.
This closes an actual excluded-case witness, **not** a proof/refutation of
the qualified BK theorem. The full E[r] dimension, Frobenius determinant,
fixed-point/descent and Weil-pairing necessity bridges remain unformalized.
The requested all-p<500 census remains unexecuted. [pcc:unverified]

## Corrected prime-field arithmetic

The subsequent CompositeTorsion.lean removes the subgroup-primality
assumption: `corrected_integer_embedding_degree` applies to every r>2
dividing the actual point count, retaining Hasse explicitly. An actual
point P=(3,3)on the source F7curve has order8and modular embedding order2.
No full Hasse/field-degree/geometric-torsion bridge is thereby supplied.
See [COMPOSITE_TORSION.md](COMPOSITE_TORSION.md). [pcc:unverified]

The subsequent S4Degree.lean also proves canonical individual-variable
degree4for m4in every characteristic, with the remaining three inputs
formal, and coefficient4=S3²after specialization. The actual smooth
F5curve y²=x³−x gives S4(0,1,4,T)=1, refuting unrestricted degree preservation,
not the source's generic degree theorem. General all-mdegree and
irreducibility remain open. See [S4_DEGREE.md](S4_DEGREE.md) for primary
source read scopes and exact declaration bindings. [pcc:unverified]

The subsequent ResultantDegreeStep.lean proves an arbitraryDfixed-size
resultant leading-coefficient/degree theorem and actual canonical S5degree8
in its last input, with the other inputs independent. A separate Lean
counterexample refutes resetting the declared Sylvester size after
specialization. This is not a defined all-mfamily/every-input degree or
zero-sum recursion proof. See
[RESULTANT_DEGREE_STEP.md](RESULTANT_DEGREE_STEP.md). [pcc:unverified]

The subsequent RecursiveSemaevDegree.lean defines the actual canonical
family and proves its last-input degree2^(m−2)for every aritym≥3, its
successor leading-coefficient square and an exact canonical S4bridge.
The other input degrees/symmetry, geometric zero-sum equivalence beyondS4
and irreducibility remain separate open targets. See
[RECURSIVE_SEMAEV_DEGREE.md](RECURSIVE_SEMAEV_DEGREE.md). [pcc:unverified]

`corrected_prime_embedding_degree` derives k=2 for odd primes r>2 from the
source trace definition, primality of p≥5, actual group-order divisibility,
and the explicit Hasse premise t²≤4p. `hasse_trace_zero` supplies the missing
arithmetic t=0 step. Hasse's theorem itself remains a premise here.
This conditional theorem is not presented as a kernel proof of Hasse or of
the full prime-field classification. [pcc:unverified]

## Gaudry2000 and Hildebrand1994: source qualifications checked

Gaudry Proposition4 is a fixed-genus limit as q grows, for the whole
Jacobian. The algorithm samples a large prime subgroup (§2.3); equality of
smoothness proportions in that subgroup is a distinct unresolved claim.
The subsequent complete F3reduced-Mumford polynomial-pair model proves
8valid/7split representatives, fraction7/8and finite deviation3/8from1/2.
Geometric Picard identification, actual Cantor/subgroup arithmetic and a
native multi-field/scaling census remain open. See
../jacobian_smoothness_audit_20261109/README.md.
[pcc:unverified]

Hildebrand Theorem1 averages total-variation distance over distinct random
support points, with fixed support size≥2, fixed positive probabilities,
and sufficiently large prime modulus. Constants depend on the tolerance
and probabilities. Theorem2 treats worst supports on a different scale.
Neither statement establishes a fixed-hash adding-walk collision constant.
The subsequent walk study completed its bounded exact support/hash census
and scoped Lean counterexamples; the qualified random-support asymptotic
theorem's full proof remains open. See ../walk_audit_20261109/README.md.
[pcc:unverified]

Exact publisher URLs, PDF hashes and read ranges are in `sources.json`.
The later MMT2002§4.2–4.3read scopes and hashes are in the separate
Jacobian-smoothness study's sources.json, including the visually checked
heuristic beforeeq13and identity convention ineq12.
Initial archive/author endpoints failed, then publisher PDFs succeeded.
The KB service remains unavailable; no literature-novelty claim is made.
The full44-unread-node snapshot and source index are tracked separately
in `paper-audit-backlog.json`; no untouched paper is labelled audited.
[pcc:unverified]

## Reproduction

Run `lake build` and `lake env lean Refutations.lean` in `lean/`, then the
repository proof gate at `research/theorem_census_20261109/lean`. Current
receipts are under `evidence/recursive-semaev-degree-gate/`; earlier torsion
receipts remain in `evidence/proof-gate-torsion-fst/`. Failed incremental
builds are retained. This latest proof piece runs only its scoped scalar
identity supplement, without a new curve population, performance assertion,
paid cloud job or independent-host census.
[pcc:unverified]
