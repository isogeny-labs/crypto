import KoblitzRound12Boundary
import Mathlib.Algebra.CharP.Basic
import Mathlib.RingTheory.PowerSeries.Order
namespace KoblitzRound12

theorem constant_multiplication_saturation {R : Type*} [CommRing R]
 (h out a c x : R) : h*(out-c*x)=(h*out-a*x)+x*(a-c*h) := by ring

theorem constant_inverse_saturation {R : Type*} [CommRing R]
 (h out a c d : R) (hc : c*d=1) :
 h*(out-d*h)=d*((a*out-h^2)-(a-c*h)*out) := by
 have hd : d*c=1 := by rw [mul_comm,hc]
 calc
  h*(out-d*h)=h*out-d*h^2 := by ring
  _=d*c*h*out-d*h^2 := by rw [hd];ring
  _=d*((a*out-h^2)-(a-c*h)*out) := by ring

theorem saturated_constant_product {R : Type*} [CommRing R]
 (h out a b c d : R) : h*(out-c*d*h)=
 (h*out-a*b)+(a-c*h)*b+c*h*(b-d*h) := by ring

theorem strict_saturation_from_counterpoint {R S : Type*} [CommRing R] [CommRing S]
 (I : Ideal R) (h g : R) (ev : R →+* S)
 (hI : ∀ f∈I, ev f=0) (hg : h*g∈I) (hne : ev g≠0) :
 g∉I ∧ ∃ n : ℕ, h^n*g∈I := by
 constructor
 · intro hi;exact hne (hI g hi)
 · exact ⟨1,by simpa using hg⟩

theorem homogeneous_constant_false_point :
 ((0:ZMod 2)*1-0*0=0) ∧ ((0:ZMod 2)-1*0=0) ∧
 ((1:ZMod 2)-1*0≠0) := by decide +kernel

theorem velu_discriminant_residual {R : Type*} [CommRing R] (s c : R) :
 (s^2*(1+s*c+s^3+s^4*c)+s*(s+s^2*c)+(s+s^2*c)^2)-
 (s^2+s^4*c^2+s^5+s^6*c)=2*(s^2+2*s^3*c) := by ring

theorem two_seven_quotient_coefficients {R : Type*} [CommRing R]
 [CharP R 2] (s c d : R) :
 (s+s^2*c)+(s+s^2*d)=s^2*(c+d) ∧
 (1+s*c+s^3+s^4*c)+(1+s*d+s^3+s^4*d)=s*(c+d)+s^4*(c+d) := by
 have h2 : (2:R)=0 := by simpa using (CharP.natCast_eq_natCast_mod R 2 2)
 constructor
 · linear_combination s*h2
 · linear_combination (1+s^3)*h2

theorem seven_parameter_factorization {R : Type*} [CommRing R] [CharP R 2]
 (t : R) : t^8+t^4+1=(t^2+t+1)^4 := by
 have h : (t^2+t+1)^4=(t^2+t)^4+1 := by
  simpa using (add_pow_char_pow (t^2+t) (1:R) 2 2)
 have hh : (t^2+t)^4=t^8+t^4 := by
  calc
   (t^2+t)^4=(t^2)^4+t^4 := by
    simpa using (add_pow_char_pow (t^2) t 2 2)
   _=t^8+t^4 := by ring
 rw [h,hh]

theorem seven_parameter_ratio {F : Type*} [Field F] (a j k t : F)
 (ht : t≠0) (hj : j≠0) (h0 : a=j*t^7) (h1 : a=k*t) : k/j=t^6 := by
 have h : j*t^7=k*t := h0.symm.trans h1
 have e : j*t^6=k := by
  apply mul_right_cancel₀ ht
  calc j*t^6*t=j*t^7 := by ring
       _=k*t := h
 field_simp
 exact e.symm

theorem eta_y_recovery_unique {F : Type*} [Field F] (d a y z : F)
 (hd : d≠0) (hy : d*y=a) (hz : d*z=a) : y=z := by
 exact mul_left_cancel₀ hd (hy.trans hz.symm)

theorem uniformizer_order_unit_factor {R : Type*} [CommRing R] [IsDomain R]
 (u f : PowerSeries R) (hu : PowerSeries.order u=0) :
 PowerSeries.order (u*f)=PowerSeries.order f := by
 rw [PowerSeries.order_mul,hu,zero_add]

theorem coprime_primary_image_zero {A : Type*} [AddCommGroup A] (p : A)
 (h8 : (8:ℤ) • p=0) (h11 : (11:ℤ) • p=0) : p=0 := by
 have h : (7*8-5*11:ℤ) • p=0 := by rw [sub_zsmul,mul_zsmul,mul_zsmul,h8,h11];simp
 norm_num at h
 exact h

def E49 {R : Type*} [CommRing R] (x y : R) : R := y^2+x*y+x^3+x^2+1

theorem F4_six_supersingular_target_points {R : Type*} [CommRing R] [CharP R 2]
 (w : R) (hw : w^2+w+1=0) :
 E49 1 w=0 ∧ E49 1 (w^2)=0 ∧ E49 w 1=0 ∧
 E49 w (w^2)=0 ∧ E49 (w^2) 1=0 ∧ E49 (w^2) w=0 := by
 have h2 : (2:R)=0 := by simpa using (CharP.natCast_eq_natCast_mod R 2 2)
 have h3 : w^3=1 := by linear_combination (w-1)*hw
 have h4 : w^4=w := by calc w^4=w*w^3 := by ring
                                         _=w := by rw [h3];ring
 have h6 : w^6=1 := by calc w^6=(w^3)^2 := by ring
                                         _=1 := by rw [h3];ring
 unfold E49
 refine ⟨?_,?_,?_,?_,?_,?_⟩
 · linear_combination hw+h2
 · linear_combination hw+h2+h4
 · linear_combination hw+h2+h3
 · linear_combination hw+h4+w^3*h2
 · linear_combination hw+h2+h6+h4
 · linear_combination hw+h2+h6+h4+h3

theorem antidiagonal_SS_degree : 420*1+40*2+60*4=740 ∧ 1240-740=500 := by norm_num

theorem SS_closed_point_count : 308*10=3080 ∧ 4*12+2*4=56 := by norm_num

theorem eight_divisor_degree_partition : List.zipWith (·+·)
 ([1600,760,1540,1020,0,2060,2080,1720] : List ℕ)
 [500,1760,1880,2760,0,580,1370,2180]=
 [2100,2520,3420,3780,0,2640,3450,3900] := by decide +kernel

end KoblitzRound12
#print axioms KoblitzRound12.constant_multiplication_saturation
#print axioms KoblitzRound12.constant_inverse_saturation
#print axioms KoblitzRound12.saturated_constant_product
#print axioms KoblitzRound12.strict_saturation_from_counterpoint
#print axioms KoblitzRound12.homogeneous_constant_false_point
#print axioms KoblitzRound12.velu_discriminant_residual
#print axioms KoblitzRound12.two_seven_quotient_coefficients
#print axioms KoblitzRound12.seven_parameter_factorization
#print axioms KoblitzRound12.seven_parameter_ratio
#print axioms KoblitzRound12.eta_y_recovery_unique
#print axioms KoblitzRound12.uniformizer_order_unit_factor
#print axioms KoblitzRound12.coprime_primary_image_zero
#print axioms KoblitzRound12.F4_six_supersingular_target_points
#print axioms KoblitzRound12.antidiagonal_SS_degree
#print axioms KoblitzRound12.SS_closed_point_count
#print axioms KoblitzRound12.eight_divisor_degree_partition
