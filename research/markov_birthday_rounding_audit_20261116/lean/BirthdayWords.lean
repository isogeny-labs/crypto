import MarkovCollision

namespace MarkovBirthdayRoundingAudit
noncomputable section
open StaticHashCollisionAudit
variable (p : ℕ) [NeZero p]

omit [NeZero p] in
theorem reset_word_succ (x : ZMod p) {n : ℕ} (w : Fin n → ZMod p) (j : Fin n) :
    wordPath (resetUpdate p) x w j.succ.val=w j := by
  exact wordPath_succ (resetUpdate p) x w j.val j.isLt

omit [NeZero p] in
/-- Actual selfavoidance is exactly an injective chronological word avoiding the start. -/
theorem reset_selfAvoiding_iff (x : ZMod p) {n : ℕ} (w : Fin n → ZMod p) :
    SelfAvoiding (resetUpdate p) x w ↔ Function.Injective w ∧ ∀ j,w j≠x := by
  constructor
  · intro h
    constructor
    · intro i j he
      apply Fin.succ_injective
      apply h
      simpa only [reset_word_succ] using he
    · intro j he
      have hij : j.succ=(0 : Fin (n+1)) := h (by
        simpa only [reset_word_succ,wordPath_zero,Fin.val_zero] using he)
      have hv:=congrArg Fin.val hij
      simp only [Fin.val_succ,Fin.val_zero] at hv
      omega
  · rintro ⟨hw,hx⟩ i j he
    cases i using Fin.cases with
    | zero =>
      cases j using Fin.cases with
      | zero => rfl
      | succ j =>
        have hj : x=w j := by simpa only [wordPath_zero,reset_word_succ,Fin.val_zero] using he
        exact False.elim (hx j hj.symm)
    | succ i =>
      cases j using Fin.cases with
      | zero =>
        have hi : w i=x := by simpa only [wordPath_zero,reset_word_succ,Fin.val_zero] using he
        exact False.elim (hx i hi)
      | succ j =>
        apply congrArg Fin.succ
        apply hw
        simpa only [reset_word_succ] using he

abbrev avoidingWords (x : ZMod p) (n : ℕ) :=
  {w : Fin n → ZMod p // SelfAvoiding (resetUpdate p) x w}

/-- Genuine finite sampling bijection, not an assumed count of valid words. -/
def avoidingEmbeddingEquiv (x : ZMod p) (n : ℕ) :
    avoidingWords p x n ≃ (Fin n ↪ {y : ZMod p // y≠x}) where
  toFun w :=
    { toFun:=fun j=>⟨w.val j,((reset_selfAvoiding_iff p x w.val).mp w.property).2 j⟩
      inj':=by
        intro i j he
        exact ((reset_selfAvoiding_iff p x w.val).mp w.property).1 (congrArg Subtype.val he) }
  invFun e := ⟨fun j=>(e j).val,(reset_selfAvoiding_iff p x _).mpr ⟨by
    intro i j he
    exact e.injective (Subtype.ext he),fun j=>(e j).property⟩⟩
  left_inv w := Subtype.ext rfl
  right_inv e := by ext j;rfl

theorem complement_card (x : ZMod p) : Fintype.card {y : ZMod p // y≠x}=p-1 := by
  rw [Fintype.card_subtype_compl (fun y : ZMod p=>y=x),Fintype.card_subtype_eq,ZMod.card]

theorem avoiding_words_card (x : ZMod p) (n : ℕ) :
    Fintype.card (avoidingWords p x n)=(p-1).descFactorial n := by
  rw [Fintype.card_congr (avoidingEmbeddingEquiv p x n),Fintype.card_embedding_eq,
    complement_card,Fintype.card_fin]

#print axioms reset_selfAvoiding_iff
#print axioms avoiding_words_card
end
end MarkovBirthdayRoundingAudit
