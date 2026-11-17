/-!
Kernel-checked arithmetic and finite odd-subgroup projection certificates for
the Frobenius-cycled triangle family. REPORT.md gives the geometric proof;
the function-field descent and Jacobian isogeny are not formalized here.
-/

namespace GHSTriangle

set_option maxRecDepth 8192
set_option maxHeartbeats 0

theorem q16_factorization : 17 * 241 = 4096 + 1 := by decide

theorem q16_inverse_exponents :
    (17 * 2168) % 4095 = 1 ∧
    (34 * 1084) % 4095 = 1 ∧
    (2 * 2168) % 4095 = 241 := by
  decide

theorem q16_family_count :
    4095 * 2 = 8190 ∧ 2047 + 2048 = 4095 := by
  decide

private def doubleMod (r x : Nat) : Nat := (2 * x) % r

-- These are the four named prime-order source groups in the certificates.
-- An injective projection [2] suffices to certify their odd-primary map.
theorem source_331_double_kernel :
    ∀ x : Fin 331, doubleMod 331 x.val = 0 → x.val = 0 := by decide

theorem source_2003_double_kernel :
    ∀ x : Fin 2003, doubleMod 2003 x.val = 0 → x.val = 0 := by decide

theorem source_2099_double_kernel :
    ∀ x : Fin 2099, doubleMod 2099 x.val = 0 → x.val = 0 := by decide

theorem source_2111_double_kernel :
    ∀ x : Fin 2111, doubleMod 2111 x.val = 0 → x.val = 0 := by decide

end GHSTriangle
