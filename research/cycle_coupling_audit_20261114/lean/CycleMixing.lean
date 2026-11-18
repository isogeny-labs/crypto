module

public import CycleGeometry

@[expose] public section

namespace CycleCouplingAudit

variable {n : ℕ} [NeZero n]

def walkStep (b : Bool) (x : ZMod n) : ZMod n := if b then x + 1 else x

omit [NeZero n] in
theorem left_margin (t : ℕ) (f : ZMod n → ℝ) (x y : ZMod n) :
    average coupleStep t (fun s => f s.1) (x,y) = average walkStep t f x := by
  induction t generalizing x y with
  | zero => rfl
  | succ t ih =>
    change (average coupleStep t (fun s => f s.1) (coupleStep false (x,y)) +
      average coupleStep t (fun s => f s.1) (coupleStep true (x,y))) / 2 = _
    by_cases h : x = y <;> simp only [coupleStep, h, ↓reduceIte] <;>
      rw [ih, ih] <;> rfl

omit [NeZero n] in
theorem right_margin (t : ℕ) (f : ZMod n → ℝ) (x y : ZMod n) :
    average coupleStep t (fun s => f s.2) (x,y) = average walkStep t f y := by
  induction t generalizing x y with
  | zero => rfl
  | succ t ih =>
    change (average coupleStep t (fun s => f s.2) (coupleStep false (x,y)) +
      average coupleStep t (fun s => f s.2) (coupleStep true (x,y))) / 2 = _
    by_cases h : x = y <;> simp only [coupleStep, h, ↓reduceIte] <;> rw [ih, ih]
    · rfl
    · simp only [average, walkStep, Bool.false_eq_true, ↓reduceIte]
      ring

noncomputable def eventIndicator (A : Finset (ZMod n)) (x : ZMod n) : ℝ := if x ∈ A then 1 else 0

noncomputable def walkProbability (t : ℕ) (x : ZMod n) (A : Finset (ZMod n)) : ℝ :=
  expectation (fairLaw walkStep t x) (eventIndicator A)

theorem walkProbability_eq (t : ℕ) (x : ZMod n) (A : Finset (ZMod n)) :
    walkProbability t x A = average walkStep t (eventIndicator A) x :=
  actual_expectation walkStep t (eventIndicator A) x

omit [NeZero n] in
theorem event_gap_pointwise (A : Finset (ZMod n)) (s : ZMod n × ZMod n) :
    eventIndicator A s.1 - eventIndicator A s.2 ≤ mismatch s := by
  unfold eventIndicator mismatch
  by_cases h : s.1 = s.2
  · simp [h]
  · simp only [h, ↓reduceIte]
    split_ifs <;> norm_num

theorem event_coupling (t : ℕ) (x y : ZMod n) (A : Finset (ZMod n)) :
    |walkProbability t x A - walkProbability t y A| ≤ average coupleStep t mismatch (x,y) := by
  rw [walkProbability_eq, walkProbability_eq]
  have h₁ := average_mono coupleStep t (event_gap_pointwise A) (x,y)
  rw [average_sub, left_margin, right_margin] at h₁
  have hp (s : ZMod n × ZMod n) :
      eventIndicator A s.2 - eventIndicator A s.1 ≤ mismatch s := by
    have := event_gap_pointwise A (s.2,s.1)
    simpa [mismatch, eq_comm] using this
  have h₂ := average_mono coupleStep t hp (x,y)
  rw [average_sub, right_margin, left_margin] at h₂
  exact abs_le.mpr ⟨by linarith, h₁⟩

theorem mismatch_bound (hn : 2 ≤ n) {t : ℕ} (ht : 0 < t) (x y : ZMod n) :
    average coupleStep t mismatch (x,y) ≤ (n : ℝ)^2 / (4 * t) := by
  have h := drift_bound coupleStep pairPotential mismatch
    (fun s => potential_nonneg (s.1-s.2)) mismatch_coalescent (pair_drift hn) t (x,y)
  have hv := potential_max (x-y)
  change (t : ℝ) * average coupleStep t mismatch (x,y) ≤ potential (x-y) at h
  have htr : (0 : ℝ) < t := by exact_mod_cast ht
  apply (le_div_iff₀ (by positivity : (0 : ℝ) < 4 * t)).mpr
  nlinarith

theorem two_start_bound (hn : 2 ≤ n) {t : ℕ} (ht : 0 < t)
    (x y : ZMod n) (A : Finset (ZMod n)) :
    |walkProbability t x A - walkProbability t y A| ≤ (n : ℝ)^2 / (4 * t) :=
  (event_coupling t x y A).trans (mismatch_bound hn ht x y)

theorem sum_walk_average (t : ℕ) (f : ZMod n → ℝ) :
    ∑ x, average walkStep t f x = ∑ x, f x := by
  induction t with
  | zero => rfl
  | succ t ih =>
    simp only [average, walkStep, Bool.false_eq_true, ↓reduceIte]
    rw [← Finset.sum_div, Finset.sum_add_distrib]
    have hs : (∑ x : ZMod n, average walkStep t f (x+1)) =
        ∑ x : ZMod n, average walkStep t f x := by
      exact Fintype.sum_equiv (Equiv.addRight (1 : ZMod n))
        (fun x => average walkStep t f (x+1)) (average walkStep t f) (fun _ => rfl)
    rw [hs, ih]
    ring

theorem sum_walkProbability (t : ℕ) (A : Finset (ZMod n)) :
    ∑ x, walkProbability t x A = (A.card : ℝ) := by
  simp_rw [walkProbability_eq]
  rw [sum_walk_average]
  simp [eventIndicator]

/-- Paper-normalized event distance to uniform, for the actual independent-increment PMF. -/
theorem cyclic_mixing_bound (hn : 2 ≤ n) {t : ℕ} (ht : 0 < t)
    (x : ZMod n) (A : Finset (ZMod n)) :
    |walkProbability t x A - (A.card : ℝ) / n| ≤ (n : ℝ)^2 / (4 * t) := by
  let B : ℝ := (n : ℝ)^2 / (4 * t)
  have upper (y : ZMod n) : walkProbability t x A - walkProbability t y A ≤ B :=
    (le_abs_self _).trans (two_start_bound hn ht x y A)
  have lower (y : ZMod n) : -B ≤ walkProbability t x A - walkProbability t y A :=
    (abs_le.mp (two_start_bound hn ht x y A)).1
  have hu := Finset.sum_le_sum (s := Finset.univ) (fun y _ => upper y)
  have hl := Finset.sum_le_sum (s := Finset.univ) (fun y _ => lower y)
  simp only [Finset.sum_sub_distrib, Finset.sum_const, nsmul_eq_mul, Finset.card_univ,
    ZMod.card, sum_walkProbability] at hu hl
  have hnreal : (0 : ℝ) < n := by exact_mod_cast (show 0 < n by omega)
  have he : walkProbability t x A - (A.card : ℝ) / n =
      ((n : ℝ) * walkProbability t x A - A.card) / n := by
    field_simp
  rw [he]
  apply abs_le.mpr
  constructor
  · apply (le_div_iff₀ hnreal).mpr
    change -B * (n : ℝ) ≤ _
    nlinarith
  · apply (div_le_iff₀ hnreal).mpr
    change _ ≤ B * (n : ℝ)
    nlinarith

end CycleCouplingAudit
