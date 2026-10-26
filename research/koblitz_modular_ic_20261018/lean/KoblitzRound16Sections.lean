import KoblitzRound15Theta
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace KoblitzRound16Sections
def B0 : Matrix (Fin 8) (Fin 8) ℤ := !![0,0,0,0,7,0,-10,8;0,0,0,0,0,7,-4,-6;1,0,0,-2,-1,-2,-6,-8;0,1,1,-1,1,-2,4,-10;0,0,3,0,-6,4,2,-6;0,0,0,3,-2,-4,3,-1;0,0,0,0,0,0,15,0;0,0,0,0,0,0,0,15]
def J0 : Matrix (Fin 8) (Fin 8) ℤ := !![0,5,1,1,-1,3,1,3;-5,0,-2,2,-2,-2,-4,2;-1,2,0,3,-1,1,-1,3;-1,-2,-3,0,0,-2,-2,-2;1,2,1,0,0,3,1,1;-3,2,-1,2,-3,0,-2,2;-1,4,1,2,-1,2,0,5;-3,-2,-3,2,-1,-2,-5,0]
def D0 : Matrix (Fin 8) (Fin 8) ℤ := !![0,3,0,0,0,0,0,0;-3,0,0,0,0,0,0,0;0,0,0,21,0,0,0,0;0,0,-21,0,0,0,0,0;0,0,0,0,0,105,0,0;0,0,0,0,-105,0,0,0;0,0,0,0,0,0,0,285;0,0,0,0,0,0,-285,0]
theorem actual_diagonalization0 : B0.transpose*J0*B0=D0 := by decide +kernel
theorem actual_section_count0 : 87723*(99225:ℕ)=8704314675 := by norm_num
def B1 : Matrix (Fin 8) (Fin 8) ℤ := !![0,0,2,0,0,-4,-3,10;0,0,0,2,2,-2,-5,2;1,0,-1,0,-4,-2,2,-10;0,1,0,-1,1,-5,5,-3;0,0,0,0,8,0,-7,2;0,0,0,0,0,8,-1,-6;0,0,0,0,0,0,14,0;0,0,0,0,0,0,0,14]
def J1 : Matrix (Fin 8) (Fin 8) ℤ := !![0,3,0,2,-1,2,0,1;-3,0,-2,0,-1,-2,-1,0;0,2,0,4,-1,3,-1,2;-2,0,-4,0,-2,-2,-1,-2;1,1,1,2,0,4,0,2;-2,2,-3,2,-4,0,-2,0;0,1,1,1,0,2,0,3;-1,0,-2,2,-2,0,-3,0]
def D1 : Matrix (Fin 8) (Fin 8) ℤ := !![0,4,0,0,0,0,0,0;-4,0,0,0,0,0,0,0;0,0,0,8,0,0,0,0;0,0,-8,0,0,0,0,0;0,0,0,0,0,112,0,0;0,0,0,0,-112,0,0,0;0,0,0,0,0,0,0,266;0,0,0,0,0,0,-266,0]
theorem actual_diagonalization1 : B1.transpose*J1*B1=D1 := by decide +kernel
theorem actual_section_count1 : 87723*(50176:ℕ)=4401589248 := by norm_num
def B2 : Matrix (Fin 8) (Fin 8) ℤ := !![0,0,0,0,7,0,-10,8;0,0,0,0,0,7,-4,-6;1,0,0,-2,-1,-2,-6,-8;0,1,1,-1,1,-2,4,-10;0,0,3,0,-6,4,2,-6;0,0,0,3,-2,-4,3,-1;0,0,0,0,0,0,15,0;0,0,0,0,0,0,0,15]
def J2 : Matrix (Fin 8) (Fin 8) ℤ := !![0,5,1,1,-1,3,1,3;-5,0,-2,2,-2,-2,-4,2;-1,2,0,3,-1,1,-1,3;-1,-2,-3,0,0,-2,-2,-2;1,2,1,0,0,3,1,1;-3,2,-1,2,-3,0,-2,2;-1,4,1,2,-1,2,0,5;-3,-2,-3,2,-1,-2,-5,0]
def D2 : Matrix (Fin 8) (Fin 8) ℤ := !![0,3,0,0,0,0,0,0;-3,0,0,0,0,0,0,0;0,0,0,21,0,0,0,0;0,0,-21,0,0,0,0,0;0,0,0,0,0,105,0,0;0,0,0,0,-105,0,0,0;0,0,0,0,0,0,0,285;0,0,0,0,0,0,-285,0]
theorem actual_diagonalization2 : B2.transpose*J2*B2=D2 := by decide +kernel
theorem actual_section_count2 : 87723*(99225:ℕ)=8704314675 := by norm_num
def B3 : Matrix (Fin 8) (Fin 8) ℤ := !![0,0,2,0,0,-4,-3,10;0,0,0,2,2,-2,-5,2;1,0,-1,0,-4,-2,2,-10;0,1,0,-1,1,-5,5,-3;0,0,0,0,8,0,-7,2;0,0,0,0,0,8,-1,-6;0,0,0,0,0,0,14,0;0,0,0,0,0,0,0,14]
def J3 : Matrix (Fin 8) (Fin 8) ℤ := !![0,3,0,2,-1,2,0,1;-3,0,-2,0,-1,-2,-1,0;0,2,0,4,-1,3,-1,2;-2,0,-4,0,-2,-2,-1,-2;1,1,1,2,0,4,0,2;-2,2,-3,2,-4,0,-2,0;0,1,1,1,0,2,0,3;-1,0,-2,2,-2,0,-3,0]
def D3 : Matrix (Fin 8) (Fin 8) ℤ := !![0,4,0,0,0,0,0,0;-4,0,0,0,0,0,0,0;0,0,0,8,0,0,0,0;0,0,-8,0,0,0,0,0;0,0,0,0,0,112,0,0;0,0,0,0,-112,0,0,0;0,0,0,0,0,0,0,266;0,0,0,0,0,0,-266,0]
theorem actual_diagonalization3 : B3.transpose*J3*B3=D3 := by decide +kernel
theorem actual_section_count3 : 87723*(50176:ℕ)=4401589248 := by norm_num
end KoblitzRound16Sections
#print axioms KoblitzRound16Sections.actual_diagonalization0
#print axioms KoblitzRound16Sections.actual_section_count0
#print axioms KoblitzRound16Sections.actual_diagonalization1
#print axioms KoblitzRound16Sections.actual_section_count1
#print axioms KoblitzRound16Sections.actual_diagonalization2
#print axioms KoblitzRound16Sections.actual_section_count2
#print axioms KoblitzRound16Sections.actual_diagonalization3
#print axioms KoblitzRound16Sections.actual_section_count3
