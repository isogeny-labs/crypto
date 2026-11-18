module
public import InfinityEvaluation

@[expose] public section
namespace GeometricPrincipalDivisorAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit
open FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K] [IsAlgClosed K]

/-- Actual no-poles functions are constants, derived from actual degree zero and the
actual infinity residue, without assuming a global-regularity classification. -/
theorem no_poles_constant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    {f : W.FunctionField} (hf : f ≠ 0) (hn : NoPoles W f) :
    ∃ c : K, c ≠ 0 ∧ f = constantHom W c := by
  classical
  have hv : functionValuation W f ≤ 1 := by
    have he := (point_order_nonnegative_iff W hf 0).mp (hn 0)
    change chartAdicValuation W f ≤ 1 at he
    rwa [chart_adic_eq_norm] at he
  obtain ⟨c, hc⟩ := infinity_residue_constant W f hv
  have he : f = constantHom W c := by
    by_contra hne
    have hg : f - constantHom W c ≠ 0 := sub_ne_zero.mpr hne
    have hng := no_poles_sub_constant W hf hn c hg
    have hd := function_divisor_degree_zero W hΔ hg
    have hz := nonnegative_degree_zero W (functionDivisor W (f - constantHom W c) hg) hng hd
    have ho := congrArg (fun D : W.Point →₀ ℤ => D 0) hz
    change pointOrder W (f - constantHom W c) 0 = 0 at ho
    have hp : 0 < pointOrder W (f - constantHom W c) 0 := by
      apply (point_order_positive_iff W hg 0).mpr
      change chartAdicValuation W (f - constantHom W c) < 1
      rw [chart_adic_eq_norm]
      exact hc
    omega
  refine ⟨c, ?_, he⟩
  intro hc0
  exact hf (he.trans (by rw [hc0, map_zero]))

theorem no_poles_iff_constant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    {f : W.FunctionField} (hf : f ≠ 0) :
    NoPoles W f ↔ ∃ c : K, c ≠ 0 ∧ f = constantHom W c := by
  constructor
  · exact no_poles_constant W hΔ hf
  · rintro ⟨c, hc, rfl⟩ P
    rw [constant_point_order W hc P]

theorem divisor_zero_iff_constant (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    {f : W.FunctionField} (hf : f ≠ 0) :
    functionDivisor W f hf = 0 ↔ ∃ c : K, c ≠ 0 ∧ f = constantHom W c := by
  constructor
  · intro hz
    apply no_poles_constant W hΔ hf
    intro P
    have he := congrArg (fun D : W.Point →₀ ℤ => D P) hz
    change pointOrder W f P = 0 at he
    rw [he]
  · rintro ⟨c, hc, he⟩
    exact (function_divisor_congr W hf (constant_function_nonzero W hc) he).trans
      (constant_function_divisor W hc)

theorem same_divisor_iff_scalar (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    {f g : W.FunctionField} (hf : f ≠ 0) (hg : g ≠ 0) :
    functionDivisor W f hf = functionDivisor W g hg ↔
      ∃ c : K, c ≠ 0 ∧ f = constantHom W c * g := by
  constructor
  · intro hfg
    have hz : functionDivisor W (f / g) (div_ne_zero hf hg) = 0 := by
      rw [function_divisor_div W hf hg, hfg, sub_self]
    obtain ⟨c, hc, he⟩ := (divisor_zero_iff_constant W hΔ (div_ne_zero hf hg)).mp hz
    exact ⟨c, hc, (div_eq_iff hg).mp he⟩
  · rintro ⟨c, hc, he⟩
    have hD := function_divisor_congr W hf (mul_ne_zero (constant_function_nonzero W hc) hg) he
    rw [function_divisor_mul W (constant_function_nonzero W hc) hg,
      constant_function_divisor W hc, zero_add] at hD
    exact hD

#print axioms no_poles_constant
#print axioms divisor_zero_iff_constant
#print axioms same_divisor_iff_scalar
end
end GeometricPrincipalDivisorAudit
