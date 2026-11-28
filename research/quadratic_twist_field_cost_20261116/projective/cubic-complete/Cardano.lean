import Mathlib.Tactic

namespace TwistCubicComplete
variable {K : Type*} [Field K]

theorem depress_identity (a b c x : K) (h3 : (3:K) ≠ 0) :
    x^3+a*x^2+b*x+c =
      (x+a/3)^3+(b-a^2/3)*(x+a/3)+(2*a^3/27-a*b/3+c) := by
  have h27 : (27:K) ≠ 0 := by
    have heq : (3:K)^3=(27:K) := by ring
    rw [← heq]
    exact pow_ne_zero 3 h3
  field_simp [h3,h27]
  <;> ring

theorem resolvent_product (p q d : K)
    (hd : d^2=(q/2)^2+(p/3)^3) :
    (-q/2+d)*(-q/2-d)=-(p/3)^3 := by
  linear_combination -hd

theorem proportional_common_root (f h a b : K) (ha : a ≠ 0)
    (he : b*f-a*h=0) : (f=0 ∧ h=0) ↔ f=0 := by
  constructor
  · exact And.left
  · intro hf
    refine ⟨hf,?_⟩
    have hz : a*h=0 := by rw [hf] at he; linear_combination -he
    exact (mul_eq_zero.mp hz).resolve_left ha

theorem cardano_factorization (u v w z : K) (hw : w^2+w+1=0) :
    (z-(u+v))*(z-(w*u+w^2*v))*(z-(w^2*u+w*v)) =
      z^3-3*u*v*z-(u^3+v^3) := by
  have hw3 : w^3=1 := by linear_combination (w-1)*hw
  have hw4 : w^4=w := by calc
    w^4 = w^3*w := by ring
    _ = w := by rw [hw3,one_mul]
  have hbc : (w*u+w^2*v)*(w^2*u+w*v)=u^2+v^2-u*v := by
    calc
      _ = w^3*(u^2+v^2)+(w^2+w^4)*(u*v) := by ring
      _ = u^2+v^2-u*v := by
        rw [hw3,hw4]
        linear_combination (u*v)*hw
  have hs : (u+v)+(w*u+w^2*v)+(w^2*u+w*v)=0 := by
    linear_combination (u+v)*hw
  have hpair : (u+v)*(w*u+w^2*v)+(u+v)*(w^2*u+w*v)+
      (w*u+w^2*v)*(w^2*u+w*v) = -3*u*v := by
    rw [hbc]
    linear_combination (u+v)*hs
  have hp : (u+v)*(w*u+w^2*v)*(w^2*u+w*v)=u^3+v^3 := by
    calc
      _ = (u+v)*((w*u+w^2*v)*(w^2*u+w*v)) := by ring
      _ = u^3+v^3 := by rw [hbc]; ring
  linear_combination -z^2*hs+z*hpair-hp

theorem cardano_roots_complete (p q u v w z : K)
    (hw : w^2+w+1=0) (hp : p= -3*u*v) (hq : q= -(u^3+v^3)) :
    z^3+p*z+q=0 ↔ z=u+v ∨ z=w*u+w^2*v ∨ z=w^2*u+w*v := by
  rw [hp,hq]
  have h := cardano_factorization u v w z hw
  have he : z^3+-3*u*v*z+-(u^3+v^3) =
      (z-(u+v))*(z-(w*u+w^2*v))*(z-(w^2*u+w*v)) := by
    linear_combination -h
  rw [he]
  simp only [mul_eq_zero,sub_eq_zero]
  tauto

theorem cube_pow61 (z : K) (hz : z^20=1) : (z^7)^3=z := by
  calc
    (z^7)^3 = z^(1+20) := by rw [← pow_mul]
    _ = z := by rw [pow_add,pow_one,hz,mul_one]

theorem cube_pow3721 (z : K) (hz : z^1240=1) : (z^827)^3=z := by
  calc
    (z^827)^3 = z^(1+1240*2) := by rw [← pow_mul]
    _ = z := by rw [pow_add,pow_mul,pow_one,hz,one_pow,mul_one]
end TwistCubicComplete

#print axioms TwistCubicComplete.depress_identity
#print axioms TwistCubicComplete.resolvent_product
#print axioms TwistCubicComplete.proportional_common_root
#print axioms TwistCubicComplete.cardano_factorization
#print axioms TwistCubicComplete.cardano_roots_complete
#print axioms TwistCubicComplete.cube_pow61
#print axioms TwistCubicComplete.cube_pow3721
