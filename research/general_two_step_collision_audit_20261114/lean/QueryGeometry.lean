import Cylinder
import LabelCoefficient
import FiniteGraphs

namespace GeneralTwoStepCollisionAudit
noncomputable section
open scoped BigOperators
variable {G I : Type*} [AddCommGroup G]

/-- The product order is ((last-left,last-right),(first-left,first-right)),
matching the existing weighted label sum. -/
abbrev Labels (I : Type*) := (I × I) × (I × I)

def labelsAt (w : Labels I) : Fin 4 → I := ![w.2.1, w.1.1, w.2.2, w.1.2]

def Admissible (w : Labels I) : Prop :=
  w.1.1 ≠ w.1.2 ∧ ¬(w.2.2 = w.1.1 ∧ w.1.2 = w.2.1)

def offsets (g : I → G) (w : Labels I) : Fin 4 → G :=
  ![g w.2.1 + g w.1.1, g w.1.1, g w.2.2 + g w.1.2, g w.1.2]

def query (g : I → G) (v : G) (w : Labels I) (a : Fin 4) : G := v - offsets g w a

theorem query_injective (g : I → G) (hg : TwoStepCollisionAudit.DegreeTwoGeneric g)
    (v : G) (w : Labels I) (hw : Admissible w) : Function.Injective (query g v w) := by
  have h01 : offsets g w 0 ≠ offsets g w 1 := by
    change g w.2.1 + g w.1.1 ≠ g w.1.1
    intro h
    exact hg.2.1 _ (add_right_cancel (h.trans (zero_add (g w.1.1)).symm))
  have h23 : offsets g w 2 ≠ offsets g w 3 := by
    change g w.2.2 + g w.1.2 ≠ g w.1.2
    intro h
    exact hg.2.1 _ (add_right_cancel (h.trans (zero_add (g w.1.2)).symm))
  have h03 : offsets g w 0 ≠ offsets g w 3 := hg.2.2.2.1 _ _ _
  have h12 : offsets g w 1 ≠ offsets g w 2 := (hg.2.2.2.1 _ _ _).symm
  have h13 : offsets g w 1 ≠ offsets g w 3 := fun h => hw.1 (hg.1 h)
  have h02 : offsets g w 0 ≠ offsets g w 2 := by
    intro h
    rcases hg.2.2.2.2 _ _ _ _ h with ⟨_, hjl⟩ | ⟨hil, hjk⟩
    · exact hw.1 hjl
    · exact hw.2 ⟨hjk.symm, hil.symm⟩
  have hoff : Function.Injective (offsets g w) := by
    intro a b he
    fin_cases a <;> fin_cases b <;>
      first | rfl | exact (h01 he).elim | exact (h01 he.symm).elim |
        exact (h02 he).elim | exact (h02 he.symm).elim |
        exact (h03 he).elim | exact (h03 he.symm).elim |
        exact (h12 he).elim | exact (h12 he.symm).elim |
        exact (h13 he).elim | exact (h13 he.symm).elim |
        exact (h23 he).elim | exact (h23 he.symm).elim
  intro a b he
  exact hoff (sub_right_injective he)

theorem admissible_of_query_injective (g : I → G) (v : G) (w : Labels I)
    (hi : Function.Injective (query g v w)) : Admissible w := by
  constructor
  · intro he
    have hq : query g v w 1 = query g v w 3 := by simp [query, offsets, he]
    exact (by decide : (1 : Fin 4) ≠ 3) (hi hq)
  · rintro ⟨hk, hl⟩
    have hq : query g v w 0 = query g v w 2 := by
      simp [query, offsets, hk, hl, add_comm]
    exact (by decide : (0 : Fin 4) ≠ 2) (hi hq)

theorem query_injective_iff (g : I → G) (hg : TwoStepCollisionAudit.DegreeTwoGeneric g)
    (v : G) (w : Labels I) : Function.Injective (query g v w) ↔ Admissible w :=
  ⟨admissible_of_query_injective g v w, query_injective g hg v w⟩

def step (g : I → G) (h : G → I) (x : G) : G := x + g (h x)

def pairQueries (g : I → G) (h : G → I) (xy : G × G) : Fin 4 → G :=
  ![xy.1, step g h xy.1, xy.2, step g h xy.2]

def FreshAt (g : I → G) (h : G → I) (v : G) (xy : G × G) : Prop :=
  Function.Injective (pairQueries g h xy) ∧
    step g h (step g h xy.1) = v ∧ step g h (step g h xy.2) = v

def pairLabels (g : I → G) (h : G → I) (xy : G × G) : Labels I :=
  ((h (step g h xy.1), h (step g h xy.2)), (h xy.1, h xy.2))

def Match (g : I → G) (h : G → I) (v : G) (w : Labels I) : Prop :=
  ∀ a : Fin 4, h (query g v w a) = labelsAt w a

#print axioms query_injective_iff
end
end GeneralTwoStepCollisionAudit
