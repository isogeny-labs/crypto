import Mathlib.RingTheory.Polynomial.Resultant.Basic
import Mathlib.Algebra.Polynomial.Bivariate
import Mathlib.Algebra.Polynomial.BigOperators
import Mathlib.Tactic

namespace ResultantInterchangeAudit
open Polynomial
noncomputable section
variable {K : Type*} [CommRing K]

def lift (p : Polynomial K) : Polynomial (Polynomial K) := p.map Polynomial.C

/-- Q has inner variable x and outer variable y. -/
def innerY (p : Polynomial K) (q : Polynomial (Polynomial K)) (dy : ℕ) : Polynomial K :=
  Polynomial.resultant (lift p) q p.natDegree dy

def innerX (q : Polynomial (Polynomial K)) (r : Polynomial K) (dx : ℕ) : Polynomial K :=
  Polynomial.resultant (Polynomial.Bivariate.swap q) (lift r) dx r.natDegree

def leftFixed (p : Polynomial K) (q : Polynomial (Polynomial K)) (r : Polynomial K)
    (dx dy : ℕ) : K := Polynomial.resultant (innerY p q dy) r (p.natDegree*dx) r.natDegree

def rightFixed (p : Polynomial K) (q : Polynomial (Polynomial K)) (r : Polynomial K)
    (dx dy : ℕ) : K := Polynomial.resultant p (innerX q r dx) p.natDegree (dy*r.natDegree)

def leftNatural (p : Polynomial K) (q : Polynomial (Polynomial K)) (r : Polynomial K)
    (dy : ℕ) : K := Polynomial.resultant (innerY p q dy) r

def rightNatural (p : Polynomial K) (q : Polynomial (Polynomial K)) (r : Polynomial K)
    (dx : ℕ) : K := Polynomial.resultant p (innerX q r dx)

end
end ResultantInterchangeAudit
