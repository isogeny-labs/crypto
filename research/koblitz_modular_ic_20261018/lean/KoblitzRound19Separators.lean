import KoblitzRound18FinalBoundary
set_option maxHeartbeats 0
set_option maxRecDepth 100000
namespace KoblitzRound19Separators
def terms0 : List (Nat×Nat×Nat) := [(19,0,0),(17,0,2),(15,0,4),(14,0,5),(11,0,8),(10,0,9),(8,0,11),(6,0,13),(5,0,14),(3,0,16),(2,0,17),(1,0,18),(18,1,0),(17,1,1),(12,1,6),(11,1,7),(9,1,9),(8,1,10),(6,1,12),(1,1,17),(17,2,0),(13,2,4),(10,2,7),(6,2,11),(5,2,12),(4,2,13),(2,2,15),(15,3,1),(14,3,2),(12,3,4),(10,3,6),(9,3,7),(6,3,10),(4,3,12),(3,3,13),(1,3,15),(14,4,1),(12,4,3),(11,4,4),(8,4,7),(7,4,8),(5,4,10),(3,4,12),(14,5,0),(12,5,2),(11,5,3),(7,5,7),(6,5,8),(4,5,10),(2,5,12),(1,5,13),(13,6,0),(11,6,2),(10,6,3),(9,6,4),(7,6,6),(6,6,7),(5,6,8),(1,6,12),(12,7,0),(10,7,2),(5,7,7),(4,7,8),(3,7,9),(1,7,11),(9,8,2),(8,8,3),(7,8,4),(6,8,5),(4,8,7),(1,8,10),(9,9,1),(7,9,3),(6,9,4),(5,9,5),(3,9,7),(1,9,9),(8,10,1),(7,10,2),(5,10,4),(3,10,6),(7,11,1),(6,11,2),(5,11,3),(3,11,5),(2,11,6),(1,11,7),(6,12,1),(3,12,4),(1,12,6),(6,13,0),(5,13,1),(2,13,4),(1,13,5),(5,14,0),(4,14,1),(3,14,2),(1,14,4),(4,15,0),(3,15,1),(2,16,1),(2,17,0),(1,17,1),(1,18,0),(0,19,0)]
def separator0 (h U V:ZMod 2) : ZMod 2 := (terms0.map fun (a,b,c) => h^a*U^b*V^c).sum
theorem boundary0 (V:ZMod 2) : separator0 0 1 V=1 := by revert V; decide +kernel
theorem actual_Q28_unit0 : KoblitzRound18FinalBoundary.point0.contains 95=true := by decide +kernel
theorem actual_y_gate0 : KoblitzRound12Boundary.program0[123]?=some (.mul 1 122) := by decide +kernel
def terms1 : List (Nat×Nat×Nat) := [(18,0,1),(17,0,2),(16,0,3),(15,0,4),(10,0,9),(7,0,12),(6,0,13),(4,0,15),(2,0,17),(17,1,1),(15,1,3),(14,1,4),(11,1,7),(10,1,8),(4,1,14),(1,1,17),(17,2,0),(16,2,1),(15,2,2),(14,2,3),(11,2,6),(6,2,11),(5,2,12),(4,2,13),(3,2,14),(2,2,15),(1,2,16),(16,3,0),(14,3,2),(13,3,3),(12,3,4),(11,3,5),(10,3,6),(8,3,8),(6,3,10),(14,4,1),(8,4,7),(6,4,9),(2,4,13),(13,5,1),(11,5,3),(9,5,5),(5,5,9),(13,6,0),(10,6,3),(8,6,5),(5,6,8),(3,6,10),(1,6,12),(12,7,0),(11,7,1),(10,7,2),(9,7,3),(7,7,5),(5,7,7),(4,7,8),(2,7,10),(1,7,11),(9,8,2),(7,8,4),(5,8,6),(4,8,7),(3,8,8),(2,8,9),(1,8,10),(10,9,0),(8,9,2),(5,9,5),(2,9,8),(1,9,9),(8,10,1),(6,10,3),(5,10,4),(3,10,6),(2,10,7),(1,10,8),(5,11,3),(4,11,4),(1,11,7),(7,12,0),(5,12,2),(4,12,3),(3,12,4),(2,12,5),(1,12,6),(5,13,1),(4,13,2),(1,13,5),(4,14,1),(3,14,2),(2,14,3),(4,15,0),(3,15,1),(1,16,2),(2,17,0),(1,17,1),(1,18,0),(0,19,0)]
def separator1 (h U V:ZMod 2) : ZMod 2 := (terms1.map fun (a,b,c) => h^a*U^b*V^c).sum
theorem boundary1 (V:ZMod 2) : separator1 0 1 V=1 := by revert V; decide +kernel
theorem actual_Q28_unit1 : KoblitzRound18FinalBoundary.point1.contains 95=true := by decide +kernel
theorem actual_y_gate1 : KoblitzRound12Boundary.program1[123]?=some (.mul 1 122) := by decide +kernel
def terms2 : List (Nat×Nat×Nat) := [(19,0,0),(17,0,2),(16,0,3),(11,0,8),(10,0,9),(9,0,10),(7,0,12),(1,0,18),(17,1,1),(16,1,2),(13,1,5),(12,1,6),(11,1,7),(7,1,11),(2,1,16),(17,2,0),(16,2,1),(12,2,5),(8,2,9),(7,2,10),(5,2,12),(4,2,13),(1,2,16),(16,3,0),(14,3,2),(13,3,3),(12,3,4),(11,3,5),(4,3,12),(3,3,13),(2,3,14),(1,3,15),(8,4,7),(7,4,8),(6,4,9),(5,4,10),(2,4,13),(1,4,14),(14,5,0),(13,5,1),(10,5,4),(8,5,6),(7,5,7),(5,5,9),(3,5,11),(13,6,0),(6,6,7),(3,6,10),(1,6,12),(10,7,2),(9,7,3),(8,7,4),(6,7,6),(4,7,8),(1,7,11),(10,8,1),(8,8,3),(6,8,5),(4,8,7),(3,8,8),(1,8,10),(8,9,2),(4,9,6),(2,9,8),(9,10,0),(6,10,3),(4,10,5),(2,10,7),(1,10,8),(5,11,3),(2,11,6),(1,11,7),(7,12,0),(6,12,1),(3,12,4),(1,12,6),(5,13,1),(4,13,2),(5,14,0),(3,14,2),(1,14,4),(4,15,0),(2,15,2),(2,16,1),(1,17,1),(0,19,0)]
def separator2 (h U V:ZMod 2) : ZMod 2 := (terms2.map fun (a,b,c) => h^a*U^b*V^c).sum
theorem boundary2 (V:ZMod 2) : separator2 0 1 V=1 := by revert V; decide +kernel
theorem actual_Q28_unit2 : KoblitzRound18FinalBoundary.point2.contains 95=true := by decide +kernel
theorem actual_y_gate2 : KoblitzRound12Boundary.program2[123]?=some (.mul 1 122) := by decide +kernel
def terms3 : List (Nat×Nat×Nat) := [(19,0,0),(18,0,1),(16,0,3),(14,0,5),(13,0,6),(12,0,7),(11,0,8),(5,0,14),(4,0,15),(2,0,17),(1,0,18),(17,1,1),(13,1,5),(12,1,6),(9,1,9),(7,1,11),(2,1,16),(1,1,17),(17,2,0),(16,2,1),(14,2,3),(12,2,5),(11,2,6),(10,2,7),(9,2,8),(8,2,9),(7,2,10),(2,2,15),(16,3,0),(12,3,4),(11,3,5),(10,3,6),(7,3,9),(5,3,11),(4,3,12),(3,3,13),(15,4,0),(13,4,2),(12,4,3),(9,4,6),(6,4,9),(5,4,10),(3,4,12),(2,4,13),(1,4,14),(14,5,0),(13,5,1),(11,5,3),(10,5,4),(9,5,5),(8,5,6),(7,5,7),(13,6,0),(10,6,3),(8,6,5),(7,6,6),(6,6,7),(5,6,8),(11,7,1),(9,7,3),(1,7,11),(11,8,0),(9,8,2),(8,8,3),(6,8,5),(5,8,6),(4,8,7),(3,8,8),(2,8,9),(1,8,10),(8,9,2),(7,9,3),(6,9,4),(3,9,7),(2,9,8),(7,10,2),(4,10,5),(3,10,6),(8,11,0),(7,11,1),(2,11,6),(1,11,7),(6,12,1),(3,12,4),(2,12,5),(1,12,6),(6,13,0),(5,13,1),(4,13,2),(3,13,3),(1,13,5),(3,14,2),(2,14,3),(1,14,4),(4,15,0),(3,16,0),(2,16,1),(1,18,0),(0,19,0)]
def separator3 (h U V:ZMod 2) : ZMod 2 := (terms3.map fun (a,b,c) => h^a*U^b*V^c).sum
theorem boundary3 (V:ZMod 2) : separator3 0 1 V=1 := by revert V; decide +kernel
theorem actual_Q28_unit3 : KoblitzRound18FinalBoundary.point3.contains 95=true := by decide +kernel
theorem actual_y_gate3 : KoblitzRound12Boundary.program3[123]?=some (.mul 1 122) := by decide +kernel
end KoblitzRound19Separators
#print axioms KoblitzRound19Separators.boundary0
#print axioms KoblitzRound19Separators.actual_Q28_unit0
#print axioms KoblitzRound19Separators.actual_y_gate0
#print axioms KoblitzRound19Separators.boundary1
#print axioms KoblitzRound19Separators.actual_Q28_unit1
#print axioms KoblitzRound19Separators.actual_y_gate1
#print axioms KoblitzRound19Separators.boundary2
#print axioms KoblitzRound19Separators.actual_Q28_unit2
#print axioms KoblitzRound19Separators.actual_y_gate2
#print axioms KoblitzRound19Separators.boundary3
#print axioms KoblitzRound19Separators.actual_Q28_unit3
#print axioms KoblitzRound19Separators.actual_y_gate3
