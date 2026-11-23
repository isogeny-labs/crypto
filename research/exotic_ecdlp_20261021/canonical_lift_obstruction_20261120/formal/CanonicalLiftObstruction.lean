import Mathlib

/-! Structural obstruction for logarithms of homomorphic lifts and the defect of
an arbitrary section of a reduction homomorphism. -/

namespace CanonicalLiftObstruction

variable {G H A : Type*} [AddCommGroup G] [AddCommGroup H] [AddCommGroup A]

/-- Any section of reduction has an addition defect in the kernel of reduction. -/
def defect (s : G → H) (P Q : G) : H := s (P + Q) - s P - s Q

theorem defect_reduces_to_zero (ρ : H →+ G) (s : G → H)
    (hs : ∀ P, ρ (s P) = P) (P Q : G) :
    ρ (defect s P Q) = 0 := by
  simp [defect, map_sub, hs]

/-- The defect vanishes when the section is additive. -/
theorem defect_of_additive_section (s : G →+ H) (P Q : G) :
    defect s P Q = 0 := by
  simp [defect, map_add]

/-- The logarithm of a sum of selected lifts includes the logarithm of the
addition defect. This assumes the logarithm is defined on all of `H`. -/
theorem log_section_addition (s : G → H) (log : H →+ A) (P Q : G) :
    log (s (P + Q)) = log (s P) + log (s Q) + log (defect s P Q) := by
  simp only [defect, map_sub]
  abel

/-- A homomorphic lift of a point of finite order has zero image under any
additive logarithm into a group satisfying the stated torsion-freeness law. -/
theorem log_homomorphic_lift_eq_zero (s : G →+ H) (log : H →+ A)
    (htf : ∀ n : ℕ, 0 < n → ∀ a : A, n • a = 0 → a = 0)
    (n : ℕ) (hn : 0 < n) (P : G) (hP : n • P = 0) :
    log (s P) = 0 := by
  apply htf n hn
  calc
    n • log (s P) = log (s (n • P)) := by simp
    _ = 0 := by simp [hP]

/-- A finite source supplies a positive common annihilator to every point. -/
theorem finite_source_log_homomorphic_lift_eq_zero
    [Finite G] (s : G →+ H) (log : H →+ A)
    (htf : ∀ n : ℕ, 0 < n → ∀ a : A, n • a = 0 → a = 0) (P : G) :
    log (s P) = 0 := by
  exact log_homomorphic_lift_eq_zero s log htf (Nat.card G)
    (Nat.card_pos) P (card_nsmul_eq_zero' : Nat.card G • P = 0)

#print axioms CanonicalLiftObstruction.defect_reduces_to_zero
#print axioms CanonicalLiftObstruction.defect_of_additive_section
#print axioms CanonicalLiftObstruction.log_section_addition
#print axioms CanonicalLiftObstruction.log_homomorphic_lift_eq_zero
#print axioms CanonicalLiftObstruction.finite_source_log_homomorphic_lift_eq_zero

end CanonicalLiftObstruction
