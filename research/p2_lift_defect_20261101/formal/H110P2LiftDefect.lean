import Mathlib.Algebra.Group.Hom.Basic
import Mathlib.Data.Finset.Basic
import Mathlib.Tactic

/-!
# H110: first-layer lift-defect interfaces

This file proves the abstract identities used by the H110 experiment.  It does
not assume or prove the empirical low-degree claim.  `section` is deliberately
only a function; no homomorphism structure is attached to it.
-/

namespace H110P2LiftDefect

section SectionDefect

variable {Q A K : Type*}
variable [AddCommGroup A] [AddCommGroup K]

/-- The order multiple of a set-theoretic section lands in the modeled kernel. -/
def defect (order : ℕ) (liftSection : Q → A) (q : Q) : A := order • liftSection q

/-- Changing a section by a kernel displacement changes its defect by the
order multiple of that displacement.  No additivity of `section` is used. -/
theorem defect_transport (order : ℕ) (liftSection displacement : Q → A) (q : Q) :
    defect order (fun x => liftSection x + displacement x) q =
      defect order liftSection q + order • displacement q := by
  simp [defect, nsmul_add]

/-- An additive first-layer coordinate transports the section displacement
linearly after multiplication by the subgroup order. -/
theorem coordinate_defect_transport
    (coordinate : A →+ K) (order : ℕ) (liftSection displacement : Q → A) (q : Q) :
    coordinate (defect order (fun x => liftSection x + displacement x) q) =
      coordinate (defect order liftSection q) + order • coordinate (displacement q) := by
  simp [defect, nsmul_add]

/-- A correction whose coordinate cancels the original defect produces zero
first-layer coordinate. -/
theorem corrected_coordinate_zero
    (coordinate : A →+ K) (order : ℕ) (liftSection displacement : Q → A) (q : Q)
    (hcancel : coordinate (defect order liftSection q) +
      order • coordinate (displacement q) = 0) :
    coordinate (defect order (fun x => liftSection x + displacement x) q) = 0 := by
  rw [coordinate_defect_transport]
  exact hcancel

end SectionDefect

section Recovery

variable {Scalar Point Value : Type*}
variable [DecidableEq Point]

/-- Roots of the public certificate value, filtered by the original EC
verification equation. -/
def verifiedCandidates (domain : Finset Scalar) (certificate : Scalar → Value)
    [DecidableEq Value] (targetValue : Value) (mul : Scalar → Point)
    (target : Point) : Finset Scalar :=
  domain.filter fun candidate =>
    certificate candidate = targetValue ∧ mul candidate = target

/-- An exact certificate and the original EC equation ensure that the true
scalar survives candidate generation and verification. -/
theorem true_scalar_mem_verifiedCandidates
    (domain : Finset Scalar) (certificate : Scalar → Value) [DecidableEq Value]
    (targetValue : Value) (mul : Scalar → Point) (target : Point) (k : Scalar)
    (hk : k ∈ domain) (hcert : certificate k = targetValue)
    (hec : mul k = target) :
    k ∈ verifiedCandidates domain certificate targetValue mul target := by
  simp [verifiedCandidates, hk, hcert, hec]

/-- Uniqueness after EC verification turns membership into exact recovery. -/
theorem verifiedCandidates_eq_singleton
    [DecidableEq Scalar] (domain : Finset Scalar) (certificate : Scalar → Value) [DecidableEq Value]
    (targetValue : Value) (mul : Scalar → Point) (target : Point) (k : Scalar)
    (hk : k ∈ domain) (hcert : certificate k = targetValue)
    (hec : mul k = target)
    (hunique : ∀ candidate ∈ domain, mul candidate = target → candidate = k) :
    verifiedCandidates domain certificate targetValue mul target = {k} := by
  ext candidate
  constructor
  · intro hc
    have hparts := (Finset.mem_filter.mp hc).2
    have hdomain := (Finset.mem_filter.mp hc).1
    have : candidate = k := hunique candidate hdomain hparts.2
    simpa [this]
  · intro hc
    have : candidate = k := by simpa using hc
    subst candidate
    exact true_scalar_mem_verifiedCandidates domain certificate targetValue mul target k hk hcert hec

end Recovery

section Cost

/-- The complete charged path used by H110. -/
def totalCost (lift multiply interpolate factor verify checks : ℕ) : ℕ :=
  lift + multiply + interpolate + factor + verify + checks

theorem totalCost_bound
    (lift multiply interpolate factor verify checks budget : ℕ)
    (h : lift + multiply + interpolate + factor + verify + checks ≤ budget) :
    totalCost lift multiply interpolate factor verify checks ≤ budget := by
  exact h

end Cost

#print axioms defect_transport
#print axioms coordinate_defect_transport
#print axioms corrected_coordinate_zero
#print axioms true_scalar_mem_verifiedCandidates
#print axioms verifiedCandidates_eq_singleton
#print axioms totalCost_bound

end H110P2LiftDefect
