import KoblitzRound17LiftProof
namespace KoblitzRound18

theorem square_injective_char_two {K:Type*} [Field K] [CharP K 2]
 (a b:K) (h:a^2=b^2) : a=b := by
 have h2:(2:K)=0 := CharP.cast_eq_zero K 2
 have hz:(a-b)^2=0 := by
  calc
   (a-b)^2=a^2-b^2-2*b*(a-b) := by ring
   _=0 := by rw [h,h2];ring
 exact sub_eq_zero.mp (sq_eq_zero_iff.mp hz)

theorem symmetric_scalar_unique {K:Type*} [Field K] [CharP K 2]
 (a b d:K) (hd:d≠0) (ha:a^2*d=1) (hb:b^2*d=1) : a=b := by
 apply square_injective_char_two
 exact mul_right_cancel₀ hd (ha.trans hb.symm)

theorem self_inverse_unit_char_two {K:Type*} [Field K] [CharP K 2]
 (u:Kˣ) (h:u=u⁻¹) : u=1 := by
 apply Units.ext
 apply square_injective_char_two
 have hh:u*u=1 := by
  calc
   u*u=u*u⁻¹ := congrArg (fun z:Kˣ=>u*z) h
   _=1 := mul_inv_cancel u
 simpa [pow_two] using congrArg (fun v:Kˣ=>(v:K)) hh

theorem symmetric_character_unique {G K:Type*} [Group G] [Field K] [CharP K 2]
 (chi:G→*Kˣ) (h:∀g,chi g=chi g⁻¹) : chi=1 := by
 apply MonoidHom.ext
 intro g
 exact self_inverse_unit_char_two (chi g) (by simpa only [map_inv] using h g)

theorem Frobenius_normalization_unique {K:Type*} [Field K] [CharP K 2]
 (sigma:K→+*K) (s t d:K) (hd:sigma d≠0)
 (hs:s^2*d=1) (ht:t^2*sigma d=1) : sigma s=t := by
 apply symmetric_scalar_unique (sigma s) t (sigma d) hd _ ht
 simpa using congrArg sigma hs

theorem separator_membership_depth {R:Type*} [CommRing R]
 (h x U V c0 c1 c2 c3:R) (k:Nat)
 (hf:h^k*(x^3-h^2*U)=0)
 (hg:h^k*(c3*x^4+c2*h*x^3+c1*h^2*x^2+c0*h^3*x-h^3*V)=0) :
 h^(k+4)*KoblitzRound16.separator h U V c0 c1 c2 c3=0 := by
 calc
  h^(k+4)*KoblitzRound16.separator h U V c0 c1 c2 c3=
   h^k*(h^4*KoblitzRound16.separator h U V c0 c1 c2 c3) := by rw [pow_add];ring
  _=(h^k*(c3*x^4+c2*h*x^3+c1*h^2*x^2+c0*h^3*x-h^3*V))*KoblitzRound16.adjugate h x U V c0 c1 c2 c3-
   (h^k*(x^3-h^2*U))*KoblitzRound16.cofactor h x U V c0 c1 c2 c3 := by rw [KoblitzRound16.separator_residual h x U V c0 c1 c2 c3];ring
  _=0 := by rw [hf,hg];ring

theorem square_cube_residual {R:Type*} [CommRing R] (h x U V:R) :
 h^3*(U^3-h*V^2)=(x^3-h^2*V)*(x^3+h^2*V)-
 (x^2-h*U)*(x^4+h*x^2*U+h^2*U^2) := by ring

theorem square_cube_membership_depth {R:Type*} [CommRing R]
 (h x U V:R) (k:Nat) (hf:h^k*(x^2-h*U)=0) (hg:h^k*(x^3-h^2*V)=0) :
 h^(k+3)*(U^3-h*V^2)=0 := by
 calc
  h^(k+3)*(U^3-h*V^2)=h^k*(h^3*(U^3-h*V^2)) := by rw [pow_add];ring
  _=(h^k*(x^3-h^2*V))*(x^3+h^2*V)-(h^k*(x^2-h*U))*(x^4+h*x^2*U+h^2*U^2) := by rw [square_cube_residual h x U V];ring
  _=0 := by rw [hf,hg];ring

theorem commuting_period {G:Type*} [Monoid G] (r p:G)
 (hc:Commute r p) (hr:r^5=1) (hp:p^168=1) : (r*p)^840=1 := by
 rw [hc.mul_pow]
 have hR:r^840=(r^5)^168 := by rw [←pow_mul]
 have hP:p^840=(p^168)^5 := by rw [←pow_mul]
 rw [hR,hP,hr,hp]
 simp

theorem elliptic_map_cleared {R:Type*} [CommRing R] (x t a:R) :
 (t*x)^2+x*(t*x)+x^3+a*x^2+1=x^2*(t^2+t+x+a)+1 := by ring

theorem elliptic_map_division {K:Type*} [Field K] (x y a:K) (hx:x≠0) :
 y^2+x*y+x^3+a*x^2+1=x^2*((y/x)^2+y/x+x+a+(1/x)^2) := by
 field_simp [hx]
 <;> ring
end KoblitzRound18
#print axioms KoblitzRound18.square_injective_char_two
#print axioms KoblitzRound18.symmetric_scalar_unique
#print axioms KoblitzRound18.self_inverse_unit_char_two
#print axioms KoblitzRound18.symmetric_character_unique
#print axioms KoblitzRound18.Frobenius_normalization_unique
#print axioms KoblitzRound18.separator_membership_depth
#print axioms KoblitzRound18.square_cube_residual
#print axioms KoblitzRound18.elliptic_map_division

#print axioms KoblitzRound18.square_cube_membership_depth
#print axioms KoblitzRound18.commuting_period
#print axioms KoblitzRound18.elliptic_map_cleared
