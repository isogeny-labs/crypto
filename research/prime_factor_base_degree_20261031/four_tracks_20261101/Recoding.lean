import PencilFixture

noncomputable section
namespace Recoding
variable {K : Type*} [Field K]

def xor (u v : K) : K := u+v-2*u*v

theorem xor_boolean {u v : K} (hu : u = 0 ∨ u = 1) (hv : v = 0 ∨ v = 1) :
    xor u v = 0 ∨ xor u v = 1 := by
  rcases hu with rfl | rfl <;> rcases hv with rfl | rfl <;> norm_num [xor]

theorem xor_involution (u : K) {v : K} (hv : v = 0 ∨ v = 1) :
    xor (xor u v) v = u := by
  rcases hv with rfl | rfl
  · simp [xor]
  · unfold xor
    ring

theorem xor_boolean_iff (u : K) {v : K} (hv : v = 0 ∨ v = 1) :
    (xor u v = 0 ∨ xor u v = 1) ↔ (u = 0 ∨ u = 1) := by
  constructor
  · intro h
    simpa only [xor_involution u hv] using xor_boolean h hv
  · exact fun h => xor_boolean h hv

def transform (u : Fin 4 → K) : Fin 4 → K :=
  ![xor (u 0) (u 1), u 1, xor (u 2) (u 3), u 3]

theorem transform_boolean (u : Fin 4 → K) (h : PencilSelector.Boolean u) :
    PencilSelector.Boolean (transform u) := by
  intro i
  fin_cases i
  · exact xor_boolean (h 0) (h 1)
  · exact h 1
  · exact xor_boolean (h 2) (h 3)
  · exact h 3

theorem transform_involution (u : Fin 4 → K) (h : PencilSelector.Boolean u) :
    transform (transform u) = u := by
  funext i
  fin_cases i <;> simp [transform, xor_involution _ (h 1), xor_involution _ (h 3)]

theorem exact_offset (u v : K) : 2*xor u v+10*v = 2*u+12*v-4*u*v := by
  unfold xor
  ring

def graph (a b c x u v : K) : K := a*x^2+b*x+c+2*u+12*v-4*u*v

theorem graph_transform (a b c x u v : K) :
    graph a b c x u v = a*x^2+b*x+c+2*xor u v+10*v := by
  unfold graph xor
  ring

open SecantFixture
local instance : Fact (Nat.Prime 17) := ⟨by decide⟩

theorem left_graph (x : F) (u : Fin 4 → F) :
    graph 4 9 9 x (u 0) (u 1) =
      PencilSelector.graph 4 9 9 0 ![2,10]
        (PencilFixture.leftLabel (transform u)) x := by
  simp [graph, PencilSelector.graph, PencilSelector.offset,
    PencilFixture.leftLabel, transform, xor, Fin.sum_univ_succ]
  ring

theorem right_graph (z : F) (u : Fin 4 → F) :
    graph 4 9 9 z (u 2) (u 3) =
      PencilSelector.graph 4 9 9 0 ![2,10]
        (PencilFixture.rightLabel (transform u)) z := by
  simp [graph, PencilSelector.graph, PencilSelector.offset,
    PencilFixture.rightLabel, transform, xor, Fin.sum_univ_succ]
  ring

def System (r s e f x z m : F) (u : Fin 4 → F) : Prop :=
  PencilSelector.Boolean u ∧ SecantGroupLaw.System 2 r s x z
    (e*graph 4 9 9 x (u 0) (u 1)) (f*graph 4 9 9 z (u 2) (u 3)) m

theorem system_to_original {r s e f x z m : F} {u : Fin 4 → F}
    (h : System r s e f x z m u) :
    PencilSelector.System 2 4 9 9 0 r s e f x z m ![2,10]
      (PencilFixture.leftLabel (transform u)) (PencilFixture.rightLabel (transform u)) := by
  obtain ⟨hu, hs⟩ := h
  have ht := transform_boolean u hu
  refine ⟨?_, ?_, ?_⟩
  · intro i
    fin_cases i <;> simpa [PencilFixture.leftLabel] using ht _
  · intro i
    fin_cases i <;> simpa [PencilFixture.rightLabel] using ht _
  · simpa only [left_graph, right_graph] using hs

theorem original_to_system {r s e f x z m : F} {u : Fin 4 → F}
    (h : PencilSelector.System 2 4 9 9 0 r s e f x z m ![2,10]
      (PencilFixture.leftLabel u) (PencilFixture.rightLabel u)) :
    System r s e f x z m (transform u) := by
  have hu : PencilSelector.Boolean u := by
    intro i
    fin_cases i
    · exact h.1 0
    · exact h.1 1
    · exact h.2.1 0
    · exact h.2.1 1
  refine ⟨transform_boolean u hu, ?_⟩
  rw [left_graph, right_graph, transform_involution u hu]
  exact h.2.2

/-- Exact same offset image, before any curve or target is chosen. -/
theorem same_offset_set : ∀ t : F,
    (∃ u v : Bool, t = 2*(if u then 1 else 0)+12*(if v then 1 else 0)-
      4*(if u then 1 else 0)*(if v then 1 else 0)) ↔ t = 0 ∨ t = 2 ∨ t = 10 ∨ t = 12 := by
  decide

end Recoding

#print axioms Recoding.transform_boolean
#print axioms Recoding.transform_involution
#print axioms Recoding.system_to_original
#print axioms Recoding.original_to_system
#print axioms Recoding.same_offset_set
