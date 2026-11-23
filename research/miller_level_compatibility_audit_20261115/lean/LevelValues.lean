module
public import LevelFunctions

@[expose] public section
namespace MillerLevelCompatibilityAudit
noncomputable section
open WeierstrassCurve.Affine
open FunctionDivisorFinitenessAudit FinitePointEvaluationAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- The higher final domain is derived from the lower domain and actual torsion. -/
theorem divisibleDomain (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (P : W.Point)
    (n N : ℕ) (hn : n • P = 0) (hN : n ∣ N) (D : W.Point →₀ ℤ)
    (hd : Disjoint D.support (functionDivisor W (millerUnit W P n) (millerUnit W P n).ne_zero).support) :
    Disjoint D.support (functionDivisor W (millerUnit W P N) (millerUnit W P N).ne_zero).support := by
  apply (disjoint_principal_support_iff W (millerUnit W P N) D).mpr
  rw [miller_unit_divisible W hΔ P n N hn hN]
  exact (supportUnits W D.support).pow_mem
    ((disjoint_principal_support_iff W (millerUnit W P n) D).mp hd) (N / n)

theorem value_divisible (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (P : W.Point)
    (n N : ℕ) (hn : n • P = 0) (hN : n ∣ N) (D : W.Point →₀ ℤ)
    (hd : Disjoint D.support (functionDivisor W (millerUnit W P n) (millerUnit W P n).ne_zero).support) :
    evaluateDivisor W D (millerUnit W P N) (divisibleDomain W hΔ P n N hn hN D hd) =
      evaluateDivisor W D (millerUnit W P n) hd ^ (N / n) := by
  let f : supportUnits W D.support := ⟨millerUnit W P N,
    (disjoint_principal_support_iff W (millerUnit W P N) D).mp (divisibleDomain W hΔ P n N hn hN D hd)⟩
  have he : f = finalLift W P n D hd ^ (N / n) := by
    apply Subtype.ext
    exact miller_unit_divisible W hΔ P n N hn hN
  change divisorEvaluation W D f = divisorEvaluation W D (finalLift W P n D hd) ^ (N / n)
  rw [he, map_pow]

theorem exponent_product (n N M : ℕ) (hn : 0 < n) (hN : 0 < N) (hnN : n ∣ N) (hNM : N ∣ M) :
    (N / n) * (M / N) = M / n := by
  obtain ⟨a, rfl⟩ := hnN
  obtain ⟨b, rfl⟩ := hNM
  rw [Nat.mul_div_cancel_left a hn, Nat.mul_div_cancel_left b hN, Nat.mul_assoc,
    Nat.mul_div_cancel_left (a * b) hn]

theorem power_compatibility {G : Type*} [Monoid G] (v : G) (n N M : ℕ)
    (hn : 0 < n) (hN : 0 < N) (hnN : n ∣ N) (hNM : N ∣ M) :
    (v ^ (N / n)) ^ (M / N) = v ^ (M / n) := by
  rw [← pow_mul, exponent_product n N M hn hN hnN hNM]

/-- Actual chosen-function evaluation core of Exercise26.3.8.
No canonical Tate pairing or representative-independence premise is supplied. -/
theorem final_exponent_compatibility (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) (n N M : ℕ) (hn : 0 < n) (hN : 0 < N)
    (hP : n • P = 0) (hnN : n ∣ N) (hNM : N ∣ M) (D : W.Point →₀ ℤ)
    (hd : Disjoint D.support (functionDivisor W (millerUnit W P n) (millerUnit W P n).ne_zero).support) :
    evaluateDivisor W D (millerUnit W P N) (divisibleDomain W hΔ P n N hP hnN D hd) ^ (M / N) =
      evaluateDivisor W D (millerUnit W P n) hd ^ (M / n) := by
  rw [value_divisible W hΔ P n N hP hnN D hd]
  exact power_compatibility _ n N M hn hN hnN hNM

#print axioms value_divisible
#print axioms exponent_product
#print axioms final_exponent_compatibility
end
end MillerLevelCompatibilityAudit
