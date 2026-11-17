import Mathlib.Data.Finset.Card
import Mathlib.Data.Fintype.Prod
import Lean.Elab.Tactic.Omega

namespace Infinity60

/-- Balanced ramification at the three smaller degree-two points forces
this sharper cap at the first point. Geometric interpretation is external. -/
theorem balanced_first_bound (e0 e1 e2 e3 : Int)
    (hs : e0 + e1 + e2 + e3 = 0)
    (h1 : -4 ≤ e1 ∧ e1 ≤ 4) (h2 : -7 ≤ e2 ∧ e2 ≤ 7)
    (h3 : -1 ≤ e3 ∧ e3 ≤ 1) : -12 ≤ e0 ∧ e0 ≤ 12 := by
  omega

/-- For the geometric convention d=2 sum positive(e), every such map
has degree at most24. This includes22 and supplies no coprimality filter. -/
theorem balanced_degree_bound (e0 e1 e2 e3 : Int)
    (hs : e0 + e1 + e2 + e3 = 0)
    (h1 : -4 ≤ e1 ∧ e1 ≤ 4) (h2 : -7 ≤ e2 ∧ e2 ≤ 7)
    (h3 : -1 ≤ e3 ∧ e3 ≤ 1) :
    2 * (max e0 0 + max e1 0 + max e2 0 + max e3 0) ≤ 24 := by
  omega

/-- Translate indices by (4,7,1) to obtain the minor integer intervals.
The first entry is uniquely determined by balance. -/
theorem nonzero_minor_patterns :
    ((Finset.univ : Finset (Fin 9 × Fin 15 × Fin 3)).erase
      ((4 : Fin 9), (7 : Fin 15), (1 : Fin 3))).card = 404 := by
  rw [Finset.card_erase_of_mem (Finset.mem_univ _)]
  simp only [Finset.card_univ, Fintype.card_prod, Fintype.card_fin]

end Infinity60
