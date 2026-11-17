import TorsionIndex

namespace PairingGeometricCoordinateAudit
noncomputable section
open PairingMillerCurveAudit

private theorem omega_relation : omega^2+omega+1=0 := by decide +kernel
private theorem omega_cube : omega^3=1 := by decide +kernel

variable {K : Type*} [Field K] [Algebra (ZMod 5) K]
variable (f : F25 →ₐ[ZMod 5] K)

theorem cubic_factorization (x : K) :
    (x-1)*(x-f omega)*(x-(f omega)^2)=x^3-1 := by
  have hs : (f omega)^2+f omega+1=0 := by
    simpa only [map_add,map_pow,map_one,map_zero] using congrArg f omega_relation
  have hc : (f omega)^3=1 := by
    simpa only [map_pow,map_one] using congrArg f omega_cube
  linear_combination (x-x^2)*hs+(x-1)*hc

theorem cubic_coordinate_descends (x : K) (hx : x=0 ∨ x^3=1) :
    ∃ a : F25,f a=x := by
  rcases hx with rfl | hx
  · exact ⟨0,map_zero f⟩
  · have h : (x-1)*(x-f omega)*(x-(f omega)^2)=0 := by
      rw [cubic_factorization f,hx,sub_self]
    rcases mul_eq_zero.mp h with h | h
    · rcases mul_eq_zero.mp h with h | h
      · exact ⟨1,by rw [map_one];exact (sub_eq_zero.mp h).symm⟩
      · exact ⟨omega,(sub_eq_zero.mp h).symm⟩
    · exact ⟨omega^2,by rw [map_pow];exact (sub_eq_zero.mp h).symm⟩

theorem square_coordinate_descends (y : K) (b : F25) (hy : y^2=(f b)^2) :
    ∃ a : F25,f a=y := by
  have h : (y-f b)*(y+f b)=0 := by
    calc
      (y-f b)*(y+f b)=y^2-(f b)^2 := by ring
      _=0 := by rw [hy,sub_self]
  rcases mul_eq_zero.mp h with h | h
  · exact ⟨b,(sub_eq_zero.mp h).symm⟩
  · exact ⟨-b,by rw [map_neg];exact (eq_neg_of_add_eq_zero_left h).symm⟩

theorem y_coordinate_descends (x y : K)
    (he : y^2=x^3+1) (hx : x=0 ∨ x^3=1) :
    ∃ b : F25,f b=y := by
  rcases hx with hx | hx
  · apply square_coordinate_descends f y 1
    simpa only [hx,zero_pow (by decide : (3:ℕ)≠0),zero_add,map_one,one_pow] using he
  · apply square_coordinate_descends f y beta
    have hb : (f beta)^2=2 := by
      simpa only [map_pow,map_ofNat] using congrArg f beta_square
    rw [hb]
    simpa only [hx,one_add_one_eq_two] using he

#print axioms cubic_factorization
#print axioms cubic_coordinate_descends
#print axioms y_coordinate_descends
end
end PairingGeometricCoordinateAudit
