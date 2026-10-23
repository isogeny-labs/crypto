import KoblitzRound18
namespace KoblitzRound18Refinement
def B : Matrix (Fin 8) (Fin 8) ℤ := ![![0,0,0,0,7,0,-10,8],![0,0,0,0,0,7,-4,-6],![1,0,0,-2,-1,-2,-6,-8],![0,1,1,-1,1,-2,4,-10],![0,0,3,0,-6,4,2,-6],![0,0,0,3,-2,-4,3,-1],![0,0,0,0,0,0,15,0],![0,0,0,0,0,0,0,15]]
def C : Matrix (Fin 8) (Fin 8) ℤ := ![![35,70,105,0,0,70,70,70],![-35,70,0,105,-35,35,-35,105],![30,-20,0,0,35,0,10,-10],![10,20,0,0,0,35,5,5],![15,0,0,0,0,0,10,-8],![0,15,0,0,0,0,4,6],![0,0,0,0,0,0,7,0],![0,0,0,0,0,0,0,7]]
def R : Matrix (Fin 8) (Fin 8) ℤ := ![![-1,0,-1,0,-1,0,-1,0],![0,-1,0,-1,0,-1,0,-1],![1,0,0,0,0,0,0,0],![0,1,0,0,0,0,0,0],![0,0,1,0,0,0,0,0],![0,0,0,1,0,0,0,0],![0,0,0,0,1,0,0,0],![0,0,0,0,0,1,0,0]]
theorem actual_scalar_refinement : B*C=(105:ℤ) • 1 ∧ C*B=(105:ℤ) • 1 := by decide +kernel
theorem actual_rotation_compatibility : R^5=1 ∧ R*((105:ℤ) • 1)=((105:ℤ) • 1)*R := by decide +kernel
def pi0 : Matrix (Fin 2) (Fin 2) (ZMod 105) := ![![45,62],![14,59]]
theorem actual_torsion_period0 : pi0^168=1 ∧ pi0^84≠1 ∧ pi0^56≠1 ∧ pi0^24≠1 := by decide +kernel
def pi1 : Matrix (Fin 2) (Fin 2) (ZMod 105) := ![![60,43],![91,46]]
theorem actual_torsion_period1 : pi1^168=1 ∧ pi1^84≠1 ∧ pi1^56≠1 ∧ pi1^24≠1 := by decide +kernel
end KoblitzRound18Refinement
#print axioms KoblitzRound18Refinement.actual_scalar_refinement
#print axioms KoblitzRound18Refinement.actual_rotation_compatibility
#print axioms KoblitzRound18Refinement.actual_torsion_period0
#print axioms KoblitzRound18Refinement.actual_torsion_period1
