module
public import PicardReduction

@[expose] public section
namespace EllipticPicardBridgeAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

/-- An actual degree-zero divisor is principal exactly when its actual point sum is zero. -/
theorem principal_zero_iff_point_sum (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (D : ZeroDivisors W) : D ∈ principalZero W hΔ ↔ divisorPointSum W D.val=0 := by
  have he := divisor_class_reduction W hΔ D.val
  rw [adjust_degree_zero] at he
  constructor
  · intro hm
    apply point_class_injective W hΔ
    rw [map_zero,he]
    exact (QuotientAddGroup.eq_zero_iff D).mpr hm
  · intro hs
    apply (QuotientAddGroup.eq_zero_iff D).mp
    change divisorClass W hΔ D=0
    rw [← he,hs,map_zero]

/-- Full actual elliptic principal-divisor criterion, with degree and point sum both derived. -/
theorem principal_iff_degree_and_point_sum (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (D : W.Point →₀ ℤ) :
    D ∈ principalPointDivisors W ↔ divisorDegree W D=0 ∧ divisorPointSum W D=0 := by
  constructor
  · rintro ⟨u,hu⟩
    have hd : divisorDegree W D=0 := by
      rw [← hu]
      exact function_divisor_degree_zero W hΔ u.toMul.ne_zero
    have hm : (⟨D,hd⟩ : ZeroDivisors W) ∈ principalZero W hΔ :=
      ⟨u,Subtype.ext hu⟩
    exact ⟨hd,(principal_zero_iff_point_sum W hΔ ⟨D,hd⟩).mp hm⟩
  · rintro ⟨hd,hs⟩
    obtain ⟨u,hu⟩ := (principal_zero_iff_point_sum W hΔ ⟨D,hd⟩).mpr hs
    exact ⟨u,congrArg Subtype.val hu⟩

theorem function_divisor_point_sum_zero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    {f : W.FunctionField} (hf : f ≠ 0) : divisorPointSum W (functionDivisor W f hf)=0 :=
  ((principal_iff_degree_and_point_sum W hΔ _).mp (function_divisor_mem_principal W f hf)).2

/-- Distinct actual geometric points cannot have a principal difference. -/
theorem point_difference_principal_iff (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P Q : W.Point) :
    (∃ (f : W.FunctionField) (hf : f ≠ 0),
      functionDivisor W f hf=Finsupp.single P 1-Finsupp.single Q 1) ↔ P=Q := by
  constructor
  · rintro ⟨f,hf,hD⟩
    have hs := function_divisor_point_sum_zero W hΔ hf
    rw [hD,map_sub,point_sum_single,point_sum_single,one_zsmul,one_zsmul,sub_eq_zero] at hs
    exact hs
  · rintro rfl
    exact ⟨1,one_ne_zero,by rw [function_divisor_one,sub_self]⟩

/-- The actual geometric quotient is nontrivial, witnessed by an actual affine point. -/
theorem actual_picard_nontrivial (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    ∃ P : W.Point, pointClass W hΔ P≠0 := by
  obtain ⟨y,hy⟩ := geometric_fiber W hΔ 0
  refine ⟨Point.some 0 y hy,?_⟩
  intro hz
  apply Point.some_ne_zero hy
  apply point_class_injective W hΔ
  simpa only [map_zero] using hz

#print axioms principal_iff_degree_and_point_sum
#print axioms point_difference_principal_iff
end
end EllipticPicardBridgeAudit
