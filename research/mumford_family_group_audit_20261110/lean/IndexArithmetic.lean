import FamilyCertificate
import Mathlib.Data.ZMod.Basic
import Mathlib.Data.Fintype.EquivFin
import Mathlib.Algebra.Group.TransferInstance

namespace MumfordIndexArithmeticAudit
noncomputable section
open MumfordCurveFamilyAudit MumfordOrbitFamilyAudit MumfordFamilyCycleAudit

private instance exponentNonzero (t : ℕ) : NeZero (exponent t) :=
  ⟨Nat.ne_of_gt (exponent_positive t)⟩

def sameIndex (t : ℕ) (b : Bool) (d e : Fin (genus t)) : OrbitIndex t :=
  if h : d.val+e.val+2≤genus t then
    some (b,⟨d.val+e.val+1,by omega⟩)
  else some (!b,⟨exponent t-(d.val+e.val+2)-1,by
    have hd:=d.isLt;have he:=e.isLt;have hn:=exponent_genus t;omega⟩)

def differenceIndex (t : ℕ) (b : Bool) (d e : Fin (genus t)) : OrbitIndex t :=
  if d.val=e.val then none else
    if d.val<e.val then some (!b,⟨e.val-d.val-1,by have he:=e.isLt;omega⟩)
    else some (b,⟨d.val-e.val-1,by have hd:=d.isLt;omega⟩)

def combinedIndex (t : ℕ) : OrbitIndex t → OrbitIndex t → OrbitIndex t
  | none,j => j
  | i,none => i
  | some (b,d),some (c,e) =>
      if b=c then sameIndex t b d e else differenceIndex t b d e

def residue (t : ℕ) : OrbitIndex t → ZMod (exponent t)
  | none => 0
  | some (false,d) => d.val+1
  | some (true,d) => -(d.val+1)

theorem residue_eq_position (t : ℕ) (i : OrbitIndex t) :
    residue t i=((position t i).val : ZMod (exponent t)) := by
  cases i with
  | none => rfl
  | some p =>
    rcases p with ⟨b,d⟩
    have hd:=d.isLt
    have hn:=exponent_genus t
    cases b
    · simp [residue,position]
    · simp only [residue,position,Nat.cast_sub (by omega : d.val+1≤exponent t),
        ZMod.natCast_self,zero_sub,Nat.cast_add,Nat.cast_one]

theorem residue_injective (t : ℕ) : Function.Injective (residue t) := by
  intro i j h
  rw [residue_eq_position,residue_eq_position] at h
  have hh:=congrArg ZMod.val h
  rw [ZMod.val_natCast_of_lt (position t i).isLt,
    ZMod.val_natCast_of_lt (position t j).isLt] at hh
  exact position_injective t (Fin.ext hh)

theorem residue_bijective (t : ℕ) : Function.Bijective (residue t) := by
  apply (Fintype.bijective_iff_injective_and_card _).mpr
  refine ⟨residue_injective t,?_⟩
  rw [ZMod.card]
  simpa only [Nat.card_eq_fintype_card] using index_cardinality t

def residueEquiv (t : ℕ) : OrbitIndex t≃ZMod (exponent t) :=
  Equiv.ofBijective (residue t) (residue_bijective t)

theorem sameIndex_residue (t : ℕ) (b : Bool) (d e : Fin (genus t)) :
    residue t (sameIndex t b d e)=
      residue t (some (b,d))+residue t (some (b,e)) := by
  have hd:=d.isLt
  have he:=e.isLt
  have hn:=exponent_genus t
  by_cases h : d.val+e.val+2≤genus t
  · cases b <;> simp [sameIndex,h,residue] <;> ring
  · have hs : d.val+e.val+2<exponent t := by omega
    have hp : exponent t-(d.val+e.val+2)-1+1=exponent t-(d.val+e.val+2) := by omega
    cases b <;> simp only [sameIndex,dite_eq_right h,Bool.not_false,Bool.not_true,residue]
    · rw [←Nat.cast_add_one,hp,Nat.cast_sub (Nat.le_of_lt hs),ZMod.natCast_self]
      push_cast
      ring
    · rw [←Nat.cast_add_one,hp,Nat.cast_sub (Nat.le_of_lt hs),ZMod.natCast_self]
      push_cast
      ring

theorem differenceIndex_residue (t : ℕ) (b : Bool) (d e : Fin (genus t)) :
    residue t (differenceIndex t b d e)=
      residue t (some (b,d))+residue t (some (!b,e)) := by
  have hd:=d.isLt
  have he:=e.isLt
  by_cases h : d.val=e.val
  · cases b <;> simp [differenceIndex,h,residue]
  · by_cases hlt : d.val<e.val
    · have hp : e.val-d.val-1+1=e.val-d.val := by omega
      cases b <;> simp only [differenceIndex,ite_eq_right h,ite_eq_left hlt,
        Bool.not_false,Bool.not_true,residue]
      · rw [←Nat.cast_add_one,hp,Nat.cast_sub (Nat.le_of_lt hlt)]
        ring
      · rw [←Nat.cast_add_one,hp,Nat.cast_sub (Nat.le_of_lt hlt)]
        ring
    · have hp : d.val-e.val-1+1=d.val-e.val := by omega
      cases b <;> simp only [differenceIndex,ite_eq_right h,ite_eq_right hlt,
        Bool.not_false,Bool.not_true,residue]
      · rw [←Nat.cast_add_one,hp,Nat.cast_sub (by omega : e.val≤d.val)]
        ring
      · rw [←Nat.cast_add_one,hp,Nat.cast_sub (by omega : e.val≤d.val)]
        ring

theorem combinedIndex_residue (t : ℕ) (i j : OrbitIndex t) :
    residue t (combinedIndex t i j)=residue t i+residue t j := by
  cases i with
  | none => simp [combinedIndex,residue]
  | some p =>
    rcases p with ⟨b,d⟩
    cases j with
    | none => simp [combinedIndex,residue]
    | some q =>
      rcases q with ⟨c,e⟩
      by_cases h : b=c
      · subst c
        simp only [combinedIndex]
        exact sameIndex_residue t b d e
      · have hc : c= !b := by cases b <;> cases c <;> simp_all
        rw [hc]
        have hb : b≠ !b := by cases b <;> decide
        simp only [combinedIndex,ite_eq_right hb]
        exact differenceIndex_residue t b d e

#print axioms residue_bijective
#print axioms combinedIndex_residue
end
end MumfordIndexArithmeticAudit
