import WeightedOverlapEquality

set_option autoImplicit false
set_option maxRecDepth 4096
set_option maxHeartbeats 0
noncomputable section
namespace WeightedOverlapMapEquality
open WeightedOverlapEquality WeightedOverlapFull Common22SeparateChartMap
variable {K : Type*} [Field K]

theorem projTransition_eq_of_scalar (a3 a2 a1 a0 : K)
    (hscalar : ∀ c : K,
      projTransition a3 a2 a1 a0
        (algebraMap K (AffineOpen a3 a2 a1 a0) c) =
      algebraMap K (InfinityOpen a3 a2 a1 a0) c) :
    projTransition a3 a2 a1 a0 =
      (separateChartOpenEquiv a3 a2 a1 a0).toRingEquiv := by
  apply RingEquiv.ext
  intro t
  have heq : (projTransition a3 a2 a1 a0).toRingHom =
      (separateChartOpenEquiv a3 a2 a1 a0).toRingEquiv.toRingHom := by
    apply IsLocalization.ringHom_ext (Submonoid.powers (xbar a3 a2 a1 a0))
    apply Ideal.Quotient.ringHom_ext
    apply MvPolynomial.ringHom_ext
    · intro c
      change projTransition a3 a2 a1 a0
        (algebraMap K (AffineOpen a3 a2 a1 a0) c) =
        (separateChartOpenEquiv a3 a2 a1 a0)
          (algebraMap K (AffineOpen a3 a2 a1 a0) c)
      rw [hscalar]
      exact (separateChartOpenEquiv a3 a2 a1 a0).commutes c |>.symm
    · intro i
      fin_cases i
      · change projTransition a3 a2 a1 a0 (affineX a3 a2 a1 a0) =
          (separateChartOpenEquiv a3 a2 a1 a0) (affineX a3 a2 a1 a0)
        rw [projTransition_affineX]
        exact (affineOpenToInfinityOpen_affineX a3 a2 a1 a0).symm
      · change projTransition a3 a2 a1 a0 (affineY a3 a2 a1 a0) =
          (separateChartOpenEquiv a3 a2 a1 a0) (affineY a3 a2 a1 a0)
        rw [projTransition_affineY]
        exact (affineOpenToInfinityOpen_affineY a3 a2 a1 a0).symm
  exact congrArg (fun h : AffineOpen a3 a2 a1 a0 →+* InfinityOpen a3 a2 a1 a0 => h t) heq

#print axioms projTransition_eq_of_scalar
end WeightedOverlapMapEquality
