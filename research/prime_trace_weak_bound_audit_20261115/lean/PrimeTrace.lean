module
public import WeakCount
public import Mathlib.Algebra.Field.ZMod

@[expose] public section
namespace PrimeTraceWeakBoundAudit
noncomputable section

variable (p : ℕ) [Fact p.Prime]

/-- The actual trace/gcd definition printed in FST2010 p230. -/
def sourceSupersingular (W : WeierstrassCurve.Affine (ZMod p)) : Prop :=
  Int.gcd (actualTrace W) (p : ℤ) ≠ 1

/-- Prior gcd arithmetic is credited; no Hasse or trace-zero premise. -/
theorem prime_source_trace_divisible (W : WeierstrassCurve.Affine (ZMod p))
    (hss : sourceSupersingular p W) : (p : ℤ) ∣ actualTrace W := by
  apply Int.natCast_dvd.mpr
  by_contra hd
  have hc : Nat.gcd p (actualTrace W).natAbs = 1 :=
    (Fact.out : p.Prime).coprime_iff_not_dvd.mpr hd
  apply hss
  simpa [Int.gcd_def, Nat.gcd_comm] using hc

theorem prime_trace_zero_of_divisible (hp : 5 ≤ p)
    (W : WeierstrassCurve.Affine (ZMod p)) (hΔ : W.Δ ≠ 0)
    (hd : (p : ℤ) ∣ actualTrace W) : actualTrace W = 0 := by
  have hq : 3 < Fintype.card (ZMod p) := by rw [ZMod.card]; omega
  have hF : ringChar (ZMod p) ≠ 2 := by rw [ZMod.ringChar_zmod_n]; omega
  obtain ⟨hl, hu⟩ := actual_weak_trace_bound hq hF W hΔ
  rw [ZMod.card] at hl hu
  by_cases ht : 0 ≤ actualTrace W
  · exact Int.eq_zero_of_dvd_of_nonneg_of_lt ht hu hd
  · have hn : 0 ≤ -actualTrace W := by omega
    have hn' : -actualTrace W < (p : ℤ) := by omega
    have hz := Int.eq_zero_of_dvd_of_nonneg_of_lt hn hn' (dvd_neg.mpr hd)
    omega

theorem prime_source_trace_zero (hp : 5 ≤ p)
    (W : WeierstrassCurve.Affine (ZMod p)) (hΔ : W.Δ ≠ 0)
    (hss : sourceSupersingular p W) : actualTrace W = 0 :=
  prime_trace_zero_of_divisible p hp W hΔ (prime_source_trace_divisible p W hss)

theorem prime_source_iff_trace_zero (hp : 5 ≤ p)
    (W : WeierstrassCurve.Affine (ZMod p)) (hΔ : W.Δ ≠ 0) :
    sourceSupersingular p W ↔ actualTrace W = 0 := by
  constructor
  · exact prime_source_trace_zero p hp W hΔ
  · intro h
    unfold sourceSupersingular
    rw [h, Int.gcd_zero_left, Int.natAbs_natCast]
    omega

theorem prime_source_count (hp : 5 ≤ p)
    (W : WeierstrassCurve.Affine (ZMod p)) (hΔ : W.Δ ≠ 0)
    (hss : sourceSupersingular p W) : Nat.card W.Point = p + 1 := by
  have ht := prime_source_trace_zero p hp W hΔ hss
  unfold actualTrace at ht
  rw [ZMod.card] at ht
  omega

/-- Modular-order conclusion, valid for composite r>2 as well. This theorem
does not define a cyclotomic field or assume the missing point count. -/
theorem prime_source_modular_order (hp : 5 ≤ p)
    (W : WeierstrassCurve.Affine (ZMod p)) (hΔ : W.Δ ≠ 0)
    (hss : sourceSupersingular p W) (r : ℕ) (hr : 2 < r) (hd : r ∣ Nat.card W.Point) :
    orderOf (p : ZMod r) = 2 := by
  rw [prime_source_count p hp W hΔ hss] at hd
  have hz : (p : ZMod r) + 1 = 0 := by
    simpa only [Nat.cast_add, Nat.cast_one] using (ZMod.natCast_eq_zero_iff (p + 1) r).mpr hd
  have hq : (p : ZMod r) = -1 := eq_neg_iff_add_eq_zero.mpr hz
  apply orderOf_eq_prime
  · rw [hq]
    ring
  · intro h
    have htwo : (2 : ZMod r) = 0 := by linear_combination hz - h
    have hdiv : r ∣ 2 := (ZMod.natCast_eq_zero_iff 2 r).mp htwo
    exact (not_le.mpr hr) (Nat.le_of_dvd (by decide) hdiv)

#print axioms prime_trace_zero_of_divisible
#print axioms prime_source_iff_trace_zero
#print axioms prime_source_count
#print axioms prime_source_modular_order
end
end PrimeTraceWeakBoundAudit
