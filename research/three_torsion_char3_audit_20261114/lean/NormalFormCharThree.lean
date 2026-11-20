import GeneralThreeBKCertificate

namespace ThreeTorsionCharThreeAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General ThreeTorsionCharacterAudit
variable {K : Type*} [Field K] [DecidableEq K] [CharP K 3]

omit [DecidableEq K] in
theorem normal_discriminant (a b : K) : (curve a b).Δ=b^3*a^3 := by
  rw [discriminant_value]
  have h3 : (3:K)=0 := CharP.cast_eq_zero K 3
  have h27 : (27:K)=0 := by linear_combination 9*h3
  simp only [h27,zero_mul,sub_zero]

omit [DecidableEq K] in
theorem normal_division (a b x : K) : (curve a b).Ψ₃.eval x=a^2*x^3 := by
  rw [division_value]
  have h3 : (3:K)=0 := CharP.cast_eq_zero K 3
  simp only [cubic,h3,zero_mul,zero_add,add_zero]
  ring

theorem normal_coordinates (a b : K) (ha : a≠0) (x y : K)
    (hN : (curve a b).Nonsingular x y) (ht : (3:ℕ) • Point.some x y hN=0) :
    x=0 ∧ (y=0 ∨ y=-b) := by
  have hx : x=0 := by
    have hp := (three_point_iff (curve a b) hN).mp ht
    change (curve a b).Ψ₃.eval x=0 at hp
    rw [normal_division] at hp
    have hx3 : x^3=0 := (mul_eq_zero.mp hp).resolve_left (pow_ne_zero 2 ha)
    by_contra hn
    exact pow_ne_zero 3 hn hx3
  refine ⟨hx,?_⟩
  have he : y*(y+b)=0 := by
    have h := hN.1
    rw [WeierstrassCurve.Affine.equation_iff] at h
    simp only [curve,hx,mul_zero,zero_mul,zero_pow (by decide : (3:ℕ)≠0),
      zero_pow (by decide : (2:ℕ)≠0),add_zero] at h
    linear_combination h
  exact (mul_eq_zero.mp he).imp id eq_neg_of_add_eq_zero_left

def normalCode (a b : K) : ThreeTorsion (curve a b) → Option Bool
  | ⟨.zero,_⟩ => none
  | ⟨.some _ y _,_⟩ => some (if y=0 then false else true)

omit [DecidableEq K] [CharP K 3] in
theorem normal_minus_nonsingular (a b : K) (hb : b≠0) : (curve a b).Nonsingular 0 (-b) := by
  rw [WeierstrassCurve.Affine.nonsingular_iff']
  constructor
  · simp only [WeierstrassCurve.Affine.equation_iff,curve]
    ring
  · right
    simp only [curve,mul_zero]
    intro h
    apply hb
    linear_combination -h

omit [CharP K 3] in
theorem normal_minus_three (a b : K) (hb : b≠0) :
    (3:ℕ) • Point.some 0 (-b) (normal_minus_nonsingular a b hb)=0 := by
  apply (three_point_iff (curve a b) (normal_minus_nonsingular a b hb)).mpr
  change (curve a b).Ψ₃.eval 0=0
  rw [division_value,zero_mul]

def normalPoint (a b : K) (hb : b≠0) : Option Bool → ThreeTorsion (curve a b)
  | none => 0
  | some false => torsionOrigin a b hb
  | some true => ⟨.some 0 (-b) (normal_minus_nonsingular a b hb),normal_minus_three a b hb⟩

def normalTorsionEquiv (a b : K) (ha : a≠0) (hb : b≠0) : ThreeTorsion (curve a b) ≃ Option Bool where
  toFun := normalCode a b
  invFun := normalPoint a b hb
  left_inv P := by
    rcases P with ⟨P,hP⟩
    cases P with
    | zero => rfl
    | some x y hN =>
      obtain ⟨hx,hy⟩ := normal_coordinates a b ha x y hN hP
      rcases hy with hy | hy
      · subst x
        subst y
        apply Subtype.ext
        simp [normalCode,normalPoint,torsionOrigin,origin]
      · subst x
        subst y
        apply Subtype.ext
        simp [normalCode,normalPoint,neg_ne_zero.mpr hb]
  right_inv Q := by
    cases Q with
    | none => rfl
    | some c => cases c <;> simp [normalPoint,normalCode,torsionOrigin,origin,neg_ne_zero.mpr hb]

theorem normal_three_card (a b : K) (hΔ : (curve a b).Δ≠0) :
    Nat.card (ThreeTorsion (curve a b))=3 := by
  rw [normal_discriminant] at hΔ
  have hb : b≠0 := fun h => hΔ (by rw [h,zero_pow (by decide),zero_mul])
  have ha : a≠0 := fun h => hΔ (by rw [h,zero_pow (by decide),mul_zero])
  rw [Nat.card_congr (normalTorsionEquiv a b ha hb),Nat.card_eq_fintype_card]
  decide +kernel

#print axioms normalTorsionEquiv
#print axioms normal_three_card
end
end ThreeTorsionCharThreeAudit
