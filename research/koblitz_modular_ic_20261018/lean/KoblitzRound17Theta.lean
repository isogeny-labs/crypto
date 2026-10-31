import KoblitzRound16Sections
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace KoblitzRound17Theta
def U : Matrix (Fin 8) (Fin 8) ℤ := ![![0,0,1,0,0,0,0,0],![0,0,0,1,0,0,0,0],![-1,0,0,0,-4,8,0,0],![0,-1,0,0,-4,0,0,0],![-6,-2,0,0,-28,42,-9,-6],![1,-7,0,0,-21,-7,3,-12],![0,10,0,0,35,0,-2,18],![-5,5,0,0,0,35,-9,7]]
def B : Matrix (Fin 8) (Fin 8) ℤ := ![![0,0,0,0,7,0,-10,8],![0,0,0,0,0,7,-4,-6],![1,0,0,-2,-1,-2,-6,-8],![0,1,1,-1,1,-2,4,-10],![0,0,3,0,-6,4,2,-6],![0,0,0,3,-2,-4,3,-1],![0,0,0,0,0,0,15,0],![0,0,0,0,0,0,0,15]]
def V : Matrix (Fin 8) (Fin 8) ℤ := ![![1,0,3,4,-56,-136,-4340,1330],![0,1,-2,5,68,-124,-665,-3675],![0,0,2,-2,-60,44,-755,2780],![0,0,1,1,-22,-38,-1390,635],![0,0,1,0,-26,-8,-884,1012],![0,0,0,1,4,-30,-506,-378],![0,0,0,0,1,0,28,-42],![0,0,0,0,0,1,21,7]]
def D : Matrix (Fin 8) (Fin 8) ℤ := ![![1,0,0,0,0,0,0,0],![0,1,0,0,0,0,0,0],![0,0,1,0,0,0,0,0],![0,0,0,1,0,0,0,0],![0,0,0,0,3,0,0,0],![0,0,0,0,0,3,0,0],![0,0,0,0,0,0,105,0],![0,0,0,0,0,0,0,105]]
theorem actual_odd_Smith : U*B*V=D := by decide +kernel
theorem actual_odd_kernel_order : 3^2*105^2=99225 := by norm_num
def Ui : Matrix (Fin 8) (Fin 8) ℤ := ![![0,0,7,0,-64,-16,-60,72],![0,0,0,7,8,-72,-36,-24],![1,0,0,0,0,0,0,0],![0,1,0,0,0,0,0,0],![0,0,0,-2,-2,18,9,6],![0,0,1,-1,-9,7,-3,12],![0,0,0,0,5,0,4,-6],![0,0,0,0,0,5,3,1]]
def Vi : Matrix (Fin 8) (Fin 8) ℤ := ![![1,0,0,-2,-1,-2,-6,-8],![0,1,1,-1,1,-2,4,-10],![0,0,-12,24,1,-48,26,8],![0,0,-12,0,24,-23,-4,30],![0,0,-28,42,14,-98,1,0],![0,0,-21,-7,49,-35,0,1],![0,0,1,0,-2,2,0,0],![0,0,0,1,-1,-1,0,0]]
theorem actual_U_inverse : U*Ui=1 ∧ Ui*U=1 := by decide +kernel
theorem actual_V_inverse : V*Vi=1 ∧ Vi*V=1 := by decide +kernel
def K0_3 : Matrix (Fin 8) (Fin 4) (ZMod 3) := ![![1,1,1,2],![1,2,2,0],![0,1,1,1],![1,1,1,2],![1,2,1,2],![2,0,2,0],![1,0,1,0],![0,1,0,1]]
def Omega0_3 : Matrix (Fin 8) (Fin 8) (ZMod 3) := ![![0,1,0,0,0,0,0,0],![2,0,0,0,0,0,0,0],![0,0,0,1,0,0,0,0],![0,0,2,0,0,0,0,0],![0,0,0,0,0,2,0,0],![0,0,0,0,1,0,0,0],![0,0,0,0,0,0,0,2],![0,0,0,0,0,0,1,0]]
theorem actual_kernel_isotropic0_3 : (K0_3).transpose*Omega0_3*K0_3=0 := by decide +kernel
def K0_5 : Matrix (Fin 8) (Fin 4) (ZMod 5) := ![![0,0,0,0],![0,0,0,0],![0,0,0,0],![0,0,0,0],![0,0,1,3],![0,0,1,2],![0,0,3,2],![0,0,4,2]]
def Omega0_5 : Matrix (Fin 8) (Fin 8) (ZMod 5) := ![![0,0,0,0,0,0,0,0],![0,0,0,0,0,0,0,0],![0,0,0,0,0,0,0,0],![0,0,0,0,0,0,0,0],![0,0,0,0,0,1,0,0],![0,0,0,0,4,0,0,0],![0,0,0,0,0,0,0,2],![0,0,0,0,0,0,3,0]]
theorem actual_kernel_isotropic0_5 : (K0_5).transpose*Omega0_5*K0_5=0 := by decide +kernel
def K0_7 : Matrix (Fin 8) (Fin 4) (ZMod 7) := ![![0,0,0,0],![0,0,0,0],![0,0,3,4],![0,0,0,3],![0,0,6,2],![0,0,0,6],![0,0,0,0],![0,0,0,0]]
def Omega0_7 : Matrix (Fin 8) (Fin 8) (ZMod 7) := ![![0,0,0,0,0,0,0,0],![0,0,0,0,0,0,0,0],![0,0,0,3,0,0,0,0],![0,0,4,0,0,0,0,0],![0,0,0,0,0,1,0,0],![0,0,0,0,6,0,0,0],![0,0,0,0,0,0,0,0],![0,0,0,0,0,0,0,0]]
theorem actual_kernel_isotropic0_7 : (K0_7).transpose*Omega0_7*K0_7=0 := by decide +kernel
def K2_3 : Matrix (Fin 8) (Fin 4) (ZMod 3) := ![![1,1,1,2],![1,2,2,0],![0,1,1,1],![1,1,1,2],![1,2,1,2],![2,0,2,0],![1,0,1,0],![0,1,0,1]]
def Omega2_3 : Matrix (Fin 8) (Fin 8) (ZMod 3) := ![![0,1,0,0,0,0,0,0],![2,0,0,0,0,0,0,0],![0,0,0,1,0,0,0,0],![0,0,2,0,0,0,0,0],![0,0,0,0,0,2,0,0],![0,0,0,0,1,0,0,0],![0,0,0,0,0,0,0,2],![0,0,0,0,0,0,1,0]]
theorem actual_kernel_isotropic2_3 : (K2_3).transpose*Omega2_3*K2_3=0 := by decide +kernel
def K2_5 : Matrix (Fin 8) (Fin 4) (ZMod 5) := ![![0,0,0,0],![0,0,0,0],![0,0,0,0],![0,0,0,0],![0,0,1,3],![0,0,1,2],![0,0,3,2],![0,0,4,2]]
def Omega2_5 : Matrix (Fin 8) (Fin 8) (ZMod 5) := ![![0,0,0,0,0,0,0,0],![0,0,0,0,0,0,0,0],![0,0,0,0,0,0,0,0],![0,0,0,0,0,0,0,0],![0,0,0,0,0,1,0,0],![0,0,0,0,4,0,0,0],![0,0,0,0,0,0,0,2],![0,0,0,0,0,0,3,0]]
theorem actual_kernel_isotropic2_5 : (K2_5).transpose*Omega2_5*K2_5=0 := by decide +kernel
def K2_7 : Matrix (Fin 8) (Fin 4) (ZMod 7) := ![![0,0,0,0],![0,0,0,0],![0,0,3,4],![0,0,0,3],![0,0,6,2],![0,0,0,6],![0,0,0,0],![0,0,0,0]]
def Omega2_7 : Matrix (Fin 8) (Fin 8) (ZMod 7) := ![![0,0,0,0,0,0,0,0],![0,0,0,0,0,0,0,0],![0,0,0,3,0,0,0,0],![0,0,4,0,0,0,0,0],![0,0,0,0,0,1,0,0],![0,0,0,0,6,0,0,0],![0,0,0,0,0,0,0,0],![0,0,0,0,0,0,0,0]]
theorem actual_kernel_isotropic2_7 : (K2_7).transpose*Omega2_7*K2_7=0 := by decide +kernel
end KoblitzRound17Theta
#print axioms KoblitzRound17Theta.actual_odd_Smith
#print axioms KoblitzRound17Theta.actual_odd_kernel_order
#print axioms KoblitzRound17Theta.actual_kernel_isotropic0_3
#print axioms KoblitzRound17Theta.actual_kernel_isotropic0_5
#print axioms KoblitzRound17Theta.actual_kernel_isotropic0_7
#print axioms KoblitzRound17Theta.actual_kernel_isotropic2_3
#print axioms KoblitzRound17Theta.actual_kernel_isotropic2_5
#print axioms KoblitzRound17Theta.actual_kernel_isotropic2_7
#print axioms KoblitzRound17Theta.actual_U_inverse
#print axioms KoblitzRound17Theta.actual_V_inverse
