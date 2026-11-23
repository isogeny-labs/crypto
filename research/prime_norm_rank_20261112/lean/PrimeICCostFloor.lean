module
public import Mathlib.Analysis.SpecialFunctions.Pow.Real
import Mathlib.Tactic.Linarith
import Mathlib.Tactic.NormNum
import Mathlib.Tactic.Ring
import Mathlib.Tactic.Positivity
public section
namespace PrimeICCostFloor

/-- Universal identity underlying the nonnegative-surplus certificate. -/
theorem surplusIdentity {R : Type*} [CommRing R] (p b t : R) :
    (t*b^3-p)*b+(t-b^4)*t=t^2-p*b := by ring

/-- Model inequality only: these hypotheses are charges, not actual solver lower bounds. -/
theorem floorsNoBeat (p b t : ℝ) (hp : 1<p) (hb : 1≤b) (ht : 0≤t)
    (hcall : b^4≤t) (hyield : p≤t*b^3) : p<t^2 := by
  by_cases hbone : b=1
  · subst b
    have hpt : p≤t := by simpa using hyield
    nlinarith [sq_nonneg (t-1)]
  · have hbg : 1<b := lt_of_le_of_ne hb (Ne.symm hbone)
    have hp0 : 0<p := lt_trans zero_lt_one hp
    have hb0 : 0≤b := le_trans zero_le_one hb
    calc
      p < p*b := by simpa using mul_lt_mul_of_pos_left hbg hp0
      _ ≤ (t*b^3)*b := mul_le_mul_of_nonneg_right hyield hb0
      _ = t*b^4 := by ring
      _ ≤ t*t := mul_le_mul_of_nonneg_left hcall ht
      _ = t^2 := by ring

/-- Optimistic all-output call count for m=8, with a single-target one-call floor. -/
noncomputable def work (p b c : ℝ) : ℝ := max 1 (p/b^7)*c+b^2

theorem workCallFloor (p b c : ℝ) (hc : b^4≤c) : b^4≤work p b c := by
  have hb4 : 0≤b^4 := by nlinarith [sq_nonneg (b^2)]
  have hc0 : 0≤c := le_trans hb4 hc
  have hmul := mul_le_mul_of_nonneg_right (le_max_left 1 (p/b^7)) hc0
  have hcwork : c≤work p b c := by
    dsimp [work]
    nlinarith [sq_nonneg b]
  exact hc.trans hcwork

theorem workYieldFloor (p b c : ℝ) (hb : 1≤b) (hc : b^4≤c) :
    p≤work p b c*b^3 := by
  have hb0 : 0≤b := le_trans zero_le_one hb
  have hbp : 0<b := lt_of_lt_of_le zero_lt_one hb
  have hmax0 : 0≤max 1 (p/b^7) := le_trans zero_le_one (le_max_left _ _)
  have hbase : p≤max 1 (p/b^7)*b^7 :=
    (div_le_iff₀ (pow_pos hbp 7)).mp (le_max_right _ _)
  have hcost : max 1 (p/b^7)*b^4≤max 1 (p/b^7)*c :=
    mul_le_mul_of_nonneg_left hc hmax0
  calc
    p ≤ max 1 (p/b^7)*b^7 := hbase
    _ = (max 1 (p/b^7)*b^4)*b^3 := by ring
    _ ≤ (max 1 (p/b^7)*c)*b^3 := mul_le_mul_of_nonneg_right hcost (pow_nonneg hb0 3)
    _ ≤ work p b c*b^3 := by
      exact mul_le_mul_of_nonneg_right (le_add_of_nonneg_right (sq_nonneg b)) (pow_nonneg hb0 3)

theorem workNoBeat (p b c : ℝ) (hp : 1<p) (hb : 1≤b) (hc : b^4≤c) :
    p<(work p b c)^2 := by
  have hcall := workCallFloor p b c hc
  exact floorsNoBeat p b (work p b c) hp hb
    (le_trans (pow_nonneg (le_trans zero_le_one hb) 4) hcall)
    hcall (workYieldFloor p b c hb hc)

noncomputable def binaryEntropy (s : ℝ) : ℝ :=
  -(s*Real.log s+(1-s)*Real.log (1-s))/Real.log 2
noncomputable def thetaEight : ℝ := 8*binaryEntropy (1/8)

theorem thetaEightFormula : thetaEight=24-7*(Real.log 7/Real.log 2) := by
  have hl2 : Real.log 2≠0 := ne_of_gt (Real.log_pos (by norm_num))
  have hl8 : Real.log 8=3*Real.log 2 := by
    rw [show (8:ℝ)=2^3 by norm_num,Real.log_pow]
    norm_num
  have h1 := Real.log_div (by norm_num : (1:ℝ)≠0) (by norm_num : (8:ℝ)≠0)
  have h7 := Real.log_div (by norm_num : (7:ℝ)≠0) (by norm_num : (8:ℝ)≠0)
  unfold thetaEight binaryEntropy
  rw [show (1:ℝ)-1/8=7/8 by norm_num,h1,h7,Real.log_one,hl8]
  field_simp [hl2]
  ring

theorem thetaEightGtFour : 4<thetaEight := by
  have hl2 : 0<Real.log 2 := Real.log_pos (by norm_num)
  have hlog := Real.log_lt_log (by positivity : (0:ℝ)<7^7)
    (by norm_num : (7:ℝ)^7<2^20)
  rw [Real.log_pow,Real.log_pow] at hlog
  norm_num only [Nat.cast_ofNat] at hlog
  have hd : (7*Real.log 7)/Real.log 2<20 := (div_lt_iff₀ hl2).mpr hlog
  rw [mul_div_assoc] at hd
  rw [thetaEightFormula]
  linarith

theorem thetaEightLtFive : thetaEight<5 := by
  have hl2 : 0<Real.log 2 := Real.log_pos (by norm_num)
  have hlog := Real.log_lt_log (by positivity : (0:ℝ)<2^19)
    (by norm_num : (2:ℝ)^19<7^7)
  rw [Real.log_pow,Real.log_pow] at hlog
  norm_num only [Nat.cast_ofNat] at hlog
  have hd : 19<(7*Real.log 7)/Real.log 2 := (lt_div_iff₀ hl2).mpr hlog
  rw [mul_div_assoc] at hd
  rw [thetaEightFormula]
  linarith

/-- This is the incoming projection before enforcing the batch floor. -/
theorem unclampedExponentBounds : (2:ℝ)/5<2/(9-thetaEight) ∧
    2/(9-thetaEight)<1/2 := by
  have hlo := thetaEightGtFour
  have hhi := thetaEightLtFive
  have hd : 0<9-thetaEight := by linarith
  constructor
  · apply (lt_div_iff₀ hd).mpr
    nlinarith
  · apply (div_lt_iff₀ hd).mpr
    nlinarith

/-- Its balanced factor-base size would demand fewer than one whole solve. -/
theorem unclampedCallExponentNegative : 1-7/(9-thetaEight)<0 := by
  have hlo := thetaEightGtFour
  have hhi := thetaEightLtFive
  have hd : 0<9-thetaEight := by linarith
  have hf : 1<7/(9-thetaEight) := (lt_div_iff₀ hd).mpr (by linarith)
  linarith

theorem entropyChargeFloor (b : ℝ) (hb : 1≤b) : b^4≤b^thetaEight := by
  have h := Real.rpow_le_rpow_of_exponent_le hb (le_of_lt thetaEightGtFour)
  change b^((4:ℕ):ℝ)≤b^thetaEight at h
  rw [Real.rpow_natCast] at h
  exact h

/-- No single-target rho improvement in this declared dense entropy-charge model.
It does not lower-bound an actual sparse solver or exclude another encoding. -/
theorem entropyWorkNoBeat (p b : ℝ) (hp : 1<p) (hb : 1≤b) :
    p<(work p b (b^thetaEight))^2 :=
  workNoBeat p b (b^thetaEight) hp hb (entropyChargeFloor b hb)
end PrimeICCostFloor
#print axioms PrimeICCostFloor.surplusIdentity
#print axioms PrimeICCostFloor.floorsNoBeat
#print axioms PrimeICCostFloor.workNoBeat
#print axioms PrimeICCostFloor.thetaEightFormula
#print axioms PrimeICCostFloor.thetaEightGtFour
#print axioms PrimeICCostFloor.thetaEightLtFive
#print axioms PrimeICCostFloor.unclampedExponentBounds
#print axioms PrimeICCostFloor.unclampedCallExponentNegative
#print axioms PrimeICCostFloor.entropyChargeFloor
#print axioms PrimeICCostFloor.entropyWorkNoBeat
