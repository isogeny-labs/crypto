import KoblitzRound17
namespace KoblitzRound17LiftProof
def A {R:Type*} [CommRing R] (x y s t a:R) : R := (4:R)*s^2*t^1 + (1:R)*s^2*t^2 + (2:R)*s^3 + (1:R)*s^3*t^1 + (1:R)*s^4*a^1 + (1:R)*s^5 + (2:R)*y^1*s^2 + (2:R)*y^1*s^2*t^1 + (1:R)*y^1*s^3 + (1:R)*y^2*s^2 + (2:R)*x^1*s^1 + (6:R)*x^1*s^1*t^1 + (4:R)*x^1*s^1*t^2 + (4:R)*x^1*s^2 + (5:R)*x^1*s^2*t^1 + (1:R)*x^1*s^3 + (2:R)*x^1*s^3*a^1 + (3:R)*x^1*s^4 + (2:R)*x^1*y^1*s^1 + (2:R)*x^1*y^1*s^1*t^1 + (2:R)*x^1*y^1*s^2 + (1:R)*x^2 + (2:R)*x^2*t^1 + (1:R)*x^2*t^2 + (2:R)*x^2*s^1 + (2:R)*x^2*s^1*t^1 + (1:R)*x^2*s^2 + (3:R)*x^2*s^3
def B {R:Type*} [CommRing R] (x y s t a:R) : R := (2:R)*y^1*s^2 + (-2:R)*x^1*s^1 + (2:R)*x^1*s^1*t^1 + (2:R)*x^1*s^2 + (6:R)*x^1*y^1*s^1 + (2:R)*x^1*y^1*s^1*t^1 + (1:R)*x^1*y^1*s^2 + (-1:R)*x^2 + (2:R)*x^2*t^1 + (1:R)*x^2*t^2 + (4:R)*x^2*s^1 + (2:R)*x^2*s^1*t^1 + (1:R)*x^2*s^2 + (-1:R)*x^2*s^2*a^1 + (4:R)*x^2*y^1 + (2:R)*x^2*y^1*t^1 + (1:R)*x^2*y^1*s^1 + (2:R)*x^3 + (1:R)*x^3*t^1 + (1:R)*x^3*s^1 + (-2:R)*x^3*s^1*a^1 + (2:R)*x^3*s^2 + (-1:R)*x^4*s^1
def C {R:Type*} [CommRing R] (x y s t a:R) : R := (-2:R)*s^2*t^1 + (-1:R)*s^3 + (-2:R)*y^1*s^2 + (-4:R)*x^1*s^1*t^1 + (-3:R)*x^1*s^2 + (-1:R)*x^1*s^3 + (2:R)*x^1*s^3*a^1 + (2:R)*x^1*s^4 + (-4:R)*x^1*y^1*s^1 + (1:R)*x^1*y^1*s^4 + (-2:R)*x^2*t^1 + (-3:R)*x^2*s^1 + (-2:R)*x^2*s^2 + (4:R)*x^2*s^2*a^1 + (4:R)*x^2*s^3 + (3:R)*x^2*s^3*t^1 + (1:R)*x^2*s^4 + (-2:R)*x^2*y^1 + (4:R)*x^2*y^1*s^3 + (2:R)*x^2*y^1*s^3*t^1 + (1:R)*x^2*y^1*s^4 + (-1:R)*x^3 + (-1:R)*x^3*s^1 + (2:R)*x^3*s^1*a^1 + (4:R)*x^3*s^2 + (4:R)*x^3*s^2*t^1 + (2:R)*x^3*s^3 + (1:R)*x^3*s^3*t^1 + (1:R)*x^3*s^4 + (-2:R)*x^3*s^4*a^1 + (-2:R)*x^3*s^5 + (3:R)*x^3*y^1*s^2 + (2:R)*x^3*y^1*s^2*t^1 + (1:R)*x^3*y^1*s^3 + (2:R)*x^4*s^1 + (1:R)*x^4*s^1*t^1 + (1:R)*x^4*s^2 + (1:R)*x^4*s^2*t^1 + (1:R)*x^4*s^3 + (-2:R)*x^4*s^3*a^1 + (-2:R)*x^4*s^4 + (-2:R)*x^5*s^3
theorem correction_square_residual {R:Type*} [CommRing R] (x y s t a:R) :
 ((y+t)^2+(y+t)*(x+s)+(x+s+a)*(x+s)^2)*(s*y+(t+s+1)*x+s)^2-s*x*(s*x+1)^2*(x+s)^2=
 A x y s t a*(y^2+x*y+x^3+a*x^2+1)+B x y s t a*(t^2+s*t+s^3+a*s^2+1)+2*C x y s t a := by
 unfold A B C
 ring
theorem correction_square_cleared {R:Type*} [CommRing R] [CharP R 2]
 (x y s t a:R) (hp:y^2+x*y+x^3+a*x^2+1=0) (hq:t^2+s*t+s^3+a*s^2+1=0) :
 ((y+t)^2+(y+t)*(x+s)+(x+s+a)*(x+s)^2)*(s*y+(t+s+1)*x+s)^2=s*x*(s*x+1)^2*(x+s)^2 := by
 have h:=correction_square_residual x y s t a
 have h2:(2:R)=0:=CharP.cast_eq_zero R 2
 rw [hp,hq,h2] at h
 simpa using sub_eq_zero.mp (by simpa using h)
theorem correction_square_rational {R:Type*} [Field R] [CharP R 2]
 (x y s t a:R) (hp:y^2+x*y+x^3+a*x^2+1=0) (hq:t^2+s*t+s^3+a*s^2+1=0)
 (hu:x+s≠0) (hd:s*y+(t+s+1)*x+s≠0) :
 ((y+t)^2+(y+t)*(x+s)+(x+s+a)*(x+s)^2)/(x+s)^2=
 s*x*((s*x+1)/(s*y+(t+s+1)*x+s))^2 := by
 have h:=correction_square_cleared x y s t a hp hq
 rw [div_pow, ←mul_div_assoc]
 exact (div_eq_div_iff (pow_ne_zero 2 hu) (pow_ne_zero 2 hd)).2 h
theorem CM_norm_product {R:Type*} [CommRing R] (a b c d:R) :
 (a*c-2*b*d)^2-(a*c-2*b*d)*(a*d+b*c-b*d)+2*(a*d+b*c-b*d)^2=
 (a^2-a*b+2*b^2)*(c^2-c*d+2*d^2) := by ring
end KoblitzRound17LiftProof
#print axioms KoblitzRound17LiftProof.correction_square_residual
#print axioms KoblitzRound17LiftProof.correction_square_cleared
#print axioms KoblitzRound17LiftProof.correction_square_rational
#print axioms KoblitzRound17LiftProof.CM_norm_product
