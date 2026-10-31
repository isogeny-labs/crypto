import KoblitzRound15
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace KoblitzRound15Theta
def rotation : Matrix (Fin 4) (Fin 4) ℤ := !![-1,-1,-1,-1;1,0,0,0;0,1,0,0;0,0,1,0]
theorem rotation_order : rotation^5=1 := by decide +kernel
def pic0 : Matrix (Fin 4) (Fin 4) (ZMod 2) := !![1,1,0,0;1,0,1,0;1,0,0,1;1,0,0,0]
def corr0 : Fin 4 → ZMod 2 := ![1,1,1,1]
def drift0 : Fin 4 → ZMod 2 := ![1,1,1,0]
theorem actual_theta_correction0 : Matrix.mulVec pic0 corr0+drift0=corr0 := by decide +kernel
theorem picard_inverse0 : (1-pic0)*(pic0+pic0^3)=1 ∧ (pic0+pic0^3)*(1-pic0)=1 := by decide +kernel
def charFrob0 : Matrix (Fin 2) (Fin 2) (ZMod 19) := !![17,18;2,0]
def charInv0 : Matrix (Fin 2) (Fin 2) (ZMod 19) := !![4,15;8,12]
theorem actual_character_inverse0 : (1-charFrob0)*charInv0=1 ∧ charInv0*(1-charFrob0)=1 := by decide +kernel
def pic1 : Matrix (Fin 4) (Fin 4) (ZMod 2) := !![1,1,0,0;1,0,1,0;1,0,0,1;1,0,0,0]
def corr1 : Fin 4 → ZMod 2 := ![1,0,0,1]
def drift1 : Fin 4 → ZMod 2 := ![0,1,0,0]
theorem actual_theta_correction1 : Matrix.mulVec pic1 corr1+drift1=corr1 := by decide +kernel
theorem picard_inverse1 : (1-pic1)*(pic1+pic1^3)=1 ∧ (pic1+pic1^3)*(1-pic1)=1 := by decide +kernel
def charFrob1 : Matrix (Fin 2) (Fin 2) (ZMod 19) := !![17,18;2,0]
def charInv1 : Matrix (Fin 2) (Fin 2) (ZMod 19) := !![4,15;8,12]
theorem actual_character_inverse1 : (1-charFrob1)*charInv1=1 ∧ charInv1*(1-charFrob1)=1 := by decide +kernel
def pic2 : Matrix (Fin 4) (Fin 4) (ZMod 2) := !![1,1,0,0;1,0,1,0;1,0,0,1;1,0,0,0]
def corr2 : Fin 4 → ZMod 2 := ![1,1,1,1]
def drift2 : Fin 4 → ZMod 2 := ![1,1,1,0]
theorem actual_theta_correction2 : Matrix.mulVec pic2 corr2+drift2=corr2 := by decide +kernel
theorem picard_inverse2 : (1-pic2)*(pic2+pic2^3)=1 ∧ (pic2+pic2^3)*(1-pic2)=1 := by decide +kernel
def charFrob2 : Matrix (Fin 2) (Fin 2) (ZMod 19) := !![2,18;2,0]
def charInv2 : Matrix (Fin 2) (Fin 2) (ZMod 19) := !![1,18;2,18]
theorem actual_character_inverse2 : (1-charFrob2)*charInv2=1 ∧ charInv2*(1-charFrob2)=1 := by decide +kernel
def pic3 : Matrix (Fin 4) (Fin 4) (ZMod 2) := !![1,1,0,0;1,0,1,0;1,0,0,1;1,0,0,0]
def corr3 : Fin 4 → ZMod 2 := ![1,0,0,1]
def drift3 : Fin 4 → ZMod 2 := ![0,1,0,0]
theorem actual_theta_correction3 : Matrix.mulVec pic3 corr3+drift3=corr3 := by decide +kernel
theorem picard_inverse3 : (1-pic3)*(pic3+pic3^3)=1 ∧ (pic3+pic3^3)*(1-pic3)=1 := by decide +kernel
def charFrob3 : Matrix (Fin 2) (Fin 2) (ZMod 19) := !![2,18;2,0]
def charInv3 : Matrix (Fin 2) (Fin 2) (ZMod 19) := !![1,18;2,18]
theorem actual_character_inverse3 : (1-charFrob3)*charInv3=1 ∧ charInv3*(1-charFrob3)=1 := by decide +kernel
end KoblitzRound15Theta
#print axioms KoblitzRound15Theta.rotation_order
#print axioms KoblitzRound15Theta.actual_theta_correction0
#print axioms KoblitzRound15Theta.picard_inverse0
#print axioms KoblitzRound15Theta.actual_character_inverse0
#print axioms KoblitzRound15Theta.actual_theta_correction1
#print axioms KoblitzRound15Theta.picard_inverse1
#print axioms KoblitzRound15Theta.actual_character_inverse1
#print axioms KoblitzRound15Theta.actual_theta_correction2
#print axioms KoblitzRound15Theta.picard_inverse2
#print axioms KoblitzRound15Theta.actual_character_inverse2
#print axioms KoblitzRound15Theta.actual_theta_correction3
#print axioms KoblitzRound15Theta.picard_inverse3
#print axioms KoblitzRound15Theta.actual_character_inverse3
