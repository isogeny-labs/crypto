import GeometricTorsion
import Mathlib.FieldTheory.IsAlgClosed.Basic

namespace PairingGeometricClosureAudit
noncomputable section
open PairingMillerCurveAudit PairingMillerTorsionAudit PairingGeometricTorsionAudit
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
variable {K : Type*} [Field K] [Algebra (ZMod 5) K] [IsAlgClosed K] [DecidableEq K]

def chosenEmbedding : F25 →ₐ[ZMod 5] K := IsAlgClosed.lift

/-- Actual geometric3-torsion cardinality for y²=x³+1 in characteristic5.
The finite basis and point-map surjectivity were proved from coordinates;
no general torsion rank theorem is a premise. -/
theorem actual_geometric_torsion_cardinality :
    Nat.card (extendedTorsion (K:=K))=9 :=
  all_extension_torsion_cardinality (chosenEmbedding (K:=K))

def actualGeometricBasis : Index ≃+ extendedTorsion (K:=K) :=
  PairingMillerTorsionAudit.torsionEquiv.trans
    (torsionEquivalence (chosenEmbedding (K:=K)))

theorem actual_geometric_basis_values (v : Index) :
    (actualGeometricBasis (K:=K) v).val=
      pointMap (chosenEmbedding (K:=K)) (indexedPoint v) := rfl

theorem actual_geometric_torsion_certificate :
    Nat.card (extendedTorsion (K:=K))=9 ∧
    (∃ e : Index ≃+ extendedTorsion (K:=K),
      ∀ v : Index,(e v).val=pointMap (chosenEmbedding (K:=K)) (indexedPoint v)) ∧
    (∀ R : (extendedCurve (K:=K)).Point,(3:ℕ) • R=0 →
      ∃ U : curve.Point,(3:ℕ) • U=0 ∧ pointMap (chosenEmbedding (K:=K)) U=R) :=
  ⟨actual_geometric_torsion_cardinality,⟨actualGeometricBasis,
    actual_geometric_basis_values⟩,actual_torsion_descends chosenEmbedding⟩

#print axioms actual_geometric_torsion_cardinality
#print axioms actual_geometric_torsion_certificate
end
end PairingGeometricClosureAudit
