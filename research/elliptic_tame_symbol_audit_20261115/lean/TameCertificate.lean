import TameDisjoint
import TameGeometricControls

namespace EllipticTameSymbolAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit

/-- Actual local tame-symbol laws and finite products, with explicit source convention,
parameter independence, proper disjoint evaluation and concrete omitted-condition controls.
No general global reciprocity or norm-projection premise is supplied. -/
theorem actual_elliptic_tame_certificate
    (K : Type*) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    (∀ (P : W.Point) (f g h : W.FunctionFieldˣ),
      pointOrder W (balancedUnit W P f g) P=0 ∧
      tameSymbol W P (f*g) h=tameSymbol W P f h*tameSymbol W P g h ∧
      tameSymbol W P f (g*h)=tameSymbol W P f g*tameSymbol W P f h ∧
      tameSymbol W P g f=(tameSymbol W P f g)⁻¹ ∧
      tameSymbol W P f f=(-1 : Kˣ)^pointOrder W f P) ∧
    (∀ (P : W.Point) (f g : W.FunctionFieldˣ),
      (f : W.FunctionField)+(g : W.FunctionField)=1 → tameSymbol W P f g=1) ∧
    (∀ P : W.Point,pointOrder W (pointParameterUnit W hΔ P) P=1) ∧
    (∀ (P : W.Point) (π ρ : W.FunctionFieldˣ)
      (hπ : pointOrder W π P=1) (hρ : pointOrder W ρ P=1) (f g : W.FunctionFieldˣ),
      tameSymbol W P f g=parameterSymbol W P π hπ f g ∧
      parameterSymbol W P π hπ f g=parameterSymbol W P ρ hρ f g) ∧
    (∀ f g : W.FunctionFieldˣ,Function.HasFiniteMulSupport (fun P => tameSymbol W P f g)) ∧
    (∀ (c : Kˣ) (g : W.FunctionFieldˣ),
      globalTameProduct W (coefficientUnit W c) g=c^divisorDegree W (functionDivisor W g g.ne_zero)) ∧
    (∀ (f g : W.FunctionFieldˣ)
      (hd : Disjoint (functionDivisor W f f.ne_zero).support (functionDivisor W g g.ne_zero).support),
      globalTameProduct W f g=
        evaluateDivisor W (functionDivisor W g g.ne_zero) f hd.symm/
          evaluateDivisor W (functionDivisor W f f.ne_zero) g hd) ∧
    (IsAlgClosed K → ∀ (c : Kˣ) (f : W.FunctionFieldˣ),
      globalTameProduct W (coefficientUnit W c) f=1 ∧ globalTameProduct W f f=1) ∧
    (tameCurve.Δ≠0 ∧
      tameSymbol tameCurve 0 (infinityParameterUnit tameCurve) (coefficientUnit tameCurve twoUnit)≠
        milnorSymbol tameCurve 0 (infinityParameterUnit tameCurve) (coefficientUnit tameCurve twoUnit)) ∧
    (∃ f g : tameCurve.FunctionFieldˣ,
      (f : tameCurve.FunctionField)+(g : tameCurve.FunctionField)=1 ∧
      pointOrder tameCurve f 0= -1 ∧ pointOrder tameCurve g 0= -1 ∧
      tameSymbol tameCurve 0 f g=1 ∧ unsignedSymbol tameCurve 0 f g≠1) ∧
    ((globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit : ControlBase)=4 ∧
      globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit≠1) ∧
    (globalTameProduct geometricTameCurve (coefficientUnit geometricTameCurve geometricTwo)
      geometricMissingRoot=1) := by
  refine ⟨?_,tame_steinberg W,point_parameter_order W hΔ,?_,tame_support_finite W,
    global_constant_degree W,global_disjoint_evaluation W,?_,?_,f5_missing_sign_counter.2,
    f5_rational_only_product_counter.2,matched_geometric_product_one.2⟩
  · intro P f g h
    exact ⟨balanced_order W P f g,tame_mul_left W P f g h,tame_mul_right W P f g h,
      tame_skew W P f g,tame_diagonal W P f⟩
  · intro P π ρ hπ hρ f g
    exact ⟨tame_parameter_formula W P π hπ f g,parameter_independence W P π ρ hπ hρ f g⟩
  · intro hK c f
    let _ : IsAlgClosed K := hK
    exact ⟨geometric_constant_product W hΔ c f,geometric_diagonal_product W hΔ f⟩
  · exact ⟨f5_convention_counter.1,f5_convention_counter.2.1⟩

#print axioms actual_elliptic_tame_certificate
end
end EllipticTameSymbolAudit
