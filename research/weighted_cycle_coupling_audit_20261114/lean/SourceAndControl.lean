module

public import WeightedCycle

@[expose] public section

namespace WeightedCycleCouplingAudit

open CycleCouplingAudit

variable {I : Type*} [Fintype I]

noncomputable def labelLaw (w : I → NNReal) (hw : ∑ i, w i = 1) : PMF I :=
  PMF.ofFintype (fun i => (w i : ENNReal)) (by exact_mod_cast hw)

theorem label_law_weight (w : I → NNReal) (hw : ∑ i, w i = 1) (i : I) :
    weight (labelLaw w hw) i = (w i : ℝ) := by simp [weight,labelLaw]

/-- The uniform-support quadratic theorem for any finite k≥2 and fixed positive weights. -/
theorem hildebrand_theorem_two [Nontrivial I] (w : I → NNReal) (hw : ∑ i, w i = 1)
    (hpos : ∀ i, 0 < w i) (ε : ℝ) (hε : 0 < ε) :
    ∃ γ : ℝ, 0 < γ ∧ ∀ (p : ℕ) [Fact p.Prime] (g : I → ZMod p), Function.Injective g →
      ∀ t : ℕ, γ*(p : ℝ)^2 ≤ t → ∀ (x : ZMod p) (A : Finset (ZMod p)),
      abs (probability (labelLaw w hw) g t x A-(A.card : ℝ)/p) < ε := by
  obtain ⟨a,b,hab⟩ := exists_pair_ne I
  have ha : 0 < weight (labelLaw w hw) a := by rw [label_law_weight]; exact_mod_cast hpos a
  have hb : 0 < weight (labelLaw w hw) b := by rw [label_law_weight]; exact_mod_cast hpos b
  obtain ⟨γ,hγ,h⟩ := hildebrand_uniform_quadratic (labelLaw w hw) a b ha hb ε hε
  refine ⟨γ,hγ,?_⟩
  intro p _ g hg t ht x A
  exact h p g (fun he => hab (hg he)) t ht x A

def compositeIncrements (b : Bool) : ZMod 4 := if b then 2 else 0

def evenEvent : Finset (ZMod 4) := {0,2}

theorem composite_distinct : Function.Injective compositeIncrements := by decide +kernel

theorem composite_closed : ∀ x : ZMod 4, x ∈ evenEvent →
    ∀ i : Bool, incrementStep compositeIncrements i x ∈ evenEvent := by decide +kernel

theorem fair_weights (i : Bool) : weight fairCoin i = 1/2 := by
  norm_num [weight,fairCoin]

theorem composite_confined (t : ℕ) (x : ZMod 4) (hx : x ∈ evenEvent) :
    weightedAverage fairCoin (incrementStep compositeIncrements) t (eventIndicator evenEvent) x = 1 := by
  induction t generalizing x with
  | zero => simp [weightedAverage,eventIndicator,hx]
  | succ t ih =>
    change (∑ i, weight fairCoin i * weightedAverage fairCoin (incrementStep compositeIncrements)
      t (eventIndicator evenEvent) (incrementStep compositeIncrements i x)) = 1
    calc
      _ = ∑ i, weight fairCoin i * 1 := by
        apply Finset.sum_congr rfl
        intro i _
        rw [ih _ (composite_closed x hx i)]
      _ = 1 := by simp only [mul_one,weight_sum]

theorem composite_event_distance (t : ℕ) :
    abs (probability fairCoin compositeIncrements t 0 evenEvent-(evenEvent.card : ℝ)/4) = 1/2 := by
  rw [probability_eq,composite_confined t 0 (by decide +kernel)]
  have hc : evenEvent.card = 2 := by decide +kernel
  rw [hc]
  norm_num

theorem composite_no_quadratic_constant :
    ¬ ∃ γ : ℝ, 0 < γ ∧ ∀ t : ℕ, γ*(4 : ℝ)^2 ≤ t →
      ∀ (x : ZMod 4) (A : Finset (ZMod 4)),
      abs (probability fairCoin compositeIncrements t x A-(A.card : ℝ)/4) < 1/4 := by
  rintro ⟨γ,_,h⟩
  obtain ⟨t,ht⟩ := exists_nat_ge (γ*(4 : ℝ)^2)
  have hbad := h t ht 0 evenEvent
  rw [composite_event_distance] at hbad
  norm_num at hbad

end WeightedCycleCouplingAudit
