import Mathlib.Analysis.Real.Sqrt
import Mathlib.Analysis.Real.Pi.Bounds
import Mathlib.Data.Fin.VecNotation
import Mathlib.Tactic

/-! Exact report arithmetic on the frozen native totals. This file does not
prove execution authenticity, a native compiler refinement, curve cardinalities
or a limiting collision law. The native session/export provide those recorded
integers; every theorem here is about the explicitly displayed integers. -/
namespace RhoConstantReportAudit
noncomputable section

def normalizedMean (r n total : ℕ) : ℝ := (total:ℝ)/((n:ℝ)*Real.sqrt r)

theorem normalized_interval (r n total : ℕ) (hr : 0<r) (hn : 0<n)
    (lo hi : ℝ) (hl : 0≤lo) (hu : 0≤hi)
    (hlo : (lo*(n:ℝ))^2*(r:ℝ)<(total:ℝ)^2)
    (hhi : (total:ℝ)^2<(hi*(n:ℝ))^2*(r:ℝ)) :
    lo<normalizedMean r n total ∧ normalizedMean r n total<hi := by
  have hs : 0<Real.sqrt (r:ℝ) := Real.sqrt_pos.mpr (Nat.cast_pos.mpr hr)
  have hd : 0<(n:ℝ)*Real.sqrt (r:ℝ) := mul_pos (Nat.cast_pos.mpr hn) hs
  have ht : 0≤(total:ℝ) := Nat.cast_nonneg total
  unfold normalizedMean
  constructor
  · apply (lt_div_iff₀ hd).mpr
    apply (sq_lt_sq₀ (mul_nonneg hl hd.le) ht).mp
    simpa only [mul_pow,Real.sq_sqrt (Nat.cast_nonneg r),mul_assoc] using hlo
  · apply (div_lt_iff₀ hd).mpr
    apply (sq_lt_sq₀ ht (mul_nonneg hu hd.le)).mp
    simpa only [mul_pow,Real.sq_sqrt (Nat.cast_nonneg r),mul_assoc] using hhi

/-- Values copied literally from the frozen native per-curve totals. -/
def subgroupOrders : Fin 4→ℕ := ![16231,65003,261673,1046999]
def negationTotals : Fin 4→ℕ := ![10486,22500,33133,61160]
def plainTotals : Fin 4→ℕ := ![13318,23157,37619,80091]
def setupTotals : Fin 4→ℕ := ![2669,6401,6644,6744]
def searchTotals : Fin 4→ℕ := ![6935,15088,25283,53123]
def verificationTotals : Fin 4→ℕ := ![882,1011,1206,1293]
def emittedUnpricedCounts : Fin 4→ℕ := ![6983,15136,25331,53171]
def lowerBounds : Fin 4→ℝ := ![17147/10000,18385/10000,13493/10000,12452/10000]
def upperBounds : Fin 4→ℝ := ![17148/10000,18386/10000,13494/10000,12453/10000]

theorem recorded_mean_intervals (i : Fin 4) :
    lowerBounds i<normalizedMean (subgroupOrders i) 48 (negationTotals i) ∧
    normalizedMean (subgroupOrders i) 48 (negationTotals i)<upperBounds i := by
  apply normalized_interval
  all_goals fin_cases i <;> norm_num [subgroupOrders,negationTotals,lowerBounds,upperBounds]

theorem exact_phase_totals : ∀ i : Fin 4,
    setupTotals i+searchTotals i+verificationTotals i=negationTotals i := by
  decide +kernel

/-- Same-denominator native emitted-counter band, weights0 and1 only.
It is not a statement about unmeasured work or complete bit-operation cost. -/
theorem emitted_weight_order_flips : ∀ i : Fin 4,
    negationTotals i<plainTotals i ∧
    plainTotals i<negationTotals i+emittedUnpricedCounts i := by
  decide +kernel

theorem ideal_reference_lt_one : Real.sqrt (Real.pi/4)<1 := by
  have hs:=Real.sq_sqrt (show 0≤Real.pi/4 by positivity)
  have hn:=Real.sqrt_nonneg (Real.pi/4)
  nlinarith [Real.pi_lt_four]

/-- A finite observed mean comparison, not an expectation/asymptotic disproof. -/
theorem recorded_means_above_reference (i : Fin 4) :
    Real.sqrt (Real.pi/4)<normalizedMean (subgroupOrders i) 48 (negationTotals i) := by
  have hl : (1:ℝ)<lowerBounds i := by
    fin_cases i <;> norm_num [lowerBounds]
  exact ideal_reference_lt_one.trans (hl.trans (recorded_mean_intervals i).1)

theorem finite_report_arithmetic_certificate :
    (∀ i : Fin 4,lowerBounds i<normalizedMean (subgroupOrders i) 48 (negationTotals i) ∧
      normalizedMean (subgroupOrders i) 48 (negationTotals i)<upperBounds i) ∧
    (∀ i : Fin 4,setupTotals i+searchTotals i+verificationTotals i=negationTotals i) ∧
    (∀ i : Fin 4,negationTotals i<plainTotals i ∧
      plainTotals i<negationTotals i+emittedUnpricedCounts i) ∧
    (∀ i : Fin 4,Real.sqrt (Real.pi/4)<normalizedMean (subgroupOrders i) 48 (negationTotals i)) :=
  ⟨recorded_mean_intervals,exact_phase_totals,emitted_weight_order_flips,
    recorded_means_above_reference⟩

#print axioms normalized_interval
#print axioms finite_report_arithmetic_certificate
end
end RhoConstantReportAudit
