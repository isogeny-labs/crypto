import Init

/-! Finite inequalities for the selected s=7 subgroup's Weil-bound floor.
The Weil bound and the order-preservation implication are mathematical inputs. -/
namespace GenusFloorArithmetic

theorem field_size : 3 ^ 7 = 2187 := by decide
theorem root_cutoff : 2187 < 47 ^ 2 := by decide
theorem four_dim_envelope : 48 ^ 8 = 28179280429056 := by decide
theorem selected_prime_exceeds_envelope :
    28179280429056 < 185302017921931 := by decide
theorem dimension_five_exists_arithmetic : 5 > 4 := by decide

end GenusFloorArithmetic
