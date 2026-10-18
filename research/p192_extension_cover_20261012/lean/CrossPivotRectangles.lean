import Mathlib.Algebra.Group.Hom.Basic
import Mathlib.Tactic.Abel

/-!
Four line cuts through two pivots and two selected endpoints cancel their
common section class and produce an eight-column relation among the four
residual pairs.  The P-192 finite-field cut and fixed-origin divisor
instances are checked by Sage and independent PARI/GP replay.
-/

namespace P192ExtensionCover

variable {G : Type*} [AddCommGroup G]

theorem cross_pivot_rectangle
    (S a b u v xau yau xav yav xbu ybu xbv ybv : G)
    (hau : a + u + xau + yau = S)
    (hav : a + v + xav + yav = S)
    (hbu : b + u + xbu + ybu = S)
    (hbv : b + v + xbv + ybv = S) :
    (xau + yau) + (xbv + ybv) =
      (xav + yav) + (xbu + ybu) := by
  have h₁ : xau + yau = S - a - u := by
    calc
      xau + yau = (a + u + xau + yau) - a - u := by abel
      _ = S - a - u := by rw [hau]
  have h₂ : xav + yav = S - a - v := by
    calc
      xav + yav = (a + v + xav + yav) - a - v := by abel
      _ = S - a - v := by rw [hav]
  have h₃ : xbu + ybu = S - b - u := by
    calc
      xbu + ybu = (b + u + xbu + ybu) - b - u := by abel
      _ = S - b - u := by rw [hbu]
  have h₄ : xbv + ybv = S - b - v := by
    calc
      xbv + ybv = (b + v + xbv + ybv) - b - v := by abel
      _ = S - b - v := by rw [hbv]
  rw [h₁, h₂, h₃, h₄]
  abel

theorem rectangle_target_formula
    (S a b u v xau yau xbv ybv : G)
    (hau : a + u + xau + yau = S)
    (hbv : b + v + xbv + ybv = S) :
    (xau + yau) + (xbv + ybv) =
      S + S - a - b - u - v := by
  have h₁ : xau + yau = S - a - u := by
    calc
      xau + yau = (a + u + xau + yau) - a - u := by abel
      _ = S - a - u := by rw [hau]
  have h₂ : xbv + ybv = S - b - v := by
    calc
      xbv + ybv = (b + v + xbv + ybv) - b - v := by abel
      _ = S - b - v := by rw [hbv]
  rw [h₁, h₂]
  abel

theorem rectangle_known_scalar_log {K : Type*} [AddCommGroup K]
    (log : G →+ K) (generator x₁ x₂ x₃ x₄ : G) (n : ℕ)
    (hit : (x₁ + x₂) + (x₃ + x₄) =
      n • (-(generator + generator))) :
    (log x₁ + log x₂) + (log x₃ + log x₄) =
      n • (-(log generator + log generator)) := by
  calc
    (log x₁ + log x₂) + (log x₃ + log x₄) =
        log ((x₁ + x₂) + (x₃ + x₄)) := by simp
    _ = log (n • (-(generator + generator))) := congrArg log hit
    _ = n • (-(log generator + log generator)) := by simp

#print axioms cross_pivot_rectangle
#print axioms rectangle_target_formula
#print axioms rectangle_known_scalar_log

end P192ExtensionCover
