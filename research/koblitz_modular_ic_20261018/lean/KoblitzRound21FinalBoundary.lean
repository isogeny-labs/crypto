import KoblitzRound21Separators
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace KoblitzRound21FinalBoundary
open KoblitzRound12Boundary KoblitzRound13Boundary KoblitzRound13SquareData KoblitzRound14Boundary
def point0 : List Nat := [180,182]
theorem actual_remaining_model0 : (sparseCheck program0 point0 residual0 && extra point0 zeros0 eqs0 inv0 (qs0++KoblitzRound14Boundary.pairs0) && (KoblitzRound14Boundary.linear0.all fun e => !(parity point0 e)) && (KoblitzRound14Boundary.expanded0.all fun e => !(multi point0 e)))=true := by decide +kernel
theorem cube_zero0 : ([19,31,42,53,64,18,30,41,52,63,95,0,17,18,19,20,23,24,71,73,74,76,77,79,80,82,83,85,86,88,89,91,92,94,99,101,102,104,105,107,108,110,111,113,114,116,117,119,120,122,379,381,382,384,385,387,388,390,391,393,394,396,397,399,400,402,407,408,410,411,413,414,416,417,419,420,422,423,425,426,428,123,183,243,303,429,485,541,597].all fun i => !(point0.contains i))=true := by decide +kernel
theorem nonzero0 : point0.isEmpty=false := by decide +kernel
theorem all_new_linear0 : ([[123,126],[123,126],[183,186],[243,246],[303,306],[429,431],[485,487],[541,543],[597,599]] : List (List Nat)).all (fun e=> !(parity point0 e))=true := by decide +kernel
theorem actual_new_linear0 : parity point0 [123,126]=false := by decide +kernel
def point1 : List Nat := [180,182]
theorem actual_remaining_model1 : (sparseCheck program1 point1 residual1 && extra point1 zeros1 eqs1 inv1 (qs1++KoblitzRound14Boundary.pairs1) && (KoblitzRound14Boundary.linear1.all fun e => !(parity point1 e)) && (KoblitzRound14Boundary.expanded1.all fun e => !(multi point1 e)))=true := by decide +kernel
theorem cube_zero1 : ([19,31,42,53,64,18,30,41,52,63,95,0,17,18,19,20,23,24,71,73,74,76,77,79,80,82,83,85,86,88,89,91,92,94,99,101,102,104,105,107,108,110,111,113,114,116,117,119,120,122,379,381,382,384,385,387,388,390,391,393,394,396,397,399,400,402,407,408,410,411,413,414,416,417,419,420,422,423,425,426,428,123,183,243,303,429,485,541,597].all fun i => !(point1.contains i))=true := by decide +kernel
theorem nonzero1 : point1.isEmpty=false := by decide +kernel
theorem all_new_linear1 : ([[123,126],[123,126],[183,186],[243,246],[303,306],[429,431],[485,487],[541,543],[597,599]] : List (List Nat)).all (fun e=> !(parity point1 e))=true := by decide +kernel
theorem actual_new_linear1 : parity point1 [123,126]=false := by decide +kernel
def point2 : List Nat := [180,182]
theorem actual_remaining_model2 : (sparseCheck program2 point2 residual2 && extra point2 zeros2 eqs2 inv2 (qs2++KoblitzRound14Boundary.pairs2) && (KoblitzRound14Boundary.linear2.all fun e => !(parity point2 e)) && (KoblitzRound14Boundary.expanded2.all fun e => !(multi point2 e)))=true := by decide +kernel
theorem cube_zero2 : ([19,30,41,52,63,18,29,40,51,62,95,0,17,18,19,20,21,22,23,71,73,74,76,77,79,80,82,83,85,86,88,89,91,92,94,99,101,102,104,105,107,108,110,111,113,114,116,117,119,120,122,379,381,382,384,385,387,388,390,391,393,394,396,397,399,400,402,407,408,410,411,413,414,416,417,419,420,422,423,425,426,428,123,183,243,303,429,485,541,597].all fun i => !(point2.contains i))=true := by decide +kernel
theorem nonzero2 : point2.isEmpty=false := by decide +kernel
theorem all_new_linear2 : ([[123,126],[123,126],[183,186],[243,246],[303,306],[429,431],[485,487],[541,543],[597,599]] : List (List Nat)).all (fun e=> !(parity point2 e))=true := by decide +kernel
theorem actual_new_linear2 : parity point2 [123,126]=false := by decide +kernel
def point3 : List Nat := [180,182]
theorem actual_remaining_model3 : (sparseCheck program3 point3 residual3 && extra point3 zeros3 eqs3 inv3 (qs3++KoblitzRound14Boundary.pairs3) && (KoblitzRound14Boundary.linear3.all fun e => !(parity point3 e)) && (KoblitzRound14Boundary.expanded3.all fun e => !(multi point3 e)))=true := by decide +kernel
theorem cube_zero3 : ([19,30,41,52,63,18,29,40,51,62,95,0,17,18,19,20,21,22,23,71,73,74,76,77,79,80,82,83,85,86,88,89,91,92,94,99,101,102,104,105,107,108,110,111,113,114,116,117,119,120,122,379,381,382,384,385,387,388,390,391,393,394,396,397,399,400,402,407,408,410,411,413,414,416,417,419,420,422,423,425,426,428,123,183,243,303,429,485,541,597].all fun i => !(point3.contains i))=true := by decide +kernel
theorem nonzero3 : point3.isEmpty=false := by decide +kernel
theorem all_new_linear3 : ([[123,126],[123,126],[183,186],[243,246],[303,306],[429,431],[485,487],[541,543],[597,599]] : List (List Nat)).all (fun e=> !(parity point3 e))=true := by decide +kernel
theorem actual_new_linear3 : parity point3 [123,126]=false := by decide +kernel
end KoblitzRound21FinalBoundary
#print axioms KoblitzRound21FinalBoundary.actual_remaining_model0
#print axioms KoblitzRound21FinalBoundary.cube_zero0
#print axioms KoblitzRound21FinalBoundary.nonzero0
#print axioms KoblitzRound21FinalBoundary.actual_new_linear0
#print axioms KoblitzRound21FinalBoundary.all_new_linear0
#print axioms KoblitzRound21FinalBoundary.actual_remaining_model1
#print axioms KoblitzRound21FinalBoundary.cube_zero1
#print axioms KoblitzRound21FinalBoundary.nonzero1
#print axioms KoblitzRound21FinalBoundary.actual_new_linear1
#print axioms KoblitzRound21FinalBoundary.all_new_linear1
#print axioms KoblitzRound21FinalBoundary.actual_remaining_model2
#print axioms KoblitzRound21FinalBoundary.cube_zero2
#print axioms KoblitzRound21FinalBoundary.nonzero2
#print axioms KoblitzRound21FinalBoundary.actual_new_linear2
#print axioms KoblitzRound21FinalBoundary.all_new_linear2
#print axioms KoblitzRound21FinalBoundary.actual_remaining_model3
#print axioms KoblitzRound21FinalBoundary.cube_zero3
#print axioms KoblitzRound21FinalBoundary.nonzero3
#print axioms KoblitzRound21FinalBoundary.actual_new_linear3
#print axioms KoblitzRound21FinalBoundary.all_new_linear3
