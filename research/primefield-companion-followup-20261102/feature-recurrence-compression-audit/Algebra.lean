import Std
namespace SectionAudit
theorem triangular_compose {R : Type} [Lean.Grind.CommRing R]
    (a1 b1 d1 a2 b2 d2 t s : R) :
    (b2*a1+d2*b1)*t+(d2*d1)*s = b2*(a1*t)+d2*(b1*t+d1*s) := by grind
theorem normalize {R : Type} [Lean.Grind.CommRing R]
    (d inv value : R) (h : inv*d=1) : inv*(d*value)=value := by grind
theorem wrap_once {R : Type} [Lean.Grind.CommRing R]
    (c xm v : R) (relation : xm=v) : c*xm=c*v := by grind
#print axioms triangular_compose
#print axioms normalize
#print axioms wrap_once
end SectionAudit
