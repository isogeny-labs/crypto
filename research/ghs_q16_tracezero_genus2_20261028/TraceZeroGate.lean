import Init.Grind.Module.Basic
set_option autoImplicit false

/-! Abstract subgroup-transfer theorem and arithmetic checks for the q16 census.
The finite-field census, polynomial irreducibility, Tate theorem, and
Howe-Nart-Ritzenthaler classification are external to these Lean lemmas. -/
namespace GHSQ16TraceZeroGate

open Lean.Grind

variable {E : Type} [IntModule E]

/-- On an order-r subgroup with no nonzero Frobenius-fixed r-torsion,
the evaluated trace-zero map P ↦ P-F(P) has trivial kernel. -/
theorem tracezero_injective_on_r_torsion
    (frob : E → E) (r : Int)
    (fixed_torsion_trivial : ∀ x : E, r • x = 0 → frob x = x → x = 0)
    (x : E) (hx : r • x = 0) (himage : x - frob x = 0) : x = 0 := by
  apply fixed_torsion_trivial x hx
  exact (AddCommGroup.sub_eq_zero_iff.mp himage).symm

/-- The difference map preserves an arbitrary scalar relation when
Frobenius commutes with scalar multiplication. -/
theorem tracezero_preserves_scalar
    (frob : E → E) (n : Int) (x : E)
    (frob_scalar : frob (n • x) = n • frob x) :
    (n • x) - frob (n • x) = n • (x - frob x) := by
  rw [frob_scalar, IntModule.zsmul_sub]

theorem population_partition :
    7584 + 606 = 8190 ∧ 56 + 550 = 606 ∧ 379 + 171 = 550 := by decide

theorem split_class_counts :
    28 + 52 + 119 + 104 + 104 + 119 + 52 + 28 = 606 ∧
    28 + 28 = 56 ∧ 52 + 119 + 104 + 104 + 119 + 52 = 550 ∧
    52 + 104 + 104 + 119 = 379 := by decide

theorem hnr_exception_arithmetic :
    3^2 + 7 = 16 ∧ 7 % 3 = 1 := by decide

theorem selected_prime_factor_orders :
    20 * 199 = 3980 ∧ 22 * 181 = 3982 ∧ 18 * 225 = 4050 ∧
    10 * 409 = 4090 ∧ 24 * 171 = 4104 ∧ 16 * 259 = 4144 ∧
    12 * 351 = 4212 ∧ 14 * 301 = 4214 := by decide

end GHSQ16TraceZeroGate
