# Canonical Semaev polynomials: all-arity geometric equivalence

For every algebraically closed field K, every general Weierstrass model W
with nonzero discriminant, every n≥0, and every tuple xs of n+3 field
elements, Lean proves that the actual canonical recursive polynomial
evaluates to zero exactly when actual Mathlib curve points with those
affine x-coordinates sum to zero. Stage n is arity m=n+3. This includes
every characteristic, repeated inputs and zero intermediate sums.
[pcc:unverified]

The family is `RecursiveSemaevDegreeAudit.recursivePolynomial`, defined
before this component using actual Sylvester resultants over independent
polynomial variables. `inputEval` and `evalCanonical` evaluate those
polynomials through explicit ring homomorphisms; they do not define a
replacement family by the desired geometric property. [pcc:unverified]

## The two recurrences

Writing S(xs) for canonical evaluation and M(xs) for the actual point-family
predicate, both have the same extension rule. [pcc:unverified]

$$S(xs,z,t)=0\iff
(\exists u,\ S(xs,u)=0\ \land\ S_3(z,t,u)=0)
\ \lor\ (S(xs)=0\ \land\ z=t).$$

The analogous rule replaces S=0 by M. The second alternative is necessary:
the old point sum can be zero and the last two points can be opposites.
The polynomial proof preserves the declared resultant sizes after
specialization. Its two declared leading coefficients are the square of
the preceding canonical evaluation and (z−t)². The general fixed-size root
criterion therefore gives exactly the same alternative as the group proof.
Strong induction uses the independently checked S3 and S4 bases.
[pcc:unverified]

## Exact negative controls

The nonsingular F5 curve y²=x³+1 has canonical S3(3,1,4)=0, but there is
no rational point with x=3: y² would equal3 in F5. Lean directly disproves
the rational-only all-arity universal converse on this actual curve.
This does not contradict the algebraic-closure theorem. [pcc:unverified]

A separate arity6 control takes four actual points P,−P,P,−P with x=4,
then prescribes the unliftable inputs3,3. The old sum is zero and the last
inputs are equal, but no six-point family exists over F5. Thus the
point-list recurrence also needs closure or appropriate liftability
hypotheses when inputs are unrestricted. It does not disprove a recurrence
whose hypotheses already supply rational point lifts. [pcc:unverified]

## Proof map

| Module and declaration | Exact obligation |
|:--|:--|
| SemaevPointLists.ManyPoints / sum_snoc / tuple helpers | Actual finite families of Mathlib points, prescribed affine coordinates, actual group sums |
| SemaevPointLists.two_point_recurrence | Nonzero prefix sum gives a finite intermediate point; zero prefix sum gives equal last inputs; converse glues using equal x implies equal or opposite points |
| SemaevPointLists.closure_free_recurrence_false | Direct negation using the actual arity6 F5 control |
| RecursiveSemaevEvaluation.resultantStep_pairEval | Evaluation commutes with the actual fixed-size Sylvester resultant |
| RecursiveSemaevEvaluation.inputEval_scalar / mapped_top_coefficient | Actual coefficient embeddings evaluate correctly; actual mapped top coefficient is the preceding evaluation squared |
| RecursiveSemaevEvaluation.canonical_two_point_recurrence | Exact finite-root OR both-leading-zero alternative, algebraic closure explicit |
| RecursiveSemaevEvaluation.evalCanonical_three / four | Actual family specializes to the previously proved S3/S4 bases |
| RecursiveSemaevGeometry.canonical_zero_iff | Strong induction proving the full actual-point equivalence |
| RecursiveSemaevGeometry.rational_only_allarity_false | Direct universal negation over the nonsingular F5 curve |
| RecursiveSemaevGeometry.geometric_domain_audit_certificate | Fully quantified positive theorem and nonsingular rational-only disproof in one binding |

All names above are under their corresponding namespace ending in Audit.
Exact statements, source hashes, toolchain and actual gate/packet receipts
are in `semaev-geometry-sources.json` and `semaev-geometry-verification.json`.
No hole, custom axiom, native-decision exception or linter waiver is used.
The standard foundations are propext, Classical.choice and Quot.sound.
The fresh lean4checker executable is unavailable; actual kernel builds
and the whole-project transitive axiom audit are used. [pcc:unverified]

## Sources and scope

Reviewed the existing actual GeneralS3/GeneralS4 coordinate and gluing
proofs; Mathlib Resultant.Basic's map, degree-padding and root-product
lemmas; IsAlgClosed splitting; Fin.snoc, Fin.init and finite group sums.
The available-tool discovery still exposes no research-KB/Conductor tool;
the bounded repository/Mathlib fallback is recorded. Original Semaev2004
remains unread after403, and retained Kosters–Yeo read scope is unchanged.
No novelty or source-status promotion. The existing identity supplement
`IDC1h0872017c01480c3d` remains limited to the scalar fixed-D=8 resultant
rewrite; the universal geometric statement rests on the Lean proof.
[pcc:unverified]

The earlier all-input formal degree theorem remains unchanged. This
component closes its outstanding all-arity geometric equivalence bridge.
Full polynomial permutation invariance and irreducibility are separate
open obligations. Frozen earlier reports retain their historical scopes.
There is no new native experiment, performance result or paid compute.
Leaderboard, benchmark and curve-database graphs have no new measured
value to update; the new proof-dependency diagram accompanies this result.
Signed PCC standing remains unverified until a trusted signed verdict.
[pcc:unverified]

Local source commit `2d252c2b` precedes packet `PCC-20261110-af2329`.
The full producer-local verification passes at05:12:28UTC on762f5fc120c9;
signed standing remains unverified. Whole census gate471passes with zero
failures/zero trusted-native exceptions. See [actual bindings](semaev-geometry-verification.json),
[inspected two-page report](reports/proof-20261110-semaev-geometry/report.pdf),
and [editable report](reports/proof-20261110-semaev-geometry/report.md).
[pcc:unverified]
