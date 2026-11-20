module
public import BinaryCertificate

@[expose] public section
namespace MillerLevelCompatibilityAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerRecursionAudit MillerBasechangeAudit FinitePointEvaluationAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- Actual chosen functions, including all exceptional branches and all characteristics. -/
theorem miller_multiple (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) (n : ℕ) (hn : n • P = 0) (a : ℕ) :
    millerFunction W P (a * n) = millerFunction W P n ^ a := by
  induction a with
  | zero =>
      rw [Nat.zero_mul, pow_zero]
      rfl
  | succ a ih =>
      have ha : (a * n) • P = 0 := by rw [Nat.mul_comm a n, mul_nsmul, hn, smul_zero]
      rw [Nat.succ_mul, chosen_miller_addition_all_fields W hΔ P (a * n) n, ih, ha, hn]
      change millerFunction W P n ^ a * millerFunction W P n * 1 = millerFunction W P n ^ (a + 1)
      rw [mul_one, pow_succ]

def millerUnit (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) : W.FunctionFieldˣ :=
  Units.mk0 (millerFunction W P n) (miller_function_nonzero W P n)

theorem miller_unit_multiple (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) (n : ℕ) (hn : n • P = 0) (a : ℕ) :
    millerUnit W P (a * n) = millerUnit W P n ^ a := by
  apply Units.ext
  exact miller_multiple W hΔ P n hn a

theorem miller_unit_divisible (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) (n N : ℕ) (hn : n • P = 0) (hN : n ∣ N) :
    millerUnit W P N = millerUnit W P n ^ (N / n) := by
  simpa only [Nat.div_mul_cancel hN] using miller_unit_multiple W hΔ P n hn (N / n)

/-- An actual final-domain guard yields an actual local-unit-valued final function. -/
def finalLift (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) (D : W.Point →₀ ℤ)
    (hd : Disjoint D.support
      (FunctionDivisorFinitenessAudit.functionDivisor W (millerUnit W P n) (millerUnit W P n).ne_zero).support) :
    supportUnits W D.support :=
  ⟨millerUnit W P n, (disjoint_principal_support_iff W (millerUnit W P n) D).mp hd⟩

/-- Raising an actual final-domain lift computes the actual higher-level function.
No intermediate binary-factor domain is assumed or inferred. -/
theorem final_lift_multiple (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) (n : ℕ) (hn : n • P = 0) (D : W.Point →₀ ℤ)
    (hd : Disjoint D.support
      (FunctionDivisorFinitenessAudit.functionDivisor W (millerUnit W P n) (millerUnit W P n).ne_zero).support)
    (a : ℕ) :
    (((finalLift W P n D hd ^ a : supportUnits W D.support) : W.FunctionFieldˣ) : W.FunctionField) =
      millerFunction W P (a * n) := by
  change millerFunction W P n ^ a = millerFunction W P (a * n)
  exact (miller_multiple W hΔ P n hn a).symm

theorem final_lift_value_power (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ)
    (D : W.Point →₀ ℤ)
    (hd : Disjoint D.support
      (FunctionDivisorFinitenessAudit.functionDivisor W (millerUnit W P n) (millerUnit W P n).ne_zero).support)
    (a : ℕ) :
    divisorEvaluation W D (finalLift W P n D hd ^ a) = divisorEvaluation W D (finalLift W P n D hd) ^ a :=
  map_pow (divisorEvaluation W D) (finalLift W P n D hd) a

#print axioms miller_multiple
#print axioms miller_unit_multiple
#print axioms final_lift_multiple
#print axioms final_lift_value_power
end
end MillerLevelCompatibilityAudit
