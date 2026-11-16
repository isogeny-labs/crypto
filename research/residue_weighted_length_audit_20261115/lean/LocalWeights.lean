import JVSchemeCertificate
import ResidueLengthImports

namespace ResidueWeightedLengthAudit
noncomputable section

section Local
variable (K B : Type*) [Field K] [CommRing B] [IsLocalRing B] [Algebra K B]
  [Module.Finite K B]

/-- A field's residue map does not alter scalar length. -/
theorem base_residue_scalar_length :
    Module.length (IsLocalRing.ResidueField K) (IsLocalRing.ResidueField B) =
      Module.length K (IsLocalRing.ResidueField B) := by
  exact (Module.length_eq_of_surjective (S := K) (R := IsLocalRing.ResidueField K)
    (M := IsLocalRing.ResidueField B) (IsLocalRing.residue_surjective)).symm

/-- The established local-ring extension theorem, with the actual residue-degree weight. -/
theorem local_module_weight (M : Type*) [AddCommGroup M] [Module K M] [Module B M]
    [IsScalarTower K B M] :
    Module.length K M = Module.length B M * Module.finrank K (IsLocalRing.ResidueField B) := by
  have he := IsLocalRing.length_restrictScalars K B M
  rw [base_residue_scalar_length K B,
    Module.length_eq_finrank K (IsLocalRing.ResidueField B)] at he
  exact he

theorem local_self_dimension : Module.finrank K B =
    (Module.length B B).toNat * Module.finrank K (IsLocalRing.ResidueField B) := by
  have he := local_module_weight K B B
  rw [Module.length_eq_finrank K B] at he
  simpa only [ENat.toNat_mul, ENat.toNat_natCast] using congrArg ENat.toNat he

def maximalResidueQuotientEquiv (I : Ideal B) [I.IsMaximal] :
    (B ⧸ I) ≃ₐ[K] I.ResidueField :=
  AlgEquiv.ofBijective (IsScalarTower.toAlgHom K (B ⧸ I) I.ResidueField)
    I.bijective_algebraMap_quotient_residueField

end Local

#print axioms local_module_weight
#print axioms local_self_dimension
end
end ResidueWeightedLengthAudit
