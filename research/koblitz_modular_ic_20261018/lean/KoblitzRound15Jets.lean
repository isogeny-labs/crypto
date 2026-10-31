import KoblitzRound15
import KoblitzRound14Boundary
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace KoblitzRound15Jets
def residuals0 (x : Nat → ZMod 2) : List (ZMod 2) := [(1)]
theorem actual_first_jet_obstruction0 (x : Nat → ZMod 2) : (residuals0 x).sum=1 := by
 simp only [residuals0,List.sum_cons,List.sum_nil]
 ring
theorem actual_first_jet_nonzero0 (x : Nat → ZMod 2) : (residuals0 x).sum ≠ 0 := by
 rw [actual_first_jet_obstruction0]
 decide
theorem actual_gate0 : KoblitzRound12Boundary.program0[12]? = some (.mul 1 1) := by decide +kernel
theorem actual_base0 : (KoblitzRound14Boundary.point0.contains 1)=false ∧ (KoblitzRound14Boundary.point0.contains 12)=true := by decide +kernel
theorem actual_gate_jet0 {R : Type*} [CommRing R] [Nontrivial R] (x1 : R) : (0:R)*x1+x1*0 ≠ 1 := KoblitzRound15.square_unramified_obstruction x1
def residuals1 (x : Nat → ZMod 2) : List (ZMod 2) := [(1)]
theorem actual_first_jet_obstruction1 (x : Nat → ZMod 2) : (residuals1 x).sum=1 := by
 simp only [residuals1,List.sum_cons,List.sum_nil]
 ring
theorem actual_first_jet_nonzero1 (x : Nat → ZMod 2) : (residuals1 x).sum ≠ 0 := by
 rw [actual_first_jet_obstruction1]
 decide
theorem actual_gate1 : KoblitzRound12Boundary.program1[12]? = some (.mul 1 1) := by decide +kernel
theorem actual_base1 : (KoblitzRound14Boundary.point1.contains 1)=false ∧ (KoblitzRound14Boundary.point1.contains 12)=true := by decide +kernel
theorem actual_gate_jet1 {R : Type*} [CommRing R] [Nontrivial R] (x1 : R) : (0:R)*x1+x1*0 ≠ 1 := KoblitzRound15.square_unramified_obstruction x1
def residuals2 (x : Nat → ZMod 2) : List (ZMod 2) := [(1)]
theorem actual_first_jet_obstruction2 (x : Nat → ZMod 2) : (residuals2 x).sum=1 := by
 simp only [residuals2,List.sum_cons,List.sum_nil]
 ring
theorem actual_first_jet_nonzero2 (x : Nat → ZMod 2) : (residuals2 x).sum ≠ 0 := by
 rw [actual_first_jet_obstruction2]
 decide
theorem actual_gate2 : KoblitzRound12Boundary.program2[12]? = some (.mul 1 1) := by decide +kernel
theorem actual_base2 : (KoblitzRound14Boundary.point2.contains 1)=false ∧ (KoblitzRound14Boundary.point2.contains 12)=true := by decide +kernel
theorem actual_gate_jet2 {R : Type*} [CommRing R] [Nontrivial R] (x1 : R) : (0:R)*x1+x1*0 ≠ 1 := KoblitzRound15.square_unramified_obstruction x1
def residuals3 (x : Nat → ZMod 2) : List (ZMod 2) := [(1)]
theorem actual_first_jet_obstruction3 (x : Nat → ZMod 2) : (residuals3 x).sum=1 := by
 simp only [residuals3,List.sum_cons,List.sum_nil]
 ring
theorem actual_first_jet_nonzero3 (x : Nat → ZMod 2) : (residuals3 x).sum ≠ 0 := by
 rw [actual_first_jet_obstruction3]
 decide
theorem actual_gate3 : KoblitzRound12Boundary.program3[12]? = some (.mul 1 1) := by decide +kernel
theorem actual_base3 : (KoblitzRound14Boundary.point3.contains 1)=false ∧ (KoblitzRound14Boundary.point3.contains 12)=true := by decide +kernel
theorem actual_gate_jet3 {R : Type*} [CommRing R] [Nontrivial R] (x1 : R) : (0:R)*x1+x1*0 ≠ 1 := KoblitzRound15.square_unramified_obstruction x1
end KoblitzRound15Jets
#print axioms KoblitzRound15Jets.actual_first_jet_obstruction0
#print axioms KoblitzRound15Jets.actual_first_jet_nonzero0
#print axioms KoblitzRound15Jets.actual_gate0
#print axioms KoblitzRound15Jets.actual_base0
#print axioms KoblitzRound15Jets.actual_gate_jet0
#print axioms KoblitzRound15Jets.actual_first_jet_obstruction1
#print axioms KoblitzRound15Jets.actual_first_jet_nonzero1
#print axioms KoblitzRound15Jets.actual_gate1
#print axioms KoblitzRound15Jets.actual_base1
#print axioms KoblitzRound15Jets.actual_gate_jet1
#print axioms KoblitzRound15Jets.actual_first_jet_obstruction2
#print axioms KoblitzRound15Jets.actual_first_jet_nonzero2
#print axioms KoblitzRound15Jets.actual_gate2
#print axioms KoblitzRound15Jets.actual_base2
#print axioms KoblitzRound15Jets.actual_gate_jet2
#print axioms KoblitzRound15Jets.actual_first_jet_obstruction3
#print axioms KoblitzRound15Jets.actual_first_jet_nonzero3
#print axioms KoblitzRound15Jets.actual_gate3
#print axioms KoblitzRound15Jets.actual_base3
#print axioms KoblitzRound15Jets.actual_gate_jet3
