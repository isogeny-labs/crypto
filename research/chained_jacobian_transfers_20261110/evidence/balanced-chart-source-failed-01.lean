import Init.Grind.Module.Basic
import Init.Grind.Ring.Basic

/-! The new degree1 finite support and actual balanced infinity accounting.
Finite fields,projective chart extension,principal divisors and Picard
functoriality remain written bridges. No torsion-free point-group premise. -/
set_option autoImplicit false
namespace BalancedChart
open Lean.Grind

theorem linear_support_transform {K : Type} [CommRing K]
    (x a l w u0 inverse : K) (hu : x+u0=0)
    (hchart : (x-a)*w=l) (hi : (a+u0)*inverse=1) :
    w+l*inverse=0 := by
  have h : (a+u0)*w+l=0 := by grind
  have scaled := congrArg (fun z : K => inverse*z) h
  grind

theorem balanced_infinity_images {H : Type} [IntModule H]
    (finite qplus qminus : H) (m n : Int) (hminus : qminus = -qplus) :
    finite+(n-1) • qplus+(1-m-n) • qminus =
      finite+(2*n+m-2) • qplus := by
  rw [hminus]
  grind

end BalancedChart
#print axioms BalancedChart.linear_support_transform
#print axioms BalancedChart.balanced_infinity_images
