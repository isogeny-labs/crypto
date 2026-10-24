import KoblitzRound19FinalBoundary
set_option maxHeartbeats 0
namespace KoblitzRound20Separators
def terms0 : List (Nat×Nat×Nat) := [(19,0,0),(17,0,2),(16,0,3),(15,0,4),(14,0,5),(13,0,6),(11,0,8),(10,0,9),(8,0,11),(6,0,13),(5,0,14),(4,0,15),(1,0,18),(0,0,19),(17,1,1),(16,1,2),(15,1,3),(14,1,4),(13,1,5),(12,1,6),(11,1,7),(8,1,10),(3,1,15),(2,1,16),(1,1,17),(0,1,18),(17,2,0),(12,2,5),(11,2,6),(9,2,8),(6,2,11),(0,2,17),(16,3,0),(15,3,1),(14,3,2),(13,3,3),(11,3,5),(8,3,8),(6,3,10),(5,3,11),(4,3,12),(3,3,13),(0,3,16),(11,4,4),(10,4,5),(9,4,6),(5,4,10),(4,4,11),(2,4,13),(1,4,14),(13,5,1),(12,5,2),(8,5,6),(6,5,8),(5,5,9),(4,5,10),(1,5,13),(11,6,2),(10,6,3),(8,6,5),(5,6,8),(2,6,11),(1,6,12),(10,7,2),(9,7,3),(8,7,4),(6,7,6),(5,7,7),(4,7,8),(2,7,10),(1,7,11),(3,8,8),(9,9,1),(8,9,2),(6,9,4),(5,9,5),(4,9,6),(2,9,8),(1,9,9),(8,10,1),(7,10,2),(6,11,2),(5,11,3),(1,11,7),(6,12,1),(5,12,2),(4,12,3),(1,12,6),(5,13,1),(4,13,2),(2,13,4),(1,13,5),(4,14,1),(3,14,2),(2,14,3),(2,15,2),(1,15,3),(3,16,0),(2,16,1),(0,16,3),(0,17,2),(0,18,1),(0,19,0)]
def boundary0 (U V:ZMod 2):ZMod 2 := (terms0.map fun (h,u,v)=> (0:ZMod 2)^h*U^u*V^v).sum
theorem actual_scalar_satisfaction0 : boundary0 1 1=0 := by decide +kernel
theorem actual_Q32_units0 : KoblitzRound19FinalBoundary.point0.contains 123=true ∧ KoblitzRound19FinalBoundary.point0.contains 126=true := by decide +kernel
theorem actual_denominator_gate0 : KoblitzRound12Boundary.program0[125]?=some (.inv 98) := by decide +kernel
theorem actual_top_shape0 : (terms0.filterMap fun (h,u,v)=> if h=0 then some (u,v) else none)=[(0,19),(1,18),(2,17),(3,16),(16,3),(17,2),(18,1),(19,0)] := by decide +kernel
def terms1 : List (Nat×Nat×Nat) := [(19,0,0),(18,0,1),(16,0,3),(15,0,4),(14,0,5),(13,0,6),(12,0,7),(11,0,8),(9,0,10),(8,0,11),(6,0,13),(4,0,15),(3,0,16),(2,0,17),(0,0,19),(18,1,0),(16,1,2),(15,1,3),(14,1,4),(8,1,10),(5,1,13),(3,1,15),(2,1,16),(1,1,17),(0,1,18),(15,2,2),(12,2,5),(9,2,8),(8,2,9),(7,2,10),(4,2,13),(3,2,14),(0,2,17),(15,3,1),(13,3,3),(12,3,4),(11,3,5),(10,3,6),(9,3,7),(4,3,12),(2,3,14),(0,3,16),(13,4,2),(12,4,3),(7,4,8),(6,4,9),(5,4,10),(3,4,12),(13,5,1),(7,5,7),(6,5,8),(5,5,9),(3,5,11),(2,5,12),(1,5,13),(12,6,1),(9,6,4),(8,6,5),(5,6,8),(4,6,9),(9,7,3),(7,7,5),(5,7,7),(4,7,8),(1,7,11),(8,8,3),(7,8,4),(3,8,8),(2,8,9),(8,9,2),(7,9,3),(4,9,6),(3,9,7),(2,9,8),(1,9,9),(7,10,2),(4,10,5),(2,10,7),(6,11,2),(5,11,3),(3,11,5),(2,11,6),(1,11,7),(6,12,1),(5,12,2),(4,12,3),(3,12,4),(2,12,5),(5,13,1),(3,13,3),(2,13,4),(4,14,1),(2,14,3),(3,15,1),(2,15,2),(3,16,0),(2,16,1),(1,16,2),(0,16,3),(0,17,2),(0,18,1),(0,19,0)]
def boundary1 (U V:ZMod 2):ZMod 2 := (terms1.map fun (h,u,v)=> (0:ZMod 2)^h*U^u*V^v).sum
theorem actual_scalar_satisfaction1 : boundary1 1 1=0 := by decide +kernel
theorem actual_Q32_units1 : KoblitzRound19FinalBoundary.point1.contains 123=true ∧ KoblitzRound19FinalBoundary.point1.contains 126=true := by decide +kernel
theorem actual_denominator_gate1 : KoblitzRound12Boundary.program1[125]?=some (.inv 98) := by decide +kernel
theorem actual_top_shape1 : (terms1.filterMap fun (h,u,v)=> if h=0 then some (u,v) else none)=[(0,19),(1,18),(2,17),(3,16),(16,3),(17,2),(18,1),(19,0)] := by decide +kernel
def terms2 : List (Nat×Nat×Nat) := [(16,0,3),(11,0,8),(10,0,9),(9,0,10),(7,0,12),(5,0,14),(4,0,15),(2,0,17),(0,0,19),(18,1,0),(17,1,1),(16,1,2),(15,1,3),(13,1,5),(9,1,9),(8,1,10),(7,1,11),(6,1,12),(1,1,17),(0,1,18),(16,2,1),(15,2,2),(14,2,3),(11,2,6),(10,2,7),(9,2,8),(7,2,10),(6,2,11),(5,2,12),(3,2,14),(2,2,15),(1,2,16),(0,2,17),(14,3,2),(11,3,5),(9,3,7),(7,3,9),(0,3,16),(13,4,2),(10,4,5),(9,4,6),(8,4,7),(4,4,11),(3,4,12),(2,4,13),(1,4,14),(13,5,1),(12,5,2),(10,5,4),(8,5,6),(6,5,8),(4,5,10),(2,5,12),(11,6,2),(9,6,4),(8,6,5),(7,6,6),(6,6,7),(5,6,8),(9,7,3),(8,7,4),(7,7,5),(6,7,6),(2,7,10),(1,7,11),(8,8,3),(3,8,8),(2,8,9),(1,8,10),(6,9,4),(5,9,5),(1,9,9),(8,10,1),(6,10,3),(2,10,7),(1,10,8),(7,11,1),(6,11,2),(5,11,3),(4,11,4),(3,11,5),(1,11,7),(6,12,1),(5,12,2),(4,12,3),(3,12,4),(2,12,5),(1,12,6),(5,13,1),(3,13,3),(1,13,5),(4,14,1),(2,14,3),(1,14,4),(2,15,2),(1,15,3),(1,16,2),(0,16,3),(0,17,2),(0,18,1),(0,19,0)]
def boundary2 (U V:ZMod 2):ZMod 2 := (terms2.map fun (h,u,v)=> (0:ZMod 2)^h*U^u*V^v).sum
theorem actual_scalar_satisfaction2 : boundary2 1 1=0 := by decide +kernel
theorem actual_Q32_units2 : KoblitzRound19FinalBoundary.point2.contains 123=true ∧ KoblitzRound19FinalBoundary.point2.contains 126=true := by decide +kernel
theorem actual_denominator_gate2 : KoblitzRound12Boundary.program2[125]?=some (.inv 98) := by decide +kernel
theorem actual_top_shape2 : (terms2.filterMap fun (h,u,v)=> if h=0 then some (u,v) else none)=[(0,19),(1,18),(2,17),(3,16),(16,3),(17,2),(18,1),(19,0)] := by decide +kernel
def terms3 : List (Nat×Nat×Nat) := [(15,0,4),(14,0,5),(13,0,6),(12,0,7),(7,0,12),(6,0,13),(5,0,14),(4,0,15),(0,0,19),(16,1,2),(15,1,3),(12,1,6),(11,1,7),(10,1,8),(7,1,11),(4,1,14),(0,1,18),(16,2,1),(14,2,3),(13,2,4),(12,2,5),(11,2,6),(10,2,7),(9,2,8),(8,2,9),(7,2,10),(3,2,14),(0,2,17),(16,3,0),(14,3,2),(13,3,3),(12,3,4),(11,3,5),(9,3,7),(8,3,8),(7,3,9),(5,3,11),(3,3,13),(2,3,14),(0,3,16),(13,4,2),(12,4,3),(10,4,5),(9,4,6),(5,4,10),(4,4,11),(1,4,14),(11,5,3),(10,5,4),(9,5,5),(4,5,10),(1,5,13),(10,6,3),(8,6,5),(7,6,6),(5,6,8),(3,6,10),(2,6,11),(1,6,12),(9,7,3),(6,7,6),(2,7,10),(1,7,11),(9,8,2),(8,8,3),(7,8,4),(4,8,7),(9,9,1),(7,9,3),(5,9,5),(2,9,8),(8,10,1),(5,10,4),(3,10,6),(1,10,8),(7,11,1),(6,11,2),(1,11,7),(6,12,1),(5,12,2),(5,13,1),(4,13,2),(3,13,3),(2,13,4),(2,14,3),(1,14,4),(2,15,2),(2,16,1),(1,16,2),(0,16,3),(0,17,2),(1,18,0),(0,18,1),(0,19,0)]
def boundary3 (U V:ZMod 2):ZMod 2 := (terms3.map fun (h,u,v)=> (0:ZMod 2)^h*U^u*V^v).sum
theorem actual_scalar_satisfaction3 : boundary3 1 1=0 := by decide +kernel
theorem actual_Q32_units3 : KoblitzRound19FinalBoundary.point3.contains 123=true ∧ KoblitzRound19FinalBoundary.point3.contains 126=true := by decide +kernel
theorem actual_denominator_gate3 : KoblitzRound12Boundary.program3[125]?=some (.inv 98) := by decide +kernel
theorem actual_top_shape3 : (terms3.filterMap fun (h,u,v)=> if h=0 then some (u,v) else none)=[(0,19),(1,18),(2,17),(3,16),(16,3),(17,2),(18,1),(19,0)] := by decide +kernel
end KoblitzRound20Separators
#print axioms KoblitzRound20Separators.actual_scalar_satisfaction0
#print axioms KoblitzRound20Separators.actual_top_shape0
#print axioms KoblitzRound20Separators.actual_Q32_units0
#print axioms KoblitzRound20Separators.actual_denominator_gate0
#print axioms KoblitzRound20Separators.actual_scalar_satisfaction1
#print axioms KoblitzRound20Separators.actual_top_shape1
#print axioms KoblitzRound20Separators.actual_Q32_units1
#print axioms KoblitzRound20Separators.actual_denominator_gate1
#print axioms KoblitzRound20Separators.actual_scalar_satisfaction2
#print axioms KoblitzRound20Separators.actual_top_shape2
#print axioms KoblitzRound20Separators.actual_Q32_units2
#print axioms KoblitzRound20Separators.actual_denominator_gate2
#print axioms KoblitzRound20Separators.actual_scalar_satisfaction3
#print axioms KoblitzRound20Separators.actual_top_shape3
#print axioms KoblitzRound20Separators.actual_Q32_units3
#print axioms KoblitzRound20Separators.actual_denominator_gate3
