import EquationCount
import RationalTorsion

namespace FiniteCharTwoTraceAudit
noncomputable section
open WeierstrassCurve.Affine TwoTorsionRankAudit
variable {K : Type*} [Field K] [DecidableEq K] [CharP K 2]

/-- The earlier algebraic-closure construction descends over any finite field
of characteristic two, using the official finite reduced-ring square theorem. -/
theorem finite_ordinary_nonzero_killed [Finite K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (ha : W.a₁≠0) : ∃ P : W.Point,P≠0 ∧ 2 • P=0 := by
  let x : K := -W.a₃/W.a₁
  have hx : W.a₁*x+W.a₃=0 := by dsimp [x]; field_simp; ring
  obtain ⟨y,hy⟩ := isSquare_of_charTwo' (x^3+W.a₂*x^2+W.a₄*x+W.a₆)
  have hy' : y^2=x^3+W.a₂*x^2+W.a₄*x+W.a₆ := by simpa only [pow_two] using hy.symm
  have he : W.Equation x y := by
    rw [WeierstrassCurve.Affine.equation_iff]
    linear_combination hy'+y*hx
  have hn := (W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp he
  exact ⟨Point.some x y hn,Point.some_ne_zero hn,(char_two_killed_iff W x y hn).mpr hx⟩

theorem finite_two_card [Finite K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Nat.card (TwoTorsion W)=if W.a₁=0 then 1 else 2 := by
  by_cases ha : W.a₁=0
  · rw [ite_eq_left ha]
    exact char_two_torsion_card_one W hΔ ha
  · rw [ite_eq_right ha]
    obtain ⟨P,hP,hk⟩ := finite_ordinary_nonzero_killed W hΔ ha
    apply torsion_card_two_of_classification W P hP hk
    intro Q hQ
    by_cases hz : Q=0
    · exact Or.inl hz
    · exact Or.inr (nonzero_killed_unique W ha Q P hz hP hQ hk)

theorem two_dvd_actual_count_iff [Finite K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    2∣Nat.card W.Point ↔ W.a₁≠0 := by
  constructor
  · intro hd ha
    obtain ⟨P,hP,_,hk⟩ := BKLinearAudit.actual_rational_prime_torsion W 2 (by decide) hd
    exact hP (only_zero_when_a1_zero W hΔ ha P hk)
  · intro ha
    obtain ⟨P,hP,hk⟩ := finite_ordinary_nonzero_killed W hΔ ha
    have ho : addOrderOf P=2 := addOrderOf_eq_prime hk hP
    rw [←ho]
    exact addOrderOf_dvd_natCard P

#print axioms finite_ordinary_nonzero_killed
#print axioms finite_two_card
#print axioms two_dvd_actual_count_iff
end
end FiniteCharTwoTraceAudit
