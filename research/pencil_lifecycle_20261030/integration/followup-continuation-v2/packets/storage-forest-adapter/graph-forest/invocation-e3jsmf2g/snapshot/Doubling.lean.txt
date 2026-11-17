import Add11
set_option autoImplicit false
namespace WeightedDoubling
open Polynomial
variable {K : Type*} [Field K]
noncomputable def curve (a0 a1 a2 a3 : K) : K[X] := X^5+C a3*X^3+C a2*X^2+C a1*X+C a0
def value (a0 a1 a2 a3 x : K) : K := x^5+a3*x^3+a2*x^2+a1*x+a0
def deriv (a1 a2 a3 x : K) : K := 5*x^4+3*a3*x^2+2*a2*x+a1

theorem tangent_identity (a0 a1 a2 a3 x y m : K) :
 curve a0 a1 a2 a3 - (C m*X+C (y-m*x))^2 =
 (X-C x)^2 * (X^3+C (2*x)*X^2+C (3*x^2+a3)*X+C (4*x^3+2*a3*x+a2-m^2)) +
 C (value a0 a1 a2 a3 x-y^2) + C (deriv a1 a2 a3 x-2*y*m)*(X-C x) := by
 simp only [curve,value,deriv,C_add,C_sub,C_mul,C_pow,C_neg,map_ofNat]
 ring

theorem tangent_valid (a0 a1 a2 a3 x y : K)
 (hp : value a0 a1 a2 a3 x = y^2) (hy : 2*y ≠ 0) :
 let m := deriv a1 a2 a3 x/(2*y)
 (X-C x)^2 ∣ curve a0 a1 a2 a3-(C m*X+C (y-m*x))^2 := by
 dsimp
 have hd : deriv a1 a2 a3 x - 2*y*(deriv a1 a2 a3 x/(2*y)) = 0 := by rw [mul_comm (2*y), div_mul_cancel₀ _ hy, sub_self]
 rw [tangent_identity,hp,sub_self,hd,C_0,zero_mul,add_zero,add_zero]
 exact dvd_mul_right _ _


structure Out (K : Type*) where
 q : K
 r : K
 s : K
 t : K
 z : K
 w : K

def output (a0 a1 a2 a3 x y z w : K) : Out K :=
 let z2 := z*z
 let z4 := z2*z2
 let z6 := z4*z2
 let z8 := z4*z4
 let z10 := z8*z2
 let x2 := x*x
 let x3 := x2*x
 let x4 := x2*x2
 let x5 := x4*x
 ⟨-(2*x),x2,(((a1*z8+2*(a2*(x*z6)))+3*(a3*(x2*z4)))+5*x4)*w,
   (((2*(a0*z10)+a1*(x*z8))-a3*(x3*z4))-3*x5)*w,z,2*y⟩

theorem normalized_output (a0 a1 a2 a3 x y z w : K)
 (hz : z ≠ 0) (hw : w ≠ 0) (h2 : (2:K) ≠ 0) (hy : y ≠ 0) :
 let c := output a0 a1 a2 a3 x y z w
 let xx := x/z^2
 let yy := y/(z^5*w)
 c.z ≠ 0 ∧ c.w ≠ 0 ∧ 2*yy ≠ 0 ∧
 c.q/c.z^2 = -2*xx ∧ c.r/c.z^4 = xx^2 ∧
 c.s/(c.z^3*c.w) = deriv a1 a2 a3 xx/(2*yy) ∧
 c.t/(c.z^5*c.w) = (2*a0+a1*xx-a3*xx^3-3*xx^5)/(2*yy) := by
 dsimp
 refine ⟨hz,mul_ne_zero h2 hy,mul_ne_zero h2 (div_ne_zero hy (mul_ne_zero (pow_ne_zero 5 hz) hw)),?_,?_,?_,?_⟩
 all_goals dsimp [output,deriv]; field_simp <;> ring

theorem intercept (a0 a1 a2 a3 x y : K) (hp : value a0 a1 a2 a3 x=y^2)
 (h2 : (2:K) ≠ 0) (hy : y ≠ 0) :
 (2*a0+a1*x-a3*x^3-3*x^5)/(2*y) = y-(deriv a1 a2 a3 x/(2*y))*x := by
 have hid : 2*a0+a1*x-a3*x^3-3*x^5=2*y^2-x*deriv a1 a2 a3 x := by
  rw [←hp]; unfold value deriv; ring
 rw [hid]
 field_simp
 <;> ring

theorem weighted_preserves (a0 a1 a2 a3 x y z w : K)
 (hz : z ≠ 0) (hw : w ≠ 0) (h2 : (2:K) ≠ 0) (hy : y ≠ 0)
 (hp : value a0 a1 a2 a3 (x/z^2)=(y/(z^5*w))^2) :
 let c := output a0 a1 a2 a3 x y z w
 c.z ≠ 0 ∧ c.w ≠ 0 ∧
 (X^2+C (c.q/c.z^2)*X+C (c.r/c.z^4)) ∣
 curve a0 a1 a2 a3-(C (c.s/(c.z^3*c.w))*X+C (c.t/(c.z^5*c.w)))^2 := by
 obtain ⟨hz',hw',hy',hq,hr,hs,ht⟩ := normalized_output a0 a1 a2 a3 x y z w hz hw h2 hy
 refine ⟨hz',hw',?_⟩
 rw [hq,hr,hs,ht,intercept a0 a1 a2 a3 (x/z^2) (y/(z^5*w)) hp h2 (div_ne_zero hy (mul_ne_zero (pow_ne_zero 5 hz) hw))]
 have hu : X^2+C (-2*(x/z^2))*X+C ((x/z^2)^2) = (X-C (x/z^2))^2 := by
  simp only [C_mul,C_neg,C_pow,map_ofNat]; ring
 rw [hu]
 exact tangent_valid a0 a1 a2 a3 (x/z^2) (y/(z^5*w)) hp hy'


theorem self_guard [DecidableEq K] (x y z w : K)
 (hz : z ≠ 0) (hw : w ≠ 0) (h2 : (2:K) ≠ 0) (hy : y ≠ 0) :
 Add11Distinct.classify (x*z^2) (y*(z^5*w)) (x*z^2) (y*(z^5*w)) = .doubling := by
 have hv : y*(z^5*w) ≠ 0 := mul_ne_zero hy (mul_ne_zero (pow_ne_zero 5 hz) hw)
 have hn : y*(z^5*w) ≠ -(y*(z^5*w)) := by
  intro he
  have hh : 2*(y*(z^5*w)) = 0 := by
   calc
    _ = y*(z^5*w)+y*(z^5*w) := by ring
    _ = 0 := by nth_rw 1 [he]; ring
  exact (mul_ne_zero h2 hv) hh
 simp [Add11Distinct.classify,hn]

theorem zero_y_inverse [DecidableEq K] (x z w : K) :
 Add11Distinct.classify (x*z^2) ((0:K)*(z^5*w)) (x*z^2) ((0:K)*(z^5*w)) = .inverse := by
 simp [Add11Distinct.classify]
end WeightedDoubling
