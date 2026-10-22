import KoblitzRound16Separators
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace KoblitzRound16Boundary
open KoblitzRound12Boundary KoblitzRound13Boundary KoblitzRound13SquareData KoblitzRound14Boundary
def point0 : List Nat := [26,28,31,34,35]
theorem actual_remaining_model0 : (sparseCheck program0 point0 residual0 && extra point0 zeros0 eqs0 inv0 (qs0++KoblitzRound14Boundary.pairs0) && (KoblitzRound14Boundary.linear0.all fun e => !(parity point0 e)) && (KoblitzRound14Boundary.expanded0.all fun e => !(multi point0 e)))=true := by decide +kernel
theorem cube_zero0 : (point0.contains 19)=false := by decide +kernel
theorem nonzero0 : point0.isEmpty=false := by decide +kernel
def point1 : List Nat := [26,28,31,34,35]
theorem actual_remaining_model1 : (sparseCheck program1 point1 residual1 && extra point1 zeros1 eqs1 inv1 (qs1++KoblitzRound14Boundary.pairs1) && (KoblitzRound14Boundary.linear1.all fun e => !(parity point1 e)) && (KoblitzRound14Boundary.expanded1.all fun e => !(multi point1 e)))=true := by decide +kernel
theorem cube_zero1 : (point1.contains 19)=false := by decide +kernel
theorem nonzero1 : point1.isEmpty=false := by decide +kernel
def point2 : List Nat := [25,27,30,33,34]
theorem actual_remaining_model2 : (sparseCheck program2 point2 residual2 && extra point2 zeros2 eqs2 inv2 (qs2++KoblitzRound14Boundary.pairs2) && (KoblitzRound14Boundary.linear2.all fun e => !(parity point2 e)) && (KoblitzRound14Boundary.expanded2.all fun e => !(multi point2 e)))=true := by decide +kernel
theorem cube_zero2 : (point2.contains 19)=false := by decide +kernel
theorem nonzero2 : point2.isEmpty=false := by decide +kernel
def point3 : List Nat := [25,27,30,33,34]
theorem actual_remaining_model3 : (sparseCheck program3 point3 residual3 && extra point3 zeros3 eqs3 inv3 (qs3++KoblitzRound14Boundary.pairs3) && (KoblitzRound14Boundary.linear3.all fun e => !(parity point3 e)) && (KoblitzRound14Boundary.expanded3.all fun e => !(multi point3 e)))=true := by decide +kernel
theorem cube_zero3 : (point3.contains 19)=false := by decide +kernel
theorem nonzero3 : point3.isEmpty=false := by decide +kernel
end KoblitzRound16Boundary
#print axioms KoblitzRound16Boundary.actual_remaining_model0
#print axioms KoblitzRound16Boundary.cube_zero0
#print axioms KoblitzRound16Boundary.nonzero0
#print axioms KoblitzRound16Boundary.actual_remaining_model1
#print axioms KoblitzRound16Boundary.cube_zero1
#print axioms KoblitzRound16Boundary.nonzero1
#print axioms KoblitzRound16Boundary.actual_remaining_model2
#print axioms KoblitzRound16Boundary.cube_zero2
#print axioms KoblitzRound16Boundary.nonzero2
#print axioms KoblitzRound16Boundary.actual_remaining_model3
#print axioms KoblitzRound16Boundary.cube_zero3
#print axioms KoblitzRound16Boundary.nonzero3
