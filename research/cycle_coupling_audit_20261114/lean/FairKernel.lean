module

public import Mathlib.Probability.ProbabilityMassFunction.Constructions
public import Mathlib.Tactic

@[expose] public section

namespace CycleCouplingAudit

noncomputable def fairCoin : PMF Bool :=
  PMF.ofFintype (fun _ => (2 : ENNReal)⁻¹) (by
    simp only [Fintype.sum_bool, ← two_mul]
    exact ENNReal.mul_inv_cancel (by norm_num) (by simp))

noncomputable def fairLaw {S : Type*} (step : Bool → S → S) : ℕ → S → PMF S
  | 0, s => PMF.pure s
  | t + 1, s => fairCoin.bind (fun b => fairLaw step t (step b s))

noncomputable def average {S : Type*} (step : Bool → S → S) : ℕ → (S → ℝ) → S → ℝ
  | 0, f, s => f s
  | t + 1, f, s =>
      (average step t f (step false s) + average step t f (step true s)) / 2

theorem average_nonneg {S : Type*} (step : Bool → S → S) (t : ℕ)
    {f : S → ℝ} (h : ∀ s, 0 ≤ f s) : ∀ s, 0 ≤ average step t f s := by
  induction t with
  | zero => exact h
  | succ t ih => intro s; exact div_nonneg (add_nonneg (ih _) (ih _)) (by norm_num)

theorem average_mono {S : Type*} (step : Bool → S → S) (t : ℕ)
    {f g : S → ℝ} (h : ∀ s, f s ≤ g s) : ∀ s, average step t f s ≤ average step t g s := by
  induction t with
  | zero => exact h
  | succ t ih => intro s; exact div_le_div_of_nonneg_right (add_le_add (ih _) (ih _)) (by norm_num)

theorem average_sub {S : Type*} (step : Bool → S → S) (t : ℕ) (f g : S → ℝ) (s : S) :
    average step t (fun s => f s - g s) s = average step t f s - average step t g s := by
  induction t generalizing s with
  | zero => rfl
  | succ t ih => simp only [average, ih]; ring

theorem average_succ_right {S : Type*} (step : Bool → S → S) (t : ℕ) (f : S → ℝ) (s : S) :
    average step (t + 1) f s =
      average step t (fun s => (f (step false s) + f (step true s)) / 2) s := by
  induction t generalizing s with
  | zero => rfl
  | succ t ih =>
    change (average step (t + 1) f (step false s) + average step (t + 1) f (step true s)) / 2 = _
    rw [ih, ih]
    rfl

noncomputable def expectation {S : Type*} [Fintype S] (p : PMF S) (f : S → ℝ) : ℝ :=
  ∑ s, (p s).toReal * f s

theorem expectation_pure {S : Type*} [Fintype S] (s : S) (f : S → ℝ) :
    expectation (PMF.pure s) f = f s := by
  classical
  simp [expectation, PMF.pure_apply, apply_ite]

theorem expectation_fair_bind {S : Type*} [Fintype S] (p : Bool → PMF S) (f : S → ℝ) :
    expectation (fairCoin.bind p) f = (expectation (p false) f + expectation (p true) f) / 2 := by
  classical
  have ht (b : Bool) (a : S) : (2 : ENNReal)⁻¹ * p b a ≠ ⊤ :=
    ENNReal.mul_ne_top (by simp) ((p b).apply_ne_top a)
  have hpoint (a : S) : (fairCoin.bind p a).toReal =
      ((p false a).toReal + (p true a).toReal) / 2 := by
    simp only [PMF.bind_apply, tsum_fintype, Fintype.sum_bool, fairCoin, PMF.ofFintype_apply]
    rw [ENNReal.toReal_add (ht true a) (ht false a)]
    simp only [ENNReal.toReal_mul, ENNReal.toReal_inv, ENNReal.toReal_ofNat]
    ring
  unfold expectation
  simp_rw [hpoint]
  calc
    _ = ∑ a, ((p false a).toReal * f a + (p true a).toReal * f a) / 2 := by
      apply Finset.sum_congr rfl
      intro a _
      ring
    _ = _ := by rw [← Finset.sum_div, Finset.sum_add_distrib]

theorem actual_expectation {S : Type*} [Fintype S] (step : Bool → S → S)
    (t : ℕ) (f : S → ℝ) (s : S) :
    expectation (fairLaw step t s) f = average step t f s := by
  induction t generalizing s with
  | zero => exact expectation_pure s f
  | succ t ih => rw [fairLaw, expectation_fair_bind]; simp only [ih, average]

/-- Telescoping drift plus coalescence gives a finite-time tail bound. -/
theorem drift_bound {S : Type*} (step : Bool → S → S) (V M : S → ℝ)
    (hV : ∀ s, 0 ≤ V s)
    (hM : ∀ s, (M (step false s) + M (step true s)) / 2 ≤ M s)
    (drift : ∀ s, (V (step false s) + V (step true s)) / 2 = V s - M s)
    (t : ℕ) (s : S) : (t : ℝ) * average step t M s ≤ V s := by
  have mono (i : ℕ) (a : S) : average step (i + 1) M a ≤ average step i M a := by
    rw [average_succ_right]
    exact average_mono step i hM a
  have decr (i : ℕ) (a : S) : average step (i + 1) V a =
      average step i V a - average step i M a := by
    rw [average_succ_right]
    simp_rw [drift]
    exact average_sub step i V M a
  have aux (i : ℕ) : (i : ℝ) * average step i M s ≤ V s - average step i V s := by
    induction i with
    | zero => simp [average]
    | succ i ih =>
      have hm := mono i s
      have hd := decr i s
      have hi : (0 : ℝ) ≤ i := Nat.cast_nonneg i
      rw [Nat.cast_add, Nat.cast_one]
      nlinarith
  have hv := average_nonneg step t hV s
  linarith [aux t]

end CycleCouplingAudit
