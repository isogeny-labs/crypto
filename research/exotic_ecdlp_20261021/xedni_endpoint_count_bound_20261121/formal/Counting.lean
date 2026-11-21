import Mathlib.Data.ZMod.Basic
import Mathlib.Data.Finset.Card
import Mathlib.Tactic

/-! Counting the distinct public targets in a bounded endpoint table. -/

namespace XedniEndpointCount

set_option maxRecDepth 20000
set_option maxHeartbeats 0

theorem endpoint_card_le {α : Type*} [DecidableEq α]
    (M C : ℕ) (endpoint : Fin M × Fin C × Bool → α) :
    (Finset.univ.image endpoint).card ≤ 2 * M * C := by
  calc
    (Finset.univ.image endpoint).card ≤
        (Finset.univ : Finset (Fin M × Fin C × Bool)).card :=
      Finset.card_image_le
    _ = 2 * M * C := by simp [Nat.mul_comm, Nat.mul_left_comm]

theorem covered_target_count_le {α : Type*} [DecidableEq α]
    (M C : ℕ) (endpoint : Fin M × Fin C × Bool → α)
    (covered : Finset α)
    (hcovered : covered ⊆ Finset.univ.image endpoint) :
    covered.card ≤ 2 * M * C :=
  (Finset.card_le_card hcovered).trans (endpoint_card_le M C endpoint)

def coefficient43 (c : Fin 3) : ZMod 43 :=
  if c.val = 0 then 1 else if c.val = 1 then 2 else 6

def endpoint43 (M : ℕ) (t : Fin M × Fin 3 × Bool) : ZMod 43 :=
  let value := coefficient43 t.2.1 * ((t.1.val + 1 : ℕ) : ZMod 43)
  if t.2.2 then value else -value

def table43 (M : ℕ) : Finset (ZMod 43) :=
  Finset.univ.image (endpoint43 M)

theorem prefix13_covered_count : (table43 13).card = 40 := by decide

theorem prefix14_covered_count : (table43 14).card = 42 := by decide

theorem prefix14_covers_nonzero :
    ∀ x : ZMod 43, x ≠ 0 → x ∈ table43 14 := by decide

#print axioms endpoint_card_le
#print axioms covered_target_count_le
#print axioms prefix13_covered_count
#print axioms prefix14_covered_count
#print axioms prefix14_covers_nonzero

end XedniEndpointCount
