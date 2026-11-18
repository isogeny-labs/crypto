import TateMillerNineteen

namespace TateMillerEvaluationAudit
noncomputable section
open WeierstrassCurve.Affine MillerFinitePointAudit MillerRecursionAudit
open MillerNormValuationAudit WeilRationalDomainAudit MillerBasechangeAudit
open WeilAuxiliaryRatioAudit FinitePointEvaluationAudit FunctionDivisorFinitenessAudit

def ActualChosenBridge : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (W : WeierstrassCurve.Affine K)
    (_hΔ : W.Δ≠0) (x y : K) (hP : W.Nonsingular x y),
    (3 : ℕ) • Point.some x y hP=0 →
      millerFunction W (.some x y hP) 3=lineFunction W x y (W.slope x x y y)

def ActualShortResidueBridge : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (W : WeierstrassCurve.Affine K)
    (_hΔ : W.Δ≠0) (_ha1 : W.a₁=0) (_ha2 : W.a₂=0) (_ha3 : W.a₃=0)
    (x y u v : K) (hP : W.Nonsingular x y)
    (_h3 : (3 : ℕ) • Point.some x y hP=0) (hR : W.Nonsingular u v)
    (hu : pointOrder W (chosenFunctionUnit W (.some x y hP) 3) (.some u v hR)=0),
    (localUnitValue W (.some u v hR) (chosenFunctionUnit W (.some x y hP) 3) hu : K)=
      shortLine W x y u v

def ActualShiftedValueBridge : Prop:=
  ∀ (K : Type) (_ : Field K) (_ : DecidableEq K) (W : WeierstrassCurve.Affine K)
    (f : W.FunctionFieldˣ) (A B : W.Point) (hA : pointOrder W f A=0) (hB : pointOrder W f B=0),
    GeometricPrincipalDivisorAudit.divisorDegree W (shiftedDivisor W A B)=0 ∧
    evaluateDivisor W (shiftedDivisor W A B) f (shifted_disjoint W f A B hA hB)=
      localUnitValue W A f hA/localUnitValue W B f hB

def ActualMovingBridge : Prop:=
  ∀ (K L : Type) (_ : Field K) (_ : Field L) (_ : Algebra K L)
    (_ : DecidableEq K) (_ : DecidableEq L) (W : WeierstrassCurve.Affine K) (S Q : W.Point),
    shiftedDivisor (extendedCurve (L:=L) W)
        (pointBaseChange (L:=L) W S+pointBaseChange (L:=L) W Q) (pointBaseChange (L:=L) W S)+
      functionDivisor (extendedCurve (L:=L) W) (fieldBaseChange (L:=L) W (stepFunction W S Q))
        (field_base_nonzero W (step_function_nonzero W S Q))=
      Finsupp.single (pointBaseChange (L:=L) W Q) 1-Finsupp.single 0 1

def ActualNineteenPresentation : Prop:=
  Curve.Δ≠0 ∧ P=(6 : ℕ) • G ∧ (3 : ℕ) • P=0 ∧ P≠0 ∧
    S=(3 : ℕ) • G ∧ S+G=A ∧ pointOrder Curve f S=0 ∧ pointOrder Curve f A=0 ∧
    toTateRight Curve 3 G≠0 ∧ (sourceValue : ZMod 19)^6=11 ∧
    readout^3=1 ∧ readout≠1 ∧ directReadout=11 ∧ readout=directReadout ∧
    Nat.Coprime 3 19 ∧ 3∣19-1 ∧ (¬∃ u : (ZMod 19)ˣ,u^3=sourceValue) ∧
    GeometricMiller Curve P 3 (millerFunction Curve P 3) ∧
    Disjoint (shiftedDivisor Curve A S).support (functionDivisor Curve f f.ne_zero).support ∧
    shiftedDivisor Curve A S+
      functionDivisor Curve (stepFunction Curve S G) (step_function_nonzero Curve S G)=
        Finsupp.single G 1-Finsupp.single 0 1

def ActualGeometricSourceSupport : Prop:=by
  classical
  exact Disjoint (shiftedDivisor (extendedCurve (L:=Closure) Curve)
      (pointBaseChange (L:=Closure) Curve A) (pointBaseChange (L:=Closure) Curve S)).support
    (functionDivisor (extendedCurve (L:=Closure) Curve) (extendedChosenUnit (L:=Closure) Curve P 3)
      (extendedChosenUnit (L:=Closure) Curve P 3).ne_zero).support

theorem actual_tate_miller_evaluation_certificate :
    ActualChosenBridge ∧ ActualShortResidueBridge ∧ ActualShiftedValueBridge ∧
      ActualMovingBridge ∧ ActualNineteenPresentation ∧ ActualGeometricSourceSupport := by
  refine ⟨?_,?_,?_,?_,?_,actual_geometric_source_support⟩
  · intro K _ _ W hΔ x y hP h3
    exact actual_chosen_three_tangent W hΔ hP h3
  · intro K _ _ W hΔ ha1 ha2 ha3 x y u v hP h3 hR hu
    exact actual_three_short_value W hΔ ha1 ha2 ha3 hP h3 hR hu
  · intro K _ _ W f A B hA hB
    exact ⟨shifted_degree_zero W A B,shifted_evaluation W f A B hA hB⟩
  · intro K L _ _ _ _ _ W S Q
    exact actual_geometric_shifted_representative W S Q
  · refine ⟨BKGeneralFrobeniusAudit.SourceNineteen.curve_nonsingular,
      actual_coordinates.1,actual_P_three,actual_P_nonzero,actual_coordinates.2.1,
      actual_coordinates.2.2,source_unit_S,source_unit_A,tate_three_generator_class_ne_zero,
      actual_source_readout.trans actual_expression_controls.2.2.2.1,
      actual_expression_controls.2.2.2.2.2.1,actual_expression_controls.2.2.2.2.2.2.1,
      actual_expression_controls.2.2.2.2.1,actual_expression_controls.2.2.2.2.2.2.2,
      by decide,by decide,actual_source_not_cube,
      constructed_geometric_miller Curve BKGeneralFrobeniusAudit.SourceNineteen.curve_nonsingular P 3,
      shifted_disjoint Curve f A S source_unit_A source_unit_S,?_⟩
    rw [←actual_coordinates.2.2]
    exact actual_shifted_representative Curve S G

#print axioms actual_tate_miller_evaluation_certificate
end
end TateMillerEvaluationAudit
