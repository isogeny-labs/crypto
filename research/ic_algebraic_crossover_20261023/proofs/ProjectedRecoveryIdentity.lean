import Std.Tactic

/- The residual is an unconditional polynomial identity. Its vanishing and
scalar recovery additionally require an inverse; group/cardinality/sampling
claims are outside this statement. -/
namespace ICProjectedRecovery
open Lean.Grind

theorem scalar_recovery_residual {F : Type} [CommRing F]
    (h hInv d k : F) :
    hInv * (h * (d + k)) - k - d = (hInv * h - 1) * (d + k) := by
  grind

#print axioms scalar_recovery_residual
end ICProjectedRecovery
