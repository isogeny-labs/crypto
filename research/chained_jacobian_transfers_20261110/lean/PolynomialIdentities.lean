import Init.Grind.Ring.Basic

/-! Denominator-cleared identities and the concrete F101 models. These are
polynomial facts, not a substitute for the Picard/Richelot geometric bridge. -/
set_option autoImplicit false
namespace ChainedTransferIdentities
open Lean.Grind

variable {K : Type} [CommRing K]

theorem source_splitting_cleared (x : K) :
    (x^2+1)*(x^2+10*x-1)*(x^2-10*x-1) = x^6+1-101*(x^4+x^2) := by
  grind

theorem target_numerator_cleared (x : K) :
    (-20*x^2-20)*(10*x^2+4*x-10)*(10*x^2-4*x-10) =
      -2000*x^6+2320*x^4+2320*x^2-2000 := by
  grind

variable [IsCharP K 101]

theorem source_splitting_fp101 (x : K) :
    (x^2+1)*(x^2+10*x-1)*(x^2-10*x-1) = x^6+1 := by
  grind

theorem source_chart_fp101 (t : K) :
    (10*t+1)^6+t^6 = 1+60*t+86*t^2+2*t^3+15*t^4+60*t^5 := by
  grind

theorem elliptic_isomorphism_fp101 (u v : K) :
    (98*v)^2 - ((82*u+60)^3+9*(82*u+60)^2+66*(82*u+60)+39) =
      98^2*(v^2-(u^3+6*u^2-3*u)) := by
  grind

theorem elliptic_isomorphism_inverse_fp101 (u v : K) :
    85*((82*u+60)-60)=u ∧ 67*(98*v)=v := by
  grind

end ChainedTransferIdentities

#print axioms ChainedTransferIdentities.source_splitting_cleared
#print axioms ChainedTransferIdentities.target_numerator_cleared
#print axioms ChainedTransferIdentities.source_splitting_fp101
#print axioms ChainedTransferIdentities.source_chart_fp101
#print axioms ChainedTransferIdentities.elliptic_isomorphism_fp101
#print axioms ChainedTransferIdentities.elliptic_isomorphism_inverse_fp101
