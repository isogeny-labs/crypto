import AuxiliaryLocal
import TameImports

namespace EllipticTameSymbolAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open scoped WithZero
variable {K : Type*} [Field K]

theorem order_zpow (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (n : ℤ) :
    pointOrder W (f^n : W.FunctionFieldˣ) P=n*pointOrder W f P := by
  rw [Units.val_zpow_eq_zpow_val,point_order_formula,point_order_formula,
    map_zpow₀,WithZero.log_zpow]
  simp only [smul_eq_mul]
  ring

/-- The actual field-unit word with cancelling actual local orders. -/
def balancedUnit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) : W.FunctionFieldˣ :=
  f^pointOrder W g P/g^pointOrder W f P

theorem balanced_order (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) : pointOrder W (balancedUnit W P f g) P=0 := by
  unfold balancedUnit
  rw [Units.val_div_eq_div_val,point_order_div W (Units.ne_zero _) (Units.ne_zero _),
    order_zpow,order_zpow]
  ring

/-- The balanced word in the actual local-ring unit subgroup, with membership derived. -/
def balancedLocalUnit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) : (RegularRing W P).unitGroup :=
  ⟨balancedUnit W P f g,(mem_point_units_iff W P _).mpr (balanced_order W P f g)⟩

/-- Classical tame-symbol convention: a^ord(b) / b^ord(a), with the necessary sign.
Evaluation is on the proved local unit, even at common zeros and poles. -/
def tameSymbol (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) : Kˣ :=
  (-1 : Kˣ)^(pointOrder W f P*pointOrder W g P)*
    pointUnitEvaluation W P (balancedLocalUnit W P f g)

/-- Corrected Gille–Szamuely/Milnor basis convention, inverse of the classical formula. -/
def milnorSymbol (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) : Kˣ := (tameSymbol W P f g)⁻¹

theorem local_value_one (W : WeierstrassCurve.Affine K) (P : W.Point) :
    localUnitValue W P 1 (point_order_one W P)=1 :=
  (pointUnitEvaluation W P).map_one

theorem local_value_inv (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (hf : pointOrder W f P=0) :
    localUnitValue W P f⁻¹ (by
      simpa only [zpow_neg_one] using (order_zpow W P f (-1)).trans (by rw [hf];ring))=
      (localUnitValue W P f hf)⁻¹ :=
  (pointUnitEvaluation W P).map_inv ⟨f,(mem_point_units_iff W P f).mpr hf⟩

theorem local_value_zpow (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (hf : pointOrder W f P=0) (n : ℤ) :
    localUnitValue W P (f^n) (by rw [order_zpow,hf,mul_zero])=
      (localUnitValue W P f hf)^n :=
  (pointUnitEvaluation W P).map_zpow ⟨f,(mem_point_units_iff W P f).mpr hf⟩ n

theorem local_value_constant (W : WeierstrassCurve.Affine K) (P : W.Point) (c : Kˣ) :
    localUnitValue W P (Units.map (constantHom W).toMonoidHom c)
      (constant_point_order W c.ne_zero P)=c := by
  apply Units.ext
  change regularEvaluation W P (regularConstant W P (c : K))=(c : K)
  exact regular_eval_constant W P c

theorem tame_units (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) (hf : pointOrder W f P=0) (hg : pointOrder W g P=0) :
    tameSymbol W P f g=1 := by
  have hb : balancedLocalUnit W P f g=1 := by
    apply Subtype.ext
    simp [balancedLocalUnit,balancedUnit,hf,hg]
  simp only [tameSymbol,hf,hg,mul_zero,zpow_zero,one_mul]
  simp only [hb,map_one]

#print axioms balanced_order
#print axioms local_value_zpow
#print axioms tame_units
end
end EllipticTameSymbolAudit
