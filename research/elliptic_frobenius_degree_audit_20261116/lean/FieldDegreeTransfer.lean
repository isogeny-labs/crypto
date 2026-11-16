import FrobeniusDegreeImports

namespace EllipticFrobeniusDegreeAudit
noncomputable section
variable {K L : Type*} [Field K] [Field L] [Algebra K L]

/-- Two actual towers transfer the degree through any finite positive intermediate extension. -/
theorem field_image_degree_transfer (σ : L →ₐ[K] L) (H : IntermediateField K L)
    (hstable : H.map σ ≤ H) (m : Nat) (hm : 0 < m)
    (hdegree : Module.finrank H L = m) :
    Module.finrank σ.fieldRange L = (H.map σ).relfinrank H := by
  have hmap : (H.map σ).relfinrank σ.fieldRange = m := by
    rw [AlgHom.fieldRange_eq_map,IntermediateField.relfinrank_map_map,
      IntermediateField.relfinrank_top_right,hdegree]
  have hle : H.map σ ≤ σ.fieldRange := by
    rw [AlgHom.fieldRange_eq_map]
    exact IntermediateField.map_mono σ le_top
  have hleft := IntermediateField.relfinrank_mul_finrank_top hle
  have hright := IntermediateField.relfinrank_mul_finrank_top hstable
  rw [hmap] at hleft
  rw [hdegree] at hright
  have he : m * Module.finrank σ.fieldRange L = m * (H.map σ).relfinrank H := by
    rw [hleft,←hright,Nat.mul_comm]
  exact Nat.eq_of_mul_eq_mul_left hm he

#print axioms field_image_degree_transfer
end
end EllipticFrobeniusDegreeAudit
