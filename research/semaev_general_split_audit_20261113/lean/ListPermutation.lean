import TowerEvaluation
import Mathlib.Data.List.OfFn

namespace SemaevGeneralSplitAudit
noncomputable section

def consIndexEquiv {m n : ℕ} (e : Fin m ≃ Fin n) : Fin (m+1) ≃ Fin (n+1) where
  toFun := Fin.cases 0 (fun i => (e i).succ)
  invFun := Fin.cases 0 (fun i => (e.symm i).succ)
  left_inv := by
    intro i
    refine Fin.cases ?_ (fun j => ?_) i
    · rfl
    · simp
  right_inv := by
    intro i
    refine Fin.cases ?_ (fun j => ?_) i
    · rfl
    · simp

def swapFirstFun (n : ℕ) : Fin (n+2) → Fin (n+2) :=
  Fin.cases 1 (Fin.cases 0 (fun i => i.succ.succ))

theorem swapFirstFun_involutive (n : ℕ) : Function.Involutive (swapFirstFun n) := by
  intro i
  refine Fin.cases ?_ (fun j => Fin.cases ?_ (fun k => ?_) j) i <;> rfl

def swapFirst (n : ℕ) : Equiv.Perm (Fin (n+2)) :=
  Function.Involutive.toPerm (swapFirstFun n) (swapFirstFun_involutive n)

/-- Construct a permutation of positions, without assuming distinct values. -/
theorem listPerm_indices {α : Type*} {l r : List α} (h : l.Perm r) :
    ∃ e : Fin l.length ≃ Fin r.length,∀ i,r.get (e i)=l.get i := by
  induction h with
  | nil =>
      refine ⟨Equiv.refl _,?_⟩
      intro i
      exact Fin.elim0 i
  | cons a h ih =>
      obtain ⟨e,he⟩ := ih
      refine ⟨consIndexEquiv e,?_⟩
      intro i
      refine Fin.cases ?_ (fun j => ?_) i
      · rfl
      · simpa [consIndexEquiv] using he j
  | swap a b l =>
      refine ⟨swapFirst l.length,?_⟩
      intro i
      refine Fin.cases ?_ (fun j => Fin.cases ?_ (fun k => ?_) j) i <;> rfl
  | trans h₁ h₂ ih₁ ih₂ =>
      obtain ⟨e,he⟩ := ih₁
      obtain ⟨g,hg⟩ := ih₂
      exact ⟨e.trans g,fun i => (hg (e i)).trans (he i)⟩

def castIndexEquiv {m n : ℕ} (h : m=n) : Fin m ≃ Fin n where
  toFun := Fin.cast h
  invFun := Fin.cast h.symm
  left_inv := by intro i; cases h; rfl
  right_inv := by intro i; cases h; rfl

theorem ofFn_perm_indices {α : Type*} {n : ℕ} (xs ys : Fin n → α)
    (h : (List.ofFn xs).Perm (List.ofFn ys)) :
    ∃ e : Equiv.Perm (Fin n),∀ i,ys (e i)=xs i := by
  obtain ⟨e,he⟩ := listPerm_indices h
  let hl : (List.ofFn xs).length=n := List.length_ofFn
  let hr : (List.ofFn ys).length=n := List.length_ofFn
  refine ⟨(castIndexEquiv hl.symm).trans (e.trans (castIndexEquiv hr)),?_⟩
  intro i
  have hi := he (Fin.cast hl.symm i)
  change ys (Fin.cast hr (e (Fin.cast hl.symm i)))=xs i
  simpa only [List.get_ofFn,Fin.cast_cast,Fin.cast_refl,id_eq] using hi

variable {K S : Type*} [Field K] [CommRing S]

theorem sourceValue_ofFn_perm (W : WeierstrassCurve.Affine K) (n : ℕ) (f : K →+* S)
    (xs ys : Fin (n+3) → S) (h : (List.ofFn xs).Perm (List.ofFn ys)) :
    sourceValue W n f xs=sourceValue W n f ys := by
  obtain ⟨e,he⟩ := ofFn_perm_indices xs ys h
  have hh : ys ∘ e=xs := funext he
  rw [← hh]
  exact sourceValue_permute W n f ys e

#print axioms sourceValue_ofFn_perm
end
end SemaevGeneralSplitAudit
