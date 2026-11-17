import Mathlib.Analysis.Real.Sqrt
import Mathlib.Tactic

/-! The arithmetic last step of Diem2011 Proposition4.11. The geometric
Hasse--Weil/genus/singular-point estimate remains an explicit premise. -/
namespace DiemThresholdAudit
noncomputable section

theorem small_power_bound (n : ℕ) (hn : 1≤n) : n≤2^(n-1) := by
  have h : ∀ m : ℕ,m+1≤2^m := by
    intro m
    induction m with
    | zero => decide
    | succ m ih =>
      rw [pow_succ]
      omega
  have hm := h (n-1)
  omega

theorem squared_error_bound (n q : ℕ) (hn : 2≤n) (hq : 2^(7*n)≤q) :
    (n*2^(n+4))^2≤q := by
  have hp := small_power_bound n (by omega)
  calc
    (n*2^(n+4))^2≤(2^(n-1)*2^(n+4))^2 := by gcongr
    _=2^(4*n+6) := by
      rw [←pow_add,←pow_mul]
      congr 1
      omega
    _≤2^(7*n) := by gcongr; omega
    _≤q := hq

theorem error_absorbed (n q : ℕ) (hn : 2≤n) (hq : 2^(7*n)≤q) :
    (n:ℝ)*2^(n+2)*(Real.sqrt (q:ℝ)+1)≤((q:ℝ)+1)/2 := by
  have hnat := squared_error_bound n q hn hq
  have hc : ((n:ℝ)*2^(n+4))^2≤(q:ℝ) := by exact_mod_cast hnat
  have hq0 : (0:ℝ)≤q := Nat.cast_nonneg q
  have hs0 := Real.sqrt_nonneg (q:ℝ)
  have hs2 := Real.sq_sqrt hq0
  have hq1n : 1≤q := (Nat.one_le_pow _ _ (by decide)).trans hq
  have hq1 : (1:ℝ)≤q := by exact_mod_cast hq1n
  have hs1 : 1≤Real.sqrt (q:ℝ) := by nlinarith
  have ha : (0:ℝ)≤(n:ℝ)*2^(n+2) := by positivity
  have hb : (0:ℝ)≤(n:ℝ)*2^(n+4) := by positivity
  have hroot : (n:ℝ)*2^(n+4)≤Real.sqrt (q:ℝ) := by nlinarith
  have h4 : 4*((n:ℝ)*2^(n+2))≤Real.sqrt (q:ℝ) := by
    rw [show n+4=(n+2)+2 by omega,pow_add] at hroot
    norm_num at hroot
    nlinarith
  have hprod := mul_nonneg hs0 (sub_nonneg.mpr h4)
  have hprod' := mul_nonneg ha (sub_nonneg.mpr hs1)
  nlinarith

theorem half_count_of_geometric_estimate (n q N : ℕ)
    (hn : 2≤n) (hq : 2^(7*n)≤q)
    (hgeom : (q:ℝ)+1-(n:ℝ)*2^(n+2)*(Real.sqrt (q:ℝ)+1)≤N) :
    ((q:ℝ)+1)/2≤(N:ℝ) := by
  have he := error_absorbed n q hn hq
  linarith

theorem half_count_nat_of_geometric_estimate (n q N : ℕ)
    (hn : 2≤n) (hq : 2^(7*n)≤q)
    (hgeom : (q:ℝ)+1-(n:ℝ)*2^(n+2)*(Real.sqrt (q:ℝ)+1)≤N) :
    q+1≤2*N := by
  have h := half_count_of_geometric_estimate n q N hn hq hgeom
  exact_mod_cast (show (q:ℝ)+1≤2*(N:ℝ) by linarith)

theorem small_arithmetic_control :
    (4:ℝ)+1-(2:ℝ)*2^(2+2)*(Real.sqrt (4:ℝ)+1)≤0 ∧
      ¬ (4+1≤2*(0:ℕ)) := by
  norm_num

theorem size_free_arithmetic_inference_false :
    ¬ ∀ n q N : ℕ,2≤n →
      (q:ℝ)+1-(n:ℝ)*2^(n+2)*(Real.sqrt (q:ℝ)+1)≤N → q+1≤2*N := by
  intro h
  exact small_arithmetic_control.2 (h 2 4 0 (by decide) (by norm_num))

theorem diem_threshold_audit_certificate :
    (∀ n q N : ℕ,2≤n → 2^(7*n)≤q →
      (q:ℝ)+1-(n:ℝ)*2^(n+2)*(Real.sqrt (q:ℝ)+1)≤N → q+1≤2*N) ∧
    (¬ ∀ n q N : ℕ,2≤n →
      (q:ℝ)+1-(n:ℝ)*2^(n+2)*(Real.sqrt (q:ℝ)+1)≤N → q+1≤2*N) := by
  exact ⟨half_count_nat_of_geometric_estimate,size_free_arithmetic_inference_false⟩

#print axioms error_absorbed
#print axioms half_count_of_geometric_estimate
#print axioms half_count_nat_of_geometric_estimate
#print axioms size_free_arithmetic_inference_false
#print axioms diem_threshold_audit_certificate
end
end DiemThresholdAudit
