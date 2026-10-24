import KoblitzRound19OrbitCover
import KoblitzRound15Theta
namespace KoblitzRound20Picard
open scoped BigOperators
def orbitBias (i:Fin 4) : ZMod 2 := ∑ k:Fin 4,∑ j:Fin 5,((KoblitzRound19OrbitCover.weights k)*(KoblitzRound19OrbitCover.rows k j i).1*(KoblitzRound19OrbitCover.rows k j i).2:ℤ)
theorem actual_axis_match0 : orbitBias=KoblitzRound15Theta.corr0 := by decide +kernel
theorem actual_axis_match2 : orbitBias=KoblitzRound15Theta.corr2 := by decide +kernel
theorem no_fixed_Pic2_0 : ∀v:Fin 4→ZMod 2,KoblitzRound15Theta.pic0.mulVec v=v→v=0 := by decide +kernel
theorem no_fixed_Pic2_2 : ∀v:Fin 4→ZMod 2,KoblitzRound15Theta.pic2.mulVec v=v→v=0 := by decide +kernel
theorem canonical_difference_vanishes (d:Fin 4→ZMod 2) (h:KoblitzRound15Theta.pic0.mulVec d=d) : d=0 := no_fixed_Pic2_0 d h
end KoblitzRound20Picard
#print axioms KoblitzRound20Picard.actual_axis_match0
#print axioms KoblitzRound20Picard.actual_axis_match2
#print axioms KoblitzRound20Picard.no_fixed_Pic2_0
#print axioms KoblitzRound20Picard.no_fixed_Pic2_2
#print axioms KoblitzRound20Picard.canonical_difference_vanishes
