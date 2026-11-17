/-!
Kernel-checked finite arithmetic for the q16 three-screen isogeny frontier.
The exhaustive finite-field enumeration and Tate isogeny theorem are checked
in the retained Sage replay and mathematical argument, not in this module.
-/

namespace GHSFrontierQ16

set_option maxRecDepth 8192
set_option maxHeartbeats 0

theorem cohort_and_closure :
    2 * (16^3 - 1) = 8190 ∧
    8190 - 6158 = 2032 ∧
    8190 - 8124 = 66 ∧
    12 + 30 + 24 = 66 := by decide

theorem source_factors :
    3974 = 2 * 1987 ∧
    3978 = 234 * 17 ∧
    4178 = 2 * 2089 := by decide

theorem simple_root_gates :
    Nat.gcd 1987 (3 * (16^3 - 1)) = 1 ∧
    Nat.gcd 17 (3 * (16^3 - 1)) = 1 ∧
    Nat.gcd 2089 (3 * (16^3 - 1)) = 1 := by decide

theorem candidate_prime_trial_division :
    (∀ d : Fin 45, 2 ≤ d.val → 1987 % d.val ≠ 0) ∧
    (∀ d : Fin 46, 2 ≤ d.val → 2089 % d.val ≠ 0) ∧
    (∀ d : Fin 5, 2 ≤ d.val → 17 % d.val ≠ 0) := by decide

end GHSFrontierQ16
