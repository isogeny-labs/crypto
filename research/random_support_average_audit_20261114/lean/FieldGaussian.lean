module

public import GaussianCertificate
public import GeneralSupports
public import PrimeFieldBoxes

@[expose] public section

namespace RandomSupportAverageAudit

open Real SupportGaussianAudit InjectiveBoxAudit HildebrandCellAudit
open WalkCharacterAudit WeightedCycleCouplingAudit CycleCouplingAudit

variable {p : ℕ} [Fact p.Prime]

theorem transformed_center_nonzero (j : ZMod p) (hj : j ≠ 0) (u : Choice p) :
    centered (-u.val*j) ≠ 0 := by
  intro h
  have hc := congrArg (fun z : ℤ => (z : ZMod p)) h
  rw [centered_cast,Int.cast_zero] at hc
  exact (mul_ne_zero (neg_ne_zero.mpr u.property) hj) hc

theorem field_gaussian_sum_bound {a : ℝ} (ha : 0<a) (j : ZMod p) (hj : j ≠ 0) :
    (∑ u : Choice p, exp (-a*(centered (-u.val*j) : ℝ)^2)) ≤ sqrt (Real.pi/a) := by
  classical
  let S : Finset ℤ := Finset.univ.image (fun u : Choice p => centered (-u.val*j))
  have hs : 0 ∉ S := by
    intro h
    obtain ⟨u,_hu,he⟩ := Finset.mem_image.mp h
    exact transformed_center_nonzero j hj u he
  have he : (∑ z ∈ S, exp (-a*(z : ℝ)^2)) =
      ∑ u : Choice p, exp (-a*(centered (-u.val*j) : ℝ)^2) :=
    Finset.sum_image (fun _ _ _ _ h => transformed_center_injective j hj h)
  rw [← he]
  exact finite_integer_gaussian_bound ha S hs

theorem relative_centered_radius {d : ℕ} (a₀ j : ZMod p) (e : Sample (Choice p) d) :
    centeredRadius (supportsFromSample a₀ e) 0 j =
      ∑ i : Fin d, ((centered (-(e i).val*j) : ℝ)/(p : ℝ))^2 := by
  have hz : centered (0 : ZMod p)=0 := by simp [centered]
  unfold centeredRadius
  rw [Fin.sum_univ_succ]
  have hf : supportsFromSample a₀ e 0=a₀ := rfl
  rw [hf,sub_self,zero_mul,hz]
  simp only [Int.cast_zero,zero_div,zero_pow (by decide : (2 : ℕ) ≠ 0),zero_add]
  apply Finset.sum_congr rfl
  intro i _
  rw [(full_source_argument a₀ j e i).2]

theorem exponential_radius_product {d : ℕ} (a₀ j : ZMod p) (e : Sample (Choice p) d)
    (c : ℝ) :
    exp (-c*centeredRadius (supportsFromSample a₀ e) 0 j) =
      ∏ i : Fin d, exp (-(c/(p : ℝ)^2)*(centered (-(e i).val*j) : ℝ)^2) := by
  rw [relative_centered_radius,← Real.exp_sum]
  congr 1
  rw [Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro i _
  rw [div_pow]
  ring

theorem relative_gaussian_average {d : ℕ} (h : d ≤ p-1) (a₀ j : ZMod p)
    (hj : j ≠ 0) {c : ℝ} (hc : 0<c) :
    expectation (sampleLaw (Choice p) d (by rw [choice_card]; exact h))
      (fun e => exp (-c*centeredRadius (supportsFromSample a₀ e) 0 j)) ≤
      (sqrt (Real.pi/(c/(p : ℝ)^2)))^d/((p-1).descFactorial d : ℝ) := by
  let H : Fin d → Choice p → ℝ := fun _ u => exp (-(c/(p : ℝ)^2)*(centered (-u.val*j) : ℝ)^2)
  have hH : ∀ i u, 0 ≤ H i u := fun _ _ => (exp_pos _).le
  have ha : 0<c/(p : ℝ)^2 := by
    have hp : (0 : ℝ)<p := by exact_mod_cast (Fact.out : p.Prime).pos
    positivity
  have hb := injective_product_expectation_bound (X := Choice p)
    (by rw [choice_card]; exact h) H hH
  have hn : (∏ i : Fin d, ∑ u : Choice p, H i u) ≤ (sqrt (Real.pi/(c/(p : ℝ)^2)))^d := by
    calc
      _ ≤ ∏ _i : Fin d, sqrt (Real.pi/(c/(p : ℝ)^2)) :=
        Finset.prod_le_prod₀ (fun i _ => Finset.sum_nonneg (fun u _ => hH i u))
          (fun _ _ => field_gaussian_sum_bound ha j hj)
      _ = _ := by simp
  have he : expectation (sampleLaw (Choice p) d (by rw [choice_card]; exact h))
      (fun e => exp (-c*centeredRadius (supportsFromSample a₀ e) 0 j)) =
      injectiveProductExpectation (Choice p) d (by rw [choice_card]; exact h) H := by
    unfold injectiveProductExpectation
    apply congrArg (expectation _)
    funext e
    exact exponential_radius_product a₀ j e c
  rw [he]
  have hd := div_le_div_of_nonneg_right hn (by positivity : (0 : ℝ) ≤ (Fintype.card (Choice p)).descFactorial d)
  simpa only [choice_card] using hb.trans hd

end RandomSupportAverageAudit
