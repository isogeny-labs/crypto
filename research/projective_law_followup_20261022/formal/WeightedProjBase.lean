import WeightedProjProper

set_option autoImplicit false
noncomputable section
namespace WeightedProjBase
open MvPolynomial Common22WeightedProjHead Common22WeightedProjQuotient
  WeightedRingComponent WeightedRingGraded WeightedProjCover WeightedProjProper
open AlgebraicGeometry CategoryTheory
variable {K : Type*} [Field K]

def evalOrigin (_a3 _a2 _a1 _a0 : K) : Poly K →ₐ[K] K :=
  MvPolynomial.aeval (fun _ : Fin 3 => (0 : K))

theorem evalOrigin_equation (a3 a2 a1 a0 : K) :
    evalOrigin a3 a2 a1 a0 (equation a3 a2 a1 a0) = 0 := by
  simp [evalOrigin, equation]

def toK (a3 a2 a1 a0 : K) : WeightedRing a3 a2 a1 a0 →ₐ[K] K :=
  Ideal.Quotient.liftₐ (relationIdeal a3 a2 a1 a0)
    (evalOrigin a3 a2 a1 a0) (by
      intro p hp
      rw [relationIdeal, Ideal.mem_span_singleton] at hp
      obtain ⟨q, rfl⟩ := hp
      simp [evalOrigin_equation])

def scalarToZero (a3 a2 a1 a0 : K) :
    K →+* degreeSubmodule a3 a2 a1 a0 0 :=
  (GradedRing.projZeroRingHom' (degreeSubmodule a3 a2 a1 a0)).comp
    (algebraMap K (WeightedRing a3 a2 a1 a0))

theorem scalarToZero_coe (a3 a2 a1 a0 : K) (c : K) :
    (scalarToZero a3 a2 a1 a0 c : WeightedRing a3 a2 a1 a0) =
      Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (C c) := by
  have hc : Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (C c) ∈
      degreeSubmodule a3 a2 a1 a0 0 := by
    change component a3 a2 a1 a0 0
      (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (C c)) = _
    simpa only [ite_true] using
      component_homogeneous a3 a2 a1 a0 0 0 (C c)
        (isWeightedHomogeneous_C weights c)
  have h := GradedRing.projZeroRingHom'_apply_coe
    (degreeSubmodule a3 a2 a1 a0)
    ⟨Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (C c), hc⟩
  have hmap : algebraMap K (WeightedRing a3 a2 a1 a0) c =
      Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (C c) := by
    simpa only [MvPolynomial.algebraMap_eq] using
      (Ideal.Quotient.mk_algebraMap K (relationIdeal a3 a2 a1 a0) c).symm
  change ((GradedRing.projZeroRingHom' (degreeSubmodule a3 a2 a1 a0))
    (algebraMap K (WeightedRing a3 a2 a1 a0) c) : WeightedRing a3 a2 a1 a0) = _
  rw [hmap]
  exact congrArg Subtype.val h

theorem toK_scalar (a3 a2 a1 a0 : K) (c : K) :
    toK a3 a2 a1 a0
      (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (C c)) = c := by
  have hmap : Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (C c) =
      algebraMap K (WeightedRing a3 a2 a1 a0) c := by
    simpa only [MvPolynomial.algebraMap_eq] using
      Ideal.Quotient.mk_algebraMap K (relationIdeal a3 a2 a1 a0) c
  rw [hmap]
  simpa using (toK a3 a2 a1 a0).commutes c

theorem scalarToZero_surjective (a3 a2 a1 a0 : K) :
    Function.Surjective (scalarToZero a3 a2 a1 a0) := by
  intro q
  obtain ⟨f, hf⟩ := Ideal.Quotient.mk_surjective (q : WeightedRing a3 a2 a1 a0)
  refine ⟨f.coeff 0, ?_⟩
  apply Subtype.ext
  rw [scalarToZero_coe]
  have hq : component a3 a2 a1 a0 0 (q : WeightedRing a3 a2 a1 a0) = q := q.property
  rw [← hf, component_mk, weightedHomogeneousComponent_zero
    (w := weights) f (by intro i; fin_cases i <;> decide)] at hq
  exact hq.trans hf

theorem scalarToZero_injective (a3 a2 a1 a0 : K) :
    Function.Injective (scalarToZero a3 a2 a1 a0) := by
  intro c d hcd
  have h := congrArg (fun q : degreeSubmodule a3 a2 a1 a0 0 =>
    toK a3 a2 a1 a0 (q : WeightedRing a3 a2 a1 a0)) hcd
  simpa only [scalarToZero_coe, toK_scalar] using h

def degreeZeroEquiv (a3 a2 a1 a0 : K) :
    K ≃+* degreeSubmodule a3 a2 a1 a0 0 :=
  RingEquiv.ofBijective (scalarToZero a3 a2 a1 a0)
    ⟨scalarToZero_injective a3 a2 a1 a0,
      scalarToZero_surjective a3 a2 a1 a0⟩

def toSpecK (a3 a2 a1 a0 : K) :
    Proj (degreeSubmodule a3 a2 a1 a0) ⟶ Spec (.of K) :=
  Proj.toSpecZero (degreeSubmodule a3 a2 a1 a0) ≫
    Spec.map (CommRingCat.ofHom (scalarToZero a3 a2 a1 a0))

theorem weightedProj_proper_over_K (a3 a2 a1 a0 : K) :
    IsProper (toSpecK a3 a2 a1 a0) := by
  have hiso :
      (CommRingCat.ofHom (scalarToZero a3 a2 a1 a0)) =
        (degreeZeroEquiv a3 a2 a1 a0).toCommRingCatIso.hom := rfl
  letI : CategoryTheory.IsIso
      (CommRingCat.ofHom (scalarToZero a3 a2 a1 a0)) := by
    rw [hiso]
    infer_instance
  unfold toSpecK
  infer_instance

#print axioms scalarToZero_coe
#print axioms degreeZeroEquiv
#print axioms weightedProj_proper_over_K
end WeightedProjBase
