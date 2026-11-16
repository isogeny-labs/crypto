module

public import Mathlib.Data.ZMod.Basic
public import Mathlib.Probability.ProbabilityMassFunction.Constructions
public import Mathlib.Tactic

@[expose] public section
namespace TwoStepCollisionAudit
open scoped BigOperators ENNReal

abbrev G := ZMod 7
abbrev Label := Fin 2
abbrev Hash := G → Label
abbrev Sample := Hash × (G × G)

def step (increments : Label → G) (h : Hash) (x : G) : G := x + increments (h x)

def goodIncrements : Label → G := ![1, 3]
def badIncrements : Label → G := ![1, 2]

/-- Literal source qualification: degree0,1,2 sums are distinct except for
commutativity within a degree2 sum. -/
def DegreeTwoGeneric {A B : Type*} [AddCommGroup A] (g : B → A) : Prop :=
  Function.Injective g ∧ (∀ i, g i ≠ 0) ∧ (∀ i j, g i + g j ≠ 0) ∧
    (∀ i j k, g i + g j ≠ g k) ∧
    (∀ i j k l, g i + g j = g k + g l →
      (i = k ∧ j = l) ∨ (i = l ∧ j = k))

instance degreeTwoGenericDecidable {A B : Type*} [AddCommGroup A] [Fintype B]
    [DecidableEq A] [DecidableEq B] (g : B → A) : Decidable (DegreeTwoGeneric g) := by
  unfold DegreeTwoGeneric
  infer_instance

def oneSamples (g : Label → G) : Finset Sample :=
  Finset.univ.filter (fun z => z.2.1 ≠ z.2.2 ∧ step g z.1 z.2.1 = step g z.1 z.2.2)

def twoSamples (g : Label → G) : Finset Sample :=
  Finset.univ.filter (fun z =>
    [z.2.1, z.2.2, step g z.1 z.2.1, step g z.1 z.2.2].Nodup ∧
      step g z.1 (step g z.1 z.2.1) = step g z.1 (step g z.1 z.2.2))

theorem sample_card : Fintype.card Sample = 6272 := by
  norm_num [Sample, Hash, G, Label, Fintype.card_fun]

theorem good_generic : DegreeTwoGeneric goodIncrements := by decide +kernel

theorem bad_not_generic : ¬DegreeTwoGeneric badIncrements := by decide +kernel

theorem bad_still_distinct_nonzero : Function.Injective badIncrements ∧
    (∀ i, badIncrements i ≠ 0) := by decide +kernel

theorem good_one_count : (oneSamples goodIncrements).card = 448 := by decide +kernel

theorem good_two_count : (twoSamples goodIncrements).card = 336 := by decide +kernel

theorem bad_two_count : (twoSamples badIncrements).card = 112 := by decide +kernel

#print axioms good_one_count
#print axioms good_two_count
#print axioms bad_two_count
end TwoStepCollisionAudit
