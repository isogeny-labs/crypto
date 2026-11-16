import FrobeniusDegreeImports

namespace EllipticFrobeniusDegreeAudit
noncomputable section
variable {K : Type*} [Field K] [Fintype K]

def rationalFrobenius : RatFunc K →ₐ[K] RatFunc K :=
  FiniteField.frobeniusAlgHom K (RatFunc K)

theorem rational_frobenius_range :
    (rationalFrobenius (K:=K)).fieldRange =
      IntermediateField.adjoin K {(RatFunc.X : RatFunc K)^Fintype.card K} := by
  rw [AlgHom.fieldRange_eq_map,← RatFunc.adjoin_X,IntermediateField.adjoin_map]
  simp only [Set.image_singleton]
  rfl

theorem rational_frobenius_degree :
    Module.finrank (rationalFrobenius (K:=K)).fieldRange (RatFunc K) = Fintype.card K := by
  rw [rational_frobenius_range]
  have h := RatFunc.finrank_eq_max_natDegree ((RatFunc.X : RatFunc K)^Fintype.card K)
  rw [h]
  have hx : (RatFunc.X : RatFunc K)^Fintype.card K =
      algebraMap (Polynomial K) (RatFunc K) (Polynomial.X^Fintype.card K) := by
    rw [map_pow]
    rfl
  rw [hx,RatFunc.num_algebraMap,RatFunc.denom_algebraMap,
    Polynomial.natDegree_X_pow,Polynomial.natDegree_one,Nat.max_zero]

#print axioms rational_frobenius_range
#print axioms rational_frobenius_degree
end
end EllipticFrobeniusDegreeAudit
