import KoblitzRound18
namespace KoblitzRound18Tate

def curve {R:Type*} [CommRing R] (b c x y:R) : R := y^2+(1+c)*x*y+b*y+x^3+b*x^2

def transformed {R:Type*} [CommRing R] (b c s u X Y:R) : R :=
 u^6*(Y^2+X^3)+(1+c)*u^5*X*Y+b*c*u^3*Y+(s^2+(1+c)*s)*u^4*X^2

def residualCofactor {R:Type*} [CommRing R] (b c s u X Y:R) : R :=
 s*u^5*X*Y+b*(1+c)*u^3*Y+2*b*u^4*X^2+b*(1+c)*s*u^2*X+2*b^2*u^2*X+b^2*c*(1+c)+b^3

theorem Tate_coordinate_residual {R:Type*} [CommRing R] (b c s u X Y:R) :
 curve b c (u^2*X+b) (u^3*Y+s*u^2*X+b*c)-transformed b c s u X Y-
 b*u^2*X*(c*s+(1+c)*c+b)=2*residualCofactor b c s u X Y := by
 unfold curve transformed residualCofactor
 ring

theorem Tate_coordinate_char_two {R:Type*} [CommRing R] [CharP R 2]
 (b c s u X Y:R) (hs:c*s+(1+c)*c+b=0) :
 curve b c (u^2*X+b) (u^3*Y+s*u^2*X+b*c)=transformed b c s u X Y := by
 have h:=Tate_coordinate_residual b c s u X Y
 have h2:(2:R)=0 := CharP.cast_eq_zero R 2
 rw [hs,h2] at h
 exact sub_eq_zero.mp (by simpa using h)

theorem actual_Tate_form {R:Type*} [CommRing R] [CharP R 2]
 (b c s u ap bp X Y:R) (hs:c*s+(1+c)*c+b=0)
 (ha:u*ap=1+c) (hb:u^3*bp=b*c) (hbb:u^2*bp=s^2+(1+c)*s) :
 curve b c (u^2*X+b) (u^3*Y+s*u^2*X+b*c)=
 u^6*(Y^2+ap*X*Y+bp*Y+X^3+bp*X^2) := by
 rw [Tate_coordinate_char_two b c s u X Y hs]
 unfold transformed
 linear_combination -u^5*X*Y*ha-u^3*Y*hb-u^4*X^2*hbb
end KoblitzRound18Tate
#print axioms KoblitzRound18Tate.Tate_coordinate_residual
#print axioms KoblitzRound18Tate.Tate_coordinate_char_two
#print axioms KoblitzRound18Tate.actual_Tate_form
