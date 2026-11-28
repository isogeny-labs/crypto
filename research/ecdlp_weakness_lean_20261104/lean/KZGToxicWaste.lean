import Mathlib

/-!
# KZG toxic-waste boundary

This file studies the scalar-coordinate surrogate

`c = q * (tau - z) + y`.

If the setup trapdoor `tau` is known and `z != tau`, this equation can be
solved for an accepting proof for every claimed value `y`.  At the exceptional
point `z = tau`, the proof term disappears and verification is equivalent to
`c = y`; knowing `tau` does not make an arbitrary wrong value acceptable at
that point.

The final section records the same constructor in an additive module.  These
are algebraic identities intended to isolate the linear core underlying a KZG
opening check.  This file proves no equivalence to a pairing verifier; such a
bridge needs explicit cyclic-subgroup and pairing hypotheses.  It also does
not model a polynomial degree bound, a setup ceremony, trapdoor recovery, or a
probabilistic polynomial-time security game, and therefore does not assert a
break of KZG when the trapdoor remains hidden.
-/

namespace KZGToxicWaste

section Scalar

variable {F : Type*} [Field F]

/-- Scalar-coordinate surrogate for the KZG verification equation. -/
def scalarVerify (tau z commitment value proof : F) : Prop :=
  commitment = proof * (tau - z) + value

/-- With the trapdoor, solve the verification equation for the proof scalar. -/
def forgeProof (tau z commitment value : F) : F :=
  (commitment - value) / (tau - z)

/-- Away from the exceptional point, trapdoor knowledge constructs an
accepting proof for any chosen claimed value. -/
theorem forgeProof_accepts
    (tau z commitment value : F) (hz : tau ≠ z) :
    scalarVerify tau z commitment value
      (forgeProof tau z commitment value) := by
  have hdiff : tau - z ≠ 0 := sub_ne_zero.mpr hz
  simp [scalarVerify, forgeProof, hdiff]

/-- Existential form of `forgeProof_accepts`: every value has an accepting
proof when `tau` is known and the opening point is not `tau`. -/
theorem exists_accepting_proof
    (tau z commitment value : F) (hz : tau ≠ z) :
    ∃ proof, scalarVerify tau z commitment value proof :=
  ⟨forgeProof tau z commitment value,
    forgeProof_accepts tau z commitment value hz⟩

/-- At the trapdoor point the proof term is annihilated, so acceptance is
exactly equality of the commitment scalar and claimed value. -/
theorem scalarVerify_at_trapdoor_iff
    (tau commitment value proof : F) :
    scalarVerify tau tau commitment value proof ↔ commitment = value := by
  simp [scalarVerify]

/-- In particular, at `z = tau` the verifier's answer is independent of the
supplied proof. -/
theorem scalarVerify_at_trapdoor_proof_independent
    (tau commitment value proof₁ proof₂ : F) :
    scalarVerify tau tau commitment value proof₁ ↔
      scalarVerify tau tau commitment value proof₂ := by
  simp [scalarVerify]

end Scalar

section AdditiveModule

variable {F M : Type*} [Field F] [AddCommGroup M] [Module F M]

/-- Pairing-free linear surrogate for the corresponding group equality.

This module proves no equivalence between this predicate and a pairing-based
KZG verifier; it isolates only the algebraic trapdoor constructor. -/
def groupVerify
    (tau z : F) (commitment generator proof : M) (value : F) : Prop :=
  commitment = (tau - z) • proof + value • generator

/-- Additive-group trapdoor constructor. -/
def forgeGroupProof
    (tau z : F) (commitment generator : M) (value : F) : M :=
  (tau - z)⁻¹ • (commitment - value • generator)

/-- The module constructor satisfies the group verification equation away
from the exceptional point. -/
theorem forgeGroupProof_accepts
    (tau z : F) (commitment generator : M) (value : F)
    (hz : tau ≠ z) :
    groupVerify tau z commitment generator
      (forgeGroupProof tau z commitment generator value) value := by
  have hdiff : tau - z ≠ 0 := sub_ne_zero.mpr hz
  simp [groupVerify, forgeGroupProof, smul_smul, hdiff]

/-- The same exceptional-point characterization in the additive module. -/
theorem groupVerify_at_trapdoor_iff
    (tau : F) (commitment generator proof : M) (value : F) :
    groupVerify tau tau commitment generator proof value ↔
      commitment = value • generator := by
  simp [groupVerify]

end AdditiveModule

end KZGToxicWaste
