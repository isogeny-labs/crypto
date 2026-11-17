module

public import CharacterWalk

@[expose] public section

namespace WalkCharacterAudit

open WeightedCycleCouplingAudit

theorem real_pair_variance {I : Type*} [Fintype I] (w x : I → ℝ) (hw : ∑ i, w i=1) :
    (∑ i, ∑ j, w i*w j*(x i-x j)^2) =
      2*(∑ i, w i*(x i)^2)-2*(∑ i, w i*x i)^2 := by
  have hp (i j : I) : w i*w j*(x i-x j)^2 =
      (w i*(x i)^2)*w j+w i*(w j*(x j)^2)-2*(w i*x i)*(w j*x j) := by ring
  simp_rw [hp,Finset.sum_sub_distrib,Finset.sum_add_distrib]
  simp only [← Finset.mul_sum,← Finset.sum_mul,hw,mul_one,one_mul]
  ring

theorem complex_mean_normSq {I : Type*} [Fintype I] (q : PMF I) (z : I → ℂ) :
    Complex.normSq (complexExpectation q z) =
      (∑ i, weight q i*(z i).re)^2+(∑ i, weight q i*(z i).im)^2 := by
  simp [Complex.normSq_apply,complexExpectation,pow_two]

theorem unit_phase_variance {I : Type*} [Fintype I] (q : PMF I) (z : I → ℂ)
    (hz : ∀ i, Complex.normSq (z i)=1) :
    1-Complex.normSq (complexExpectation q z) =
      (1/2 : ℝ)*(∑ i, ∑ j, weight q i*weight q j*Complex.normSq (z i-z j)) := by
  have hr := real_pair_variance (weight q) (fun i => (z i).re) (weight_sum q)
  have hi := real_pair_variance (weight q) (fun i => (z i).im) (weight_sum q)
  have hp (i j : I) : Complex.normSq (z i-z j)=
      ((z i).re-(z j).re)^2+((z i).im-(z j).im)^2 := by
    simp [Complex.normSq_apply,pow_two]
  have hs : (∑ i, ∑ j, weight q i*weight q j*Complex.normSq (z i-z j)) =
      (∑ i, ∑ j, weight q i*weight q j*((z i).re-(z j).re)^2)+
      (∑ i, ∑ j, weight q i*weight q j*((z i).im-(z j).im)^2) := by
    simp_rw [hp,mul_add,Finset.sum_add_distrib]
  have hu : (∑ i, weight q i*(z i).re^2)+(∑ i, weight q i*(z i).im^2)=1 := by
    calc
      _ = ∑ i, weight q i*Complex.normSq (z i) := by
        simp [Complex.normSq_apply,mul_add,Finset.sum_add_distrib,pow_two]
      _ = 1 := by simp_rw [hz,mul_one]; exact weight_sum q
  rw [complex_mean_normSq,hs,hr,hi]
  linarith

theorem two_terms_le_sum {I : Type*} [Fintype I] (f : I → ℝ)
    (hf : ∀ i, 0 ≤ f i) (a b : I) (hab : a ≠ b) : f a+f b ≤ ∑ i, f i := by
  classical
  calc
    _ = ∑ i ∈ ({a,b} : Finset I), f i := by simp [hab]
    _ ≤ _ := Finset.sum_le_sum_of_subset_of_nonneg (Finset.subset_univ _)
      (fun i _ _ => hf i)

theorem two_phase_defect_lower {I : Type*} [Fintype I] (q : PMF I) (z : I → ℂ)
    (a b : I) (hab : a ≠ b) :
    2*weight q a*weight q b*Complex.normSq (z a-z b) ≤
      ∑ i, ∑ j, weight q i*weight q j*Complex.normSq (z i-z j) := by
  have hn (i j : I) : 0 ≤ weight q i*weight q j*Complex.normSq (z i-z j) :=
    mul_nonneg (mul_nonneg (weight_nonneg q i) (weight_nonneg q j)) (Complex.normSq_nonneg _)
  have ha := Finset.single_le_sum (fun j _ => hn a j) (Finset.mem_univ b)
  have hb := Finset.single_le_sum (fun j _ => hn b j) (Finset.mem_univ a)
  have hout := two_terms_le_sum (fun i => ∑ j, weight q i*weight q j*Complex.normSq (z i-z j))
    (fun i => Finset.sum_nonneg (fun j _ => hn i j)) a b hab
  have hrev : Complex.normSq (z b-z a)=Complex.normSq (z a-z b) := by
    rw [← neg_sub (z a) (z b),Complex.normSq_neg]
  calc
    _ = weight q a*weight q b*Complex.normSq (z a-z b)+
        weight q b*weight q a*Complex.normSq (z b-z a) := by rw [hrev]; ring
    _ ≤ _ := (add_le_add ha hb).trans hout

theorem two_phase_contraction {I : Type*} [Fintype I] (q : PMF I) (z : I → ℂ)
    (hz : ∀ i, Complex.normSq (z i)=1) (a b : I) (hab : a ≠ b) :
    Complex.normSq (complexExpectation q z) ≤
      1-weight q a*weight q b*Complex.normSq (z a-z b) := by
  have hv := unit_phase_variance q z hz
  have hd := two_phase_defect_lower q z a b hab
  linarith

theorem strict_two_phase_contraction {I : Type*} [Fintype I] (q : PMF I) (z : I → ℂ)
    (hz : ∀ i, Complex.normSq (z i)=1) (a b : I) (hab : a ≠ b)
    (ha : 0<weight q a) (hb : 0<weight q b) (hneq : z a ≠ z b) :
    Complex.normSq (complexExpectation q z)<1 := by
  have hc := two_phase_contraction q z hz a b hab
  have hp : 0<Complex.normSq (z a-z b) := by
    rw [Complex.normSq_eq_norm_sq]
    exact sq_pos_of_pos (norm_pos_iff.mpr (sub_ne_zero.mpr hneq))
  have hd := mul_pos (mul_pos ha hb) hp
  linarith

end WalkCharacterAudit
