import KoblitzRound15
namespace KoblitzRound15CM
theorem auxiliary_prime_5 : Nat.Prime 11 := by norm_num
theorem genus_5 : 2*5*(7-1)+1=61 ∧ 2*61+( 480+320+40)=960+2 := by norm_num
theorem split_two_5 : ((5:ZMod 11)^2-5+2)=0 ∧ ((7:ZMod 11)^2-7+2)=0 := by decide +kernel
theorem auxiliary_prime_11 : Nat.Prime 23 := by norm_num
theorem genus_11 : 2*11*(15-1)+1=309 ∧ 2*309+( 2112+1408+88)=4224+2 := by norm_num
theorem split_two_11 : ((10:ZMod 23)^2-10+2)=0 ∧ ((14:ZMod 23)^2-14+2)=0 := by decide +kernel
theorem auxiliary_prime_83 : Nat.Prime 499 := by norm_num
theorem genus_83 : 2*83*(331-1)+1=54781 ∧ 2*54781+( 332000+221776+664)=664000+2 := by norm_num
theorem split_two_83 : ((152:ZMod 499)^2-152+2)=0 ∧ ((348:ZMod 499)^2-348+2)=0 := by decide +kernel
theorem auxiliary_prime_131 : Nat.Prime 263 := by norm_num
theorem genus_131 : 2*131*(175-1)+1=45589 ∧ 2*45589+( 276672+184448+1048)=553344+2 := by norm_num
theorem split_two_131 : ((124:ZMod 263)^2-124+2)=0 ∧ ((140:ZMod 263)^2-140+2)=0 := by decide +kernel
theorem actual_n5_Euler_product {R : Type*} [CommRing R] (x:R) : ((16:R)*x^0 + (-8:R)*x^1 + (-4:R)*x^2 + (6:R)*x^3 + (-1:R)*x^4 + (3:R)*x^5 + (-1:R)*x^6 + (-1:R)*x^7 + (1:R)*x^8)*((16:R)*x^0 + (8:R)*x^1 + (-4:R)*x^2 + (-6:R)*x^3 + (-1:R)*x^4 + (-3:R)*x^5 + (-1:R)*x^6 + (1:R)*x^7 + (1:R)*x^8)=(256:R)*x^0 + (-192:R)*x^2 + (80:R)*x^4 + (-12:R)*x^6 + (-11:R)*x^8 + (-3:R)*x^10 + (5:R)*x^12 + (-3:R)*x^14 + (1:R)*x^16 := by ring
theorem odd_character_involution {R : Type*} [CommRing R] (a b : R) : (-a)+(-b)=-(a+b) := by ring
end KoblitzRound15CM
#print axioms KoblitzRound15CM.auxiliary_prime_5
#print axioms KoblitzRound15CM.genus_5
#print axioms KoblitzRound15CM.split_two_5
#print axioms KoblitzRound15CM.auxiliary_prime_11
#print axioms KoblitzRound15CM.genus_11
#print axioms KoblitzRound15CM.split_two_11
#print axioms KoblitzRound15CM.auxiliary_prime_83
#print axioms KoblitzRound15CM.genus_83
#print axioms KoblitzRound15CM.split_two_83
#print axioms KoblitzRound15CM.auxiliary_prime_131
#print axioms KoblitzRound15CM.genus_131
#print axioms KoblitzRound15CM.split_two_131
#print axioms KoblitzRound15CM.actual_n5_Euler_product
#print axioms KoblitzRound15CM.odd_character_involution
