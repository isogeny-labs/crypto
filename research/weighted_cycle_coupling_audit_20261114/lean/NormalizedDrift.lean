module

public import LabelCoupling

@[expose] public section

namespace WeightedCycleCouplingAudit

open CycleCouplingAudit

variable {I : Type*} [Fintype I] {p : ℕ} [Fact p.Prime]

noncomputable def normalizedDifference (g : I → ZMod p) (a b : I) (s : ZMod p × ZMod p) : ZMod p :=
  (s.1-s.2)/(g a-g b)

noncomputable def normalizedPotential (g : I → ZMod p) (a b : I) (s : ZMod p × ZMod p) : ℝ :=
  potential (normalizedDifference g a b s)

open Classical in
omit [Fintype I] in
theorem normalized_step (g : I → ZMod p) (a b : I) (hg : g a ≠ g b)
    (s : ZMod p × ZMod p) (hs : s.1 ≠ s.2) (z : I × I) :
    normalizedDifference g a b (coupledLabelStep g a b z s) =
      if z=(a,b) then normalizedDifference g a b s + 1
      else if z=(b,a) then normalizedDifference g a b s - 1
      else normalizedDifference g a b s := by
  have hd : g a-g b ≠ 0 := sub_ne_zero.mpr hg
  have hab : a ≠ b := fun h => hg (congrArg g h)
  have hne : (a,b) ≠ (b,a) := fun h => hab (congrArg Prod.fst h)
  by_cases ha : z=(a,b)
  · subst z
    have ht : coupledLabelStep g a b (a,b) s = (s.1+g a,s.2+g b) := by
      simp [coupledLabelStep,hs,swapLabels]
    rw [ht]
    simp only [↓reduceIte]
    unfold normalizedDifference
    field_simp
    ring
  · by_cases hb : z=(b,a)
    · subst z
      have ht : coupledLabelStep g a b (b,a) s = (s.1+g b,s.2+g a) := by
        simp [coupledLabelStep,hs,swapLabels]
      rw [ht]
      simp only [hne.symm, ↓reduceIte]
      unfold normalizedDifference
      field_simp
      ring
    · have ht : coupledLabelStep g a b z s = (s.1+g z.1,s.2+g z.1) := by
        simp [coupledLabelStep,hs,swapLabels,Equiv.swap_apply_of_ne_of_ne ha hb]
      rw [ht]
      simp only [ha,hb,↓reduceIte]
      unfold normalizedDifference
      congr 1
      ring

theorem normalized_drift (q : PMF I) (g : I → ZMod p) (a b : I) (hg : g a ≠ g b)
    (s : ZMod p × ZMod p) :
    pairMean q (fun z => normalizedPotential g a b (coupledLabelStep g a b z s)) =
      normalizedPotential g a b s - (2 * weight q a * weight q b) * mismatch s := by
  classical
  by_cases hs : s.1=s.2
  · have hz (z : I × I) : normalizedPotential g a b (coupledLabelStep g a b z s) = 0 := by
      simp [normalizedPotential, normalizedDifference, coupledLabelStep,hs,potential]
    simp_rw [hz]
    simp [pairMean,normalizedPotential,normalizedDifference,hs,mismatch,potential]
  · have hab : a ≠ b := fun h => hg (congrArg g h)
    have hz (z : I × I) : normalizedPotential g a b (coupledLabelStep g a b z s) =
        if z=(a,b) then potential (normalizedDifference g a b s+1)
        else if z=(b,a) then potential (normalizedDifference g a b s-1)
        else potential (normalizedDifference g a b s) := by
      unfold normalizedPotential
      rw [normalized_step g a b hg s hs z]
      split_ifs <;> rfl
    simp_rw [hz]
    rw [pair_mean_split q a b hab]
    have hd : normalizedDifference g a b s ≠ 0 :=
      div_ne_zero (sub_ne_zero.mpr hs) (sub_ne_zero.mpr hg)
    have hpot := potential_neighbors (Fact.out : p.Prime).two_le (normalizedDifference g a b s) hd
    have hm := congrArg (fun v : ℝ => (weight q a * weight q b)*v) hpot
    simp only [mismatch,hs,↓reduceIte,mul_one,normalizedPotential]
    nlinarith

omit [Fact p.Prime] in
theorem mean_mismatch (q : PMF I) (g : I → ZMod p) (a b : I) (s : ZMod p × ZMod p) :
    pairMean q (fun z => mismatch (coupledLabelStep g a b z s)) ≤ mismatch s := by
  unfold pairMean
  calc
    _ ≤ ∑ z, weight (labelPair q) z * mismatch s :=
      Finset.sum_le_sum fun z _ =>
        mul_le_mul_of_nonneg_left (coupled_mismatch g a b z s) (weight_nonneg _ _)
    _ = _ := by rw [← Finset.sum_mul,weight_sum,one_mul]

end WeightedCycleCouplingAudit
