module

public import FourierAlgebra

@[expose] public section

namespace FiniteFourierUpperBoundAudit

open CycleCouplingAudit WeightedCycleCouplingAudit

noncomputable def eventMass {S : Type*} [Fintype S] (q : PMF S) (A : Finset S) : ℝ :=
  ∑ x ∈ A, weight q x

noncomputable def l1Distance {S : Type*} [Fintype S] (q r : PMF S) : ℝ :=
  ∑ x, |weight q x-weight r x|

/-- Source convention: half of the full L1 distance. -/
noncomputable def totalVariation {S : Type*} [Fintype S] (q r : PMF S) : ℝ := l1Distance q r/2

theorem event_mass_nonneg {S : Type*} [Fintype S] (q : PMF S) (A : Finset S) : 0 ≤ eventMass q A :=
  Finset.sum_nonneg (fun x _ => weight_nonneg q x)

theorem event_mass_le_one {S : Type*} [Fintype S] (q : PMF S) (A : Finset S) : eventMass q A ≤ 1 := by
  calc
    _ ≤ ∑ x, weight q x := Finset.sum_le_sum_of_subset_of_nonneg (Finset.subset_univ _)
      (fun x _ _ => weight_nonneg q x)
    _ = 1 := weight_sum q

open Classical in
theorem event_mass_indicator {S : Type*} [Fintype S] (q : PMF S) (A : Finset S) :
    eventMass q A=expectation q (fun x => if x ∈ A then 1 else 0) := by
  classical
  simp [eventMass,expectation,weight]

theorem total_variation_nonneg {S : Type*} [Fintype S] (q r : PMF S) : 0 ≤ totalVariation q r := by
  unfold totalVariation l1Distance
  positivity

theorem l1_distance_le_two {S : Type*} [Fintype S] (q r : PMF S) : l1Distance q r ≤ 2 := by
  calc
    _ ≤ ∑ x, (weight q x+weight r x) := by
      apply Finset.sum_le_sum
      intro x _
      have hq := weight_nonneg q x
      have hr := weight_nonneg r x
      exact abs_le.mpr ⟨by linarith,by linarith⟩
    _ = 2 := by rw [Finset.sum_add_distrib,weight_sum,weight_sum]; norm_num

theorem total_variation_le_one {S : Type*} [Fintype S] (q r : PMF S) : totalVariation q r ≤ 1 := by
  have h := l1_distance_le_two q r
  unfold totalVariation
  linarith

theorem event_difference_le_total_variation {S : Type*} [Fintype S]
    (q r : PMF S) (A : Finset S) : |eventMass q A-eventMass r A| ≤ totalVariation q r := by
  classical
  let f : S → ℝ := fun x => weight q x-weight r x
  have hs : (∑ x ∈ A, f x)+(∑ x ∈ Aᶜ, f x)=0 := by
    rw [Finset.sum_add_sum_compl]
    simp [f,Finset.sum_sub_distrib,weight_sum]
  have hn : ∑ x ∈ Aᶜ, f x=-(∑ x ∈ A, f x) := by linarith
  have ha := Finset.abs_sum_le_sum_abs f A
  have hb := Finset.abs_sum_le_sum_abs f Aᶜ
  rw [hn,abs_neg] at hb
  have ht := Finset.sum_add_sum_compl A (fun x => |f x|)
  have he : eventMass q A-eventMass r A=∑ x ∈ A, f x := by
    simp [eventMass,f,Finset.sum_sub_distrib]
  rw [he]
  change |∑ x ∈ A, f x| ≤ (∑ x, |f x|)/2
  linarith

noncomputable def positiveEvent {S : Type*} [Fintype S] (q r : PMF S) : Finset S := by
  classical
  exact Finset.univ.filter (fun x => weight r x ≤ weight q x)

theorem positive_event_attains {S : Type*} [Fintype S] (q r : PMF S) :
    eventMass q (positiveEvent q r)-eventMass r (positiveEvent q r)=totalVariation q r := by
  classical
  let f : S → ℝ := fun x => weight q x-weight r x
  let A := positiveEvent q r
  have ha : (∑ x ∈ A, |f x|)=∑ x ∈ A, f x := by
    apply Finset.sum_congr rfl
    intro x hx
    have hm : weight r x ≤ weight q x := (Finset.mem_filter.mp hx).2
    exact abs_of_nonneg (sub_nonneg.mpr hm)
  have hb : (∑ x ∈ Aᶜ, |f x|)=-(∑ x ∈ Aᶜ, f x) := by
    rw [← Finset.sum_neg_distrib]
    apply Finset.sum_congr rfl
    intro x hx
    have hn : ¬ weight r x ≤ weight q x := by
      intro h
      exact (Finset.mem_compl.mp hx) (Finset.mem_filter.mpr ⟨Finset.mem_univ x,h⟩)
    exact abs_of_nonpos (sub_nonpos.mpr (not_le.mp hn).le)
  have hs : (∑ x ∈ A, f x)+(∑ x ∈ Aᶜ, f x)=0 := by
    rw [Finset.sum_add_sum_compl]
    simp [f,Finset.sum_sub_distrib,weight_sum]
  have ht := Finset.sum_add_sum_compl A (fun x => |f x|)
  rw [ha,hb] at ht
  have he : eventMass q A-eventMass r A=∑ x ∈ A, f x := by
    simp [eventMass,f,Finset.sum_sub_distrib]
  change eventMass q A-eventMass r A=(∑ x, |f x|)/2
  rw [he]
  linarith

theorem l1_cauchy_bound {S : Type*} [Fintype S] (q r : PMF S) :
    (l1Distance q r)^2 ≤ (Fintype.card S : ℝ)*(∑ x, (weight q x-weight r x)^2) := by
  simpa only [l1Distance,one_mul,one_pow,Finset.sum_const,Finset.card_univ,
    nsmul_eq_mul,mul_one,sq_abs] using
    Finset.sum_mul_sq_le_sq_mul_sq (Finset.univ : Finset S)
      (fun _ => (1 : ℝ)) (fun x => |weight q x-weight r x|)

end FiniteFourierUpperBoundAudit
