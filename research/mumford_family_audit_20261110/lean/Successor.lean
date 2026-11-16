import FamilyTransitions

namespace MumfordFamilySuccessorAudit
noncomputable section
open Polynomial MumfordFamilyPairsAudit MumfordCurveFamilyAudit MumfordOrbitFamilyAudit
  MumfordFamilyCantorAudit MumfordFamilyTransitionsAudit

def generator (t : ℕ) : Pair t := powerPair t 1 (by decide) (by unfold genus;omega) 1 (by ring)

def successorIndex (t : ℕ) : OrbitIndex t→OrbitIndex t
  | none => some (false,⟨0,by unfold genus;omega⟩)
  | some (false,d) =>
      if h : d.val+1 < genus t then some (false,⟨d.val+1,h⟩) else some (true,d)
  | some (true,d) =>
      if d.val=0 then none else some (true,⟨d.val-1,by have h:=d.isLt;omega⟩)

def coeffA (t : ℕ) : OrbitIndex t→K[X]
  | none => 0
  | some (false,_) => 0
  | some (true,_) => 1

def coeffB (t : ℕ) : OrbitIndex t→K[X]
  | none => 1
  | some _ => 0

def coeffC (t : ℕ) : OrbitIndex t→K[X]
  | none => 0
  | some (false,_) => 2
  | some (true,_) => 0

theorem successor_run (t : ℕ) (i : OrbitIndex t) :
    OneStepRun (coeffA t i) (coeffB t i) (coeffC t i)
      (generator t) (orbitPair t i) (orbitPair t (successorIndex t i)) := by
  cases i with
  | none =>
    apply identity_run t <;> simp [generator,orbitPair,powerPair,identity,successorIndex]
  | some p =>
    rcases p with ⟨b,d⟩
    cases b with
    | false =>
      by_cases h : d.val+1 < genus t
      · dsimp only [coeffA,coeffB,coeffC]
        rw [successorIndex,dite_eq_left h]
        apply positive_run t (d.val+1) (by omega) (by omega) <;>
          simp [generator,orbitPair,powerPair]
      · have hc : d.val+1=genus t := by have hd:=d.isLt;omega
        dsimp only [coeffA,coeffB,coeffC]
        rw [successorIndex,dite_eq_right h]
        apply crossing_run t <;> simp [generator,orbitPair,powerPair,hc]
    | true =>
      by_cases h : d.val=0
      · dsimp only [coeffA,coeffB,coeffC]
        rw [successorIndex,ite_eq_left h]
        apply cancel_run t <;> simp [generator,orbitPair,powerPair,identity,h]
      · have hd : 2≤d.val+1 := by omega
        have hb : d.val+1≤genus t := by have hh:=d.isLt;omega
        have he : d.val-1+1=(d.val+1)-1 := by omega
        dsimp only [coeffA,coeffB,coeffC]
        rw [successorIndex,ite_eq_right h]
        apply negative_run t (d.val+1) hd hb <;>
          simp [generator,orbitPair,powerPair,he]

theorem run_output_exists_unique (t : ℕ) (i : OrbitIndex t) :
    ∃! R : Pair t,OneStepRun (coeffA t i) (coeffB t i) (coeffC t i)
      (generator t) (orbitPair t i) R := by
  refine ⟨orbitPair t (successorIndex t i),successor_run t i,?_⟩
  intro R hR
  exact run_output_unique _ _ _ _ _ _ _ hR (successor_run t i)

/-- Defined by the unique output of actual normalized polynomial equations;
its match with the proposed indexed successor is proved separately below. -/
def stepOutput (t : ℕ) (i : OrbitIndex t) : Pair t :=
  Classical.choose (run_output_exists_unique t i)

theorem stepOutput_run (t : ℕ) (i : OrbitIndex t) :
    OneStepRun (coeffA t i) (coeffB t i) (coeffC t i)
      (generator t) (orbitPair t i) (stepOutput t i) :=
  (Classical.choose_spec (run_output_exists_unique t i)).1

theorem stepOutput_eq (t : ℕ) (i : OrbitIndex t) :
    stepOutput t i=orbitPair t (successorIndex t i) :=
  run_output_unique _ _ _ _ _ _ _ (stepOutput_run t i) (successor_run t i)

def orbitEquiv (t : ℕ) : OrbitIndex t≃OrbitPair t :=
  Equiv.ofInjective (orbitPair t) (orbitPair_injective t)

def actualStep (t : ℕ) (D : OrbitPair t) : OrbitPair t :=
  ⟨stepOutput t ((orbitEquiv t).symm D),by
    rw [stepOutput_eq]
    exact ⟨successorIndex t ((orbitEquiv t).symm D),rfl⟩⟩

theorem actualStep_equiv (t : ℕ) (i : OrbitIndex t) :
    actualStep t ((orbitEquiv t) i)=(orbitEquiv t) (successorIndex t i) := by
  apply Subtype.ext
  change stepOutput t ((orbitEquiv t).symm ((orbitEquiv t) i))=
    orbitPair t (successorIndex t i)
  rw [Equiv.symm_apply_apply,stepOutput_eq]

#print axioms successor_run
#print axioms stepOutput_run
#print axioms actualStep_equiv
end
end MumfordFamilySuccessorAudit
