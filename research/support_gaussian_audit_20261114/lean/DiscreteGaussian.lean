module

public import UpperBoundCertificate
public import Mathlib.Analysis.SumIntegralComparisons
public import Mathlib.Analysis.SpecialFunctions.Gaussian.GaussianIntegral
public import Mathlib.Analysis.Normed.Ring.InfiniteSum
public import Mathlib.Topology.Algebra.InfiniteSum.Real

@[expose] public section

namespace SupportGaussianAudit

open Real Filter MeasureTheory Set
open scoped Topology

noncomputable def gaussianTerm (a : ℝ) (n : ℕ) : ℝ := exp (-a*(n : ℝ)^2)
noncomputable def theta (a : ℝ) : ℝ := ∑' n : ℕ, gaussianTerm a n
noncomputable def geometricClosed (a : ℝ) : ℝ := 1/(1-exp (-a))

theorem geometric_hasSum {a : ℝ} (ha : 0<a) :
    HasSum (fun n : ℕ => (exp (-a))^n) (geometricClosed a) := by
  simpa only [geometricClosed,one_div] using
    hasSum_geometric_of_lt_one (exp_pos (-a)).le (exp_lt_one_iff.mpr (by linarith))

theorem gaussian_le_geometric {a : ℝ} (ha : 0<a) (n : ℕ) : gaussianTerm a n ≤ (exp (-a))^n := by
  have hn : (n : ℝ) ≤ (n : ℝ)^2 := by
    cases n with
    | zero => norm_num
    | succ n =>
      have h : (0 : ℝ) ≤ n := Nat.cast_nonneg n
      push_cast
      nlinarith
  rw [gaussianTerm,← Real.exp_nat_mul]
  apply exp_le_exp.mpr
  have hm := mul_le_mul_of_nonpos_left hn (show -a ≤ 0 by linarith)
  nlinarith

theorem gaussian_summable {a : ℝ} (ha : 0<a) : Summable (gaussianTerm a) :=
  Summable.of_nonneg_of_le (fun n => show 0 ≤ gaussianTerm a n from (exp_pos _).le) (gaussian_le_geometric ha)
    (geometric_hasSum ha).summable

theorem theta_strictly_below_geometric {a : ℝ} (ha : 0<a) : theta a<geometricClosed a := by
  have hs : gaussianTerm a 2<(exp (-a))^2 := by
    rw [gaussianTerm,← Real.exp_nat_mul]
    apply exp_lt_exp.mpr
    norm_num
    linarith
  unfold theta
  rw [← (geometric_hasSum ha).tsum_eq]
  exact (gaussian_summable ha).tsum_lt_tsum (gaussian_le_geometric ha) hs (geometric_hasSum ha).summable

theorem theta_at_least_one {a : ℝ} (ha : 0<a) : 1 ≤ theta a := by
  have h := (gaussian_summable ha).sum_le_tsum ({0} : Finset ℕ) (fun n _ => (exp_pos _).le)
  simpa [theta,gaussianTerm] using h

theorem gaussian_antitone {a : ℝ} (ha : 0<a) :
    AntitoneOn (fun x : ℝ => exp (-a*x^2)) (Ici 0) := by
  intro x hx y hy hxy
  apply exp_le_exp.mpr
  have hs := pow_le_pow_left₀ hx hxy 2
  have hm := mul_le_mul_of_nonpos_left hs (show -a ≤ 0 by linarith)
  exact hm

theorem gaussian_positive_series_bound {a : ℝ} (ha : 0<a) :
    (∑' n : ℕ, gaussianTerm a (n+1)) ≤ Real.sqrt (Real.pi/a)/2 := by
  have hi : IntegrableOn (fun x : ℝ => exp (-a*x^2)) (Ioi 0) :=
    (integrable_exp_neg_mul_sq ha).integrableOn
  have h := (gaussian_antitone ha).tsum_add_one_le_integral hi (fun x _ => (exp_pos _).le)
  rw [integral_gaussian_Ioi a] at h
  exact h

theorem gaussian_zero_not_summable : ¬ Summable (gaussianTerm 0) := by
  intro h
  have ht := h.tendsto_atTop_zero
  have he : gaussianTerm 0=(fun _ : ℕ => (1 : ℝ)) := by funext n; simp [gaussianTerm]
  rw [he] at ht
  have hc : Tendsto (fun _ : ℕ => (1 : ℝ)) atTop (𝓝 1) := tendsto_const_nhds
  have hf := tendsto_nhds_unique ht hc
  norm_num at hf

noncomputable def doubleGaussian (a : ℝ) : ℝ :=
  ∑' v : ℕ × ℕ, exp (-a*((v.1 : ℝ)^2+(v.2 : ℝ)^2))

theorem gaussian_norm_summable {a : ℝ} (ha : 0<a) :
    Summable (fun n : ℕ => ‖gaussianTerm a n‖) := by
  have h : Summable (fun n : ℕ => gaussianTerm a n) := gaussian_summable ha
  simpa only [gaussianTerm,Real.norm_eq_abs,abs_exp] using h

theorem double_gaussian_summable {a : ℝ} (ha : 0<a) :
    Summable (fun v : ℕ × ℕ => exp (-a*((v.1 : ℝ)^2+(v.2 : ℝ)^2))) := by
  simpa only [gaussianTerm,mul_add,exp_add] using
    summable_mul_of_summable_norm (gaussian_norm_summable ha) (gaussian_norm_summable ha)

theorem double_gaussian_factorization {a : ℝ} (ha : 0<a) : doubleGaussian a=(theta a)^2 := by
  have h := tsum_mul_tsum_of_summable_norm (gaussian_norm_summable ha) (gaussian_norm_summable ha)
  simpa only [doubleGaussian,theta,gaussianTerm,mul_add,exp_add,pow_two] using h.symm

theorem double_gaussian_strict {a : ℝ} (ha : 0<a) : doubleGaussian a<(geometricClosed a)^2 := by
  have ht := theta_strictly_below_geometric ha
  have hp := theta_at_least_one ha
  have hm : 0<(geometricClosed a-theta a)*(geometricClosed a+theta a) :=
    mul_pos (by linarith) (by linarith)
  rw [double_gaussian_factorization ha]
  nlinarith

theorem literal_gaussian_geometric_equality_false {a : ℝ} (ha : 0<a) :
    ¬ doubleGaussian a=1/(1-exp (-a))^2 := by
  have h := double_gaussian_strict ha
  rw [geometricClosed,div_pow,one_pow] at h
  exact ne_of_lt h

noncomputable def printedExponentThree (p t : ℕ) (ε c : ℝ) : ℝ :=
  (t : ℝ)*c*(ε/2)^((2 : ℝ)/((3 : ℝ)-1))/(4*(p : ℝ)^((2 : ℝ)/((3 : ℝ)-1)))

noncomputable def printedSeriesCoefficient (p : ℕ) (ε : ℝ) : ℝ := 1.1*ε/(2*(p : ℝ))

theorem printed_exponent_three (p t : ℕ) (ε c : ℝ) :
    printedExponentThree p t ε c=(t : ℝ)*c*(ε/2)/(4*(p : ℝ)) := by
  norm_num [printedExponentThree]

theorem printed_exponent_positive {p t : ℕ} (hp : 0<p) (ht : 0<t)
    {ε c : ℝ} (hε : 0<ε) (hc : 0<c) : 0<printedExponentThree p t ε c := by
  rw [printed_exponent_three]
  positivity

theorem printed_series_step_strict {p t : ℕ} (hp : 0<p) (ht : 0<t)
    {ε c : ℝ} (hε : 0<ε) (hc : 0<c) (B : ℝ) :
    B+printedSeriesCoefficient p ε*doubleGaussian (printedExponentThree p t ε c) <
      B+printedSeriesCoefficient p ε/(1-exp (-printedExponentThree p t ε c))^2 := by
  have hC : 0<printedSeriesCoefficient p ε := by unfold printedSeriesCoefficient; positivity
  have ha := printed_exponent_positive hp ht hε hc
  calc
    _ < B+printedSeriesCoefficient p ε*(geometricClosed (printedExponentThree p t ε c))^2 :=
      by simpa only [add_comm] using add_lt_add_left (mul_lt_mul_of_pos_left (double_gaussian_strict ha) hC) B
    _ = _ := by rw [geometricClosed,div_pow,one_pow]; ring

theorem printed_series_equality_false {p t : ℕ} (hp : 0<p) (ht : 0<t)
    {ε c : ℝ} (hε : 0<ε) (hc : 0<c) (B : ℝ) :
    ¬ B+printedSeriesCoefficient p ε*doubleGaussian (printedExponentThree p t ε c) =
      B+printedSeriesCoefficient p ε/(1-exp (-printedExponentThree p t ε c))^2 :=
  ne_of_lt (printed_series_step_strict hp ht hε hc B)

noncomputable def gaussianInteger (a : ℝ) (z : ℤ) : ℝ :=
  if z=0 then 0 else exp (-a*(z : ℝ)^2)

theorem gaussian_integer_nonneg (a : ℝ) (z : ℤ) : 0 ≤ gaussianInteger a z := by
  unfold gaussianInteger
  split_ifs <;> positivity

theorem gaussian_integer_nat_succ (a : ℝ) (n : ℕ) :
    gaussianInteger a ((n+1 : ℕ) : ℤ)=gaussianTerm a (n+1) := by
  have hz : ((n+1 : ℕ) : ℤ) ≠ 0 := by omega
  rw [gaussianInteger,ite_eq_right hz,gaussianTerm]
  simp only [Int.cast_natCast]

theorem gaussian_integer_neg_succ (a : ℝ) (n : ℕ) :
    gaussianInteger a (-((n+1 : ℕ) : ℤ))=gaussianTerm a (n+1) := by
  have hz : -((n+1 : ℕ) : ℤ) ≠ 0 := by omega
  rw [gaussianInteger,ite_eq_right hz,gaussianTerm]
  simp only [Int.cast_neg,Int.cast_natCast,neg_sq]

theorem gaussian_integer_neg_succ_int (a : ℝ) (n : ℕ) :
    gaussianInteger a (-((n : ℤ)+1))=gaussianTerm a (n+1) := by
  have he : -((n : ℤ)+1)=-((n+1 : ℕ) : ℤ) := by omega
  rw [he]
  exact gaussian_integer_neg_succ a n

theorem gaussian_integer_summable {a : ℝ} (ha : 0<a) : Summable (gaussianInteger a) := by
  have hs : Summable (fun n : ℕ => gaussianTerm a (n+1)) :=
    (summable_nat_add_iff 1).mpr (gaussian_summable ha)
  have hn : Summable (fun n : ℕ => gaussianInteger a (n : ℤ)) := by
    apply (summable_nat_add_iff 1).mp
    simpa only [gaussian_integer_nat_succ] using hs
  have hneg : Summable (fun n : ℕ => gaussianInteger a (-(n+1))) := by
    simpa only [gaussian_integer_neg_succ_int] using hs
  exact hn.of_nat_of_neg_add_one hneg

theorem gaussian_integer_bound {a : ℝ} (ha : 0<a) :
    (∑' z : ℤ, gaussianInteger a z) ≤ Real.sqrt (Real.pi/a) := by
  have hs : Summable (fun n : ℕ => gaussianTerm a (n+1)) :=
    (summable_nat_add_iff 1).mpr (gaussian_summable ha)
  have hn : Summable (fun n : ℕ => gaussianInteger a (n : ℤ)) := by
    apply (summable_nat_add_iff 1).mp
    simpa only [gaussian_integer_nat_succ] using hs
  have hneg : Summable (fun n : ℕ => gaussianInteger a (-(n+1))) := by
    simpa only [gaussian_integer_neg_succ_int] using hs
  rw [tsum_of_nat_of_neg_add_one hn hneg,hn.tsum_eq_zero_add]
  simp only [gaussian_integer_nat_succ]
  have hzero : gaussianInteger a ((0 : ℕ) : ℤ)=0 := by norm_num [gaussianInteger]
  rw [hzero,zero_add]
  have hne : (fun n : ℕ => gaussianInteger a (-(n+1)))=(fun n => gaussianTerm a (n+1)) := by
    funext n
    exact gaussian_integer_neg_succ_int a n
  rw [hne]
  have hb := gaussian_positive_series_bound ha
  linarith

theorem finite_integer_gaussian_bound {a : ℝ} (ha : 0<a) (S : Finset ℤ) (hS : 0 ∉ S) :
    (∑ z ∈ S, exp (-a*(z : ℝ)^2)) ≤ Real.sqrt (Real.pi/a) := by
  have he : (∑ z ∈ S, exp (-a*(z : ℝ)^2))=∑ z ∈ S, gaussianInteger a z := by
    apply Finset.sum_congr rfl
    intro z hz
    have hn : z ≠ 0 := by intro h; exact hS (h ▸ hz)
    simp [gaussianInteger,hn]
  rw [he]
  exact ((gaussian_integer_summable ha).sum_le_tsum S (fun z _ => gaussian_integer_nonneg a z)).trans
    (gaussian_integer_bound ha)

theorem zero_exponent_finite_bound_false :
    ¬ (∑ z ∈ ({1} : Finset ℤ), exp (-(0 : ℝ)*(z : ℝ)^2)) ≤ Real.sqrt (Real.pi/(0 : ℝ)) := by
  norm_num

end SupportGaussianAudit
