module
public import TranslationEvaluation

@[expose] public section
namespace LocalTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerNormValuationAudit MillerFinitePointAudit
open MillerPrincipalIdealAudit MillerBasechangeAudit NegationPullbackAudit
open FunctionDivisorFinitenessAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev SignBase := ZMod 5
noncomputable abbrev signBaseCommRing : CommRing SignBase := (inferInstance : Field SignBase).toCommRing
attribute [local instance] signBaseCommRing
abbrev SignClosure := AlgebraicClosure SignBase
noncomputable abbrev signClosureCommRing : CommRing SignClosure := (inferInstance : Field SignClosure).toCommRing
attribute [local instance] signClosureCommRing
local instance : DecidableEq SignClosure := Classical.decEq _

/-- Actual scalar extension of the credited F5 order-three control. -/
abbrev signCurve : WeierstrassCurve.Affine SignClosure := extendedCurve controlCurve

theorem sign_delta : signCurve.Δ≠0 := extended_delta_nonzero controlCurve control_delta

theorem sign_actual_char5 : CharP SignClosure 5 := inferInstance

def signPoint : signCurve.Point := pointBaseChange controlCurve controlPoint

theorem sign_nonsingular : signCurve.Nonsingular 0 1 := by
  have h := base_nonsingular (L := SignClosure) controlCurve control_positive_nonsingular
  simpa only [map_zero,map_one] using h

theorem sign_point_eq : signPoint=Point.some 0 1 sign_nonsingular := by
  change Point.some (algebraMap SignBase SignClosure 0) (algebraMap SignBase SignClosure 1) _=
    Point.some 0 1 sign_nonsingular
  simp only [map_zero,map_one]

theorem sign_three : (3 : ℕ) • signPoint=0 := by
  change (3 : ℕ) • pointBaseChange controlCurve controlPoint=0
  rw [← map_nsmul]
  have ht : (3 : ℕ) • controlPoint=0 := control_positive_three
  rw [ht,map_zero]

theorem sign_point_nonzero : signPoint≠0 := by
  rw [sign_point_eq]
  exact Point.some_ne_zero sign_nonsingular

theorem sign_point_ne_negative : signPoint≠-signPoint := by
  intro he
  apply control_point_ne_negative
  apply point_base_injective (L := SignClosure) controlCurve
  simpa only [signPoint,map_neg] using he

def signFunction : signCurve.FunctionField :=
  MillerNormValuationAudit.lineFunction signCurve 0 1 (signCurve.slope 0 0 1 1)

theorem sign_function_nonzero : signFunction≠0 := MillerPrincipalIdealAudit.line_function_nonzero signCurve 0 1 _

theorem sign_function_divisor : functionDivisor signCurve signFunction sign_function_nonzero=
    Finsupp.single signPoint 3-Finsupp.single 0 3 := by
  have ht : (3 : ℕ) • Point.some 0 1 sign_nonsingular=0 := by
    rw [← sign_point_eq]
    exact sign_three
  have hd := three_tangent_principal_point_divisor signCurve sign_nonsingular ht
  rw [← sign_point_eq] at hd
  apply Finsupp.ext
  intro R
  change pointOrder signCurve signFunction R=_
  exact congrArg (fun D : signCurve.Point →₀ ℤ => D R) hd

theorem sign_point_order_three : pointOrder signCurve signFunction signPoint=3 := by
  have he := congrArg (fun D : signCurve.Point →₀ ℤ => D signPoint) sign_function_divisor
  change pointOrder signCurve signFunction signPoint=_ at he
  simpa [sign_point_nonzero] using he

theorem sign_negative_order_zero : pointOrder signCurve signFunction (-signPoint)=0 := by
  have he := congrArg (fun D : signCurve.Point →₀ ℤ => D (-signPoint)) sign_function_divisor
  change pointOrder signCurve signFunction (-signPoint)=_ at he
  have hn : (0 : signCurve.Point)≠-signPoint := (neg_ne_zero.mpr sign_point_nonzero).symm
  simpa [sign_point_ne_negative,hn] using he

theorem sign_pullback_order_three :
    pointOrder signCurve (fieldTranslation signCurve sign_delta signPoint signFunction) 0=3 := by
  rw [translation_point_order,zero_add,sign_point_order_three]

/-- Concrete actual ACF5 counterexample to using positive Q for field pullback's divisor. -/
theorem wrong_divisor_translation_sign :
    functionDivisor signCurve (fieldTranslation signCurve sign_delta signPoint signFunction)
      (translation_nonzero signCurve sign_delta signPoint sign_function_nonzero)≠
    divisorTranslation signCurve signPoint (functionDivisor signCurve signFunction sign_function_nonzero) := by
  intro he
  have ho := congrArg (fun D : signCurve.Point →₀ ℤ => D 0) he
  rw [function_divisor_apply,divisor_translation_apply,zero_sub,function_divisor_apply,
    sign_pullback_order_three,sign_negative_order_zero] at ho
  norm_num at ho

#print axioms sign_function_divisor
#print axioms wrong_divisor_translation_sign
end
end LocalTranslationAudit
