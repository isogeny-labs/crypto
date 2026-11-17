module

public import FairKernel

@[expose] public section

namespace WeightedCycleCouplingAudit

open CycleCouplingAudit

noncomputable def weight {I : Type*} (q : PMF I) (i : I) : ℝ := (q i).toReal

theorem weight_nonneg {I : Type*} (q : PMF I) (i : I) : 0 ≤ weight q i :=
  ENNReal.toReal_nonneg

theorem weight_sum {I : Type*} [Fintype I] (q : PMF I) : ∑ i, weight q i = 1 := by
  have h := q.tsum_coe
  rw [tsum_fintype] at h
  have hr := congrArg ENNReal.toReal h
  rw [ENNReal.toReal_sum (fun i _ => q.apply_ne_top i)] at hr
  exact hr

noncomputable def kernelLaw {I S : Type*} (q : PMF I) (step : I → S → S) : ℕ → S → PMF S
  | 0, s => PMF.pure s
  | t + 1, s => q.bind (fun i => kernelLaw q step t (step i s))

noncomputable def weightedAverage {I S : Type*} [Fintype I] (q : PMF I)
    (step : I → S → S) : ℕ → (S → ℝ) → S → ℝ
  | 0, f, s => f s
  | t + 1, f, s => ∑ i, weight q i * weightedAverage q step t f (step i s)

theorem expectation_bind {I S : Type*} [Fintype I] [Fintype S]
    (q : PMF I) (k : I → PMF S) (f : S → ℝ) :
    expectation (q.bind k) f = ∑ i, weight q i * expectation (k i) f := by
  have point (s : S) : ((q.bind k) s).toReal = ∑ i, weight q i * ((k i) s).toReal := by
    rw [PMF.bind_apply, tsum_fintype,
      ENNReal.toReal_sum (fun i _ => ENNReal.mul_ne_top (q.apply_ne_top i) ((k i).apply_ne_top s))]
    simp only [ENNReal.toReal_mul, weight]
  unfold expectation
  simp_rw [point, Finset.sum_mul]
  rw [Finset.sum_comm]
  apply Finset.sum_congr rfl
  intro i _
  rw [Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro s _
  ring

theorem actual_weighted_expectation {I S : Type*} [Fintype I] [Fintype S]
    (q : PMF I) (step : I → S → S) (t : ℕ) (f : S → ℝ) (s : S) :
    expectation (kernelLaw q step t s) f = weightedAverage q step t f s := by
  induction t generalizing s with
  | zero => exact expectation_pure s f
  | succ t ih => rw [kernelLaw, expectation_bind]; simp only [ih, weightedAverage]

theorem weighted_nonneg {I S : Type*} [Fintype I] (q : PMF I) (step : I → S → S)
    (t : ℕ) {f : S → ℝ} (h : ∀ s, 0 ≤ f s) : ∀ s, 0 ≤ weightedAverage q step t f s := by
  induction t with
  | zero => exact h
  | succ t ih => intro s; exact Finset.sum_nonneg fun i _ => mul_nonneg (weight_nonneg q i) (ih _)

theorem weighted_mono {I S : Type*} [Fintype I] (q : PMF I) (step : I → S → S)
    (t : ℕ) {f g : S → ℝ} (h : ∀ s, f s ≤ g s) :
    ∀ s, weightedAverage q step t f s ≤ weightedAverage q step t g s := by
  induction t with
  | zero => exact h
  | succ t ih =>
    intro s
    exact Finset.sum_le_sum fun i _ => mul_le_mul_of_nonneg_left (ih _) (weight_nonneg q i)

theorem weighted_const {I S : Type*} [Fintype I] (q : PMF I) (step : I → S → S)
    (t : ℕ) (c : ℝ) (s : S) : weightedAverage q step t (fun _ => c) s = c := by
  induction t generalizing s with
  | zero => rfl
  | succ t ih => simp only [weightedAverage, ih, ← Finset.sum_mul, weight_sum, one_mul]

theorem weighted_sub {I S : Type*} [Fintype I] (q : PMF I) (step : I → S → S)
    (t : ℕ) (f g : S → ℝ) (s : S) :
    weightedAverage q step t (fun s => f s - g s) s =
      weightedAverage q step t f s - weightedAverage q step t g s := by
  induction t generalizing s with
  | zero => rfl
  | succ t ih => simp only [weightedAverage, ih, mul_sub, Finset.sum_sub_distrib]

theorem weighted_smul {I S : Type*} [Fintype I] (q : PMF I) (step : I → S → S)
    (t : ℕ) (c : ℝ) (f : S → ℝ) (s : S) :
    weightedAverage q step t (fun s => c * f s) s = c * weightedAverage q step t f s := by
  induction t generalizing s with
  | zero => rfl
  | succ t ih =>
    simp only [weightedAverage, ih]
    rw [Finset.mul_sum]
    apply Finset.sum_congr rfl
    intro i _
    ring

theorem weighted_succ_right {I S : Type*} [Fintype I] (q : PMF I) (step : I → S → S)
    (t : ℕ) (f : S → ℝ) (s : S) :
    weightedAverage q step (t+1) f s =
      weightedAverage q step t (fun s => ∑ i, weight q i * f (step i s)) s := by
  induction t generalizing s with
  | zero => rfl
  | succ t ih =>
    change (∑ i, weight q i * weightedAverage q step (t+1) f (step i s)) = _
    simp_rw [ih]
    rfl

theorem weighted_drift_bound {I S : Type*} [Fintype I] (q : PMF I) (step : I → S → S)
    (V M : S → ℝ) (c : ℝ) (hc : 0 ≤ c) (hV : ∀ s, 0 ≤ V s)
    (hM : ∀ s, (∑ i, weight q i * M (step i s)) ≤ M s)
    (drift : ∀ s, (∑ i, weight q i * V (step i s)) = V s - c * M s)
    (t : ℕ) (s : S) : c * t * weightedAverage q step t M s ≤ V s := by
  have mono (i : ℕ) (a : S) : weightedAverage q step (i+1) M a ≤ weightedAverage q step i M a := by
    rw [weighted_succ_right]
    exact weighted_mono q step i hM a
  have decr (i : ℕ) (a : S) : weightedAverage q step (i+1) V a =
      weightedAverage q step i V a - c * weightedAverage q step i M a := by
    rw [weighted_succ_right]
    simp_rw [drift]
    rw [weighted_sub, weighted_smul]
  have aux (i : ℕ) : c * i * weightedAverage q step i M s ≤ V s - weightedAverage q step i V s := by
    induction i with
    | zero => simp [weightedAverage]
    | succ i ih =>
      have hm := mono i s
      have hd := decr i s
      have hi : (0 : ℝ) ≤ i := Nat.cast_nonneg i
      have h₁ := mul_le_mul_of_nonneg_left hm (mul_nonneg hc hi)
      have h₂ := mul_le_mul_of_nonneg_left hm hc
      rw [Nat.cast_add, Nat.cast_one]
      nlinarith
  have hv := weighted_nonneg q step t hV s
  linarith [aux t]

end WeightedCycleCouplingAudit
