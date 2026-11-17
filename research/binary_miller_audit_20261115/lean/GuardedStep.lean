module
public import BinaryState

@[expose] public section
namespace BinaryMillerAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerRecursionAudit FunctionDivisorFinitenessAudit FinitePointEvaluationAudit
variable {K : Type*} [Field K] [DecidableEq K]

omit [DecidableEq K] in
theorem step_line_nonzero (W : WeierstrassCurve.Affine K) (x y s : K) :
    MillerNormValuationAudit.lineFunction W x y s ≠ 0 := by
  apply (functionValuation W).ne_zero_iff.mp
  rw [function_valuation_line]
  exact WithZero.exp_ne_zero

/-- Actual source numerator/denominator functions, with all zero/vertical exceptions retained. -/
def stepFactors (W : WeierstrassCurve.Affine K) : W.Point → W.Point → W.FunctionFieldˣ × W.FunctionFieldˣ
  | .zero, _ => (1, 1)
  | .some _ _ _, .zero => (1, 1)
  | .some x₁ y₁ _, .some x₂ y₂ _ =>
    if x₁ = x₂ ∧ y₁ = W.negY x₂ y₂ then
      (Units.mk0 (verticalFunction W x₁) (vertical_nonzero W x₁), 1)
    else
      (Units.mk0 (MillerNormValuationAudit.lineFunction W x₁ y₁ (W.slope x₁ x₂ y₁ y₂))
        (step_line_nonzero W x₁ y₁ _),
       Units.mk0 (verticalFunction W (W.addX x₁ x₂ (W.slope x₁ x₂ y₁ y₂))) (vertical_nonzero W _))

def stepUnit (W : WeierstrassCurve.Affine K) (P Q : W.Point) : W.FunctionFieldˣ :=
  (stepFactors W P Q).1 / (stepFactors W P Q).2

theorem step_unit_field (W : WeierstrassCurve.Affine K) (P Q : W.Point) :
    (stepUnit W P Q : W.FunctionField) = stepFunction W P Q := by
  rw [stepUnit, Units.val_div_eq_div_val]
  cases P with
  | zero => cases Q <;> simp [stepFactors, stepFunction]
  | some x₁ y₁ h₁ =>
    cases Q with
    | zero => simp [stepFactors, stepFunction]
    | some x₂ y₂ h₂ =>
      simp only [stepFactors, stepFunction]
      split_ifs <;> simp

/-- Each actual numerator and denominator must be a local unit at every divisor-support point. -/
def FactorsAdmissible (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) (P Q : W.Point) : Prop :=
  Disjoint D.support (functionDivisor W (stepFactors W P Q).1 (stepFactors W P Q).1.ne_zero).support ∧
  Disjoint D.support (functionDivisor W (stepFactors W P Q).2 (stepFactors W P Q).2.ne_zero).support

def numeratorLift (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) (P Q : W.Point)
    (h : FactorsAdmissible W D P Q) : supportUnits W D.support :=
  ⟨(stepFactors W P Q).1, (disjoint_principal_support_iff W (stepFactors W P Q).1 D).mp h.1⟩

def denominatorLift (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) (P Q : W.Point)
    (h : FactorsAdmissible W D P Q) : supportUnits W D.support :=
  ⟨(stepFactors W P Q).2, (disjoint_principal_support_iff W (stepFactors W P Q).2 D).mp h.2⟩

def liftedStep (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) (P Q : W.Point)
    (h : FactorsAdmissible W D P Q) : supportUnits W D.support :=
  numeratorLift W D P Q h / denominatorLift W D P Q h

theorem lifted_step_unit (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) (P Q : W.Point)
    (h : FactorsAdmissible W D P Q) : (liftedStep W D P Q h : W.FunctionFieldˣ) = stepUnit W P Q := rfl

theorem lifted_step_field (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) (P Q : W.Point)
    (h : FactorsAdmissible W D P Q) :
    ((liftedStep W D P Q h : W.FunctionFieldˣ) : W.FunctionField) = stepFunction W P Q :=
  step_unit_field W P Q

def stepValue (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) (P Q : W.Point)
    (h : FactorsAdmissible W D P Q) : Kˣ :=
  evaluateDivisor W D (stepFactors W P Q).1 h.1 / evaluateDivisor W D (stepFactors W P Q).2 h.2

theorem lifted_step_value (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) (P Q : W.Point)
    (h : FactorsAdmissible W D P Q) : divisorEvaluation W D (liftedStep W D P Q h) = stepValue W D P Q h :=
  (divisorEvaluation W D).map_div (numeratorLift W D P Q h) (denominatorLift W D P Q h)

theorem guarded_doubling_value (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) (P : W.Point)
    (h : FactorsAdmissible W D P P) (f : supportUnits W D.support) :
    divisorEvaluation W D (f * f * liftedStep W D P P h) =
      divisorEvaluation W D f * divisorEvaluation W D f * stepValue W D P P h := by
  rw [map_mul, map_mul, lifted_step_value]

theorem guarded_addition_value (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) (P Q : W.Point)
    (h : FactorsAdmissible W D P Q) (f : supportUnits W D.support) :
    divisorEvaluation W D (f * liftedStep W D P Q h) =
      divisorEvaluation W D f * stepValue W D P Q h := by
  rw [map_mul, lifted_step_value]

#print axioms step_unit_field
#print axioms lifted_step_value
#print axioms guarded_doubling_value
end
end BinaryMillerAudit
