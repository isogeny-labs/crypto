import Mathlib.Algebra.Polynomial.SpecificDegree
import Mathlib.Tactic

namespace WeberResiduesV2
open Polynomial
instance : Fact (Nat.Prime 7) := ⟨by decide⟩

theorem auxiliary_irreducible : Irreducible (X ^ 3 + 3 * X + 5 : Polynomial (ZMod 7)) := by
  apply Polynomial.irreducible_of_degree_le_three_of_not_isRoot
  · have h : (X ^ 3 + 3 * X + 5 : Polynomial (ZMod 7)).natDegree = 3 := by
      compute_degree!
    norm_num [h, Finset.mem_Icc]
  · intro x
    fin_cases x <;> norm_num [Polynomial.IsRoot] <;> decide

theorem source_factorization :
    (X ^ 3 + 66 * X ^ 2 + 77 * X + 21 : Polynomial (ZMod 79)) =
      (X - 58) * (X - 67) * (X - 46) := by
  ring_nf
  ext n
  rcases n with _ | _ | _ | _ | n <;> simp [Polynomial.coeff_X]
  all_goals decide

theorem target_factorization :
    (X ^ 3 + 5 * X ^ 2 + 15 * X + 18 : Polynomial (ZMod 19)) =
      (X - 2) * (X - 14) * (X - 17) := by
  ring_nf
  ext n
  rcases n with _ | _ | _ | _ | n <;> simp [Polynomial.coeff_X]
  all_goals decide

end WeberResiduesV2
