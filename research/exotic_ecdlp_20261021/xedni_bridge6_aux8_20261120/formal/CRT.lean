import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

/-! Exact coefficient congruences of the constructed local bridge. -/

namespace XedniBridge6Aux8

def curve7 : List ℤ := [-141, 141, 47, -471, -96]
def curve11 : List ℤ := [-141, 0, 47, -2445, 4416]
def curveCRT : List ℤ := [-141, -517, 47, -9683, 20960]

theorem centered_multiplier_congruences :
    ((-11 : ℤ) % 7 = 3 % 7) ∧ ((-11 : ℤ) % 11 = 0) := by
  norm_num

theorem curve_coefficients_mod7 :
    curveCRT.map (fun x => x % 7) = curve7.map (fun x => x % 7) := by
  norm_num [curveCRT, curve7]

theorem curve_coefficients_mod11 :
    curveCRT.map (fun x => x % 11) = curve11.map (fun x => x % 11) := by
  norm_num [curveCRT, curve11]

#print axioms centered_multiplier_congruences
#print axioms curve_coefficients_mod7
#print axioms curve_coefficients_mod11

end XedniBridge6Aux8
