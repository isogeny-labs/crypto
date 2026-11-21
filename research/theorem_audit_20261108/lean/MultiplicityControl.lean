import Mathlib

/-! A concrete nonreduced control: support size is not scheme length.
This is not a proof of Diem's general multihomogeneous Bezout bound. -/
namespace TheoremAudit
open Polynomial
variable (K : Type*) [Field K] [DecidableEq K]

theorem double_root_quotient_dimension :
    Module.finrank K (K[X] ⧸ Ideal.span { (X:K[X])^2 })=2 := by
  rw [finrank_quotient_span_eq_natDegree]
  simp

theorem double_root_support_size :
    ((X^2 : K[X]).roots.toFinset).card=1 := by
  simp [roots_X_pow]

theorem double_root_multiplicity_mass :
    (X^2 : K[X]).roots.card=2 := by
  simp [roots_X_pow]

end TheoremAudit
