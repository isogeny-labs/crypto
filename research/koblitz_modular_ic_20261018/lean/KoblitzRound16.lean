import KoblitzRound15
namespace KoblitzRound16

def separator {R:Type*} [CommRing R] (h U V c0 c1 c2 c3:R) : R :=
 U*(c3*U+c0*h)^3+h*(c2*U-V)^3+h^2*U^2*c1^3-
 3*h*U*c1*(c3*U+c0*h)*(c2*U-V)

def adjugate {R:Type*} [CommRing R] (h x U V c0 c1 c2 c3:R) : R :=
 (h*(c2*U-V))^2-c1*(c3*U+c0*h)*h^2*U+
 (c1^2*h^2*U-(c3*U+c0*h)*h*(c2*U-V))*x+
 ((c3*U+c0*h)^2-c1*h*(c2*U-V))*x^2

def cofactor {R:Type*} [CommRing R] (h x U V c0 c1 c2 c3:R) : R :=
 (c3*x+c2*h)*adjugate h x U V c0 c1 c2 c3+
 h^2*(c1*((c3*U+c0*h)^2-c1*h*(c2*U-V))*x+
 (c3*U+c0*h)*((c3*U+c0*h)^2-c1*h*(c2*U-V))+
 c1*(c1^2*h^2*U-(c3*U+c0*h)*h*(c2*U-V)))

theorem separator_residual {R:Type*} [CommRing R] (h x U V c0 c1 c2 c3:R) :
 h^4*separator h U V c0 c1 c2 c3 =
 (c3*x^4+c2*h*x^3+c1*h^2*x^2+c0*h^3*x-h^3*V)*adjugate h x U V c0 c1 c2 c3-
 (x^3-h^2*U)*cofactor h x U V c0 c1 c2 c3 := by
 unfold separator cofactor adjugate
 ring

theorem cube_source_depth {R:Type*} [CommRing R]
 (h x one z16 z17 z18 U:R)
 (hc:one-h=0) (h16:h*z16-one*one=0)
 (h17:h*z17-x*z16=0) (h18:h*z18-z17*z17=0) (h19:h*U-x*z18=0) :
 h^2*(x^3-h^2*U)=0 := by
 have a : h*(z16-h)=0 := by linear_combination h16+(one+h)*hc
 have b : h^2*(z17-x)=0 := by linear_combination h*h17+x*a
 have c : h^2*(h*z18-x^2)=0 := by linear_combination h^2*h18+(z17+x)*b
 linear_combination -h^3*h19-x*c

theorem Horner_source_depth {R:Type*} [CommRing R]
 (h x z68 z70 z71 z73 z74 z76 z77 z79 V c0 c1 c2 c3:R)
 (h68:h*z68=0) (h70:z70-z68-c3*h=0)
 (h71:h*z71-x*z70=0) (h73:z73-z71-c2*h=0)
 (h74:h*z74-x*z73=0) (h76:z76-z74-c1*h=0)
 (h77:h*z77-x*z76=0) (h79:z79-z77-c0*h=0)
 (h80:h*V-x*z79=0) :
 h^2*(c3*x^4+c2*h*x^3+c1*h^2*x^2+c0*h^3*x-h^3*V)=0 := by
 have a : h*(z70-c3*h)=0 := by linear_combination h*h70+h68
 have b : h^2*(z71-c3*x)=0 := by linear_combination h*h71+x*a
 have c : h^2*(z73-c3*x-c2*h)=0 := by linear_combination h^2*h73+b
 have d : h^2*(h*z74-c3*x^2-c2*h*x)=0 := by linear_combination h^2*h74+x*c
 have e : h^2*(h*z76-c3*x^2-c2*h*x-c1*h^2)=0 := by linear_combination h^3*h76+d
 have f : h^2*(h^2*z77-c3*x^3-c2*h*x^2-c1*h^2*x)=0 := by linear_combination h^3*h77+x*e
 have g : h^2*(h^2*z79-c3*x^3-c2*h*x^2-c1*h^2*x-c0*h^3)=0 := by linear_combination h^4*h79+f
 linear_combination -h^4*h80-x*g

theorem separator_depth_six {R:Type*} [CommRing R]
 (h x U V c0 c1 c2 c3:R)
 (ha:h^2*(x^3-h^2*U)=0)
 (hb:h^2*(c3*x^4+c2*h*x^3+c1*h^2*x^2+c0*h^3*x-h^3*V)=0) :
 h^6*separator h U V c0 c1 c2 c3=0 := by
 have hh:=separator_residual h x U V c0 c1 c2 c3
 linear_combination h^2*hh+(adjugate h x U V c0 c1 c2 c3)*hb-(cofactor h x U V c0 c1 c2 c3)*ha

theorem separator_ideal_membership {R:Type*} [CommRing R] (I:Ideal R)
 (h x U V c0 c1 c2 c3:R)
 (ha:h^2*(x^3-h^2*U) ∈ I)
 (hb:h^2*(c3*x^4+c2*h*x^3+c1*h^2*x^2+c0*h^3*x-h^3*V) ∈ I) :
 h^6*separator h U V c0 c1 c2 c3 ∈ I := by
 have a:=I.mul_mem_right (cofactor h x U V c0 c1 c2 c3) ha
 have b:=I.mul_mem_right (adjugate h x U V c0 c1 c2 c3) hb
 have q:=I.sub_mem b a
 have hh:=separator_residual h x U V c0 c1 c2 c3
 convert q using 1
 linear_combination h^2*hh

theorem separator_boundary {R:Type*} [CommRing R] (V c0 c1 c2 c3:R) :
 separator 0 1 V c0 c1 c2 c3=c3^3 := by unfold separator;ring

theorem separator_boundary_nonzero {R:Type*} [CommRing R] [NoZeroDivisors R]
 (V c0 c1 c2 c3:R) (hc:c3≠0) : separator 0 1 V c0 c1 c2 c3≠0 := by
 rw [separator_boundary]
 exact pow_ne_zero 3 hc
end KoblitzRound16
#print axioms KoblitzRound16.separator_residual
#print axioms KoblitzRound16.cube_source_depth
#print axioms KoblitzRound16.Horner_source_depth
#print axioms KoblitzRound16.separator_depth_six
#print axioms KoblitzRound16.separator_ideal_membership
#print axioms KoblitzRound16.separator_boundary
#print axioms KoblitzRound16.separator_boundary_nonzero
