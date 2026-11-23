module

public import CubicCurveCount

@[expose] public section

namespace CubicPermutationCountAudit
noncomputable section
instance primeTwo : Fact (Nat.Prime 2) := ⟨by decide⟩
instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
instance primeSeven : Fact (Nat.Prime 7) := ⟨by decide⟩

/-- Credited elementary argument: the existing CompositeTorsionAudit
    trace-zero order proof, here with no curve-count premise assumed. -/
theorem divisor_order_two (q r : ℕ) (hr : 2 < r) (hd : r ∣ q + 1) :
    orderOf (q : ZMod r) = 2 := by
  have hz : (q : ZMod r) + 1 = 0 := by
    simpa only [Nat.cast_add, Nat.cast_one] using
      (ZMod.natCast_eq_zero_iff (q + 1) r).mpr hd
  have hq : (q : ZMod r) = -1 := (eq_neg_iff_add_eq_zero).mpr hz
  apply orderOf_eq_prime
  · rw [hq]
    ring
  · rw [hq]
    intro hh
    rcases ZMod.neg_one_eq_one_iff.mp hh with hh | hh <;> omega

variable {K : Type*} [Field K] [Fintype K]

def pointTrace (b : K) : ℤ := (Fintype.card K : ℤ) + 1 - Nat.card (curve b).Point

theorem actual_trace_zero (hq : Fintype.card K % 3 = 2)
    (h2 : (2 : K) ≠ 0) (h3 : (3 : K) ≠ 0) (b : K) (hb : b ≠ 0) :
    pointTrace b = 0 := by
  unfold pointTrace
  rw [actual_point_count hq h2 h3 b hb]
  push_cast
  ring

theorem actual_divisor_order (hq : Fintype.card K % 3 = 2)
    (h2 : (2 : K) ≠ 0) (h3 : (3 : K) ≠ 0) (b : K) (hb : b ≠ 0)
    (r : ℕ) (hr : 2 < r) (hd : r ∣ Nat.card (curve b).Point) :
    orderOf (Fintype.card K : ZMod r) = 2 := by
  apply divisor_order_two _ r hr
  simpa only [actual_point_count hq h2 h3 b hb] using hd

theorem actual_point_order_embedding [DecidableEq K] (hq : Fintype.card K % 3 = 2)
    (h2 : (2 : K) ≠ 0) (h3 : (3 : K) ≠ 0) (b : K) (hb : b ≠ 0)
    (P : (curve b).Point) (hP : 2 < addOrderOf P) :
    orderOf (Fintype.card K : ZMod (addOrderOf P)) = 2 := by
  let : Fintype (curve b).Point :=
    Fintype.ofEquiv (Option K) (actualPointEquiv hq b (curve_nonsingular h2 h3 b hb)).symm
  exact actual_divisor_order hq h2 h3 b hb (addOrderOf P) hP (addOrderOf_dvd_natCard P)

instance curveNonsingularDecidable {J : Type*} [Field J] [DecidableEq J] (b : J) :
    DecidablePred (fun xy : J × J => (curve b).Nonsingular xy.1 xy.2) := fun xy =>
  decidable_of_iff
    (xy.2 ^ 2 = xy.1 ^ 3 + b ∧ ((0 : J) ≠ 3 * xy.1 ^ 2 ∨ xy.2 ≠ -xy.2)) (by
      change (xy.2 ^ 2 = xy.1 ^ 3 + b ∧ ((0 : J) ≠ 3 * xy.1 ^ 2 ∨ xy.2 ≠ -xy.2)) ↔
        (curve b).Nonsingular xy.1 xy.2
      rw [WeierstrassCurve.Affine.nonsingular_iff, curve_equation]
      simp [curve])

theorem prime_field_count (p : ℕ) [Fact (Nat.Prime p)] (hp : 5 ≤ p)
    (hq : p % 3 = 2) (b : ZMod p) (hb : b ≠ 0) :
    (curve b).Δ ≠ 0 ∧ Nat.card (curve b).Point = p + 1 ∧ pointTrace b = 0 ∧
    (∀ r : ℕ, 2 < r → r ∣ Nat.card (curve b).Point → orderOf (p : ZMod r) = 2) := by
  have h2 : (2 : ZMod p) ≠ 0 := by
    intro h
    have hd := (ZMod.natCast_eq_zero_iff 2 p).mp h
    have := Nat.le_of_dvd (by decide : 0 < 2) hd
    omega
  have h3 : (3 : ZMod p) ≠ 0 := by
    intro h
    have hd := (ZMod.natCast_eq_zero_iff 3 p).mp h
    have := Nat.le_of_dvd (by decide : 0 < 3) hd
    omega
  have hc : Fintype.card (ZMod p) % 3 = 2 := by simpa only [ZMod.card] using hq
  refine ⟨curve_nonsingular h2 h3 b hb, ?_, actual_trace_zero hc h2 h3 b hb, ?_⟩
  · simpa only [ZMod.card] using actual_point_count hc h2 h3 b hb
  · intro r hr hd
    simpa only [ZMod.card] using actual_divisor_order hc h2 h3 b hb r hr hd

theorem coefficient_zero_count : Nat.card (curve (0 : ZMod 5)).Point = 5 := by
  rw [Nat.card_congr (curve (0 : ZMod 5)).nonsingularPointEquiv]
  change Nat.card (Option {xy : ZMod 5 × ZMod 5 //
    (curve (0 : ZMod 5)).Nonsingular xy.1 xy.2}) = 5
  let : DecidablePred (fun xy : ZMod 5 × ZMod 5 =>
    (curve (0 : ZMod 5)).Nonsingular xy.1 xy.2) := curveNonsingularDecidable 0
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem characteristic_two_count : Nat.card (curve (1 : ZMod 2)).Point = 2 := by
  rw [Nat.card_congr (curve (1 : ZMod 2)).nonsingularPointEquiv]
  change Nat.card (Option {xy : ZMod 2 × ZMod 2 //
    (curve (1 : ZMod 2)).Nonsingular xy.1 xy.2}) = 2
  let : DecidablePred (fun xy : ZMod 2 × ZMod 2 =>
    (curve (1 : ZMod 2)).Nonsingular xy.1 xy.2) := curveNonsingularDecidable 1
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem congruence_one_count : Nat.card (curve (1 : ZMod 7)).Point = 12 := by
  rw [Nat.card_congr (curve (1 : ZMod 7)).nonsingularPointEquiv]
  change Nat.card (Option {xy : ZMod 7 × ZMod 7 //
    (curve (1 : ZMod 7)).Nonsingular xy.1 xy.2}) = 12
  let : DecidablePred (fun xy : ZMod 7 × ZMod 7 =>
    (curve (1 : ZMod 7)).Nonsingular xy.1 xy.2) := curveNonsingularDecidable 1
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem deleting_coefficient_guard_false :
    ¬∀ (b : ZMod 5), Nat.card (curve b).Point = 6 := by
  intro h
  have hh := h 0
  rw [coefficient_zero_count] at hh
  contradiction

theorem deleting_characteristic_guard_false :
    ¬Nat.card (curve (1 : ZMod 2)).Point = Fintype.card (ZMod 2) + 1 := by
  rw [characteristic_two_count, ZMod.card]
  decide

theorem deleting_congruence_guard_false :
    ¬Nat.card (curve (1 : ZMod 7)).Point = Fintype.card (ZMod 7) + 1 := by
  rw [congruence_one_count, ZMod.card]
  decide

theorem r_two_control :
    (curve (1 : ZMod 5)).Δ ≠ 0 ∧ Nat.card (curve (1 : ZMod 5)).Point = 6 ∧
    2 ∣ Nat.card (curve (1 : ZMod 5)).Point ∧ orderOf (5 : ZMod 2) = 1 ∧
    ¬orderOf (5 : ZMod 2) = 2 := by
  have h := prime_field_count 5 (by decide) (by decide) 1 (by decide)
  refine ⟨h.1, h.2.1, ?_, ?_, ?_⟩
  · rw [h.2.1]
    decide
  · rw [show (5 : ZMod 2) = 1 by decide, orderOf_one]
  · rw [show (5 : ZMod 2) = 1 by decide, orderOf_one]
    decide

theorem cubic_permutation_count_certificate :
    (∀ (K : Type*) [Field K] [Fintype K] [DecidableEq K],
      Fintype.card K % 3 = 2 → (2 : K) ≠ 0 → (3 : K) ≠ 0 →
      ∀ b : K, b ≠ 0 →
        Function.Bijective (fun x : K => x ^ 3) ∧ (curve b).Δ ≠ 0 ∧
        Nonempty ((curve b).Point ≃ Option K) ∧
        Nat.card (curve b).Point = Fintype.card K + 1 ∧ pointTrace b = 0 ∧
        (∀ r : ℕ, 2 < r → r ∣ Nat.card (curve b).Point →
          orderOf (Fintype.card K : ZMod r) = 2) ∧
        (∀ P : (curve b).Point, 2 < addOrderOf P →
          orderOf (Fintype.card K : ZMod (addOrderOf P)) = 2)) ∧
    (∀ (p : ℕ) [Fact (Nat.Prime p)], 5 ≤ p → p % 3 = 2 →
      ∀ b : ZMod p, b ≠ 0 → (curve b).Δ ≠ 0 ∧ Nat.card (curve b).Point = p + 1 ∧
        pointTrace b = 0 ∧
        (∀ r : ℕ, 2 < r → r ∣ Nat.card (curve b).Point → orderOf (p : ZMod r) = 2)) ∧
    (Nat.card (curve (0 : ZMod 5)).Point = 5 ∧
      ¬∀ b : ZMod 5, Nat.card (curve b).Point = 6) ∧
    (Nat.card (curve (1 : ZMod 2)).Point = 2 ∧
      ¬Nat.card (curve (1 : ZMod 2)).Point = Fintype.card (ZMod 2) + 1) ∧
    (Nat.card (curve (1 : ZMod 7)).Point = 12 ∧
      ¬Nat.card (curve (1 : ZMod 7)).Point = Fintype.card (ZMod 7) + 1) ∧
    ((curve (1 : ZMod 5)).Δ ≠ 0 ∧ Nat.card (curve (1 : ZMod 5)).Point = 6 ∧
      2 ∣ Nat.card (curve (1 : ZMod 5)).Point ∧ orderOf (5 : ZMod 2) = 1 ∧
      ¬orderOf (5 : ZMod 2) = 2) := by
  refine ⟨?_, ?_, ⟨coefficient_zero_count, deleting_coefficient_guard_false⟩,
    ⟨characteristic_two_count, deleting_characteristic_guard_false⟩,
    ⟨congruence_one_count, deleting_congruence_guard_false⟩, r_two_control⟩
  · intro K _ _ _ hq h2 h3 b hb
    exact ⟨cube_bijective hq, curve_nonsingular h2 h3 b hb,
      ⟨actualPointEquiv hq b (curve_nonsingular h2 h3 b hb)⟩,
      actual_point_count hq h2 h3 b hb, actual_trace_zero hq h2 h3 b hb,
      (fun r hr hd => actual_divisor_order hq h2 h3 b hb r hr hd),
      (fun P hP => actual_point_order_embedding hq h2 h3 b hb P hP)⟩
  · intro p _ hp hq b hb
    exact prime_field_count p hp hq b hb

#print axioms prime_field_count
#print axioms actual_point_order_embedding
#print axioms coefficient_zero_count
#print axioms characteristic_two_count
#print axioms congruence_one_count
#print axioms r_two_control
#print axioms cubic_permutation_count_certificate
end
end CubicPermutationCountAudit
