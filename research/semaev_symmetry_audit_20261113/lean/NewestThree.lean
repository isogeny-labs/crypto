import SourceOrder
import RecursiveSemaevSymmetry

namespace SemaevSymmetryAudit
open Polynomial S4DegreeAudit ResultantDegreeStepAudit RecursiveSemaevDegreeAudit
open SemaevReassociationAudit RecursiveSemaevSymmetryAudit
noncomputable section
variable {R : Type*} [CommRing R]

abbrev Triple (R : Type*) [CommRing R] := Polynomial (Polynomial (Polynomial R))

/-- Interchange the inner and middle new variables, retaining the outer one. -/
def innerSwap : Triple R →+* Triple R := mapRingHom Polynomial.Bivariate.swap.toRingHom

def outerSwap : Triple R →+* Triple R := Polynomial.Bivariate.swap.toRingHom

theorem innerSwap_scalar (a : R) : innerSwap (tripleLift a) = tripleLift a := by
  simp [innerSwap, tripleLift, scalarLift, Polynomial.Bivariate.swap_C_C]

theorem innerSwap_Z : innerSwap (formalZ : Triple R) = formalX := by
  simp [innerSwap, formalZ, formalX, Polynomial.Bivariate.swap_X]

theorem innerSwap_X : innerSwap (formalX : Triple R) = formalZ := by
  simp [innerSwap, formalZ, formalX, Polynomial.Bivariate.swap_Y]

theorem innerSwap_Y : innerSwap (formalY : Triple R) = formalY := by
  simp [innerSwap, formalY]

theorem outerSwap_scalar (a : R) : outerSwap (tripleLift a) = tripleLift a := by
  simp [outerSwap, tripleLift, scalarLift, Polynomial.Bivariate.swap_C_C]

theorem outerSwap_Z : outerSwap (formalZ : Triple R) = formalZ := by
  simp [outerSwap, formalZ, Polynomial.Bivariate.swap_C_C]

theorem outerSwap_X : outerSwap (formalX : Triple R) = formalY := by
  simp [outerSwap, formalX, formalY, Polynomial.Bivariate.swap_X]

theorem outerSwap_Y : outerSwap (formalY : Triple R) = formalX := by
  simp [outerSwap, formalX, formalY, Polynomial.Bivariate.swap_Y]

theorem innerSwap_lift (p : Polynomial R) :
    (p.map tripleLift).map (innerSwap : Triple R →+* Triple R) = p.map tripleLift := by
  rw [Polynomial.map_map]
  have h : innerSwap.comp (tripleLift : R →+* Triple R) = tripleLift := by
    apply RingHom.ext
    intro a
    exact innerSwap_scalar a
  rw [h]

theorem lastPolynomial_first_swap (b2 b4 b6 b8 x y z : R) :
    lastPolynomial b2 b4 b6 b8 x y z = lastPolynomial b2 b4 b6 b8 y x z := by
  rw [lastPolynomial_eq_s4Ring, lastPolynomial_eq_s4Ring]
  exact s4Ring_first_swap ..

theorem sourceBalanced_innerSwap (D : ℕ) (p : Polynomial R) (W : WeierstrassCurve R) :
    innerSwap (sourceBalanced D p W) = sourceBalanced D p W := by
  unfold sourceBalanced rightAssoc
  rw [← resultant_map_map (R := Triple R) (S := Triple R)
    (p.map tripleLift)
    (lastPolynomial (tripleLift W.b₂) (tripleLift W.b₄) (tripleLift W.b₆)
      (tripleLift W.b₈) formalZ formalX formalY) D 4 (innerSwap (R := R)),
    innerSwap_lift]
  rw [SemaevReassociationAudit.lastPolynomial_map (R := Triple R) (S := Triple R)
    (innerSwap (R := R)) (tripleLift W.b₂) (tripleLift W.b₄)
    (tripleLift W.b₆) (tripleLift W.b₈) formalZ formalX formalY]
  rw [innerSwap_scalar, innerSwap_scalar, innerSwap_scalar, innerSwap_scalar,
    innerSwap_Z, innerSwap_X, innerSwap_Y]
  rw [lastPolynomial_first_swap]

theorem doubleStep_innerSwap [IsDomain R] (D : ℕ) (p : Polynomial R)
    (hp : p.natDegree ≤ D) (W : WeierstrassCurve R) :
    innerSwap (doubleStep D p W.b₂ W.b₄ W.b₆ W.b₈) =
      doubleStep D p W.b₂ W.b₄ W.b₆ W.b₈ := by
  rw [source_order_polynomial_reassociation D p hp W, sourceBalanced_innerSwap]

theorem doubleStep_outerSwap (D : ℕ) (p : Polynomial R) (b2 b4 b6 b8 : R) :
    outerSwap (doubleStep D p b2 b4 b6 b8) = doubleStep D p b2 b4 b6 b8 := by
  unfold doubleStep
  exact resultantStep_exchange ..

variable {K : Type*} [Field K]

theorem canonical_innerSwap (W : WeierstrassCurve.Affine K) (n : ℕ) :
    innerSwap (R := InputRing K n) (recursivePolynomial W (n+2)) =
      recursivePolynomial W (n+2) := by
  rw [canonical_doubleStep]
  have h := doubleStep_innerSwap (2^(n+1)) (recursivePolynomial W n)
    (recursivePolynomial_natDegree W n).le (W.map (coefficientEmbedding n))
  rw [WeierstrassCurve.map_b₂, WeierstrassCurve.map_b₄,
    WeierstrassCurve.map_b₆, WeierstrassCurve.map_b₈] at h
  exact h

theorem canonical_outerSwap (W : WeierstrassCurve.Affine K) (n : ℕ) :
    outerSwap (R := InputRing K n) (recursivePolynomial W (n+2)) =
      recursivePolynomial W (n+2) := by
  rw [canonical_doubleStep]
  exact doubleStep_outerSwap ..

#print axioms canonical_innerSwap
#print axioms canonical_outerSwap

end
end SemaevSymmetryAudit
