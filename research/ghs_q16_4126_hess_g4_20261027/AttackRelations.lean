import Init.Grind.Module.Basic

/-!
Checked algebra for the retained genus-four Hess relation system. The
concrete curve, map, Jacobian identities, factorisations, and matrix rows
are checked by the Magma run and the independent receipt audit, not Lean.
-/
set_option autoImplicit false

namespace GhsQ16HessG4

open Lean.Grind

variable {J : Type} [IntModule J]

theorem projected_relation (D T F : J) (a b m : Int)
    (h : a • D + b • T = F) :
    (m*a) • D + (m*b) • T = m • F := by
  rw [IntModule.mul_zsmul, IntModule.mul_zsmul,
      ← IntModule.zsmul_add, h]

theorem exact_jacobian_cofactor : 57764 = 28 * 2063 := by decide
theorem projector_unit : (28 * 1842) % 2063 = 1 := by decide
theorem source_cofactor : 4126 = 2 * 2063 := by decide

theorem all_jacobian_cofactors :
    88132 = 44 * 2003 ∧
    56756 = 28 * 2027 ∧
    57092 = 28 * 2039 ∧
    57764 = 28 * 2063 := by decide

theorem all_projector_units :
    (44 * 956) % 2003 = 1 ∧
    (28 * 362) % 2027 = 1 ∧
    (28 * 1238) % 2039 = 1 ∧
    (28 * 1842) % 2063 = 1 := by decide

theorem all_source_cofactors :
    4006 = 2 * 2003 ∧ 4054 = 2 * 2027 ∧
    4078 = 2 * 2039 ∧ 4126 = 2 * 2063 := by decide

end GhsQ16HessG4
