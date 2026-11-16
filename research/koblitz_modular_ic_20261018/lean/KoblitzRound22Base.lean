import KoblitzRound22
import Mathlib.Tactic.FieldSimp
import Mathlib.Algebra.CharP.Defs
namespace KoblitzRound22Base
variable {K : Type*}

theorem quadratic_norm_integer_lift [CommRing K] (a b v : K) :
    (a+b*v)*(a+b*(v+1))=
      a^2+a*b+b^2*(v^2+v)+2*a*b*v := by ring

theorem quadratic_norm [CommRing K] [CharP K 2] (u v a b : K)
    (he : v^2+v=u^3+u) :
    (a+b*v)*(a+b*(v+1))=a^2+a*b+(u^3+u)*b^2 := by
  rw [quadratic_norm_integer_lift,he]
  have h2 : (2:K)=0 := CharP.cast_eq_zero K 2
  rw [h2]
  ring

theorem base11_integer_lift [CommRing K] (u v : K) :
    ((u+1)*v+1)^2+((u+1)*v+1)*(u+1)+u^5=
    (u+1)^2*(v^2+v-(u^3+u))+
      2*((u+1)*v+1+u+u^2+u^3+u^4+u^5) := by ring

theorem base11_rational_equation [Field K] [CharP K 2] (u v : K)
    (hu : u≠0) (he : v^2+v=u^3+u) :
    let r:=((u+1)*v+1)/u^3
    let s:=1/u
    r^2+r*s^3+r*s^2+s=0 := by
  dsimp
  have h2 : (2:K)=0 := CharP.cast_eq_zero K 2
  have hn : ((u+1)*v+1)^2+((u+1)*v+1)*(u+1)+u^5=0 := by
    rw [base11_integer_lift,he,h2]
    ring
  have hz : u^6 * ((((u+1)*v+1)/u^3)^2+
      ((u+1)*v+1)/u^3*(1/u)^3+
      ((u+1)*v+1)/u^3*(1/u)^2+1/u)=0 := by
    calc
      _=((u+1)*v+1)^2+((u+1)*v+1)*(u+1)+u^5 := by field_simp [hu]; ring
      _=0 := hn
  exact (mul_eq_zero.mp hz).resolve_left (pow_ne_zero 6 hu)

theorem linear_coordinate_recovery [Field K] (a b v : K)
    (hb : b≠0) (h : a+b*v=0) : v= -a/b := by
  apply (eq_div_iff hb).mpr
  linear_combination h

theorem char_two_square [CommRing K] [CharP K 2] (x y : K) :
    (x+y)^2=x^2+y^2 := by
  have h2 : (2:K)=0 := CharP.cast_eq_zero K 2
  linear_combination x*y*h2

theorem char_two_eighth [CommRing K] [CharP K 2] (x y : K) :
    (x+y)^8=x^8+y^8 := by
  calc
    (x+y)^8=(((x+y)^2)^2)^2 := by ring
    _=(((x^2+y^2)^2)^2) := by rw [char_two_square]
    _=((x^2)^2+(y^2)^2)^2 := by rw [char_two_square]
    _=((x^2)^2)^2+((y^2)^2)^2 := char_two_square _ _
    _=x^8+y^8 := by ring

theorem linearized_decomposition [CommRing K] [CharP K 2] (l A Δ C : K) :
    (l^2+A*l)^8+Δ*(l^2+A*l)^2+A^2*Δ*(l^2+A*l)+C=
    l^16+A^8*l^8+Δ*l^4+A^3*Δ*l+C := by
  rw [char_two_eighth,char_two_square]
  have h2 : (2:K)=0 := CharP.cast_eq_zero K 2
  linear_combination A^2*Δ*l^2*h2

end KoblitzRound22Base
#print axioms KoblitzRound22Base.quadratic_norm_integer_lift
#print axioms KoblitzRound22Base.quadratic_norm
#print axioms KoblitzRound22Base.base11_integer_lift
#print axioms KoblitzRound22Base.base11_rational_equation
#print axioms KoblitzRound22Base.linear_coordinate_recovery
#print axioms KoblitzRound22Base.char_two_square
#print axioms KoblitzRound22Base.char_two_eighth
#print axioms KoblitzRound22Base.linearized_decomposition
