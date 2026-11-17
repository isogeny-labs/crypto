import Mathlib.Algebra.Polynomial.Basic
import Mathlib.Algebra.Polynomial.Degree.Domain
import Mathlib.RingTheory.Coprime.Basic
import Mathlib.Tactic.Ring

/-!
Conditional residue bridge for the common 2+2 residual polynomial. The
interpolation congruences remain explicit hypotheses; this file does not
derive them from Rust execution or a Mumford-valid input.
-/

set_option autoImplicit false

namespace Common22Residues

open Polynomial

variable {R : Type*} [CommRing R]

/-- The first interpolation congruence implies the first residual
curve-equation congruence after clearing the scale. -/
theorem first_residue
    (u1 u2 n v b support : R[X])
    (interpolates : u1 ∣ n - b * v) :
    u1 ∣ n ^ 2 - u1 * u2 * support - b ^ 2 * v ^ 2 := by
  obtain ⟨t, ht⟩ := interpolates
  refine ⟨t * (n + b * v) - u2 * support, ?_⟩
  calc
    n ^ 2 - u1 * u2 * support - b ^ 2 * v ^ 2 =
      (n - b * v) * (n + b * v) - u1 * (u2 * support) := by ring
    _ = (u1 * t) * (n + b * v) - u1 * (u2 * support) := by rw [ht]
    _ = u1 * (t * (n + b * v) - u2 * support) := by ring

/-- The second interpolation congruence implies the second residual
curve-equation congruence after clearing the scale. -/
theorem second_residue
    (u1 u2 n v b support : R[X])
    (interpolates : u2 ∣ n - b * v) :
    u2 ∣ n ^ 2 - u1 * u2 * support - b ^ 2 * v ^ 2 := by
  obtain ⟨t, ht⟩ := interpolates
  refine ⟨t * (n + b * v) - u1 * support, ?_⟩
  calc
    n ^ 2 - u1 * u2 * support - b ^ 2 * v ^ 2 =
      (n - b * v) * (n + b * v) - u2 * (u1 * support) := by ring
    _ = (u2 * t) * (n + b * v) - u2 * (u1 * support) := by rw [ht]
    _ = u2 * (t * (n + b * v) - u1 * support) := by ring

/-- The residual curve equation follows from interpolation, valid-input
curve congruences, coprimeness, and the explicit strict degree bound. -/
theorem residual_curve_eq
    {K : Type*} [Field K]
    (u1 u2 n v1 v2 b support f : K[X])
    (coprime : IsCoprime u1 u2)
    (interpolates_first : u1 ∣ n - b * v1)
    (interpolates_second : u2 ∣ n - b * v2)
    (valid_first : u1 ∣ f - v1 ^ 2)
    (valid_second : u2 ∣ f - v2 ^ 2)
    (small : degree (n ^ 2 - u1 * u2 * support - b ^ 2 * f) <
      degree (u1 * u2)) :
    n ^ 2 - u1 * u2 * support = b ^ 2 * f := by
  obtain ⟨t1, ht1⟩ := interpolates_first
  obtain ⟨t2, ht2⟩ := interpolates_second
  obtain ⟨s1, hs1⟩ := valid_first
  obtain ⟨s2, hs2⟩ := valid_second
  have hfirst : u1 ∣ n ^ 2 - u1 * u2 * support - b ^ 2 * f := by
    refine ⟨t1 * (n + b * v1) - u2 * support - b ^ 2 * s1, ?_⟩
    calc
      n ^ 2 - u1 * u2 * support - b ^ 2 * f =
        (n - b * v1) * (n + b * v1) - u1 * u2 * support -
          b ^ 2 * (f - v1 ^ 2) := by ring
      _ = u1 * (t1 * (n + b * v1) - u2 * support - b ^ 2 * s1) := by
        rw [ht1, hs1]
        ring
  have hsecond : u2 ∣ n ^ 2 - u1 * u2 * support - b ^ 2 * f := by
    refine ⟨t2 * (n + b * v2) - u1 * support - b ^ 2 * s2, ?_⟩
    calc
      n ^ 2 - u1 * u2 * support - b ^ 2 * f =
        (n - b * v2) * (n + b * v2) - u1 * u2 * support -
          b ^ 2 * (f - v2 ^ 2) := by ring
      _ = u2 * (t2 * (n + b * v2) - u1 * support - b ^ 2 * s2) := by
        rw [ht2, hs2]
        ring
  exact sub_eq_zero.mp (eq_zero_of_dvd_of_degree_lt
    (coprime.mul_dvd hfirst hsecond) small)

end Common22Residues

#print axioms Common22Residues.first_residue
#print axioms Common22Residues.second_residue
#print axioms Common22Residues.residual_curve_eq
