module
public import FiniteRegular
public import Mathlib.RingTheory.Jacobson.Ring

@[expose] public section
namespace EllipticPicardBridgeAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit MillerFinitePointAudit FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

instance coordinateFiniteType (W : WeierstrassCurve.Affine K) :
    Algebra.FiniteType K W.CoordinateRing :=
  Algebra.FiniteType.trans (inferInstance : Algebra.FiniteType K K[X])
    (inferInstance : Algebra.FiniteType K[X] W.CoordinateRing)

/-- Every actual coefficient-algebra map is ordinary polynomial evaluation on x-polynomials. -/
theorem coordinate_hom_polynomial (W : WeierstrassCurve.Affine K)
    (v : W.CoordinateRing →ₐ[K] K) (p : K[X]) :
    v (mk W (C p)) = p.eval (v (mk W (C X))) := by
  have he : v.toRingHom.comp ((mk W).comp C) = evalRingHom (v (mk W (C X))) := by
    apply Polynomial.ringHom_ext
    · intro c
      simp only [RingHom.comp_apply, coe_evalRingHom, eval_C]
      exact v.commutes c
    · simp only [RingHom.comp_apply, coe_evalRingHom, eval_X,
        AlgHom.toRingHom_eq_coe, AlgHom.coe_toRingHom]
  exact RingHom.congr_fun he p

theorem coordinate_hom_bivariate (W : WeierstrassCurve.Affine K)
    (v : W.CoordinateRing →ₐ[K] K) :
    v.toRingHom.comp (mk W) = evalEvalRingHom (v (mk W (C X))) (v (mk W X)) := by
  apply Polynomial.ringHom_ext
  · intro p
    exact (coordinate_hom_polynomial W v p).trans (evalEval_C _ _ p).symm
  · simp only [RingHom.comp_apply, coe_evalRingHom, eval_X, eval_C,
      AlgHom.toRingHom_eq_coe, AlgHom.coe_toRingHom]

theorem coordinate_hom_equation (W : WeierstrassCurve.Affine K)
    (v : W.CoordinateRing →ₐ[K] K) : W.Equation (v (mk W (C X))) (v (mk W X)) := by
  have he := RingHom.congr_fun (coordinate_hom_bivariate W v) W.polynomial
  change W.polynomial.evalEval _ _ = 0
  simpa only [RingHom.comp_apply, AdjoinRoot.mk_self, map_zero, coe_evalRingHom, evalEval]
    using he.symm

/-- Weak Nullstellensatz on the actual affine coordinate ring, retaining nonsingularity. -/
theorem maximal_ideal_is_actual_point [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (I : Ideal W.CoordinateRing) [I.IsMaximal] :
    ∃ x y : K, ∃ h : W.Nonsingular x y, I = evaluationIdeal W x y h.1 := by
  let : Field (W.CoordinateRing ⧸ I) := Ideal.Quotient.field I
  let : Module.Finite K (W.CoordinateRing ⧸ I) :=
    finite_of_finite_type_of_isJacobsonRing K (W.CoordinateRing ⧸ I)
  let v : W.CoordinateRing →ₐ[K] K :=
    (IsAlgClosed.lift : (W.CoordinateRing ⧸ I) →ₐ[K] K).comp (Ideal.Quotient.mkₐ K I)
  let x := v (mk W (C X))
  let y := v (mk W X)
  have he : W.Equation x y := coordinate_hom_equation W v
  have hn := (W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp he
  refine ⟨x,y,hn,?_⟩
  apply (evaluationMaximal W he).eq_of_le (Ideal.IsMaximal.ne_top ‹I.IsMaximal›) ?_ |>.symm
  rw [evaluation_ideal_coordinates]
  apply Ideal.span_le.mpr
  intro a ha
  simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at ha
  have hv : ∀ a, v a = 0 → a ∈ I := by
    intro a hz
    apply Ideal.Quotient.eq_zero_iff_mem.mp
    apply (IsAlgClosed.lift : (W.CoordinateRing ⧸ I) →ₐ[K] K).injective
    simpa only [v, AlgHom.comp_apply, Ideal.Quotient.mkₐ_eq_mk,
      AlgHom.toRingHom_eq_coe, AlgHom.coe_toRingHom, map_zero] using hz
  rcases ha with rfl | rfl
  · apply hv
    change v (mk W (C (X-C x))) = 0
    rw [coordinate_hom_polynomial]
    simp only [eval_sub, eval_X, eval_C, x, sub_self]
  · apply hv
    change v (mk W (X-C (C y))) = 0
    rw [map_sub, map_sub, coordinate_hom_polynomial W v (C y)]
    simp only [eval_C, y, sub_self]

#print axioms maximal_ideal_is_actual_point
end
end EllipticPicardBridgeAudit
