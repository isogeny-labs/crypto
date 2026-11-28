import CubicTwistCount
import RationalTorsion

namespace FiniteCharThreeTraceAudit
noncomputable section
open QuadraticTwistCountAudit WeierstrassCurve.Affine ThreeTorsionRankAudit.General
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {K : Type*} [Field K]

theorem twist_three_polynomial (a b c d x : K) :
    (twistModel a b c d).Ψ₃.eval (d*x)=d^4*(cubicModel a b c).Ψ₃.eval x := by
  simp only [WeierstrassCurve.Ψ₃,Polynomial.eval_add,Polynomial.eval_mul,
    Polynomial.eval_pow,Polynomial.eval_ofNat,Polynomial.eval_X,Polynomial.eval_C,
    twistModel,cubicModel,WeierstrassCurve.b₂,WeierstrassCurve.b₄,
    WeierstrassCurve.b₆,WeierstrassCurve.b₈]
  ring

theorem point_of_three_root [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (x y : K) (he : W.Equation x y) (hΨ : W.Ψ₃.eval x=0) :
    ∃ P : W.Point,P≠0 ∧ (3:ℕ) • P=0 := by
  have hN := (W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp he
  refine ⟨Point.some x y hN,Point.some_ne_zero hN,?_⟩
  apply (three_point_iff W hN).mpr
  exact hΨ

theorem three_point_dvd_count [DecidableEq K] (W : WeierstrassCurve.Affine K)
    (hex : ∃ P : W.Point,P≠0 ∧ (3:ℕ) • P=0) : 3∣Nat.card W.Point := by
  obtain ⟨P,hP,hk⟩ := hex
  have ho : addOrderOf P=3 := addOrderOf_eq_prime hk hP
  rw [←ho]
  exact addOrderOf_dvd_natCard P

variable [CharP K 3]

theorem char_three_two_ne_zero : (2:K)≠0 := by
  intro h
  have h3 : (3:K)=0 := CharP.cast_eq_zero K 3
  apply one_ne_zero (α:=K)
  linear_combination h3-h

theorem char_three_four_eq_one : (4:K)=1 := by
  linear_combination CharP.cast_eq_zero K 3

theorem char_three_odd : ringChar K≠2 := by
  intro h
  have h2 : (2:K)≠0 := char_three_two_ne_zero
  have : CharP K 2 := ringChar.of_eq h
  exact h2 (CharP.cast_eq_zero K 2)

theorem finite_cube_root [Finite K] (c : K) : ∃ x : K,x^3=c := by
  obtain ⟨x,hx⟩ := Finite.surjective_of_injective (frobenius_inj K 3) c
  exact ⟨x,hx⟩

variable [Fintype K] [DecidableEq K]

/-- Over a finite characteristic-three field, an ordinary cubic has an
actual nonzero three-torsion point on it or its nonsquare-scaled partner. -/
theorem ordinary_either_three_point (a b c d : K)
    (hΔ : (cubicModel a b c).Δ≠0) (ha : a≠0) (hd : ¬IsSquare d) :
    (∃ P : (cubicModel a b c).Point,P≠0 ∧ (3:ℕ) • P=0) ∨
    (∃ P : (twistModel a b c d).Point,P≠0 ∧ (3:ℕ) • P=0) := by
  have hb2 : (cubicModel a b c).b₂≠0 := by
    simpa [cubicModel,WeierstrassCurve.b₂,char_three_four_eq_one] using ha
  obtain ⟨x,hx⟩ := finite_cube_root (-(cubicModel a b c).b₈/(cubicModel a b c).b₂)
  have hΨ : (cubicModel a b c).Ψ₃.eval x=0 := by
    rw [SmallCharacteristicJAudit.psi_char_three,hx]
    field_simp [hb2]
    ring
  let v : K := cubicValue a b c x
  have hsum := scaled_root_card_sum char_three_odd d hd v
  by_cases hex : ∃ y : K,y^2=v
  · left
    obtain ⟨y,hy⟩ := hex
    apply point_of_three_root _ hΔ x y _ hΨ
    rw [WeierstrassCurve.Affine.equation_iff]
    simpa [cubicModel,cubicValue,v] using hy
  · right
    let : IsEmpty (squareRoots v) := ⟨fun p => hex ⟨p.val,p.property⟩⟩
    have hz : rootCard v=0 := Nat.card_of_isEmpty
    have ht : rootCard (d^3*v)=2 := by omega
    have hn : Nat.card (squareRoots (d^3*v))≠0 := by
      change rootCard (d^3*v)≠0
      rw [ht]
      decide
    obtain ⟨p⟩ := (Nat.card_ne_zero.mp hn).1
    rcases p with ⟨y,hy⟩
    have hd0 : d≠0 := by
      intro h
      apply hd
      rw [h]
      exact ⟨0,by simp⟩
    have hΔt : (twistModel a b c d).Δ≠0 := by
      rw [twist_discriminant]
      exact mul_ne_zero (pow_ne_zero 6 hd0) hΔ
    have hΨt : (twistModel a b c d).Ψ₃.eval (d*x)=0 := by
      rw [twist_three_polynomial,hΨ,mul_zero]
    apply point_of_three_root _ hΔt (d*x) y _ hΨt
    have hy' : y^2=cubicValue (d*a) (d^2*b) (d^3*c) (d*x) := by
      rw [twist_value]
      exact hy
    rw [WeierstrassCurve.Affine.equation_iff]
    simpa [twistModel,cubicModel,cubicValue] using hy'

#print axioms twist_three_polynomial
#print axioms ordinary_either_three_point
end
end FiniteCharThreeTraceAudit
