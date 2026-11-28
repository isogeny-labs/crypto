import GenericSpecialization

namespace SemaevReassociationAudit
open Polynomial S4DegreeAudit ResultantDegreeStepAudit
open ResultantInterchangeAudit RecursiveSemaevEvaluationAudit
noncomputable section
variable {R : Type*} [CommRing R]

/-- A genuinely independent leading coefficient, specialized to zero later. -/
def genericLeading (p : Polynomial R) (D : ℕ) : Polynomial (Polynomial R) :=
  p.map C + C X * X ^ D

theorem genericLeading_degree [IsDomain R] (p : Polynomial R) (D : ℕ)
    (hp : p.natDegree ≤ D) : (genericLeading p D).natDegree = D := by
  apply natDegree_eq_of_le_of_coeff_ne_zero
  · apply (natDegree_add_le _ _).trans
    apply max_le
    · exact natDegree_map_le.trans hp
    · compute_degree!
  · change (p.map C + C X * X ^ D).coeff D ≠ 0
    simp only [coeff_add, coeff_map, coeff_C_mul, coeff_X_pow_self, mul_one]
    intro hz
    have hc := congrArg (fun q : Polynomial R => q.coeff 1) hz
    norm_num at hc

theorem genericLeading_specialize (p : Polynomial R) (D : ℕ) :
    (genericLeading p D).map (evalRingHom 0) = p := by
  have h : (evalRingHom (0 : R)).comp C = RingHom.id R := by ext a; simp
  simp [genericLeading, Polynomial.map_map, h]

theorem evalHom_C (z a : R) : (evalRingHom z) (C a) = a := by simp

/-- Prescribed degrees are retained after all coefficient specializations. -/
theorem reassociation_prescribed [IsDomain R] (p : Polynomial R) (D : ℕ)
    (hp : p.natDegree ≤ D) (b2 b4 b6 b8 x y z : R) :
    leftAssoc D p b2 b4 b6 b8 x y z = rightAssoc D p b2 b4 b6 b8 x y z := by
  have h := reassociation_all_parameters (genericLeading p D)
    (C b2) (C b4) (C b6) (C b8) (C x) (C y) (C z)
  rw [genericLeading_degree p D hp] at h
  have he := congrArg (evalRingHom (0 : R)) h
  rw [leftAssoc_map, rightAssoc_map] at he
  simpa only [genericLeading_specialize, evalHom_C] using he

#print axioms reassociation_prescribed

end
end SemaevReassociationAudit
