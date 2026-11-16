module
public import ConstantKernel

@[expose] public section
namespace GeometricPrincipalDivisorAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

theorem infinity_monic_constant (W : WeierstrassCurve.Affine K) {c : K} (hc : c ≠ 0)
    (hm : InfinityMonic W (constantHom W c)) : c = 1 := by
  have ho := constant_point_order W hc 0
  have hs : infinityScaled W (constantHom W c) = constantHom W c := by
    simp only [infinityScaled, ho, neg_zero, zpow_zero, one_mul]
  have hh : functionValuation W (constantHom W (c - 1)) < 1 := by
    simpa only [InfinityMonic, hs, map_sub, map_one] using hm
  by_contra he
  rw [function_valuation_constant W (sub_ne_zero.mpr he)] at hh
  exact lt_irrefl _ hh

theorem normalized_divisor_unique [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    {f g : W.FunctionField} (hf : f ≠ 0) (hg : g ≠ 0)
    (hd : functionDivisor W f hf = functionDivisor W g hg)
    (hmf : InfinityMonic W f) (hmg : InfinityMonic W g) : f = g := by
  have hz : functionDivisor W (f / g) (div_ne_zero hf hg) = 0 := by
    rw [function_divisor_div W hf hg, hd, sub_self]
  obtain ⟨c, hc, he⟩ := (divisor_zero_iff_constant W hΔ (div_ne_zero hf hg)).mp hz
  have hm := infinity_monic_div W hf hg hmf hmg
  rw [he] at hm
  have hc1 := infinity_monic_constant W hc hm
  have hdiv : f / g = 1 := by rw [he, hc1, map_one]
  exact (div_eq_one_iff_eq hg).mp hdiv

variable [DecidableEq K]

/-- Any actual normalized Miller function equals the actually constructed one in geometric scope. -/
theorem miller_function_unique [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) (n : ℕ) {f : W.FunctionField} (hf : IsMillerFunction W P n f) :
    f = millerFunction W P n := by
  apply normalized_divisor_unique W hΔ hf.1.1 (miller_function_nonzero W P n) ?_ hf.2
    (miller_infinity_monic W P n)
  ext Q
  change pointOrder W f Q = pointOrder W (millerFunction W P n) Q
  rw [hf.1.2 Q, (miller_has_point_divisor W P n).2 Q]

/-- Exact chosen-function addition identity, including zero and vertical branches.
No binary implementation or runtime bound is asserted. -/
theorem chosen_miller_addition [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) (i j : ℕ) :
    millerFunction W P (i + j) =
      millerFunction W P i * millerFunction W P j * stepFunction W (i • P) (j • P) := by
  exact (miller_function_unique W hΔ P (i + j)
    (normalized_miller_addition W P i j (constructed_is_miller W P i)
      (constructed_is_miller W P j))).symm

#print axioms normalized_divisor_unique
#print axioms chosen_miller_addition
end
end GeometricPrincipalDivisorAudit
