import Init.Grind.Ring.Basic

/-! Actual finite-algebra identities for quartic elliptic pullback.
The quotient ring may have nilpotents. Supplied inverse hypotheses are
explicit;curve/divisor/Picard and finite-field construction remain written. -/
set_option autoImplicit false
namespace QuarticFibre
open Lean.Grind

theorem cleared_fibre_factorization {K : Type} [CommRing K]
    (f m n q a b x y : K)
    (hcover : f*m^2=n^3+a*n*q^4+b*q^6)
    (hpoint : y^2=x^3+a*x+b) :
    f*m^2-y^2*q^6 =
      (n-x*q^2)*(n^2+n*x*q^2+x^2*q^4+a*q^4) := by
  rw [hcover,hpoint]
  grind

theorem ordinate_denominator_inverse {K : Type} [CommRing K]
    (f m y q yi qi : K)
    (hfibre : f*m^2=y^2*q^6)
    (hy : y*yi=1) (hq : q*qi=1) :
    m*(f*m*yi^2*qi^6)=1 := by
  have hy2 : y^2*yi^2=1 := by
    calc
      y^2*yi^2 = (y*yi)^2 := by grind
      _ = 1 := by rw [hy]; grind
  have hq6 : q^6*qi^6=1 := by
    calc
      q^6*qi^6 = (q*qi)^6 := by grind
      _ = 1 := by rw [hq]; grind
  calc
    m*(f*m*yi^2*qi^6) = (f*m^2)*(yi^2*qi^6) := by grind
    _ = (y^2*q^6)*(yi^2*qi^6) := by rw [hfibre]
    _ = (y^2*yi^2)*(q^6*qi^6) := by grind
    _ = 1 := by rw [hy2,hq6]; grind

end QuarticFibre
#print axioms QuarticFibre.cleared_fibre_factorization
#print axioms QuarticFibre.ordinate_denominator_inverse
