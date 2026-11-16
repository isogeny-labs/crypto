import KoblitzRound16
namespace KoblitzRound16Carrier

theorem line_intersection {R:Type*} [CommRing R] [CharP R 2]
 (x A b l n:R) :
 (l*x+n)^2+A*x*(l*x+n)+b*(l*x+n)+x^3+b*x^2 =
 x^3+(b+l^2+A*l)*x^2+(A*n+b*l)*x+(n^2+b*n) := by
 have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
 linear_combination l*n*x*h2

theorem diamond_cubic {R:Type*} [CommRing R] [CharP R 2]
 (A b l n:R) :
 (b+(l+A)^2+A*(l+A), A*(n+b)+b*(l+A), (n+b)^2+b*(n+b)) =
 (b+l^2+A*l,A*n+b*l,n^2+b*n) := by
 have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
 apply Prod.ext
 · linear_combination (A^2+A*l)*h2
 · apply Prod.ext
   · linear_combination A*b*h2
   · linear_combination (b^2+b*n)*h2

theorem diamond_involution {R:Type*} [CommRing R] [CharP R 2]
 (A b l n:R) : (l+A+A,n+b+b)=(l,n) := by
 have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
 apply Prod.ext
 · linear_combination A*h2
 · linear_combination b*h2

theorem twist_curve {R:Type*} [CommRing R] [CharP R 2]
 (x y A b w:R) (hw:w^2+w=1) :
 (y+w*(A*x+b))^2+A*x*(y+w*(A*x+b))+b*(y+w*(A*x+b))+x^3+b*x^2 =
 y^2+A*x*y+b*y+x^3+(b+A^2)*x^2+b^2 := by
 have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
 linear_combination (A*x+b)^2*hw+(y*w*(A*x+b)+A*b*x)*h2

theorem twist_cubic {R:Type*} [CommRing R] [CharP R 2]
 (A b l n w:R) (hw:w^2+w=1) :
 (b+(l+w*A)^2+A*(l+w*A), A*(n+w*b)+b*(l+w*A),
 (n+w*b)^2+b*(n+w*b)) =
 (b+l^2+A*l+A^2,A*n+b*l,n^2+b*n+b^2) := by
 have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
 apply Prod.ext
 · linear_combination A^2*hw+l*w*A*h2
 · apply Prod.ext
   · linear_combination w*A*b*h2
   · linear_combination b^2*hw+n*w*b*h2

theorem twist_doubling_numerator {R:Type*} [CommRing R] [CharP R 2]
 (M A D b w:R) (hw:w^2+w=1) :
 (M+w*A*D)^2+A*(M+w*A*D)*D+b*D^2 =
 M^2+A*M*D+b*D^2+A^2*D^2 := by
 have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
 linear_combination A^2*D^2*hw+w*A*M*D*h2

theorem twist_line_doubling_term {R:Type*} [CommRing R] [CharP R 2]
 (M A D l w:R) :
 (M+w*A*D)+(A+(l+w*A))*D=M+(A+l)*D := by
 have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
 linear_combination w*A*D*h2

theorem three_cycle_kills_seven {G:Type*} [AddCommGroup G] (p:G)
 (hp: (8:ℤ) • p=p) : (7:ℤ) • p=0 := by
 have hz : ((8:ℤ)-1) • p=0 := by rw [sub_smul, one_smul, hp, sub_self]
 norm_num at hz
 exact hz

theorem line_intersection_residual {R:Type*} [CommRing R] (x A b l n:R) :
 (l*x+n)^2+A*x*(l*x+n)+b*(l*x+n)+x^3+b*x^2-
 (x^3+(b+l^2+A*l)*x^2+(A*n+b*l)*x+(n^2+b*n))=2*l*n*x := by ring

theorem twist_curve_residual {R:Type*} [CommRing R] (x y A b w:R) :
 (y+w*(A*x+b))^2+A*x*(y+w*(A*x+b))+b*(y+w*(A*x+b))+x^3+b*x^2-
 (y^2+A*x*y+b*y+x^3+(b+A^2)*x^2+b^2)=
 (w^2+w-1)*(A*x+b)^2+2*(y*w*(A*x+b)+A*b*x) := by ring

theorem twist_doubling_residual {R:Type*} [CommRing R] (M A D b w:R) :
 (M+w*A*D)^2+A*(M+w*A*D)*D+b*D^2-
 (M^2+A*M*D+b*D^2+A^2*D^2)=
 A^2*D^2*(w^2+w-1)+2*w*A*M*D := by ring

end KoblitzRound16Carrier
#print axioms KoblitzRound16Carrier.line_intersection
#print axioms KoblitzRound16Carrier.diamond_cubic
#print axioms KoblitzRound16Carrier.diamond_involution
#print axioms KoblitzRound16Carrier.twist_curve
#print axioms KoblitzRound16Carrier.twist_cubic
#print axioms KoblitzRound16Carrier.twist_doubling_numerator
#print axioms KoblitzRound16Carrier.twist_line_doubling_term
#print axioms KoblitzRound16Carrier.three_cycle_kills_seven

#print axioms KoblitzRound16Carrier.line_intersection_residual
#print axioms KoblitzRound16Carrier.twist_curve_residual
#print axioms KoblitzRound16Carrier.twist_doubling_residual
