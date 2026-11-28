module

public import WeightedKernel
public import CycleGeometry

@[expose] public section

namespace WeightedCycleCouplingAudit

open CycleCouplingAudit

variable {I : Type*} [Fintype I]

noncomputable def labelPair (q : PMF I) : PMF (I × I) :=
  PMF.ofFintype (fun z => q z.1 * q z.2) (by
    have h : ∑ i, q i = 1 := by simpa only [tsum_fintype] using q.tsum_coe
    simp only [Fintype.sum_prod_type, ← Finset.mul_sum, h, mul_one])

theorem label_pair_weight (q : PMF I) (z : I × I) :
    weight (labelPair q) z = weight q z.1 * weight q z.2 :=
  ENNReal.toReal_mul

theorem label_pair_eq_bind (q : PMF I) :
    labelPair q = q.bind (fun i => q.map (fun j => (i,j))) := by
  classical
  ext z
  rcases z with ⟨u,v⟩
  have hmap (i : I) : (q.map (fun j => (i,j))) (u,v) = if u=i then q v else 0 := by
    rw [PMF.map_apply, tsum_fintype]
    by_cases h : u=i
    · subst u
      simp [Prod.mk.injEq]
    · simp [Prod.mk.injEq,h]
  change q u * q v = _
  rw [PMF.bind_apply, tsum_fintype]
  simp_rw [hmap]
  simp [mul_ite]

noncomputable def pairMean (q : PMF I) (f : I × I → ℝ) : ℝ :=
  ∑ z, weight (labelPair q) z * f z

theorem pair_mean_first (q : PMF I) (f : I → ℝ) :
    pairMean q (fun z => f z.1) = ∑ i, weight q i * f i := by
  unfold pairMean
  simp_rw [label_pair_weight, Fintype.sum_prod_type]
  calc
    _ = ∑ i, weight q i * f i * (∑ j, weight q j) := by
      apply Finset.sum_congr rfl
      intro i _
      rw [Finset.mul_sum]
      apply Finset.sum_congr rfl
      intro j _
      ring
    _ = _ := by simp only [weight_sum, mul_one]

noncomputable def swapLabels (a b : I) : I × I ≃ I × I := by
  classical
  exact Equiv.swap (a,b) (b,a)

theorem pair_weight_swap (q : PMF I) (a b : I) (z : I × I) :
    weight (labelPair q) (swapLabels a b z) = weight (labelPair q) z := by
  classical
  by_cases ha : z = (a,b)
  · subst z; simp [swapLabels, label_pair_weight, mul_comm]
  · by_cases hb : z = (b,a)
    · subst z; simp [swapLabels, label_pair_weight, mul_comm]
    · simp [swapLabels, Equiv.swap_apply_of_ne_of_ne ha hb]

theorem pair_mean_swap (q : PMF I) (a b : I) (f : I × I → ℝ) :
    pairMean q (fun z => f (swapLabels a b z)) = pairMean q f := by
  unfold pairMean
  calc
    _ = ∑ z, weight (labelPair q) (swapLabels a b z) * f (swapLabels a b z) := by
      simp_rw [pair_weight_swap]
    _ = _ := Fintype.sum_equiv (swapLabels a b) _ _ (fun _ => rfl)

theorem pair_mean_swapped_first (q : PMF I) (a b : I) (f : I → ℝ) :
    pairMean q (fun z => f (swapLabels a b z).1) = ∑ i, weight q i * f i := by
  rw [pair_mean_swap q a b (fun z => f z.1), pair_mean_first]

open Classical in
theorem pair_mean_split (q : PMF I) (a b : I) (hab : a ≠ b) (A B C : ℝ) :
    pairMean q (fun z => if z = (a,b) then A else if z = (b,a) then B else C) =
      weight q a * weight q b * A + weight q a * weight q b * B +
        (1-2*weight q a*weight q b)*C := by
  classical
  have hne : (a,b) ≠ (b,a) := fun h => hab (congrArg Prod.fst h)
  have hf (z : I × I) :
      (if z = (a,b) then A else if z = (b,a) then B else C) =
      C + (if z = (a,b) then A-C else 0) + (if z = (b,a) then B-C else 0) := by
    by_cases ha : z = (a,b)
    · subst z; simp [hne]
    · by_cases hb : z = (b,a)
      · subst z; simp [hne.symm]
      · simp [ha,hb]
  unfold pairMean
  simp_rw [hf, mul_add, mul_ite, mul_zero]
  rw [Finset.sum_add_distrib, Finset.sum_add_distrib, ← Finset.sum_mul, weight_sum]
  simp only [Finset.sum_ite_eq', Finset.mem_univ, ↓reduceIte, label_pair_weight]
  ring

noncomputable def coupledLabelStep {n : ℕ} (g : I → ZMod n) (a b : I)
    (z : I × I) (s : ZMod n × ZMod n) : ZMod n × ZMod n :=
  (s.1 + g z.1, s.2 + g (if s.1 = s.2 then z.1 else (swapLabels a b z).1))

theorem coupled_first {n : ℕ} (q : PMF I) (g : I → ZMod n) (a b : I)
    (s : ZMod n × ZMod n) (f : ZMod n → ℝ) :
    pairMean q (fun z => f (coupledLabelStep g a b z s).1) =
      ∑ i, weight q i * f (s.1 + g i) := pair_mean_first q (fun i => f (s.1 + g i))

theorem coupled_second {n : ℕ} (q : PMF I) (g : I → ZMod n) (a b : I)
    (s : ZMod n × ZMod n) (f : ZMod n → ℝ) :
    pairMean q (fun z => f (coupledLabelStep g a b z s).2) =
      ∑ i, weight q i * f (s.2 + g i) := by
  classical
  by_cases h : s.1 = s.2
  · simp only [coupledLabelStep, h, ↓reduceIte]
    exact pair_mean_first q (fun i => f (s.2 + g i))
  · simp only [coupledLabelStep, h, ↓reduceIte]
    exact pair_mean_swapped_first q a b (fun i => f (s.2 + g i))

omit [Fintype I] in
theorem coupled_mismatch {n : ℕ} (g : I → ZMod n) (a b : I)
    (z : I × I) (s : ZMod n × ZMod n) :
    mismatch (coupledLabelStep g a b z s) ≤ mismatch s := by
  classical
  by_cases h : s.1 = s.2
  · simp [coupledLabelStep, mismatch, h]
  · unfold mismatch
    simp only [h, ↓reduceIte]
    split_ifs <;> norm_num

end WeightedCycleCouplingAudit
