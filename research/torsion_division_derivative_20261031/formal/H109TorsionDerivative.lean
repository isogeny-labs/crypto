import Mathlib.Algebra.Group.Hom.Basic
import Mathlib.Data.Nat.Bits
import Mathlib.Data.Nat.Digits
import Mathlib.Tactic.Ring

/-!
# H109 torsion-local derivative certificate interfaces

This module checks the algebraic interfaces used by H109: the chosen
derivation is tangent to a short Weierstrass equation, a multiplicative
character transports the proposed local identity, an exact half-phase yields
the parity bit, binary digits reconstruct the scalar, and component cost
bounds compose.  The division-polynomial local identity itself remains an
explicit hypothesis here; this file does not prove a generic runtime bound.
-/

namespace H109TorsionDerivative

def targetPhase (halfPhase scalar : ℕ) : ℕ :=
  if Nat.bodd scalar then halfPhase else 0

def phaseBit (halfPhase phase : ℕ) : Bool :=
  phase == halfPhase

theorem invariantDerivation_tangent
    {R : Type*} [CommRing R] (x y a : R) :
    2 * y * (3 * x ^ 2 + a) - 3 * x ^ 2 * (2 * y) - a * (2 * y) = 0 := by
  ring

theorem character_transports_local_identity
    {K G : Type*} [CommGroup K] [CommGroup G]
    (χ : K →* G) (delta negRSquared psiMinus psiPlus : K)
    (localIdentity : delta * delta = negRSquared * psiMinus * psiPlus) :
    χ delta * χ delta = χ negRSquared * χ psiMinus * χ psiPlus := by
  simpa only [map_mul] using congrArg χ localIdentity

theorem exact_half_phase_supplies_parity
    (halfPhase scalar : ℕ)
    (halfPhaseNonzero : halfPhase ≠ 0) :
    phaseBit halfPhase (targetPhase halfPhase scalar) = Nat.bodd scalar := by
  by_cases parity : Nat.bodd scalar
  · simp [phaseBit, targetPhase, parity]
  · simp [phaseBit, targetPhase, parity, Ne.symm halfPhaseNonzero]

theorem correct_binary_reconstruction (scalar : ℕ) :
    Nat.ofDigits 2 (Nat.digits 2 scalar) = scalar := by
  exact Nat.ofDigits_digits 2 scalar

def totalCost (bitSteps oracleCost verificationCost : ℕ) : ℕ :=
  bitSteps * oracleCost + verificationCost

theorem totalCost_bound
    (bitSteps oracleCost verificationCost : ℕ)
    (bitBound oracleBound verificationBound : ℕ)
    (bitStepsBound : bitSteps ≤ bitBound)
    (oracleCostBound : oracleCost ≤ oracleBound)
    (verificationCostBound : verificationCost ≤ verificationBound) :
    totalCost bitSteps oracleCost verificationCost ≤
      bitBound * oracleBound + verificationBound := by
  exact Nat.add_le_add
    (Nat.mul_le_mul bitStepsBound oracleCostBound)
    verificationCostBound

end H109TorsionDerivative

#print axioms H109TorsionDerivative.invariantDerivation_tangent
#print axioms H109TorsionDerivative.character_transports_local_identity
#print axioms H109TorsionDerivative.exact_half_phase_supplies_parity
#print axioms H109TorsionDerivative.correct_binary_reconstruction
#print axioms H109TorsionDerivative.totalCost_bound
