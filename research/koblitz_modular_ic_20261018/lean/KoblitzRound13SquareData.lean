import KoblitzRound13Boundary
import KoblitzRound13Square
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace KoblitzRound13SquareData
open KoblitzRound12Boundary KoblitzRound13Boundary
def parity (z : List Nat) (xs : List Nat) : Bool := xs.foldl (fun a i => a.xor (z.contains i)) false
def model0 : List Nat := [18]
def pairs0 : List (Nat × Nat × Nat × Nat) := [(12,20,13,13),(26,32,27,27),(37,43,38,38),(48,54,49,49)]
theorem graph0_passes_all_Q14 : (sparseCheck program0 model0 residual0 && extra model0 zeros0 eqs0 inv0 (qs0++pairs0))=true := by decide +kernel
theorem graph0_missing_square_relation : parity model0 [18,20]=true := by decide +kernel
def model1 : List Nat := [18]
def pairs1 : List (Nat × Nat × Nat × Nat) := [(12,20,13,13),(26,32,27,27),(37,43,38,38),(48,54,49,49)]
theorem graph1_passes_all_Q14 : (sparseCheck program1 model1 residual1 && extra model1 zeros1 eqs1 inv1 (qs1++pairs1))=true := by decide +kernel
theorem graph1_missing_square_relation : parity model1 [18,20]=true := by decide +kernel
def model2 : List Nat := [18]
def pairs2 : List (Nat × Nat × Nat × Nat) := [(12,20,13,13),(25,31,26,26),(36,42,37,37),(47,53,48,48)]
theorem graph2_passes_all_Q14 : (sparseCheck program2 model2 residual2 && extra model2 zeros2 eqs2 inv2 (qs2++pairs2))=true := by decide +kernel
theorem graph2_missing_square_relation : parity model2 [18,20]=true := by decide +kernel
def model3 : List Nat := [18]
def pairs3 : List (Nat × Nat × Nat × Nat) := [(12,20,13,13),(25,31,26,26),(36,42,37,37),(47,53,48,48)]
theorem graph3_passes_all_Q14 : (sparseCheck program3 model3 residual3 && extra model3 zeros3 eqs3 inv3 (qs3++pairs3))=true := by decide +kernel
theorem graph3_missing_square_relation : parity model3 [18,20]=true := by decide +kernel
end KoblitzRound13SquareData
#print axioms KoblitzRound13SquareData.graph0_passes_all_Q14
#print axioms KoblitzRound13SquareData.graph0_missing_square_relation
#print axioms KoblitzRound13SquareData.graph1_passes_all_Q14
#print axioms KoblitzRound13SquareData.graph1_missing_square_relation
#print axioms KoblitzRound13SquareData.graph2_passes_all_Q14
#print axioms KoblitzRound13SquareData.graph2_missing_square_relation
#print axioms KoblitzRound13SquareData.graph3_passes_all_Q14
#print axioms KoblitzRound13SquareData.graph3_missing_square_relation
