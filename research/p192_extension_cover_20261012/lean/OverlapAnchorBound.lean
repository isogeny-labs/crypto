import FourAnchorRecurrence
import Mathlib.Tactic.FinCases

/-!
The finite P-192 screen finds four possible new exact points shared by
different anchor groups. Two of them require distinct labels for one and the
same residual endpoint. A selection with distinct endpoint coordinates can
therefore realize at most three of the four points.

The coordinate screen verifies the four-point universe and the two support
implications for its exact candidate pool. Those instance checks are not
encoded as Lean field arithmetic here.
-/

namespace P192ExtensionCover

/-- Any subset of four indexed possibilities omitting index zero or index one
has at most three elements. -/
theorem overlap_card_le_three (S : Finset (Fin 4))
    (h : (0 : Fin 4) ∉ S ∨ (1 : Fin 4) ∉ S) : S.card ≤ 3 := by
  rcases h with hzero | hone
  · have hsubset : S ⊆ ({1, 2, 3} : Finset (Fin 4)) := by
      intro i hi
      fin_cases i <;> simp_all
    exact (Finset.card_le_card hsubset).trans (by decide)
  · have hsubset : S ⊆ ({0, 2, 3} : Finset (Fin 4)) := by
      intro i hi
      fin_cases i <;> simp_all
    exact (Finset.card_le_card hsubset).trans (by decide)

/-- An endpoint-unique selection cannot contain two different candidate
labels that carry the same endpoint coordinate. -/
theorem distinct_endpoint_excludes_pair
    {ι P : Type*} [DecidableEq ι]
    (selected : Finset ι) (endpoint : ι → P) (x y : ι)
    (hxy : x ≠ y) (heq : endpoint x = endpoint y)
    (hinj : ∀ a ∈ selected, ∀ b ∈ selected,
      endpoint a = endpoint b → a = b) :
    x ∉ selected ∨ y ∉ selected := by
  by_cases hx : x ∈ selected
  · right
    intro hy
    exact hxy (hinj x hx y hy heq)
  · exact Or.inl hx

/-- The bound transfers to an endpoint-unique selection when occurrence of
the two conflicting shared points requires the corresponding candidate. -/
theorem overlap_card_le_three_of_distinct_endpoints
    {ι P : Type*} [DecidableEq ι]
    (selected : Finset ι) (endpoint : ι → P) (x y : ι)
    (overlap : Finset (Fin 4))
    (hxy : x ≠ y) (heq : endpoint x = endpoint y)
    (hinj : ∀ a ∈ selected, ∀ b ∈ selected,
      endpoint a = endpoint b → a = b)
    (hzero : (0 : Fin 4) ∈ overlap → x ∈ selected)
    (hone : (1 : Fin 4) ∈ overlap → y ∈ selected) :
    overlap.card ≤ 3 := by
  have hmissing := distinct_endpoint_excludes_pair
    selected endpoint x y hxy heq hinj
  apply overlap_card_le_three overlap
  rcases hmissing with hx | hy
  · left
    intro h
    exact hx (hzero h)
  · right
    intro h
    exact hy (hone h)

/-- The abstract bound is attained by three indexed points. -/
theorem three_overlap_indices_exist :
    ({1, 2, 3} : Finset (Fin 4)).card = 3 := by
  decide

#print axioms overlap_card_le_three_of_distinct_endpoints
#print axioms three_overlap_indices_exist

end P192ExtensionCover
