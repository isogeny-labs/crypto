import WeightedProjDegreeZeroScratch

set_option autoImplicit false
noncomputable section
namespace WeightedRingComponent
open MvPolynomial Common22WeightedProjHead Common22WeightedProjQuotient
  WeightedProjDegreeZeroScratch
variable {K : Type*} [Field K]

def component (a3 a2 a1 a0 : K) (n : ℕ)
    (q : WeightedRing a3 a2 a1 a0) : WeightedRing a3 a2 a1 a0 :=
  Quotient.liftOn q
    (fun f : Poly K => Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)
      (weightedHomogeneousComponent weights n f))
    (by
      intro f g h
      exact (weightedRing_eq_iff_components a3 a2 a1 a0 f g).mp (Quotient.sound h) n)

@[simp] theorem component_mk (a3 a2 a1 a0 : K) (n : ℕ) (f : Poly K) :
    component a3 a2 a1 a0 n
      (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) f) =
      Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)
        (weightedHomogeneousComponent weights n f) := rfl

theorem component_add (a3 a2 a1 a0 : K) (n : ℕ)
    (q r : WeightedRing a3 a2 a1 a0) :
    component a3 a2 a1 a0 n (q + r) =
      component a3 a2 a1 a0 n q + component a3 a2 a1 a0 n r := by
  obtain ⟨f, rfl⟩ := Ideal.Quotient.mk_surjective q
  obtain ⟨g, rfl⟩ := Ideal.Quotient.mk_surjective r
  conv_lhs => rw [← map_add (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)) f g]
  rw [component_mk]
  simp only [map_add, component_mk]

theorem component_zero (a3 a2 a1 a0 : K) (n : ℕ) :
    component a3 a2 a1 a0 n 0 = 0 := by
  simpa only [map_zero] using component_mk a3 a2 a1 a0 n (0 : Poly K)

def componentAddHom (a3 a2 a1 a0 : K) (n : ℕ) :
    WeightedRing a3 a2 a1 a0 →+ WeightedRing a3 a2 a1 a0 where
  toFun := component a3 a2 a1 a0 n
  map_zero' := component_zero a3 a2 a1 a0 n
  map_add' := component_add a3 a2 a1 a0 n

theorem component_homogeneous (a3 a2 a1 a0 : K) (n m : ℕ) (f : Poly K)
    (hf : IsWeightedHomogeneous weights f m) :
    component a3 a2 a1 a0 n
      (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) f) =
      if n = m then Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) f else 0 := by
  rw [component_mk]
  classical
  have hmem : f ∈ weightedHomogeneousSubmodule K weights m := hf
  rw [weightedHomogeneousComponent_of_mem (w := weights) (m := n) hmem]
  split_ifs <;> simp

theorem component_idempotent (a3 a2 a1 a0 : K) (n m : ℕ)
    (q : WeightedRing a3 a2 a1 a0) :
    component a3 a2 a1 a0 n (component a3 a2 a1 a0 m q) =
      if n = m then component a3 a2 a1 a0 m q else 0 := by
  obtain ⟨f, rfl⟩ := Ideal.Quotient.mk_surjective q
  rw [component_mk]
  exact component_homogeneous a3 a2 a1 a0 n m _
    (weightedHomogeneousComponent_isWeightedHomogeneous (w := weights) m f)

theorem component_finite_reconstruction (a3 a2 a1 a0 : K)
    (q : WeightedRing a3 a2 a1 a0) :
    ∃ s : Finset ℕ, q = ∑ n ∈ s, component a3 a2 a1 a0 n q := by
  obtain ⟨f, rfl⟩ := Ideal.Quotient.mk_surjective q
  classical
  have hs : (Function.support fun n : ℕ => weightedHomogeneousComponent weights n f).Finite :=
    weightedHomogeneousComponent_finsupp f
  refine ⟨hs.toFinset, ?_⟩
  have hf : (∑ n ∈ hs.toFinset, weightedHomogeneousComponent weights n f) = f := by
    simpa only [finsum_eq_sum _ hs] using
      (MvPolynomial.sum_weightedHomogeneousComponent weights f)
  conv_lhs => rw [← hf]
  rw [map_sum]
  simp only [component_mk]

theorem component_separates (a3 a2 a1 a0 : K)
    (q r : WeightedRing a3 a2 a1 a0)
    (h : ∀ n : ℕ, component a3 a2 a1 a0 n q =
      component a3 a2 a1 a0 n r) : q = r := by
  obtain ⟨f, rfl⟩ := Ideal.Quotient.mk_surjective q
  obtain ⟨g, rfl⟩ := Ideal.Quotient.mk_surjective r
  exact (weightedRing_eq_iff_components a3 a2 a1 a0 f g).mpr
    (fun n => by simpa only [component_mk] using h n)

theorem component_homogeneous_sum (a3 a2 a1 a0 : K)
    (s : Finset ℕ) (p : ℕ → Poly K)
    (hp : ∀ i ∈ s, IsWeightedHomogeneous weights (p i) i) (n : ℕ) :
    component a3 a2 a1 a0 n
      (∑ i ∈ s, Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (p i)) =
      if n ∈ s then Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (p n) else 0 := by
  change (componentAddHom a3 a2 a1 a0 n)
    (∑ i ∈ s, Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (p i)) = _
  rw [map_sum]
  change (∑ i ∈ s, component a3 a2 a1 a0 n
    (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (p i))) = _
  have hsum :
      (∑ i ∈ s, component a3 a2 a1 a0 n
        (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (p i))) =
      ∑ i ∈ s, if n = i then
        Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (p i) else 0 := by
    apply Finset.sum_congr rfl
    intro i hi
    exact component_homogeneous a3 a2 a1 a0 n i (p i) (hp i hi)
  rw [hsum]
  simp

theorem homogeneous_sum_direct (a3 a2 a1 a0 : K)
    (s : Finset ℕ) (p : ℕ → Poly K)
    (hp : ∀ i ∈ s, IsWeightedHomogeneous weights (p i) i)
    (hzero : (∑ i ∈ s, Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (p i)) = 0) :
    ∀ i ∈ s, Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (p i) = 0 := by
  intro i hi
  have h := congrArg (component a3 a2 a1 a0 i) hzero
  rw [component_homogeneous_sum a3 a2 a1 a0 s p hp i, if_pos hi,
    component_zero] at h
  exact h

theorem homogeneous_product (a3 a2 a1 a0 : K)
    (f g : Poly K) (n m : ℕ)
    (hf : IsWeightedHomogeneous weights f n)
    (hg : IsWeightedHomogeneous weights g m) :
    component a3 a2 a1 a0 (n + m)
      (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) f *
        Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) g) =
      Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) f *
        Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) g := by
  conv_lhs => rw [← map_mul (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)) f g]
  rw [component_homogeneous a3 a2 a1 a0 (n + m) (n + m) (f * g) (hf.mul hg),
    if_pos rfl, map_mul]

#print axioms componentAddHom
#print axioms component_idempotent
#print axioms component_finite_reconstruction
#print axioms component_separates
#print axioms homogeneous_sum_direct
#print axioms homogeneous_product

#print axioms component_mk
end WeightedRingComponent
