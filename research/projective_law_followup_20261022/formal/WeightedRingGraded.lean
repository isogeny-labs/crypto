import WeightedRingComponent

set_option autoImplicit false
noncomputable section
namespace WeightedRingGraded
open MvPolynomial Common22WeightedProjHead Common22WeightedProjQuotient WeightedRingComponent
variable {K : Type*} [Field K]

theorem component_smul (a3 a2 a1 a0 : K) (n : ℕ) (c : K)
    (q : WeightedRing a3 a2 a1 a0) :
    component a3 a2 a1 a0 n (c • q) = c • component a3 a2 a1 a0 n q := by
  obtain ⟨f, rfl⟩ := Ideal.Quotient.mk_surjective q
  have hmk (h : Poly K) :
      Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (c • h) =
        c • Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) h := by
    simpa only [AlgHom.toLinearMap_apply, Ideal.Quotient.mkₐ_eq_mk] using
      (Ideal.Quotient.mkₐ K (relationIdeal a3 a2 a1 a0)).toLinearMap.map_smul c h
  conv_lhs => rw [← hmk f]
  rw [component_mk, map_smul, hmk, component_mk]

def degreeSubmodule (a3 a2 a1 a0 : K) (n : ℕ) :
    Submodule K (WeightedRing a3 a2 a1 a0) where
  carrier := {q | component a3 a2 a1 a0 n q = q}
  zero_mem' := component_zero a3 a2 a1 a0 n
  add_mem' := by
    intro x y hx hy
    change component a3 a2 a1 a0 n x = x at hx
    change component a3 a2 a1 a0 n y = y at hy
    change component a3 a2 a1 a0 n (x + y) = x + y
    rw [component_add, hx, hy]
  smul_mem' := by
    intro c x hx
    change component a3 a2 a1 a0 n x = x at hx
    change component a3 a2 a1 a0 n (c • x) = c • x
    rw [component_smul, hx]

theorem component_mem (a3 a2 a1 a0 : K) (n : ℕ)
    (q : WeightedRing a3 a2 a1 a0) :
    component a3 a2 a1 a0 n q ∈ degreeSubmodule a3 a2 a1 a0 n := by
  change component a3 a2 a1 a0 n (component a3 a2 a1 a0 n q) =
    component a3 a2 a1 a0 n q
  simpa only [ite_true] using component_idempotent a3 a2 a1 a0 n n q

theorem component_of_mem (a3 a2 a1 a0 : K) (n m : ℕ)
    (q : degreeSubmodule a3 a2 a1 a0 m) :
    component a3 a2 a1 a0 n (q : WeightedRing a3 a2 a1 a0) =
      if n = m then (q : WeightedRing a3 a2 a1 a0) else 0 := by
  have hq : component a3 a2 a1 a0 m (q : WeightedRing a3 a2 a1 a0) = q := q.property
  conv_lhs => rw [← hq]
  rw [component_idempotent]
  split_ifs <;> simp [hq]

theorem component_recompose (a3 a2 a1 a0 : K)
    (x : DirectSum ℕ (fun n => degreeSubmodule a3 a2 a1 a0 n)) (n : ℕ) :
    component a3 a2 a1 a0 n
      (DirectSum.coeAddMonoidHom (degreeSubmodule a3 a2 a1 a0) x) =
      (x n : WeightedRing a3 a2 a1 a0) := by
  induction x using DirectSum.induction_on with
  | zero => simp [component_zero]
  | of i v =>
      rw [DirectSum.coeAddMonoidHom_of, component_of_mem]
      by_cases h : n = i
      · subst n
        simp
      · simp [h, DirectSum.of_eq_of_ne _ _ _ h]
  | add x y hx hy =>
      rw [map_add, component_add, hx, hy]
      simp

theorem exists_homogeneous_lift (a3 a2 a1 a0 : K) (n : ℕ)
    (q : degreeSubmodule a3 a2 a1 a0 n) :
    ∃ f : Poly K, IsWeightedHomogeneous weights f n ∧
      Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) f = q := by
  obtain ⟨f, hf⟩ := Ideal.Quotient.mk_surjective (q : WeightedRing a3 a2 a1 a0)
  refine ⟨weightedHomogeneousComponent weights n f,
    weightedHomogeneousComponent_isWeightedHomogeneous (w := weights) n f, ?_⟩
  have hq : component a3 a2 a1 a0 n (q : WeightedRing a3 a2 a1 a0) = q := q.property
  rw [← hf, component_mk] at hq
  exact hq.trans hf

theorem degreeSubmodule_isInternal (a3 a2 a1 a0 : K) :
    DirectSum.IsInternal (degreeSubmodule a3 a2 a1 a0) := by
  constructor
  · intro x y hxy
    apply DFinsupp.ext
    intro n
    apply Subtype.ext
    have hn := congrArg (component a3 a2 a1 a0 n) hxy
    simpa only [component_recompose] using hn
  · intro q
    obtain ⟨s, hs⟩ := component_finite_reconstruction a3 a2 a1 a0 q
    refine ⟨∑ n ∈ s, DirectSum.of
      (fun n => degreeSubmodule a3 a2 a1 a0 n) n
      ⟨component a3 a2 a1 a0 n q, component_mem a3 a2 a1 a0 n q⟩, ?_⟩
    simp only [map_sum, DirectSum.coeAddMonoidHom_of]
    exact hs.symm

noncomputable instance (a3 a2 a1 a0 : K) :
    DirectSum.Decomposition (degreeSubmodule a3 a2 a1 a0) :=
  (degreeSubmodule_isInternal a3 a2 a1 a0).chooseDecomposition

instance (a3 a2 a1 a0 : K) :
    SetLike.GradedMonoid (degreeSubmodule a3 a2 a1 a0) where
  one_mem := by
    change component a3 a2 a1 a0 0 (1 : WeightedRing a3 a2 a1 a0) = 1
    have h := component_homogeneous a3 a2 a1 a0 0 0 (1 : Poly K)
      (isWeightedHomogeneous_one K weights)
    simpa only [map_one, ite_true] using h
  mul_mem := by
    intro i j x y hx hy
    obtain ⟨f, hf, hfx⟩ := exists_homogeneous_lift a3 a2 a1 a0 i ⟨x, hx⟩
    obtain ⟨g, hg, hgy⟩ := exists_homogeneous_lift a3 a2 a1 a0 j ⟨y, hy⟩
    change Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) f = x at hfx
    change Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) g = y at hgy
    change component a3 a2 a1 a0 (i + j) (x * y) = x * y
    rw [← hfx, ← hgy]
    exact homogeneous_product a3 a2 a1 a0 f g i j hf hg

noncomputable instance weightedGradedRing (a3 a2 a1 a0 : K) :
    GradedRing (degreeSubmodule a3 a2 a1 a0) where

#print axioms degreeSubmodule
#print axioms degreeSubmodule_isInternal
#print axioms weightedGradedRing
end WeightedRingGraded
