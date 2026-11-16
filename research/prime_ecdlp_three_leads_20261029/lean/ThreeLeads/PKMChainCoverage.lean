import Mathlib.Algebra.Group.Basic
import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point

/-! Abstract and Mathlib affine chain-order controls. Rust's coordinates and
polynomial solutions still need their own bridge to these objects. No solving
or extraction-cost bound is proved. -/
namespace PKMChainCoverage

variable {G : Type*} [AddCommGroup G]

/-- Keep the first pair and exchange the last two points if needed. One order
has both intermediate sums nonzero and the same final sum. The hypotheses are
explicit rather than inferred from unformalized Rust coordinates. -/
theorem finite_chain_order (a b c d : G) (hab : a + b ≠ 0) (hcd : c ≠ d) :
    (a + b ≠ 0 ∧ a + b + c ≠ 0) ∨
    (a + b ≠ 0 ∧ a + b + d ≠ 0 ∧ a + b + d + c = a + b + c + d) := by
  by_cases hc : a + b + c = 0
  · right
    refine ⟨hab, ?_, ?_⟩
    · intro hd
      exact hcd (add_left_cancel (hc.trans hd.symm))
    · ac_rfl
  · exact Or.inl ⟨hab, hc⟩

#print axioms finite_chain_order

open WeierstrassCurve.Affine
variable {K : Type*} [Field K] [DecidableEq K] {W : WeierstrassCurve.Affine K}

/-- Actual nonsingular affine points with different abscissae cannot sum to O. -/
theorem affine_pair_nonzero {x₀ y₀ x₁ y₁ : K}
    (h₀ : W.Nonsingular x₀ y₀) (h₁ : W.Nonsingular x₁ y₁)
    (hx : x₀ ≠ x₁) :
    Point.some x₀ y₀ h₀ + Point.some x₁ y₁ h₁ ≠ 0 := by
  intro hz
  have heq : Point.some x₀ y₀ h₀ = -Point.some x₁ y₁ h₁ :=
    eq_neg_of_add_eq_zero_left hz
  rw [Point.neg_some] at heq
  exact hx (Point.some.inj heq).1

/-- Instantiate the finite-chain control on Mathlib curve points. Pairwise
distinct abscissae imply the two coordinate inequalities used here. -/
theorem affine_finite_chain_order {x₀ y₀ x₁ y₁ x₂ y₂ x₃ y₃ : K}
    (h₀ : W.Nonsingular x₀ y₀) (h₁ : W.Nonsingular x₁ y₁)
    (h₂ : W.Nonsingular x₂ y₂) (h₃ : W.Nonsingular x₃ y₃)
    (h₀₁ : x₀ ≠ x₁) (h₂₃ : x₂ ≠ x₃) :
    let a : W.Point := Point.some x₀ y₀ h₀
    let b : W.Point := Point.some x₁ y₁ h₁
    let c : W.Point := Point.some x₂ y₂ h₂
    let d : W.Point := Point.some x₃ y₃ h₃
    (a + b ≠ 0 ∧ a + b + c ≠ 0) ∨
      (a + b ≠ 0 ∧ a + b + d ≠ 0 ∧ a + b + d + c = a + b + c + d) := by
  exact finite_chain_order _ _ _ _ (affine_pair_nonzero h₀ h₁ h₀₁)
    (fun heq => h₂₃ (Point.some.inj heq).1)

#print axioms affine_pair_nonzero
#print axioms affine_finite_chain_order
end PKMChainCoverage
