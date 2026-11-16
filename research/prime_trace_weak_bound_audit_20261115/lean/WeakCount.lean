module
public import ActualFibers

@[expose] public section
namespace PrimeTraceWeakBoundAudit
noncomputable section
open Polynomial
open scoped BigOperators
variable {K : Type*} [Field K] [Fintype K]

omit [Fintype K] in
theorem two_ne_zero_of_odd_char (hF : ringChar K ≠ 2) : (2 : K) ≠ 0 := by
  intro h
  apply Ring.neg_one_ne_one_of_char_ne_two hF
  linear_combination -h

theorem square_fiber_le_two (h2 : (2 : K) ≠ 0) (z : K) : squareFiberCard z ≤ 2 := by
  classical
  by_cases hz : z = 0
  · rw [hz, square_fiber_zero]
    decide
  by_cases hs : IsSquare z
  · rw [square_fiber_square h2 z hz hs]
  · rw [square_fiber_nonsquare z hs]
    exact Nat.zero_le _

theorem fiber_zero_euler (hF : ringChar K ≠ 2) (z : K) (hc : squareFiberCard z = 0) :
    z ^ ((Fintype.card K - 1) / 2) = -1 := by
  have h2 := two_ne_zero_of_odd_char hF
  have hz : z ≠ 0 := by
    intro h
    rw [h, square_fiber_zero] at hc
    contradiction
  have hs : ¬ IsSquare z := by
    intro h
    rw [square_fiber_square h2 z hz h] at hc
    contradiction
  have hp := (FiniteField.pow_dichotomy hF hz).resolve_left
    (fun h => hs ((FiniteField.isSquare_iff hF hz).mpr h))
  have hh : Fintype.card K / 2 = (Fintype.card K - 1) / 2 := by
    have ho := FiniteField.odd_card_of_char_ne_two hF
    omega
  simpa only [hh] using hp

theorem fiber_two_euler (hF : ringChar K ≠ 2) (z : K) (hc : squareFiberCard z = 2) :
    z ^ ((Fintype.card K - 1) / 2) = 1 := by
  classical
  have hz : z ≠ 0 := by
    intro h
    rw [h, square_fiber_zero] at hc
    contradiction
  have hs : IsSquare z := by
    by_contra h
    rw [square_fiber_nonsquare z h] at hc
    contradiction
  have hp := (FiniteField.isSquare_iff hF hz).mp hs
  have hh : Fintype.card K / 2 = (Fintype.card K - 1) / 2 := by
    have ho := FiniteField.odd_card_of_char_ne_two hF
    omega
  simpa only [hh] using hp

/-- An actual strict point bound, derived without a Hasse estimate. -/
theorem actual_weak_point_bound (hq : 3 < Fintype.card K) (hF : ringChar K ≠ 2)
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    1 < Nat.card W.Point ∧ Nat.card W.Point < 2 * Fintype.card K + 1 := by
  classical
  have h2 := two_ne_zero_of_odd_char hF
  let f := completedCubic W
  have hf : f.natDegree = 3 := completed_cubic_degree W
  obtain ⟨x, hx⟩ := not_forall.mp (cubic_euler_nonconstant hq hF f hf (-1))
  have hxpos : 0 < squareFiberCard (f.eval x) := Nat.pos_of_ne_zero
    (fun h => hx (fiber_zero_euler hF _ h))
  have hsumpos : 0 < ∑ y : K, squareFiberCard (f.eval y) :=
    lt_of_lt_of_le hxpos (Finset.single_le_sum
      (f := fun y : K => squareFiberCard (f.eval y)) (fun _ _ => Nat.zero_le _)
      (Finset.mem_univ x))
  obtain ⟨y, hy⟩ := not_forall.mp (cubic_euler_nonconstant hq hF f hf 1)
  have hylt : squareFiberCard (f.eval y) < 2 := by
    have hle := square_fiber_le_two h2 (f.eval y)
    have hne : squareFiberCard (f.eval y) ≠ 2 := fun h => hy (fiber_two_euler hF _ h)
    omega
  have hsumlt : (∑ x : K, squareFiberCard (f.eval x)) < ∑ _ : K, (2 : ℕ) :=
    Finset.sum_lt_sum (fun x _ => square_fiber_le_two h2 (f.eval x))
      ⟨y, Finset.mem_univ y, hylt⟩
  have htwos : (∑ _ : K, (2 : ℕ)) = 2 * Fintype.card K := by simp [Nat.mul_comm]
  rw [htwos] at hsumlt
  rw [actual_point_fiber_count W hΔ h2]
  change 1 < (∑ x : K, squareFiberCard (f.eval x)) + 1 ∧
    (∑ x : K, squareFiberCard (f.eval x)) + 1 < 2 * Fintype.card K + 1
  omega

def actualTrace (W : WeierstrassCurve.Affine K) : ℤ :=
  (Fintype.card K : ℤ) + 1 - (Nat.card W.Point : ℤ)

theorem actual_weak_trace_bound (hq : 3 < Fintype.card K) (hF : ringChar K ≠ 2)
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) :
    -(Fintype.card K : ℤ) < actualTrace W ∧ actualTrace W < (Fintype.card K : ℤ) := by
  obtain ⟨hl, hu⟩ := actual_weak_point_bound hq hF W hΔ
  have hl' : (1 : ℤ) < Nat.card W.Point := by exact_mod_cast hl
  have hu' : (Nat.card W.Point : ℤ) < 2 * (Fintype.card K : ℤ) + 1 := by exact_mod_cast hu
  unfold actualTrace
  omega

#print axioms actual_weak_point_bound
#print axioms actual_weak_trace_bound
end
end PrimeTraceWeakBoundAudit
