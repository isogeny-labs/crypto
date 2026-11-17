module
public import LevelNumerical

@[expose] public section
namespace MillerLevelCompatibilityAudit
noncomputable section
open WeierstrassCurve.Affine
open BinaryMillerAudit MillerPrincipalIdealAudit MillerFinitePointAudit MillerNormValuationAudit
open MillerRecursionAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit FinitePointEvaluationAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing TestField := (inferInstance : Field TestField).toCommRing
local instance : DecidableEq (AlgebraicClosure TestField) := Classical.decEq _

theorem miller_level_compatibility_certificate :
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (_ : W.Δ ≠ 0)
      (P : W.Point) (n : ℕ) (_ : n • P = 0) (a : ℕ),
      millerFunction W P (a * n) = millerFunction W P n ^ a) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
      (P : W.Point) (n N : ℕ) (hP : n • P = 0) (hN : n ∣ N) (D : W.Point →₀ ℤ)
      (hd : Disjoint D.support (functionDivisor W (millerUnit W P n) (millerUnit W P n).ne_zero).support),
      Disjoint D.support (functionDivisor W (millerUnit W P N) (millerUnit W P N).ne_zero).support ∧
      evaluateDivisor W D (millerUnit W P N) (divisibleDomain W hΔ P n N hP hN D hd) =
        evaluateDivisor W D (millerUnit W P n) hd ^ (N / n)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
      (P : W.Point) (n N M : ℕ) (_ : 0 < n) (_ : 0 < N) (hP : n • P = 0)
      (hnN : n ∣ N) (_ : N ∣ M) (D : W.Point →₀ ℤ)
      (hd : Disjoint D.support (functionDivisor W (millerUnit W P n) (millerUnit W P n).ne_zero).support),
      evaluateDivisor W D (millerUnit W P N) (divisibleDomain W hΔ P n N hP hnN D hd) ^ (M / N) =
        evaluateDivisor W D (millerUnit W P n) hd ^ (M / n)) ∧
    TestCurve.Δ ≠ 0 ∧ GeometricCurve.Δ ≠ 0 ∧ Fintype.card TestField = 5 ∧
    Fintype.card TestField - 1 = 4 ∧ (2 : ℕ) ∣ 4 ∧ (4 : ℕ) ∣ Fintype.card TestField - 1 ∧
    addOrderOf twoTorsionPoint = 2 ∧ (2 : ℕ) • twoTorsionPoint = 0 ∧
    addOrderOf basePoint = 3 ∧ (2 : ℕ) • basePoint ≠ 0 ∧ TestCurve.Nonsingular (2 : TestField) 3 ∧
    divisorDegree TestCurve positiveDivisor = 0 ∧ divisorDegree TestCurve negativeDivisor = 0 ∧
    divisorDegree GeometricCurve (geometricDifference basePoint safePoint) = 0 ∧
    divisorDegree GeometricCurve (geometricDifference point23 twoTorsionPoint) = 0 ∧
    Disjoint (geometricDifference basePoint safePoint).support
      (functionDivisor GeometricCurve (levelGeometricFunction twoTorsionPoint 2) (level_geometric_nonzero twoTorsionPoint 2)).support ∧
    Disjoint (geometricDifference basePoint safePoint).support
      (functionDivisor GeometricCurve (levelGeometricFunction twoTorsionPoint 4) (level_geometric_nonzero twoTorsionPoint 4)).support ∧
    Disjoint (geometricDifference point23 twoTorsionPoint).support
      (functionDivisor GeometricCurve (levelGeometricFunction basePoint 2) (level_geometric_nonzero basePoint 2)).support ∧
    Disjoint (geometricDifference point23 twoTorsionPoint).support
      (functionDivisor GeometricCurve (levelGeometricFunction basePoint 4) (level_geometric_nonzero basePoint 4)).support ∧
    millerFunction TestCurve basePoint 4 ≠ millerFunction TestCurve basePoint 2 ^ 2 ∧
    (evaluateDivisor TestCurve positiveDivisor (millerUnit TestCurve twoTorsionPoint 2) positive_domain_two : TestField) = 2 ∧
    (evaluateDivisor TestCurve positiveDivisor (millerUnit TestCurve twoTorsionPoint 4) positive_domain_four : TestField) = 4 ∧
    (evaluateDivisor TestCurve negativeDivisor (millerUnit TestCurve basePoint 2) negative_domain_two : TestField) = 1 ∧
    (evaluateDivisor TestCurve negativeDivisor (millerUnit TestCurve basePoint 4) negative_domain_four : TestField) = 3 ∧
    (evaluateDivisor TestCurve positiveDivisor (millerUnit TestCurve twoTorsionPoint 2) positive_domain_two : TestField) ^ (4 / 2 : ℕ) =
      (evaluateDivisor TestCurve positiveDivisor (millerUnit TestCurve twoTorsionPoint 4) positive_domain_four : TestField) ^ (4 / 4 : ℕ) ∧
    (evaluateDivisor TestCurve negativeDivisor (millerUnit TestCurve basePoint 2) negative_domain_two : TestField) ^ (4 / 2 : ℕ) ≠
      (evaluateDivisor TestCurve negativeDivisor (millerUnit TestCurve basePoint 4) negative_domain_four : TestField) ^ (4 / 4 : ℕ) ∧
    (4 : TestField) ≠ 1 := by
  refine ⟨?_, ?_, ?_, control_delta, geometric_delta, field_card_five, ?_, by decide, ?_,
    two_point_order, two_point_double, base_order_three, base_not_two_torsion, point23_nonsingular,
    positive_degree_zero, negative_degree_zero, geometric_difference_degree _ _, geometric_difference_degree _ _,
    positive_geometric_domain_two, positive_geometric_domain_four, negative_geometric_domain_two, negative_geometric_domain_four,
    negative_function_power_failure, positive_value_two, positive_value_four, negative_value_two, negative_value_four,
    positive_final_compatibility, negative_final_failure, by decide +kernel⟩
  · intro K _ _ W hΔ P n hP a
    exact miller_multiple W hΔ P n hP a
  · intro K _ _ W hΔ P n N hP hN D hd
    exact ⟨divisibleDomain W hΔ P n N hP hN D hd, value_divisible W hΔ P n N hP hN D hd⟩
  · intro K _ _ W hΔ P n N M hn hN hP hnN hNM D hd
    exact final_exponent_compatibility W hΔ P n N M hn hN hP hnN hNM D hd
  · rw [field_card_five]
  · rw [field_card_five]

#print axioms miller_level_compatibility_certificate
end
end MillerLevelCompatibilityAudit
