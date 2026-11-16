import TateMillerFunction

namespace TateMillerEvaluationAudit
noncomputable section
open WeierstrassCurve.Affine MillerRecursionAudit MillerNormValuationAudit
open MillerPrincipalIdealAudit MillerFinitePointAudit FinitePointEvaluationAudit
open FunctionDivisorFinitenessAudit WeilAuxiliaryRatioAudit WeilDivisorChoiceAudit
variable {K : Type*} [Field K] [DecidableEq K]

def shiftedDivisor (W : WeierstrassCurve.Affine K) (A B : W.Point) : W.Point →₀ ℤ:=
  Finsupp.single A 1-Finsupp.single B 1

omit [DecidableEq K] in
theorem shifted_degree_zero (W : WeierstrassCurve.Affine K) (A B : W.Point) :
    GeometricPrincipalDivisorAudit.divisorDegree W (shiftedDivisor W A B)=0 := by
  simp [shiftedDivisor,GeometricPrincipalDivisorAudit.degree_single]

theorem shifted_disjoint (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ)
    (A B : W.Point) (hA : pointOrder W f A=0) (hB : pointOrder W f B=0) :
    Disjoint (shiftedDivisor W A B).support (functionDivisor W f f.ne_zero).support := by
  apply (disjoint_principal_support_iff W f _).mpr
  apply (mem_support_units_iff W _ f).mpr
  intro R hR
  by_cases ha : R=A
  · subst R;exact hA
  by_cases hb : R=B
  · subst R;exact hB
  have hz : shiftedDivisor W A B R=0 := by
    simp [shiftedDivisor,ha,hb]
  exact False.elim ((Finsupp.mem_support_iff.mp hR) hz)

theorem shifted_evaluation (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ)
    (A B : W.Point) (hA : pointOrder W f A=0) (hB : pointOrder W f B=0) :
    evaluateDivisor W (shiftedDivisor W A B) f (shifted_disjoint W f A B hA hB)=
      localUnitValue W A f hA/localUnitValue W B f hB := by
  rw [proper_evaluation_helper]
  simp [shiftedDivisor,map_sub,exponentEvaluation,valueHelper,hA,hB]

theorem actual_three_short_unit (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (ha1 : W.a₁=0) (ha2 : W.a₂=0) (ha3 : W.a₃=0)
    {x y u v : K} (hP : W.Nonsingular x y) (h3 : (3 : ℕ) • Point.some x y hP=0)
    (hR : W.Nonsingular u v) (hv : shortLine W x y u v≠0) :
    pointOrder W (chosenFunctionUnit W (.some x y hP) 3) (.some u v hR)=0 := by
  change pointOrder W (millerFunction W (.some x y hP) 3) (.some u v hR)=0
  rw [actual_chosen_three_tangent W hΔ hP h3]
  apply (point_order_zero_iff_value_one W _ (line_function_nonzero W _ _ _) _).mpr
  change finiteAdic W hR (MillerNormValuationAudit.lineFunction W x y (W.slope x x y y))=1
  apply coordinate_nonvanishing_value W hR (lineClass W x y (W.slope x x y y))
  rw [short_three_miller_evaluation W ha1 ha2 ha3 hP h3 hR.1]
  exact hv

theorem actual_three_short_value (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (ha1 : W.a₁=0) (ha2 : W.a₂=0) (ha3 : W.a₃=0)
    {x y u v : K} (hP : W.Nonsingular x y) (h3 : (3 : ℕ) • Point.some x y hP=0)
    (hR : W.Nonsingular u v)
    (hu : pointOrder W (chosenFunctionUnit W (.some x y hP) 3) (.some u v hR)=0) :
    (localUnitValue W (.some u v hR) (chosenFunctionUnit W (.some x y hP) 3) hu : K)=
      shortLine W x y u v := by
  let f:=chosenFunctionUnit W (.some x y hP) 3
  let a : (RegularRing W (.some u v hR)).unitGroup:=
    ⟨f,(mem_point_units_iff W (.some u v hR) f).mpr hu⟩
  have he : ((RegularRing W (.some u v hR)).unitGroupMulEquiv a : RegularRing W (.some u v hR))=
      coordinateRegular W hR (lineClass W x y (W.slope x x y y)) := by
    apply Subtype.ext
    change millerFunction W (.some x y hP) 3=_
    rw [actual_chosen_three_tangent W hΔ hP h3,coordinate_regular_coe]
    rfl
  change regularEvaluation W (.some u v hR)
    ((RegularRing W (.some u v hR)).unitGroupMulEquiv a : RegularRing W (.some u v hR))=_
  rw [he,coordinate_regular_evaluation,short_three_miller_evaluation W ha1 ha2 ha3 hP h3 hR.1]
  rfl

#print axioms shifted_disjoint
#print axioms shifted_degree_zero
#print axioms shifted_evaluation
#print axioms actual_three_short_unit
#print axioms actual_three_short_value
end
end TateMillerEvaluationAudit
