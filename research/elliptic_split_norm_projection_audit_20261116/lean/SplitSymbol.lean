import SplitFiber

namespace EllipticSplitNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

theorem point_base_order_mul (W : WeierstrassCurve.Affine K) (P : W.Point)
    (a b : (RatFunc K)ˣ) :
    pointBaseOrder W P ((a*b : (RatFunc K)ˣ) : RatFunc K)=
      pointBaseOrder W P (a : RatFunc K)+pointBaseOrder W P (b : RatFunc K) :=
  valuation_order_mul (pointBaseValuation W P) a.ne_zero b.ne_zero

theorem point_base_order_div (W : WeierstrassCurve.Affine K) (P : W.Point)
    (a b : (RatFunc K)ˣ) :
    pointBaseOrder W P ((a/b : (RatFunc K)ˣ) : RatFunc K)=
      pointBaseOrder W P (a : RatFunc K)-pointBaseOrder W P (b : RatFunc K) := by
  rw [Units.val_div_eq_div_val]
  exact valuation_order_div (pointBaseValuation W P) a.ne_zero b.ne_zero

theorem point_base_order_zpow (W : WeierstrassCurve.Affine K) (P : W.Point)
    (a : (RatFunc K)ˣ) (n : ℤ) :
    pointBaseOrder W P ((a^n : (RatFunc K)ˣ) : RatFunc K)=n*pointBaseOrder W P (a : RatFunc K) := by
  rw [Units.val_zpow_eq_zpow_val,pointBaseOrder,pointBaseOrder,map_zpow₀,WithZero.log_zpow]
  simp only [smul_eq_mul]
  ring

theorem point_base_order_zero_iff (W : WeierstrassCurve.Affine K) (P : W.Point)
    (a : (RatFunc K)ˣ) :
    pointBaseOrder W P (a : RatFunc K)=0 ↔ pointBaseValuation W P (a : RatFunc K)=1 := by
  have hv : pointBaseValuation W P (a : RatFunc K)≠0 := (Valuation.ne_zero_iff _).mpr a.ne_zero
  rw [pointBaseOrder,neg_eq_zero]
  constructor
  · intro hz
    rw [←WithZero.exp_log hv,hz,WithZero.exp_zero]
  · intro he
    rw [he,WithZero.log_one]

theorem mem_point_base_units_iff (W : WeierstrassCurve.Affine K) (P : W.Point)
    (a : (RatFunc K)ˣ) :
    a∈(PointBaseRing W P).unitGroup ↔ pointBaseOrder W P (a : RatFunc K)=0 := by
  rw [Valuation.mem_unitGroup_iff]
  exact (point_base_order_zero_iff W P a).symm

def pointBaseBalancedUnit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (a b : (RatFunc K)ˣ) : (RatFunc K)ˣ :=
  a^pointBaseOrder W P (b : RatFunc K)/b^pointBaseOrder W P (a : RatFunc K)

theorem point_base_balanced_order (W : WeierstrassCurve.Affine K) (P : W.Point)
    (a b : (RatFunc K)ˣ) : pointBaseOrder W P (pointBaseBalancedUnit W P a b : RatFunc K)=0 := by
  unfold pointBaseBalancedUnit
  rw [point_base_order_div,point_base_order_zpow,point_base_order_zpow]
  ring

def pointBaseBalancedLocalUnit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (a b : (RatFunc K)ˣ) : (PointBaseRing W P).unitGroup :=
  ⟨pointBaseBalancedUnit W P a b,(mem_point_base_units_iff W P _).mpr (point_base_balanced_order W P a b)⟩

/-- Actual classical symbol for the actual restricted base valuation; normalization is separate. -/
def pointBaseTameSymbol (W : WeierstrassCurve.Affine K) (P : W.Point)
    (a b : (RatFunc K)ˣ) : Kˣ :=
  (-1 : Kˣ)^(pointBaseOrder W P (a : RatFunc K)*pointBaseOrder W P (b : RatFunc K))*
    pointBaseUnitEvaluation W P (pointBaseBalancedLocalUnit W P a b)

theorem point_base_balanced_image (W : WeierstrassCurve.Affine K) (P : W.Point)
    (a b : (RatFunc K)ˣ) :
    pointBaseUnitEmbedding W P (pointBaseBalancedLocalUnit W P a b)=
      balancedLocalUnit W P (baseFieldUnit W a) (baseFieldUnit W b) := by
  apply Subtype.ext
  change baseFieldUnit W (a^pointBaseOrder W P (b : RatFunc K)/b^pointBaseOrder W P (a : RatFunc K))=
    (baseFieldUnit W a)^pointOrder W (baseFieldUnit W b) P/
      (baseFieldUnit W b)^pointOrder W (baseFieldUnit W a) P
  rw [map_div,map_zpow,map_zpow,point_base_order_embedding,point_base_order_embedding]
  rfl

theorem point_base_tame_embedding (W : WeierstrassCurve.Affine K) (P : W.Point)
    (a b : (RatFunc K)ˣ) :
    pointBaseTameSymbol W P a b=tameSymbol W P (baseFieldUnit W a) (baseFieldUnit W b) := by
  rw [pointBaseTameSymbol,point_base_unit_eval_compatibility,point_base_balanced_image,tameSymbol,
    point_base_order_embedding,point_base_order_embedding]
  rfl

/-- Raw identity at the actual restricted valuation. At a fixed point this is still a square;
only the separately derived split normalization gives the normalized two-point projection. -/
theorem point_base_norm_orbit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    pointBaseTameSymbol W P (normUnit W f) c=
      tameSymbol W P f (baseFieldUnit W c)*tameSymbol W (-P) f (baseFieldUnit W c) := by
  rw [point_base_tame_embedding,base_field_norm_unit]
  exact actual_rational_norm_tame_orbit W P f c

theorem point_base_inverse_norm_orbit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (c : (RatFunc K)ˣ) :
    (pointBaseTameSymbol W P (normUnit W f) c)⁻¹=
      milnorSymbol W P f (baseFieldUnit W c)*milnorSymbol W (-P) f (baseFieldUnit W c) := by
  rw [point_base_norm_orbit,mul_inv_rev,milnorSymbol,milnorSymbol,mul_comm]

/-- Full actual split fiber and normalization are proved alongside exact norm projection. -/
theorem actual_split_norm_projection (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)≠ -Point.some x y h) :
    (xFiber W (Point.some x y h)={Point.some x y h,-Point.some x y h} ∧
      (xFiber W (Point.some x y h)).ncard=2) ∧
    (pointBaseOrder W (Point.some x y h) (baseVerticalUnit x : RatFunc K)=1 ∧
      pointBaseValuation W (Point.some x y h) (baseVerticalUnit x : RatFunc K)=WithZero.exp (-1 : ℤ) ∧
      Function.Surjective (pointBaseValuation W (Point.some x y h))) ∧
    (∀ f : W.FunctionFieldˣ,∀ c : (RatFunc K)ˣ,
      pointBaseTameSymbol W (Point.some x y h) (normUnit W f) c=
        tameSymbol W (Point.some x y h) f (baseFieldUnit W c)*tameSymbol W (-Point.some x y h) f (baseFieldUnit W c) ∧
      (pointBaseTameSymbol W (Point.some x y h) (normUnit W f) c)⁻¹=
        milnorSymbol W (Point.some x y h) f (baseFieldUnit W c)*milnorSymbol W (-Point.some x y h) f (baseFieldUnit W c)) := by
  refine ⟨⟨actual_x_fiber W _,actual_split_fiber_card W _ hP⟩,
    ⟨split_base_parameter_order W h hP,split_base_parameter_value W h hP,split_base_valuation_surjective W h hP⟩,?_⟩
  intro f c
  exact ⟨point_base_norm_orbit W _ f c,point_base_inverse_norm_orbit W _ f c⟩

#print axioms point_base_tame_embedding
#print axioms point_base_norm_orbit
#print axioms actual_split_norm_projection
end
end EllipticSplitNormProjectionAudit
