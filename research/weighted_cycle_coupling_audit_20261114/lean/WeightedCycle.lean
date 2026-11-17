module

public import NormalizedDrift
public import CycleMixing

@[expose] public section

namespace WeightedCycleCouplingAudit

open CycleCouplingAudit

variable {I : Type*} [Fintype I]

def incrementStep {n : ℕ} (g : I → ZMod n) (i : I) (x : ZMod n) : ZMod n := x+g i

noncomputable def probability {n : ℕ} [NeZero n] (q : PMF I) (g : I → ZMod n)
    (t : ℕ) (x : ZMod n) (A : Finset (ZMod n)) : ℝ :=
  expectation (kernelLaw q (incrementStep g) t x) (eventIndicator A)

theorem probability_eq {n : ℕ} [NeZero n] (q : PMF I) (g : I → ZMod n)
    (t : ℕ) (x : ZMod n) (A : Finset (ZMod n)) :
    probability q g t x A = weightedAverage q (incrementStep g) t (eventIndicator A) x :=
  actual_weighted_expectation q (incrementStep g) t (eventIndicator A) x

theorem weighted_left_margin {n : ℕ} (q : PMF I) (g : I → ZMod n) (a b : I)
    (t : ℕ) (f : ZMod n → ℝ) (x y : ZMod n) :
    weightedAverage (labelPair q) (coupledLabelStep g a b) t (fun s => f s.1) (x,y) =
      weightedAverage q (incrementStep g) t f x := by
  induction t generalizing x y with
  | zero => rfl
  | succ t ih =>
    have hi (s : ZMod n × ZMod n) := ih s.1 s.2
    change pairMean q (fun z => weightedAverage (labelPair q) (coupledLabelStep g a b) t
      (fun s => f s.1) (coupledLabelStep g a b z (x,y))) = _
    simp_rw [hi]
    exact coupled_first q g a b (x,y) (weightedAverage q (incrementStep g) t f)

theorem weighted_right_margin {n : ℕ} (q : PMF I) (g : I → ZMod n) (a b : I)
    (t : ℕ) (f : ZMod n → ℝ) (x y : ZMod n) :
    weightedAverage (labelPair q) (coupledLabelStep g a b) t (fun s => f s.2) (x,y) =
      weightedAverage q (incrementStep g) t f y := by
  induction t generalizing x y with
  | zero => rfl
  | succ t ih =>
    have hi (s : ZMod n × ZMod n) := ih s.1 s.2
    change pairMean q (fun z => weightedAverage (labelPair q) (coupledLabelStep g a b) t
      (fun s => f s.2) (coupledLabelStep g a b z (x,y))) = _
    simp_rw [hi]
    exact coupled_second q g a b (x,y) (weightedAverage q (incrementStep g) t f)

theorem weighted_event_coupling {n : ℕ} [NeZero n] (q : PMF I) (g : I → ZMod n) (a b : I)
    (t : ℕ) (x y : ZMod n) (A : Finset (ZMod n)) :
    abs (probability q g t x A - probability q g t y A) ≤
      weightedAverage (labelPair q) (coupledLabelStep g a b) t mismatch (x,y) := by
  rw [probability_eq,probability_eq]
  have h₁ := weighted_mono (labelPair q) (coupledLabelStep g a b) t (event_gap_pointwise A) (x,y)
  rw [weighted_sub,weighted_left_margin,weighted_right_margin] at h₁
  have hp (s : ZMod n × ZMod n) : eventIndicator A s.2-eventIndicator A s.1 ≤ mismatch s := by
    have := event_gap_pointwise A (s.2,s.1)
    simpa [mismatch,eq_comm] using this
  have h₂ := weighted_mono (labelPair q) (coupledLabelStep g a b) t hp (x,y)
  rw [weighted_sub,weighted_right_margin,weighted_left_margin] at h₂
  exact abs_le.mpr ⟨by linarith,h₁⟩

theorem weighted_mismatch_bound {p : ℕ} [Fact p.Prime] (q : PMF I) (g : I → ZMod p)
    (a b : I) (ha : 0 < weight q a) (hb : 0 < weight q b) (hg : g a ≠ g b)
    {t : ℕ} (ht : 0 < t) (x y : ZMod p) :
    weightedAverage (labelPair q) (coupledLabelStep g a b) t mismatch (x,y) ≤
      (p : ℝ)^2 / (8*weight q a*weight q b*t) := by
  have hc : 0 ≤ 2*weight q a*weight q b := by positivity
  have h := weighted_drift_bound (labelPair q) (coupledLabelStep g a b)
    (normalizedPotential g a b) mismatch (2*weight q a*weight q b) hc
    (fun s => potential_nonneg (normalizedDifference g a b s))
    (mean_mismatch q g a b) (normalized_drift q g a b hg) t (x,y)
  have hv := potential_max (normalizedDifference g a b (x,y))
  have htr : (0 : ℝ) < t := by exact_mod_cast ht
  apply (le_div_iff₀ (by positivity : (0 : ℝ) < 8*weight q a*weight q b*t)).mpr
  change _ ≤ (p : ℝ)^2
  change (2*weight q a*weight q b)*(t : ℝ)*_ ≤
    potential (normalizedDifference g a b (x,y)) at h
  nlinarith

theorem sum_weighted_average {n : ℕ} [NeZero n] (q : PMF I) (g : I → ZMod n)
    (t : ℕ) (f : ZMod n → ℝ) :
    ∑ x, weightedAverage q (incrementStep g) t f x = ∑ x, f x := by
  induction t with
  | zero => rfl
  | succ t ih =>
    simp only [weightedAverage,incrementStep]
    rw [Finset.sum_comm]
    have hs (i : I) : (∑ x : ZMod n, weightedAverage q (incrementStep g) t f (x+g i)) =
        ∑ x : ZMod n, weightedAverage q (incrementStep g) t f x :=
      Fintype.sum_equiv (Equiv.addRight (g i)) _ _ (fun _ => rfl)
    simp_rw [← Finset.mul_sum,hs,ih]
    rw [← Finset.sum_mul,weight_sum,one_mul]

theorem sum_probability {n : ℕ} [NeZero n] (q : PMF I) (g : I → ZMod n)
    (t : ℕ) (A : Finset (ZMod n)) : ∑ x, probability q g t x A = (A.card : ℝ) := by
  simp_rw [probability_eq]
  rw [sum_weighted_average]
  simp [eventIndicator]

theorem prime_weighted_mixing {p : ℕ} [Fact p.Prime] (q : PMF I) (g : I → ZMod p)
    (a b : I) (ha : 0 < weight q a) (hb : 0 < weight q b) (hg : g a ≠ g b)
    {t : ℕ} (ht : 0 < t) (x : ZMod p) (A : Finset (ZMod p)) :
    abs (probability q g t x A - (A.card : ℝ)/p) ≤
      (p : ℝ)^2 / (8*weight q a*weight q b*t) := by
  let B : ℝ := (p : ℝ)^2 / (8*weight q a*weight q b*t)
  have gap (y : ZMod p) : abs (probability q g t x A-probability q g t y A) ≤ B :=
    (weighted_event_coupling q g a b t x y A).trans (weighted_mismatch_bound q g a b ha hb hg ht x y)
  have hu := Finset.sum_le_sum (s := Finset.univ) (fun y _ => (abs_le.mp (gap y)).2)
  have hl := Finset.sum_le_sum (s := Finset.univ) (fun y _ => (abs_le.mp (gap y)).1)
  simp only [Finset.sum_sub_distrib,Finset.sum_const,nsmul_eq_mul,Finset.card_univ,
    ZMod.card,sum_probability] at hu hl
  have hp : (0 : ℝ) < p := by exact_mod_cast (Fact.out : p.Prime).pos
  have he : probability q g t x A-(A.card : ℝ)/p = ((p : ℝ)*probability q g t x A-A.card)/p := by
    field_simp
  rw [he]
  apply abs_le.mpr
  constructor
  · apply (le_div_iff₀ hp).mpr
    change -B*(p : ℝ) ≤ _
    nlinarith
  · apply (div_le_iff₀ hp).mpr
    change _ ≤ B*(p : ℝ)
    nlinarith

/-- Uniform over the modulus and all supports: the constant depends only on fixed weights and ε. -/
theorem hildebrand_uniform_quadratic (q : PMF I) (a b : I)
    (ha : 0 < weight q a) (hb : 0 < weight q b) (ε : ℝ) (hε : 0 < ε) :
    ∃ γ : ℝ, 0 < γ ∧ ∀ (p : ℕ) [Fact p.Prime] (g : I → ZMod p), g a ≠ g b →
      ∀ t : ℕ, γ*(p : ℝ)^2 ≤ t → ∀ (x : ZMod p) (A : Finset (ZMod p)),
      abs (probability q g t x A-(A.card : ℝ)/p) < ε := by
  let w := weight q a*weight q b
  have hw : 0 < w := mul_pos ha hb
  refine ⟨1/(4*w*ε),by positivity,?_⟩
  intro p _ g hg t ht x A
  have hp : (0 : ℝ) < p := by exact_mod_cast (Fact.out : p.Prime).pos
  have htr : (0 : ℝ) < t := lt_of_lt_of_le (by positivity) ht
  have htn : 0 < t := by exact_mod_cast htr
  have ht' : (p : ℝ)^2/(4*w*ε) ≤ t := by simpa only [one_div_mul_eq_div] using ht
  have hc := (div_le_iff₀ (by positivity : (0 : ℝ) < 4*w*ε)).mp ht'
  have hbound : (p : ℝ)^2/(8*w*t) ≤ ε/2 := by
    apply (div_le_iff₀ (by positivity : (0 : ℝ) < 8*w*t)).mpr
    nlinarith
  have hm := prime_weighted_mixing q g a b ha hb hg htn x A
  have hm' : abs (probability q g t x A-(A.card : ℝ)/p) ≤ (p : ℝ)^2/(8*w*t) := by
    simpa only [w,mul_assoc] using hm
  exact lt_of_le_of_lt (hm'.trans hbound) (by linarith)

end WeightedCycleCouplingAudit
