import Mathlib.Data.Fintype.Card
import Mathlib.Data.Fintype.Pi
import Mathlib.Data.Fintype.Prod

/-! The bit-vector change of variables used in the S3 leaf-sum panels. -/

namespace G2LeafSum

abbrev Bits (n : ℕ) := Fin n → Bool
abbrev Pair (n : ℕ) := Bits n × Bits n

noncomputable instance bitsFintype (n : ℕ) : Fintype (Bits n) := by
  classical
  infer_instance

noncomputable instance pairFintype (n : ℕ) : Fintype (Pair n) := by
  classical
  infer_instance

/-- The second half of the coordinates changes from `v` to `s = u xor v`. -/
def change {n : ℕ} (p : Pair n) : Pair n :=
  (p.1, fun i => xor (p.1 i) (p.2 i))

/-- In characteristic two the same substitution recovers `v = u xor s`. -/
theorem change_involutive {n : ℕ} (p : Pair n) : change (change p) = p := by
  apply Prod.ext
  · rfl
  · funext i
    simp only [change]
    cases p.1 i <;> cases p.2 i <;> rfl

/-- The coordinate substitution is an exact bijection on all Boolean pairs. -/
def changeEquiv (n : ℕ) : Pair n ≃ Pair n where
  toFun := change
  invFun := change
  left_inv := change_involutive
  right_inv := change_involutive

/-- Solutions of any original equation predicate correspond one for one to
solutions of its transformed predicate. -/
def solutionEquiv {n : ℕ} (P : Pair n → Prop) :
    {p : Pair n // P p} ≃ {p : Pair n // P (change p)} where
  toFun p := ⟨change p.1, by simpa [change_involutive] using p.2⟩
  invFun p := ⟨change p.1, p.2⟩
  left_inv p := by
    apply Subtype.ext
    exact change_involutive p.1
  right_inv p := by
    apply Subtype.ext
    exact change_involutive p.1

/-- The transformed and original Boolean systems have equal root counts. -/
theorem solution_card_eq {n : ℕ} (P : Pair n → Prop) [DecidablePred P]
    [DecidablePred (fun p : Pair n => P (change p))] :
    Fintype.card {p : Pair n // P p} =
      Fintype.card {p : Pair n // P (change p)} :=
  Fintype.card_congr (solutionEquiv P)

#print axioms G2LeafSum.change_involutive
#print axioms G2LeafSum.solutionEquiv
#print axioms G2LeafSum.solution_card_eq

end G2LeafSum
