# Continuous integer cocycles refute the broad evaluation formula

Lean disproves the evaluation-surjectivity assertion required by the
broad wording of Bruin2011, equation2.1, for arbitrary discrete Galois
modules. The example uses the **actual absolute Galois group of F2 with
its Krull topology**, the actual square Frobenius, and the discrete
integers with trivial continuous action. Every continuous one-cocycle is
zero, while the actual coinvariant quotient contains the nonzero integer
class1. Evaluation therefore cannot be surjective or induce the stated
isomorphism on any quotient of cocycles by cohomology classes.
[pcc:unverified]

The finite-module hypothesis in the following Lemma2.1 is retained. This
example concerns the earlier broad evaluation formula, outside the finite
or torsion scope; it does not refute the finite-module pairing application
or the qualified Tate/Frey–Rück theorem. The preceding
[finite character-duality packet](../finite_tate_duality_audit_20261114/README.md)
remains separate and unchanged. [pcc:unverified]

The argument is formalized with actual objects:

- `ActualGalois` is Mathlib's `Field.absoluteGaloisGroup (ZMod 2)`.
  Krull compactness is inferred from the actual algebraic-closure extension.
- `actualFrobenius` is the actual perfect-field square Frobenius as an
  F2-algebra automorphism. Lean proves its value on every element is x².
- `trivialIntegerAction` consists of identity additive equivalences;
  identity/composition laws and joint continuity are proved.
- Every continuous integer-valued function satisfying the standard
  one-cocycle identity for this action is proved zero. Bundling these
  cocycles as continuous homomorphisms loses no potential counterexample.
- `IntegerCoinvariants` is the actual quotient by the image of σ−1 for
  this trivial action. Its nonidentity `missingIntegerClass` represents
  the integer1.
- `evaluateCocycle` is the actual source evaluation formula on continuous
  cocycles and this quotient. Lean proves non-surjectivity at every actual
  Galois element, and specifically at the square Frobenius. [pcc:unverified]

All statements retain unsigned standing. [pcc:unverified]

Compactness supplies the key obstruction. A continuous homomorphism from
a compact group has finite image in a discrete group. Every element of
that finite image has finite order. A torsion-free target forces each
image element to be the identity, so every continuous integer cocycle for
the trivial action vanishes. No claimed H1 value or source isomorphism is
assumed. [pcc:unverified]

The integer class1 remains nonzero because an element in the image of
identity minus identity has the form z−z. Lean also proves there is **no**
type H, surjective representative map from these cocycles to H, and
bijection H→coinvariants whose values agree with the stated evaluation
formula. Thus taking a class quotient cannot repair non-surjectivity.
This is a direct necessary-property disproof; no homemade H1 object is
substituted for canonical continuous cohomology. [pcc:unverified]

The code writes the additive integer group as `Multiplicative ℤ` where
needed for the existing quotient API. In that notation the group identity
`1` means the additive integer0; `missingIntegerClass` is represented by
`Multiplicative.ofAdd (1:ℤ)`, not by the group identity.
[pcc:unverified]

Two controls delimit the proof. Lean proves the integer1 has no nonzero
annihilator. It also exhibits the identity continuous homomorphism on the
discrete integer group with nonidentity value at1. Continuity alone does
not imply vanishing when compactness is removed. [pcc:unverified]

Primary review: Peter Bruin, [*The Tate pairing for Abelian varieties over
finite fields*](https://www.numdam.org/article/JTNB_2011__23_2_323_0.pdf),
JTNB23(2011),323–328, DOI10.5802/jtnb.764. The preceding packet read all
six article pages and inspected all seven PDF pages. The current audit
rechecked printed325/PDF4: the action/module paragraph and equation2.1
precede the finite-module restriction in Lemma2.1. The linked
[author preprint](https://pub.math.leidenuniv.nl/~bruinpj/tate-pairing.pdf)
also states the broad formula on its page3, equation1. Only that selected
preprint context is marked read. Author/publication links and bounded
qualification/correction searches were checked; no novelty or global
errata-absence claim is made. No source author's cited Serre theorem is
marked read or refuted here. [pcc:unverified]

Mathlib's continuous-cohomology complex and its algebraic low-degree
cohomology APIs were inspected and kept distinct. This certificate checks
all continuous one-cocycles, the exact Frobenius evaluation rule and its
incompatibility with any class-quotient bijection. It does not claim a new
formal construction of canonical continuous H1, nor prove the corrected
finite/torsion cohomology identification, Lang vanishing, Cartier duality
or canonical divisor comparison. OriginalFR1994 remains unread after the
recorded access failures. The other original fourteen-item gaps continue.
[pcc:unverified]

Main declaration:
`ContinuousCocycleAudit.continuous_cocycle_evaluation_certificate`, in
[CocycleEvaluationCertificate.lean](lean/CocycleEvaluationCertificate.lean).
See [proof map](proof-map.json), [source plan](sources.json),
[source review](evidence/source-review.json) and [visual report](reports/report.pdf).
Exact local **PCC-20261114-2fd770** replay PASS at 2026-11-14T09:15:14Z; signed standing remains unverified. See [verification receipt](verification.json).
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/continuous_cocycle_audit_20261114/lean
python3 research/continuous_cocycle_audit_20261114/restore_attempts.py /tmp/continuous-cocycle-builds
```

Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 pinned.
All failed source/configuration/log/exit states retained. Standard
foundations/default limits; no holes, custom axioms, native evaluation,
raised limits, new global elaboration flags or linter waivers. KB and
Conductor unavailable, explicit primary/prior/official fallback before
approach/code. No corpus promotion, full14 closure, native experiment,
paid compute or agent upload. [pcc:unverified]
