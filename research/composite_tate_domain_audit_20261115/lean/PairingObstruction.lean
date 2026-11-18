module
public import UnitsQuotient

@[expose] public section
namespace CompositeTateDomainAudit
noncomputable section
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing F := (inferInstance : Field F).toCommRing

/-- Exactly the two multiplicative bilinearity laws on the actual Tate domains. -/
structure BilinearMap where
  toFun : LeftDomain → RightDomain → Target
  mul_left : ∀ P R Q, toFun (P * R) Q = toFun P Q * toFun R Q
  mul_right : ∀ P Q S, toFun P (Q * S) = toFun P Q * toFun P S

instance : CoeFun BilinearMap (fun _ => LeftDomain → RightDomain → Target) := ⟨BilinearMap.toFun⟩

theorem bilinear_one_left (B : BilinearMap) (Q : RightDomain) : B 1 Q = 1 := by
  have h := B.mul_left 1 1 Q
  simp only [one_mul] at h
  exact mul_left_cancel (h.symm.trans (mul_one (B 1 Q)).symm)

theorem bilinear_one_right (B : BilinearMap) (P : LeftDomain) : B P 1 = 1 := by
  have h := B.mul_right P 1 1
  simp only [one_mul] at h
  exact mul_left_cancel (h.symm.trans (mul_one (B P 1)).symm)

theorem bilinear_pow_left (B : BilinearMap) (P : LeftDomain) (Q : RightDomain) (n : ℕ) :
    B (P ^ n) Q = (B P Q) ^ n := by
  induction n with
  | zero => simp only [pow_zero, bilinear_one_left]
  | succ n ih => rw [pow_succ, B.mul_left, ih, pow_succ]

theorem bilinear_pow_right (B : BilinearMap) (P : LeftDomain) (Q : RightDomain) (n : ℕ) :
    B P (Q ^ n) = (B P Q) ^ n := by
  induction n with
  | zero => simp only [pow_zero, bilinear_one_right]
  | succ n ih => rw [pow_succ, B.mul_right, ih, pow_succ]

def LeftNondegenerate (B : BilinearMap) : Prop :=
  ∀ P : LeftDomain, (∀ Q : RightDomain, B P Q = 1) → P = 1

def RightNondegenerate (B : BilinearMap) : Prop :=
  ∀ Q : RightDomain, (∀ P : LeftDomain, B P Q = 1) → Q = 1

theorem cube_one_implies_one (z : Target) (h3 : z ^ 3 = 1) : z = 1 := by
  have h2 : z ^ 2 = 1 := target_square z
  calc
    z = z ^ 2 * z := by rw [h2, one_mul]
    _ = z ^ 3 := (pow_succ z 2).symm
    _ = 1 := h3

theorem left_point_radical (B : BilinearMap) (Q : RightDomain) : B leftPoint Q = 1 := by
  apply cube_one_implies_one
  have h := bilinear_pow_left B leftPoint Q 3
  have hP : leftPoint ^ 3 = 1 := left_point_cubed
  rw [hP, bilinear_one_left] at h
  exact h.symm

theorem right_point_radical (B : BilinearMap) (P : LeftDomain) : B P rightPoint = 1 := by
  apply cube_one_implies_one
  have h := bilinear_pow_right B P rightPoint 3
  have hQ : rightPoint ^ 3 = 1 := right_point_cubed
  rw [hQ, bilinear_one_right] at h
  exact h.symm

theorem left_nondegeneracy_impossible (B : BilinearMap) : ¬ LeftNondegenerate B := by
  intro h
  exact left_point_nontrivial (h leftPoint (left_point_radical B))

theorem right_nondegeneracy_impossible (B : BilinearMap) : ¬ RightNondegenerate B := by
  intro h
  exact right_point_nontrivial (h rightPoint (right_point_radical B))

theorem bilinear_perfectness_impossible :
    ¬ ∃ B : BilinearMap, LeftNondegenerate B ∧ RightNondegenerate B := by
  rintro ⟨B, h, _⟩
  exact left_nondegeneracy_impossible B h

def LiteralGuard : Prop := curve.Δ ≠ 0 ∧ Nat.Coprime 6 5 ∧ 6 ∣ Nat.card G ∧
  ∃ u : Fˣ, u ≠ 1 ∧ u ^ 6 = 1

theorem literal_guard_holds : LiteralGuard := by
  refine ⟨curve_delta, by decide, ?_, negativeUnit, negative_unit_nontrivial,
    negative_unit_sixth⟩
  rw [curve_count]

theorem literal_nontrivial_root_nondegeneracy_false :
    ¬ (LiteralGuard → ∃ B : BilinearMap, LeftNondegenerate B ∧ RightNondegenerate B) := by
  intro h
  exact bilinear_perfectness_impossible (h literal_guard_holds)

#print axioms left_point_radical
#print axioms right_point_radical
#print axioms bilinear_perfectness_impossible
#print axioms literal_nontrivial_root_nondegeneracy_false
end
end CompositeTateDomainAudit
