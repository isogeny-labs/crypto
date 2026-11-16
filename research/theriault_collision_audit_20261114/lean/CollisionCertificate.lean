import FiniteUrn
import RecurrenceBounds

namespace TheriaultCollisionAudit

/-- The finite source counterexample is unconditional; generic repairs expose their hypotheses. -/
theorem actual_collision_bound_certificate :
    Function.Involutive negate ∧
    (∀ p : Pool, negate p ≠ p) ∧
    (∀ p q : Pool, p.1 = q.1 ↔ p = q ∨ p = negate q) ∧
    Fintype.card Pool = 8 ∧ Fintype.card Samples = 512 ∧
    totalIntersections = 352 ∧ uniformExpectation = 11/16 ∧
    (3 ≤ (3 : ℕ) ∧ (3 : ℚ) < 8/2) ∧
    ¬ (2*(3 : ℚ)^2/(3*8) ≤ uniformExpectation) ∧
    intersections (fun _ => (0, false)) = 2 ∧
    intersections (fun i => (i.castSucc, false)) = 0 ∧
    uniformExpectation < (3 : ℚ)^2/8 ∧
    (∀ n : ℚ, 6 < n → sourceClosedForm n 3 < 2*(3 : ℚ)^2/(3*n)) ∧
    (∀ (n : ℚ), 2 ≤ n → ∀ e : ℕ → ℚ, e 0 = 0 →
      (∀ s, e (s+1) = (1-2/n)*e s+2*s/n) →
      ∀ s : ℕ, 3 ≤ s → (s : ℚ) ≤ n/2 →
        4*(s : ℚ)^2/(9*n) ≤ e s ∧ e s ≤ (s : ℚ)^2/n) := by
  exact ⟨negate_involutive, negate_fixed_point_free, same_orbit_iff,
    pool_card, sample_card, actual_intersection_sum, actual_expectation, source_domain,
    source_lower_bound_fails, triple_repeat_control, distinct_control, upper_bound_control,
    closed_form_lower_fails, repaired_quadratic_bounds⟩

#print axioms actual_collision_bound_certificate

end TheriaultCollisionAudit
