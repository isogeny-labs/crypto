import KoblitzRound10
namespace KoblitzRound11

theorem saturation_other_inverse {R : Type*} [CommRing R] (x z y v h : R) :
 h*(z*v-x*h)=x*(y*v-h^2)-v*(x*y-z*h) := by ring

theorem saturation_cubic {R : Type*} [CommRing R] (x z y u v h : R) :
 h*(z*u*v-h^3)=y*v*(x*u-h^2)+h^2*(y*v-h^2)-u*v*(x*y-z*h) := by ring

theorem saturation_cubic_redundant {R : Type*} [CommRing R] (z y u v h : R) :
 z*u*v-h^3=v*(z*u-y*h)+h*(y*v-h^2) := by ring

theorem projective_chart_transition {F : Type*} [Field F] (x c d : F)
 (hc : c≠0) (hd : d≠0) : (x/c)/(d/c)=x/d := by field_simp

theorem affine_two_inverse_recovery {R : Type*} [CommRing R]
 (x y z u v : R) (hxy : x*y=z) (hxu : x*u=1) (hyv : y*v=1) :
 z*u=y ∧ z*v=x := by
 constructor
 · rw [← hxy]; calc x*y*u=(x*u)*y := by ring
                    _=y := by rw [hxu];ring
 · rw [← hxy]; calc x*y*v=x*(y*v) := by ring
                    _=x := by rw [hyv];ring

theorem hamilton_norm_product {R : Type*} [CommRing R] (a b c d e f g h : R) :
 (a*e-b*f-c*g-d*h)^2+(a*f+b*e+c*h-d*g)^2+
 (a*g-b*h+c*e+d*f)^2+(a*h+b*g-c*f+d*e)^2=
 (a^2+b^2+c^2+d^2)*(e^2+f^2+g^2+h^2) := by ring

theorem cube_correction_residual {R : Type*} [CommRing R] (u e : R) :
 (u+e)^3-u^3-3*u^2*e=e^2*(3*u+e) := by ring

theorem normalization_tangent_determinant :
 (Matrix.det (!![0,1,0,0; 1,0,0,0; 0,0,1,0; 0,0,0,1] :
 Matrix (Fin 4) (Fin 4) (ZMod 2)))=1 := by decide +kernel

def W {R : Type*} [CommRing R] (s a b x y : R) : R :=
 y^2+s*x*y+y-x^3-a*x-b

/-- Universal integer residual, before specialization to characteristic2. -/
theorem weierstrass_transformation {R : Type*} [CommRing R]
 (s a b aa bb u r v t x y : R) :
 W s a b (u^2*x+r) (u^3*y+u^2*v*x+t)-u^6*W s aa bb x y=
 u^5*(2*v+s-s*u)*x*y+u^3*(2*t+s*r+1-u^3)*y+
 u^4*(v^2+s*v-3*r)*x^2+
 u^2*(2*v*t+s*t+s*r*v+v-3*r^2-a+u^4*aa)*x+
 (t^2+s*r*t+t-r^3-a*r-b+u^6*bb) := by unfold W;ring

theorem velu_w_residual {R : Type*} [CommRing R] (s x : R) :
 (s^2*x^2+1)+x*(s^2*x+s)-(1+s*x)=2*s^2*x^2 := by ring

theorem velu_special_coefficients {R : Type*} [CommRing R] (s c : R) :
 s^2*(s+s^2*c)+(1+s*c)=1+s*c+s^3+s^4*c := by ring

theorem local_contact_degree : 260*1+100*2+20*6+20*14=860 := by norm_num

theorem recovered_multiplicity : 260*(1-1)+100*(2-1)+20*(6-1)+20*(14-1)=460 := by norm_num

theorem closed_contact_degree : 10*(26*1+10*2+2*6+2*14)=860 := by norm_num

theorem diagonal_antidiagonal_residual :
 1280-420=860 ∧ 1360-120=1240 ∧ 860+1240=2100 := by norm_num

theorem translated_product_identity {R : Type*} [CommRing R] (x u z : R)
 (h : x*u=1) : x*(z+u)=x*z+1 := by rw [mul_add,h]

theorem translated_divisor_cancellation {A : Type*} [AddCommGroup A]
 (t o q d e : A) : (2:ℤ) • (t-o)+(d+e-(2:ℤ) • t-(2:ℤ) • q)=
 d+e-(2:ℤ) • o-(2:ℤ) • q := by simp only [zsmul_sub];abel

theorem equal_y_factorization {R : Type*} [CommRing R] (x u y : R) :
 (y^2+u*y-u^3-u^2-1)-(y^2+x*y-x^3-x^2-1)=
 (u-x)*(y-u^2-u*x-x^2-u-x) := by ring

theorem AS_recovery_residual {R : Type*} [CommRing R] (x z : R) :
 ((x*z+1)^2+x*(x*z+1)-x^3-x^2-1)-
 x^2*(z^2+z-x-1)-x=2*x*z := by ring

theorem AS_sum {R : Type*} [CommRing R] [CharP R 2] (z w a b : R)
 (hz : z^2+z=a) (hw : w^2+w=b) : (z+w)^2+(z+w)=a+b := by
 have h2 : (2:R)=0 := by simpa using (CharP.natCast_eq_natCast_mod R 2 2)
 calc
  (z+w)^2+(z+w)=(z^2+z)+(w^2+w)+2*z*w := by ring
  _=a+b := by rw [hz,hw,h2];ring

theorem reciprocal_pole_identity {F : Type*} [Field F] [CharP F 2] (x : F)
 (h0 : x≠0) (h1 : x+1≠0) : 1/x+1/(x+1)=1/(x*(x+1)) := by
 have h2 : (2:F)=0 := by simpa using (CharP.natCast_eq_natCast_mod F 2 2)
 field_simp
 linear_combination x*h2

theorem AS_genus_arithmetic : (-4+2*2 : ℤ)=0 ∧ (1+1+1 : ℕ)=3 := by norm_num
end KoblitzRound11
#print axioms KoblitzRound11.saturation_other_inverse
#print axioms KoblitzRound11.saturation_cubic
#print axioms KoblitzRound11.saturation_cubic_redundant
#print axioms KoblitzRound11.projective_chart_transition
#print axioms KoblitzRound11.affine_two_inverse_recovery
#print axioms KoblitzRound11.hamilton_norm_product
#print axioms KoblitzRound11.cube_correction_residual
#print axioms KoblitzRound11.normalization_tangent_determinant
#print axioms KoblitzRound11.weierstrass_transformation
#print axioms KoblitzRound11.velu_w_residual
#print axioms KoblitzRound11.velu_special_coefficients
#print axioms KoblitzRound11.local_contact_degree
#print axioms KoblitzRound11.recovered_multiplicity
#print axioms KoblitzRound11.closed_contact_degree
#print axioms KoblitzRound11.diagonal_antidiagonal_residual
#print axioms KoblitzRound11.translated_product_identity
#print axioms KoblitzRound11.translated_divisor_cancellation
#print axioms KoblitzRound11.equal_y_factorization
#print axioms KoblitzRound11.AS_recovery_residual
#print axioms KoblitzRound11.AS_sum
#print axioms KoblitzRound11.reciprocal_pole_identity
#print axioms KoblitzRound11.AS_genus_arithmetic
