-- Intended for leanprover/lean4:v4.35.0 (crypto-lean current toolchain).
-- UNCOMPILED: no Lean executable in this session. Arithmetic only.
import Init
namespace Ecc2k130JacobianTheory
 theorem lucas_factor : (38531015900842053623 : Nat) =
     263 * 146505763881528721 := by decide
 theorem residue_131 : (146505763881528721 : Nat) % 131 = 130 := by decide
 theorem residue_7 : (146505763881528721 : Nat) % 7 = 5 := by decide
 theorem factorization_n_minus_one : (146505763881528721 : Nat) - 1 =
     2^4 * 3 * 5 * 17 * 31 * 4001 * 289510489 := by decide
end Ecc2k130JacobianTheory
