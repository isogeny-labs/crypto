import Mathlib.Data.Fintype.Card

/-! The two exact three-block Boolean changes of variables in this screen. -/

namespace H4TripleSum

abbrev Bits (n : ℕ) := Fin n → Bool

structure Triple (n : ℕ) where
  u : Bits n
  v : Bits n
  w : Bits n

/-- Third block becomes `s = u xor w`. -/
def pairChange {n : ℕ} (t : Triple n) : Triple n :=
  ⟨t.u, t.v, fun i => xor (t.u i) (t.w i)⟩

/-- Third block becomes `s = u xor v xor w`. -/
def totalChange {n : ℕ} (t : Triple n) : Triple n :=
  ⟨t.u, t.v, fun i => xor (xor (t.u i) (t.v i)) (t.w i)⟩

theorem pairChange_involutive {n : ℕ} (t : Triple n) :
    pairChange (pairChange t) = t := by
  cases t with
  | mk u v w =>
    simp only [pairChange]
    congr 1
    funext i
    cases u i <;> cases w i <;> rfl

theorem totalChange_involutive {n : ℕ} (t : Triple n) :
    totalChange (totalChange t) = t := by
  cases t with
  | mk u v w =>
    simp only [totalChange]
    congr 1
    funext i
    cases u i <;> cases v i <;> cases w i <;> rfl

/-- Exact root correspondence for any Boolean equation predicate under
the pair-sum substitution. -/
def pairSolutionEquiv {n : ℕ} (P : Triple n → Prop) :
    {t : Triple n // P t} ≃ {t : Triple n // P (pairChange t)} where
  toFun t := ⟨pairChange t.1, by simpa [pairChange_involutive] using t.2⟩
  invFun t := ⟨pairChange t.1, t.2⟩
  left_inv t := by
    apply Subtype.ext
    exact pairChange_involutive t.1
  right_inv t := by
    apply Subtype.ext
    exact pairChange_involutive t.1

/-- Exact root correspondence under the total-sum substitution. -/
def totalSolutionEquiv {n : ℕ} (P : Triple n → Prop) :
    {t : Triple n // P t} ≃ {t : Triple n // P (totalChange t)} where
  toFun t := ⟨totalChange t.1, by simpa [totalChange_involutive] using t.2⟩
  invFun t := ⟨totalChange t.1, t.2⟩
  left_inv t := by
    apply Subtype.ext
    exact totalChange_involutive t.1
  right_inv t := by
    apply Subtype.ext
    exact totalChange_involutive t.1

#print axioms H4TripleSum.pairChange_involutive
#print axioms H4TripleSum.totalChange_involutive
#print axioms H4TripleSum.pairSolutionEquiv
#print axioms H4TripleSum.totalSolutionEquiv

end H4TripleSum
