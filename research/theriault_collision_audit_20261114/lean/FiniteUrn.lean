import Mathlib.Data.Fintype.Pi
import Mathlib.Data.Finset.Card
import Mathlib.Data.Rat.Defs
import Mathlib.Tactic

namespace TheriaultCollisionAudit

/-- Four negation orbits, each with two distinguishable elements. -/
abbrev Pool := Fin 4 × Bool
def negate (p : Pool) : Pool := (p.1, !p.2)

theorem negate_involutive : Function.Involutive negate := by
  unfold Function.Involutive
  decide +kernel

theorem negate_fixed_point_free : ∀ p : Pool, negate p ≠ p := by
  decide +kernel

theorem same_orbit_iff (p q : Pool) : p.1 = q.1 ↔ p = q ∨ p = negate q := by
  revert p q
  decide +kernel

/-- Every ordered triple is equally likely, so draws are independent with replacement. -/
abbrev Samples := Fin 3 → Pool

def orbitSupport (d : Samples) : Finset (Fin 4) := Finset.univ.image (fun i => (d i).1)

/-- Each occurrence after the first in an orbit contributes one intersection. -/
def intersections (d : Samples) : ℕ := 3 - (orbitSupport d).card

def totalIntersections : ℕ := ∑ d : Samples, intersections d

def uniformExpectation : ℚ := (totalIntersections : ℚ) / Fintype.card Samples

theorem pool_card : Fintype.card Pool = 8 := by decide +kernel
theorem sample_card : Fintype.card Samples = 512 := by decide +kernel
theorem actual_intersection_sum : totalIntersections = 352 := by decide +kernel

theorem actual_expectation : uniformExpectation = 11 / 16 := by
  unfold uniformExpectation
  rw [actual_intersection_sum, sample_card]
  norm_num

theorem source_domain : 3 ≤ (3 : ℕ) ∧ (3 : ℚ) < 8 / 2 := by norm_num

theorem source_lower_bound_fails : ¬ (2 * (3 : ℚ)^2 / (3 * 8) ≤ uniformExpectation) := by
  rw [actual_expectation]
  norm_num

/-- Three occurrences in one orbit yield two intersections, not three unordered pairs. -/
theorem triple_repeat_control : intersections (fun _ => (0, false)) = 2 := by
  decide +kernel

/-- Distinct orbits yield no intersections. -/
theorem distinct_control : intersections (fun i => (i.castSucc, false)) = 0 := by
  decide +kernel

theorem upper_bound_control : uniformExpectation < (3 : ℚ)^2 / 8 := by
  rw [actual_expectation]
  norm_num

end TheriaultCollisionAudit
