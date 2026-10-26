import KoblitzRound16
import KoblitzRound14Boundary
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace KoblitzRound16Separators
def scalar0 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^2*v^1 + h^1*u^1*v^2
theorem actual_boundary_separation0 (v:ZMod 2) : scalar0 0 1 v=1 := by simp [scalar0]
theorem old_unit_cube0 : (KoblitzRound14Boundary.point0.contains 19)=true := by decide +kernel
theorem actual_Horner_structure0 : KoblitzRound12Boundary.program0[71]?=some (.mul 0 70) ∧ KoblitzRound12Boundary.program0[74]?=some (.mul 0 73) ∧ KoblitzRound12Boundary.program0[77]?=some (.mul 0 76) ∧ KoblitzRound12Boundary.program0[80]?=some (.mul 0 79) := by decide +kernel
def scalar1 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^2*v^1 + h^1*u^1*v^2 + h^2*u^1*v^1
theorem actual_boundary_separation1 (v:ZMod 2) : scalar1 0 1 v=1 := by simp [scalar1]
theorem old_unit_cube1 : (KoblitzRound14Boundary.point1.contains 19)=true := by decide +kernel
theorem actual_Horner_structure1 : KoblitzRound12Boundary.program1[71]?=some (.mul 0 70) ∧ KoblitzRound12Boundary.program1[74]?=some (.mul 0 73) ∧ KoblitzRound12Boundary.program1[77]?=some (.mul 0 76) ∧ KoblitzRound12Boundary.program1[80]?=some (.mul 0 79) := by decide +kernel
def scalar2 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^2*u^1*v^1
theorem actual_boundary_separation2 (v:ZMod 2) : scalar2 0 1 v=1 := by simp [scalar2]
theorem old_unit_cube2 : (KoblitzRound14Boundary.point2.contains 19)=true := by decide +kernel
theorem actual_Horner_structure2 : KoblitzRound12Boundary.program2[71]?=some (.mul 0 70) ∧ KoblitzRound12Boundary.program2[74]?=some (.mul 0 73) ∧ KoblitzRound12Boundary.program2[77]?=some (.mul 0 76) ∧ KoblitzRound12Boundary.program2[80]?=some (.mul 0 79) := by decide +kernel
def scalar3 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^2*u^2*v^0 + h^3*u^1*v^0 + h^1*u^2*v^1 + h^1*u^1*v^2
theorem actual_boundary_separation3 (v:ZMod 2) : scalar3 0 1 v=1 := by simp [scalar3]
theorem old_unit_cube3 : (KoblitzRound14Boundary.point3.contains 19)=true := by decide +kernel
theorem actual_Horner_structure3 : KoblitzRound12Boundary.program3[71]?=some (.mul 0 70) ∧ KoblitzRound12Boundary.program3[74]?=some (.mul 0 73) ∧ KoblitzRound12Boundary.program3[77]?=some (.mul 0 76) ∧ KoblitzRound12Boundary.program3[80]?=some (.mul 0 79) := by decide +kernel
end KoblitzRound16Separators
#print axioms KoblitzRound16Separators.actual_boundary_separation0
#print axioms KoblitzRound16Separators.old_unit_cube0
#print axioms KoblitzRound16Separators.actual_Horner_structure0
#print axioms KoblitzRound16Separators.actual_boundary_separation1
#print axioms KoblitzRound16Separators.old_unit_cube1
#print axioms KoblitzRound16Separators.actual_Horner_structure1
#print axioms KoblitzRound16Separators.actual_boundary_separation2
#print axioms KoblitzRound16Separators.old_unit_cube2
#print axioms KoblitzRound16Separators.actual_Horner_structure2
#print axioms KoblitzRound16Separators.actual_boundary_separation3
#print axioms KoblitzRound16Separators.old_unit_cube3
#print axioms KoblitzRound16Separators.actual_Horner_structure3
