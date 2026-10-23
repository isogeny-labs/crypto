import KoblitzRound18
namespace KoblitzRound18Frobenius
def B : Matrix (Fin 8) (Fin 8) (ZMod 81) := ![![0,0,0,0,7,0,71,8],![0,0,0,0,0,7,77,75],![1,0,0,79,80,79,75,73],![0,1,1,80,1,79,4,71],![0,0,3,0,75,4,2,75],![0,0,0,3,79,77,3,80],![0,0,0,0,0,0,15,0],![0,0,0,0,0,0,0,15]]
def R : Matrix (Fin 8) (Fin 8) (ZMod 81) := ![![80,0,80,0,80,0,80,0],![0,80,0,80,0,80,0,80],![1,0,0,0,0,0,0,0],![0,1,0,0,0,0,0,0],![0,0,1,0,0,0,0,0],![0,0,0,1,0,0,0,0],![0,0,0,0,1,0,0,0],![0,0,0,0,0,1,0,0]]
def L : Matrix (Fin 1) (Fin 8) (ZMod 81) := ![![27,54,0,0,0,54,54,54]]
theorem actual_rotation_obstruction : L*B=0 ∧ L*(R*B)≠0 := by decide +kernel
theorem no_rotation_lift : ¬ ∃ A:Matrix (Fin 8) (Fin 8) (ZMod 81),B*A=R*B := by
 rintro ⟨A,hA⟩
 have hz : L*(R*B)=0 := by rw [←hA,←Matrix.mul_assoc,actual_rotation_obstruction.1,Matrix.zero_mul]
 exact actual_rotation_obstruction.2 hz
end KoblitzRound18Frobenius
#print axioms KoblitzRound18Frobenius.actual_rotation_obstruction
#print axioms KoblitzRound18Frobenius.no_rotation_lift
