import PositiveDistinct
set_option autoImplicit false
set_option maxHeartbeats 1000000
namespace MixedTripling
open Polynomial MixedCancellation
variable {K : Type*} [Field K]

theorem repeated_ready (f : K[X]) (a : WeightedAdd11.Input K) (b : D2 K)
 (haz : a.z≠0) (haw : a.w≠0) (hbz : b.z≠0) (hbw : b.w≠0)
 (ha : validPoint f a) (hb : validTwo f b) (hk : rawK a b=0)
 (hc : a.y*rawW3 b ≠ -(rawY3 a b*a.w))
 (he : a.x*(b.z*b.z)=(other a b).x) :
 (2:K)≠0 ∧ a.y≠0 ∧ 8*(a.y*a.y)*a.y≠0 ∧
 nq b= -2*WeightedAdd11.nx a ∧ nr b=(WeightedAdd11.nx a)^2 := by
 have hp := ((shared_support_conclusions f a b haz haw hbz hbw ha hb hk).1).resolve_left hc
 obtain ⟨h2,hy⟩ := MixedPositiveDistinct.positive_nonzero a b hc hp
 have h8 : (8:K)≠0 := by have hh := pow_ne_zero 3 h2;norm_num only [show (2:K)^3=8 by ring] at hh;exact hh
 have hz : (other a b).z≠0 := mul_ne_zero haz hbz
 have hx : WeightedAdd11.nx (other a b)=WeightedAdd11.nx a := by
  dsimp [WeightedAdd11.nx]
  rw [←he]
  dsimp [other]
  field_simp
 obtain ⟨hscale,_,_,hxscale,_⟩ := scales a b haz haw hbz hbw
 have hq : nq b= -2*WeightedAdd11.nx a := by
  apply sub_eq_zero.mp
  calc
   nq b-(-2*WeightedAdd11.nx a) = -((-nq b-WeightedAdd11.nx a)-WeightedAdd11.nx a) := by ring
   _ = 0 := by rw [←hxscale,hx,sub_self,neg_zero]
 have hzprod := mul_ne_zero (mul_ne_zero (pow_ne_zero 4 haz) (pow_ne_zero 4 hbz)) haw
 have hroot : (WeightedAdd11.nx a)^2+nq b*WeightedAdd11.nx a+nr b=0 := by
  rw [hk] at hscale
  exact (mul_eq_zero.mp hscale.symm).resolve_left hzprod
 have hr : nr b=(WeightedAdd11.nx a)^2 := by
  calc
   nr b = ((WeightedAdd11.nx a)^2+nq b*WeightedAdd11.nx a+nr b)+(WeightedAdd11.nx a)^2 := by rw [hq];ring
   _ = (WeightedAdd11.nx a)^2 := by rw [hroot,zero_add]
 exact ⟨h2,hy,mul_ne_zero (mul_ne_zero h8 (mul_ne_zero hy hy)) hy,hq,hr⟩

def halfSecond (a2 a3 x : K) : K := 10*x^3+3*a3*x+a2
def qout (x A : K) : K := 3*x-A^2
def rout (a3 x m A : K) : K := a3-2*A*(m-2*A*x)+3*x*(qout x A-x)
noncomputable def hermite (x y m A : K) : K[X] := C A*(X-C x)^2+C m*(X-C x)+C y

theorem hermite_identity (a0 a1 a2 a3 x y m A : K) :
 MixedGeneric.curve a0 a1 a2 a3-(hermite x y m A)^2 =
 (X-C x)^3*MixedCancellation.support (qout x A) (rout a3 x m A)+
 C (WeightedDoubling.value a0 a1 a2 a3 x-y^2)+
 C (WeightedDoubling.deriv a1 a2 a3 x-2*y*m)*(X-C x)+
 C (halfSecond a2 a3 x-m^2-2*y*A)*(X-C x)^2 := by
 simp only [MixedGeneric.curve,hermite,MixedCancellation.support,qout,rout,WeightedDoubling.value,WeightedDoubling.deriv,halfSecond,C_add,C_sub,C_mul,C_pow,C_neg,map_ofNat]
 ring

theorem affine_tripling_valid (a0 a1 a2 a3 x y : K)
 (hp : WeightedDoubling.value a0 a1 a2 a3 x=y^2) (hy : 2*y≠0) :
 let m := WeightedDoubling.deriv a1 a2 a3 x/(2*y)
 let A := (halfSecond a2 a3 x-m^2)/(2*y)
 let q := qout x A
 let r := rout a3 x m A
 MixedCancellation.support q r ∣ MixedGeneric.curve a0 a1 a2 a3-
 (ordinate (A*q-(m-2*A*x)) (A*r-(y-m*x+A*x^2)))^2 := by
 dsimp only
 let m := WeightedDoubling.deriv a1 a2 a3 x/(2*y)
 let A := (halfSecond a2 a3 x-m^2)/(2*y)
 have hd : WeightedDoubling.deriv a1 a2 a3 x-2*y*m=0 := by dsimp [m];rw [mul_comm (2*y),div_mul_cancel₀ _ hy,sub_self]
 have hh : halfSecond a2 a3 x-m^2-2*y*A=0 := by dsimp only [A];rw [mul_comm (2*y),div_mul_cancel₀ _ hy,sub_self]
 have hi := hermite_identity a0 a1 a2 a3 x y m A
 simp only [hp,sub_self,hd,hh,C_0,zero_mul,add_zero] at hi
 let u := MixedCancellation.support (qout x A) (rout a3 x m A)
 let v := ordinate (A*qout x A-(m-2*A*x)) (A*rout a3 x m A-(y-m*x+A*x^2))
 have hv : hermite x y m A+v=C A*u := by
  dsimp [hermite,v,u,MixedCancellation.support,ordinate]
  simp only [C_add,C_sub,C_mul,C_pow,C_neg,map_ofNat];ring
 change u ∣ MixedGeneric.curve a0 a1 a2 a3-v^2
 refine ⟨(X-C x)^3+C A*(hermite x y m A-v),?_⟩
 calc
  MixedGeneric.curve a0 a1 a2 a3-v^2 = (MixedGeneric.curve a0 a1 a2 a3-(hermite x y m A)^2)+(hermite x y m A-v)*(hermite x y m A+v) := by ring
  _ = u*((X-C x)^3+C A*(hermite x y m A-v)) := by rw [hi,hv];ring
def rawOutput (a1 a2 a3 : K) (a : WeightedAdd11.Input K) : D2 K :=
 let x2 := a.x*a.x
 let x3 := x2*a.x
 let x4 := x2*x2
 let z2 := a.z*a.z
 let z4 := z2*z2
 let z6 := z4*z2
 let z8 := z4*z4
 let y2 := a.y*a.y
 let fp := (((a1*z8+2*(a2*(a.x*z6)))+3*(a3*(x2*z4)))+5*x4)*a.w
 let fpp := 2*((a2*z6+3*(a3*(a.x*z4)))+10*x3)
 let k := 8*(y2*a.y)
 let aa := (2*(fpp*y2)-fp*fp)*a.w
 let bb := 2*(fp*y2)-aa*a.x
 let cc := a.y*k-(aa*a.x+2*bb)*(a.x*a.w)
 let k2 := k*k
 let k4 := k2*k2
 let z3 := k*a.z
 let q3 := 3*(a.x*k2)-aa*aa
 let r3 := a3*((z3*z3)*(z3*z3))-(4*(aa*bb)-3*(a.x*(q3-a.x*k2)))*k2
 let s3 := aa*(q3*a.w)-2*(bb*(k2*a.w))
 let t3 := aa*(r3*a.w)-cc*k4
 ⟨q3,r3,s3,t3,z3,a.w⟩

theorem raw_normalizes (a1 a2 a3 : K) (a : WeightedAdd11.Input K)
 (haz : a.z≠0) (haw : a.w≠0) (hy : a.y≠0) (h2 : (2:K)≠0) :
 let x := WeightedAdd11.nx a
 let y := WeightedAdd11.ny a
 let m := WeightedDoubling.deriv a1 a2 a3 x/(2*y)
 let A := (halfSecond a2 a3 x-m^2)/(2*y)
 let c := rawOutput a1 a2 a3 a
 c.z≠0 ∧ c.w≠0 ∧ nq c=qout x A ∧ nr c=rout a3 x m A ∧
 ns c=A*qout x A-(m-2*A*x) ∧ nt c=A*rout a3 x m A-(y-m*x+A*x^2) := by
 have h8 : (8:K)≠0 := by have hh := pow_ne_zero 3 h2;norm_num only [show (2:K)^3=8 by ring] at hh;exact hh
 dsimp only
 refine ⟨mul_ne_zero (mul_ne_zero h8 (mul_ne_zero (mul_ne_zero hy hy) hy)) haz,haw,?_,?_,?_,?_⟩
 all_goals
  dsimp [rawOutput,nq,nr,ns,nt,qout,rout,halfSecond,WeightedAdd11.nx,WeightedAdd11.ny,WeightedDoubling.deriv]
  field_simp
  <;> ring
theorem weighted_tripling_preserves (a0 a1 a2 a3 : K) (a : WeightedAdd11.Input K)
 (haz : a.z≠0) (haw : a.w≠0) (hy : a.y≠0) (h2 : (2:K)≠0)
 (ha : validPoint (MixedGeneric.curve a0 a1 a2 a3) a) :
 let c := rawOutput a1 a2 a3 a
 c.z≠0 ∧ c.w≠0 ∧ validTwo (MixedGeneric.curve a0 a1 a2 a3) c := by
 obtain ⟨hz,hw,hq,hr,hs,ht⟩ := raw_normalizes a1 a2 a3 a haz haw hy h2
 refine ⟨hz,hw,?_⟩
 change MixedCancellation.support _ _ ∣ _-(ordinate _ _)^2
 rw [hq,hr,hs,ht]
 apply affine_tripling_valid
 · exact MixedPositiveDistinct.point_value a0 a1 a2 a3 a ha
 · exact mul_ne_zero h2 (div_ne_zero hy (mul_ne_zero (pow_ne_zero 5 haz) haw))

theorem source_repeated_preserves [DecidableEq K] (a0 a1 a2 a3 : K) (a : WeightedAdd11.Input K) (b : D2 K)
 (haz : a.z≠0) (haw : a.w≠0) (hbz : b.z≠0) (hbw : b.w≠0)
 (ha : validPoint (MixedGeneric.curve a0 a1 a2 a3) a)
 (hb : validTwo (MixedGeneric.curve a0 a1 a2 a3) b) (hk : rawK a b=0)
 (hc : a.y*rawW3 b ≠ -(rawY3 a b*a.w))
 (he : a.x*(b.z*b.z)=(other a b).x) :
 let c := rawOutput a1 a2 a3 a
 classify a b=.positiveCoincidence ∧ (2:K)≠0 ∧ a.y≠0 ∧
 c.z≠0 ∧ c.w≠0 ∧ validTwo (MixedGeneric.curve a0 a1 a2 a3) c := by
 obtain ⟨h2,hy,_,_,_⟩ := repeated_ready (MixedGeneric.curve a0 a1 a2 a3) a b haz haw hbz hbw ha hb hk hc he
 have hp := ((shared_support_conclusions (MixedGeneric.curve a0 a1 a2 a3) a b haz haw hbz hbw ha hb hk).1).resolve_left hc
 exact ⟨by simp only [classify,hk,ne_eq,not_true_eq_false,ite_false];rw [if_neg hc];simp [hp],h2,hy,weighted_tripling_preserves a0 a1 a2 a3 a haz haw hy h2 ha⟩
end MixedTripling
