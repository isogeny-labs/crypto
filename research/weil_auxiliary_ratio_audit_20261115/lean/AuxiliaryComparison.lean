import AuxiliarySpecialCases

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open ThreeTangentPairingAudit
variable {K : Type*} [Field K] [DecidableEq K]

omit [DecidableEq K] in
theorem equal_x_ordinates (W : WeierstrassCurve.Affine K) (x y v : K)
    (hy : W.Equation x y) (hv : W.Equation x v) : y=v ∨ y=W.negY x v := by
  have he : (y-v)*(y-W.negY x v)=0 := by
    rw [WeierstrassCurve.Affine.negY]
    have hy' := (W.equation_iff' x y).mp hy
    have hv' := (W.equation_iff' x v).mp hv
    linear_combination hy'-hv'
  rcases mul_eq_zero.mp he with h|h
  · exact Or.inl (sub_eq_zero.mp h)
  · exact Or.inr (sub_eq_zero.mp h)

/-- Full actual three-torsion comparison, including zero, equal and opposite points. -/
theorem chosen_auxiliary_tangent [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : W.Point)
    (htP : (3 : ℕ) • P=0) (htQ : (3 : ℕ) • Q=0) :
    auxiliaryQuotient W hΔ P Q (chosenFunctionUnit W P 3) (chosenFunctionUnit W Q 3)=
      constantHom W (tangentPair W P Q) := by
  cases P with
  | zero =>
    change auxiliaryQuotient W hΔ 0 Q (millerFunction W 0 3) (millerFunction W Q 3)=constantHom W 1
    rw [miller_zero_point,auxiliary_zero_left W hΔ Q (miller_function_nonzero W Q 3),map_one]
  | some x y hP =>
    cases Q with
    | zero =>
      change auxiliaryQuotient W hΔ (.some x y hP) 0 (millerFunction W (.some x y hP) 3)
        (millerFunction W 0 3)=constantHom W 1
      rw [miller_zero_point,auxiliary_zero_right W hΔ (.some x y hP)
        (miller_function_nonzero W (.some x y hP) 3),map_one]
    | some u v hQ =>
      by_cases hx : x=u
      · subst u
        rcases equal_x_ordinates W x y v hP.1 hQ.1 with he|he
        · subst v
          have hp : tangentPair W (.some x y hP) (.some x y hQ)=1 := by
            simp [tangentPair]
          rw [hp,map_one]
          exact chosen_diagonal_one W hΔ (.some x y hP) htP
        · have hPQ : Point.some x y hP= -(Point.some x v hQ) := by
            simp only [Point.neg_some,Point.some.injEq]
            exact ⟨True.intro,he⟩
          have hQP : Point.some x v hQ= -(Point.some x y hP) := by
            calc
              Point.some x v hQ= -(-(Point.some x v hQ)) := (neg_neg _).symm
              _ = -(Point.some x y hP) := congrArg Neg.neg hPQ.symm
          have hp : tangentPair W (.some x y hP) (.some x v hQ)=1 := by
            simp [tangentPair]
          rw [hp,map_one,hQP]
          exact chosen_opposite_one W hΔ (.some x y hP) 3 htP
      · rw [chosen_three_tangent W hΔ hP htP,chosen_three_tangent W hΔ hQ htQ]
        exact tangent_auxiliary_quotient W hΔ x y u v hP hQ htP htQ hx

/-- Exact Galbraith26.1 source-order value for every actual geometric three-torsion pair. -/
theorem chosen_source_inverse [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P Q : W.Point)
    (htP : (3 : ℕ) • P=0) (htQ : (3 : ℕ) • Q=0) (S : W.Point)
    (h : AuxiliaryAdmissible W P Q (chosenFunctionUnit W P 3) (chosenFunctionUnit W Q 3) S) :
    (sourceAuxiliaryValue W hΔ P Q S (chosenFunctionUnit W P 3) (chosenFunctionUnit W Q 3) h : K)=
      (tangentPair W P Q)⁻¹ := by
  have hn := tangentPair_ne_zero W hΔ ⟨P,htP⟩ ⟨Q,htQ⟩
  have hv := auxiliary_value_of_scalar W hΔ P Q _ _ S h _ hn
    (chosen_auxiliary_tangent W hΔ P Q htP htQ)
  rw [source_auxiliary_inverse,hv]
  rfl

#print axioms equal_x_ordinates
#print axioms chosen_auxiliary_tangent
#print axioms chosen_source_inverse
end
end WeilAuxiliaryRatioAudit
