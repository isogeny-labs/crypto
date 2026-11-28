import Successor

namespace MumfordFamilyCycleAudit
noncomputable section
open MumfordFamilyPairsAudit MumfordCurveFamilyAudit MumfordOrbitFamilyAudit
  MumfordFamilyCantorAudit MumfordFamilySuccessorAudit

def position (t : ℕ) : OrbitIndex t → Fin (exponent t)
  | none => ⟨0,exponent_positive t⟩
  | some (false,d) => ⟨d.val+1,by have h:=d.isLt;have hn:=exponent_genus t;omega⟩
  | some (true,d) => ⟨exponent t-(d.val+1),by
      have h:=d.isLt;have hn:=exponent_genus t;omega⟩

theorem position_injective (t : ℕ) : Function.Injective (position t) := by
  intro i j h
  have hv:=congrArg Fin.val h
  cases i with
  | none =>
    cases j with
    | none => rfl
    | some p =>
      rcases p with ⟨b,d⟩
      have hd:=d.isLt
      have hn:=exponent_genus t
      cases b <;> simp only [position] at hv <;> omega
  | some p =>
    rcases p with ⟨b,d⟩
    have hd:=d.isLt
    have hn:=exponent_genus t
    cases j with
    | none => cases b <;> simp only [position] at hv <;> omega
    | some q =>
      rcases q with ⟨c,e⟩
      have he:=e.isLt
      cases b <;> cases c <;> simp only [position] at hv
      · congr 2;apply Fin.ext;omega
      · omega
      · omega
      · congr 2;apply Fin.ext;omega

def nextPosition (t : ℕ) (j : Fin (exponent t)) : Fin (exponent t) :=
  ⟨(j.val+1)%exponent t,Nat.mod_lt _ (exponent_positive t)⟩

theorem position_successor (t : ℕ) (i : OrbitIndex t) :
    position t (successorIndex t i)=nextPosition t (position t i) := by
  apply Fin.ext
  have hn:=exponent_genus t
  have hg : 2≤genus t := by unfold genus;omega
  cases i with
  | none =>
    simp only [successorIndex,position,nextPosition]
    rw [Nat.mod_eq_of_lt (by omega)]
  | some p =>
    rcases p with ⟨b,d⟩
    have hd:=d.isLt
    cases b with
    | false =>
      by_cases h : d.val+1<genus t
      · simp only [successorIndex,dite_eq_left h,position,nextPosition]
        rw [Nat.mod_eq_of_lt (by omega)]
      · simp only [successorIndex,dite_eq_right h,position,nextPosition]
        rw [Nat.mod_eq_of_lt (by omega)]
        omega
    | true =>
      by_cases h : d.val=0
      · simp only [successorIndex,ite_eq_left h,position,nextPosition]
        have he : exponent t-(d.val+1)+1=exponent t := by omega
        rw [he,Nat.mod_self]
      · simp only [successorIndex,ite_eq_right h,position,nextPosition]
        rw [Nat.mod_eq_of_lt (by omega)]
        omega

theorem nextPosition_iterate (t j : ℕ) (k : Fin (exponent t)) :
    ((nextPosition t)^[j] k).val=(k.val+j)%exponent t := by
  induction j with
  | zero => simp [Nat.mod_eq_of_lt k.isLt]
  | succ j ih =>
    rw [Function.iterate_succ_apply']
    change (((nextPosition t)^[j] k).val+1)%exponent t=_
    rw [ih,Nat.add_mod,Nat.mod_mod,←Nat.add_mod]
    congr 1

theorem position_iterate (t j : ℕ) (i : OrbitIndex t) :
    position t ((successorIndex t)^[j] i)=(nextPosition t)^[j] (position t i) := by
  induction j with
  | zero => rfl
  | succ j ih =>
    rw [Function.iterate_succ_apply',position_successor,ih,Function.iterate_succ_apply']

theorem index_period_iff (t j : ℕ) (i : OrbitIndex t) :
    (successorIndex t)^[j] i=i ↔ exponent t∣j := by
  have hn:=exponent_positive t
  constructor
  · intro h
    have hh:=congrArg Fin.val (position_iterate t j i)
    rw [h,nextPosition_iterate] at hh
    have he:=Nat.mod_add_div ((position t i).val+j) (exponent t)
    rw [←hh] at he
    exact ⟨((position t i).val+j)/exponent t,by omega⟩
  · rintro ⟨a,ha⟩
    apply position_injective t
    rw [position_iterate]
    apply Fin.ext
    rw [nextPosition_iterate,ha,Nat.add_mod]
    simp only [Nat.mul_mod_right,Nat.add_zero,Nat.mod_eq_of_lt (position t i).isLt]

theorem actualStep_iterate_equiv (t j : ℕ) (i : OrbitIndex t) :
    (actualStep t)^[j] ((orbitEquiv t) i)=(orbitEquiv t) ((successorIndex t)^[j] i) := by
  induction j with
  | zero => rfl
  | succ j ih =>
    rw [Function.iterate_succ_apply',ih,actualStep_equiv,Function.iterate_succ_apply']

theorem actualStep_period_iff (t j : ℕ) (D : OrbitPair t) :
    (actualStep t)^[j] D=D ↔ exponent t∣j := by
  obtain ⟨i,rfl⟩:=(orbitEquiv t).surjective D
  rw [actualStep_iterate_equiv,(orbitEquiv t).injective.eq_iff,index_period_iff]

theorem no_shorter_positive_period (t j : ℕ) (D : OrbitPair t)
    (hj : 0<j) (hb : j<exponent t) : (actualStep t)^[j] D≠D := by
  intro h
  have hd:=(actualStep_period_iff t j D).mp h
  have hle:=Nat.le_of_dvd hj hd
  omega

#print axioms actualStep_period_iff
#print axioms no_shorter_positive_period
end
end MumfordFamilyCycleAudit
