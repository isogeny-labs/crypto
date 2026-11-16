import AllPairRuns
import IndexArithmetic

namespace MumfordPairwiseOutputAudit
noncomputable section
open Polynomial MumfordFamilyPairsAudit MumfordCurveFamilyAudit MumfordOrbitFamilyAudit
  MumfordFamilyCantorAudit MumfordFamilyTransitionsAudit MumfordAllPairRunsAudit
  MumfordIndexArithmeticAudit

def sign (b : Bool) : K := if b then -1 else 1

theorem sign_sq (b : Bool) : sign b^2=1 := by cases b <;> simp [sign]
theorem sign_not (b : Bool) : sign (!b)= -sign b := by cases b <;> simp [sign]

theorem pair_coords (t : ℕ) (b : Bool) (d : Fin (genus t)) :
    (orbitPair t (some (b,d))).u=X^(d.val+1) ∧
    (orbitPair t (some (b,d))).v=C (sign b) := by
  cases b <;> simp [orbitPair,powerPair,sign]

theorem same_output_run (t : ℕ) (b : Bool) (d e : Fin (genus t)) :
    OneStepRun 0 0 (-C (sign b)) (orbitPair t (some (b,d)))
      (orbitPair t (some (b,e))) (orbitPair t (sameIndex t b d e)) := by
  have hd:=d.isLt
  have he:=e.isLt
  have hn:=exponent_genus t
  apply same_sign_run t (d.val+1) (e.val+1) (by omega) (by omega)
    (by omega) (by omega) (sign b) (sign_sq b) _ _ _
    (pair_coords t b d).1 (pair_coords t b d).2
    (pair_coords t b e).1 (pair_coords t b e).2
  by_cases h : d.val+e.val+2≤genus t
  · left
    refine ⟨by omega,?_,?_⟩
    · simp only [sameIndex,dite_eq_left h]
      rw [(pair_coords t b _).1]
      congr 1
      dsimp only
      omega
    · simp only [sameIndex,dite_eq_left h]
      exact (pair_coords t b _).2
  · right
    refine ⟨by omega,?_,?_⟩
    · simp only [sameIndex,dite_eq_right h]
      rw [(pair_coords t (!b) _).1]
      congr 1
      dsimp only
      omega
    · simp only [sameIndex,dite_eq_right h]
      rw [(pair_coords t (!b) _).2,sign_not,map_neg]

theorem opposite_output_run_le (t : ℕ) (b : Bool) (d e : Fin (genus t))
    (hle : d.val≤e.val) :
    OneStepRun 1 0 0 (orbitPair t (some (b,d)))
      (orbitPair t (some (!b,e))) (orbitPair t (differenceIndex t b d e)) := by
  have hd:=d.isLt
  have he:=e.isLt
  by_cases h : d.val=e.val
  · apply opposite_cancel_run t (d.val+1) (sign b) _ _ _
      (pair_coords t b d).1 (pair_coords t b d).2
    · rw [(pair_coords t (!b) e).1,h]
    · rw [(pair_coords t (!b) e).2,sign_not,map_neg]
    · simp [differenceIndex,h,orbitPair,identity]
    · simp [differenceIndex,h,orbitPair,identity]
  · have hlt : d.val<e.val := by omega
    apply opposite_strict_run t (d.val+1) (e.val+1) (by omega) (by omega)
      (sign b) (sign_sq b) _ _ _
      (pair_coords t b d).1 (pair_coords t b d).2 (pair_coords t (!b) e).1
    · rw [(pair_coords t (!b) e).2,sign_not,map_neg]
    · simp only [differenceIndex,ite_eq_right h,ite_eq_left hlt]
      rw [(pair_coords t (!b) _).1]
      congr 1
      dsimp only
      omega
    · simp only [differenceIndex,ite_eq_right h,ite_eq_left hlt]
      rw [(pair_coords t (!b) _).2,sign_not,map_neg]

theorem opposite_output_run_gt (t : ℕ) (b : Bool) (d e : Fin (genus t))
    (hgt : e.val<d.val) :
    OneStepRun 0 1 0 (orbitPair t (some (b,d)))
      (orbitPair t (some (!b,e))) (orbitPair t (differenceIndex t b d e)) := by
  have hd:=d.isLt
  have he:=e.isLt
  apply run_swap t 1 0 0
  apply opposite_strict_run t (e.val+1) (d.val+1) (by omega) (by omega)
    (sign (!b)) (sign_sq (!b)) _ _ _
    (pair_coords t (!b) e).1 (pair_coords t (!b) e).2 (pair_coords t b d).1
  · rw [(pair_coords t b d).2,sign_not,map_neg,neg_neg]
  · simp only [differenceIndex,ite_eq_right (by omega : d.val≠e.val),
      ite_eq_right (by omega : ¬d.val<e.val)]
    rw [(pair_coords t b _).1]
    congr 1
    dsimp only
    omega
  · simp only [differenceIndex,ite_eq_right (by omega : d.val≠e.val),
      ite_eq_right (by omega : ¬d.val<e.val)]
    rw [(pair_coords t b _).2,sign_not,map_neg,neg_neg]

def bezoutTriple (t : ℕ) : OrbitIndex t → OrbitIndex t → K[X]×K[X]×K[X]
  | none,_ => (1,0,0)
  | _,none => (0,1,0)
  | some (b,d),some (c,e) =>
      if b=c then (0,0,-C (sign b)) else if d.val≤e.val then (1,0,0) else (0,1,0)

def SelectedRun (t : ℕ) (i j : OrbitIndex t) (R : Pair t) : Prop :=
  OneStepRun (bezoutTriple t i j).1 (bezoutTriple t i j).2.1 (bezoutTriple t i j).2.2
    (orbitPair t i) (orbitPair t j) R

theorem selected_run_candidate (t : ℕ) (i j : OrbitIndex t) :
    SelectedRun t i j (orbitPair t (combinedIndex t i j)) := by
  cases i with
  | none =>
    change OneStepRun 1 0 0 (orbitPair t none) (orbitPair t j) (orbitPair t j)
    exact identity_left_run t _ _ rfl rfl
  | some p =>
    rcases p with ⟨b,d⟩
    cases j with
    | none =>
      change OneStepRun 0 1 0 (orbitPair t (some (b,d))) (orbitPair t none)
        (orbitPair t (some (b,d)))
      exact run_swap t 1 0 0 _ _ _ (identity_left_run t _ _ rfl rfl)
    | some q =>
      rcases q with ⟨c,e⟩
      cases b <;> cases c
      · simpa [SelectedRun,bezoutTriple,combinedIndex] using same_output_run t false d e
      · by_cases h : d.val≤e.val
        · simpa [SelectedRun,bezoutTriple,combinedIndex,h] using opposite_output_run_le t false d e h
        · simpa [SelectedRun,bezoutTriple,combinedIndex,h] using
            opposite_output_run_gt t false d e (by omega)
      · by_cases h : d.val≤e.val
        · simpa [SelectedRun,bezoutTriple,combinedIndex,h] using opposite_output_run_le t true d e h
        · simpa [SelectedRun,bezoutTriple,combinedIndex,h] using
            opposite_output_run_gt t true d e (by omega)
      · simpa [SelectedRun,bezoutTriple,combinedIndex] using same_output_run t true d e

theorem selected_output_exists_unique (t : ℕ) (i j : OrbitIndex t) :
    ∃! R : Pair t,SelectedRun t i j R := by
  refine ⟨orbitPair t (combinedIndex t i j),selected_run_candidate t i j,?_⟩
  intro R hR
  exact run_output_unique _ _ _ _ _ _ _ hR (selected_run_candidate t i j)

/-- Actual normalized polynomial output, defined before any group structure
on this family is transported from residues. -/
def pairwiseOutput (t : ℕ) (i j : OrbitIndex t) : Pair t :=
  Classical.choose (selected_output_exists_unique t i j)

theorem pairwiseOutput_runs (t : ℕ) (i j : OrbitIndex t) :
    SelectedRun t i j (pairwiseOutput t i j) :=
  (Classical.choose_spec (selected_output_exists_unique t i j)).1

theorem pairwiseOutput_eq (t : ℕ) (i j : OrbitIndex t) :
    pairwiseOutput t i j=orbitPair t (combinedIndex t i j) :=
  run_output_unique _ _ _ _ _ _ _ (pairwiseOutput_runs t i j) (selected_run_candidate t i j)

#print axioms selected_run_candidate
#print axioms selected_output_exists_unique
#print axioms pairwiseOutput_eq
end
end MumfordPairwiseOutputAudit
