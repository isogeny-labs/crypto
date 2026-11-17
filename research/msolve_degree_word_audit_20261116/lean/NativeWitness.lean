import F4Boundary

namespace MsolveDegreeWordAudit
noncomputable section
open MvPolynomial SolvingDegreeBound

/- BEGIN ACTUAL FROZEN-ANALYZER CONSTANTS.
These are parsed written files and captured log prefixes. Boundary native exit
was124 (timeout); no normal-completion or complete-trace claim is made. -/
def nativeControlInputs : List (List (Nat × Nat × Nat)) := [[(0, 0, 1), (2, 0, 1)], [(1, 1, 1)]]
def nativeControlBasis : List (List (Nat × Nat × Nat)) := [[(0, 1, 1)], [(0, 0, 1), (2, 0, 1)]]
def nativeControlLCM : List (List Nat) := [[1, 1, 2, 0, 2, 1, 3, 3], [1, 1, 0, 1, 1, 1, 2, 2], [2, 0, 0, 1, 2, 1, 3, 3]]
def nativeControlRows : List (List Nat) := [[3, 3, 3, 1], [3, 3, 3, 2], [2, 2, 2, 1], [2, 2, 2, 1]]
def nativeControlSelections : List (List Nat) := [[3, 2147483647], [2, 2147483647]]
def nativeControlStoredBound : Nat := 3
def nativeBoundaryInputs : List (List (Nat × Nat × Nat)) := [[(0, 0, 1), (65535, 0, 1)], [(65534, 1, 1)]]
def nativeBoundaryBasis : List (List (Nat × Nat × Nat)) := [[(0, 1, 1)], [(0, 0, 1), (65535, 0, 1)]]
def nativeBoundaryLCM : List (List Nat) := [[65534, 1, 65535, 0, 65535, 1, 0, 65536], [65534, 1, 0, 1, 65534, 1, 65535, 65535], [65535, 0, 0, 1, 65535, 1, 0, 65536]]
def nativeBoundaryRows : List (List Nat) := [[0, 0, 65536, 1], [0, 1, 65536, 2], [65535, 65535, 65535, 1], [65535, 65535, 65535, 1]]
def nativeBoundarySelections : List (List Nat) := [[0, 2147483647], [65535, 2147483647]]
def nativeBoundaryStoredBound : Nat := 65535
/- END ACTUAL FROZEN-ANALYZER CONSTANTS. -/

def decodeTerm (t : Nat × Nat × Nat) : Poly (ZMod 101) :=
  monomial (Finsupp.single 0 t.1 + Finsupp.single 1 t.2.1) (t.2.2 : ZMod 101)

def decodeTerms (p : List (Nat × Nat × Nat)) : Poly (ZMod 101) :=
  (p.map decodeTerm).sum

def decodeSet (ps : List (List (Nat × Nat × Nat))) : Set (Poly (ZMod 101)) :=
  {p | p ∈ ps.map decodeTerms}

theorem native_control_inputs : decodeSet nativeControlInputs = inputs 0 := by
  ext p
  simp [decodeSet, nativeControlInputs, decodeTerms, decodeTerm, inputs,
    firstPoly, secondPoly, highX, highXY, add_comm]

theorem native_boundary_inputs : decodeSet nativeBoundaryInputs = inputs 65533 := by
  ext p
  simp [decodeSet, nativeBoundaryInputs, decodeTerms, decodeTerm, inputs,
    firstPoly, secondPoly, highX, highXY, add_comm]

theorem written_control_contains_y : (linearY : Poly (ZMod 101)) ∈ decodeSet nativeControlBasis := by
  simp [decodeSet, nativeControlBasis, decodeTerms, decodeTerm, linearY, exponentY]

theorem written_boundary_contains_y : (linearY : Poly (ZMod 101)) ∈ decodeSet nativeBoundaryBasis := by
  simp [decodeSet, nativeBoundaryBasis, decodeTerms, decodeTerm, linearY, exponentY]

def ValidRecordedLCM : List Nat → Prop
  | [a,b,c,d,x,y,w,t] => x = max a c ∧ y = max b d ∧ t = x+y ∧ w = t%65536
  | _ => False

instance (r : List Nat) : Decidable (ValidRecordedLCM r) := by
  unfold ValidRecordedLCM
  split <;> infer_instance

theorem recorded_lcm_arithmetic :
    (∀ r ∈ nativeControlLCM, ValidRecordedLCM r) ∧
    (∀ r ∈ nativeBoundaryLCM, ValidRecordedLCM r) := by decide +kernel

theorem recorded_boundary_events :
    [65534,1,65535,0,65535,1,0,65536] ∈ nativeBoundaryLCM ∧
    [0,0,65536,1] ∈ nativeBoundaryRows ∧
    [0,2147483647] ∈ nativeBoundarySelections := by decide +kernel

def recordedBound (inputDegree : Nat) (rows selections : List (List Nat)) : Nat :=
  ((rows.map (fun r => r[1]?.getD 0)) ++
    (selections.map (fun r => r[0]?.getD 0))).foldl max inputDegree

theorem recorded_bounds :
    recordedBound 2 nativeControlRows nativeControlSelections = nativeControlStoredBound ∧
    recordedBound 65535 nativeBoundaryRows nativeBoundarySelections = nativeBoundaryStoredBound ∧
    nativeControlStoredBound = 3 ∧ nativeBoundaryStoredBound = 65535 := by decide +kernel

/-- Impossibility for the decoded written set, even though the native call timed out.
This theorem does not assert that the native execution completed or refines Lean. -/
theorem written_boundary_not_degree_word_run :
    ¬ ∃ (G : Nat → Set (Poly (ZMod 101))) (t : Nat),
      F4Run nativeBoundaryStoredBound (decodeSet nativeBoundaryInputs) G ∧
      decodeSet nativeBoundaryBasis ⊆ G t := by
  rintro ⟨G,t,hG,hsub⟩
  rw [native_boundary_inputs] at hG
  exact no_lower_run 65533 G hG t (hsub written_boundary_contains_y)

#print axioms native_boundary_inputs
#print axioms recorded_lcm_arithmetic
#print axioms written_boundary_not_degree_word_run
end
end MsolveDegreeWordAudit
