import KoblitzRound16Boundary
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace KoblitzRound17Survivors
def sep0_0 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^2*v^1 + h^1*u^1*v^2
theorem sep0_0_boundary (v:ZMod 2) : sep0_0 0 1 v=1 := by simp [sep0_0]
def sep0_1 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^2*u^2*v^0 + h^1*u^2*v^1 + h^1*u^1*v^2 + h^1*u^0*v^3
theorem sep0_1_boundary (v:ZMod 2) : sep0_1 0 1 v=1 := by simp [sep0_1]
theorem actual_Q22_unit0 : (KoblitzRound16Boundary.point0.contains 31)=true := by decide +kernel
theorem actual_quartic_gate0 : KoblitzRound12Boundary.program0[140]?=some (.mul 2 139) := by decide +kernel
def sep0_2 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^2*u^2*v^0 + h^1*u^1*v^2
theorem sep0_2_boundary (v:ZMod 2) : sep0_2 0 1 v=1 := by simp [sep0_2]
def sep0_3 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^2*v^1 + h^1*u^1*v^2 + h^2*u^1*v^1
theorem sep0_3_boundary (v:ZMod 2) : sep0_3 0 1 v=1 := by simp [sep0_3]
def sep1_0 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^2*v^1 + h^1*u^1*v^2 + h^2*u^1*v^1
theorem sep1_0_boundary (v:ZMod 2) : sep1_0 0 1 v=1 := by simp [sep1_0]
def sep1_1 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^2*v^1 + h^2*u^1*v^1
theorem sep1_1_boundary (v:ZMod 2) : sep1_1 0 1 v=1 := by simp [sep1_1]
theorem actual_Q22_unit1 : (KoblitzRound16Boundary.point1.contains 31)=true := by decide +kernel
theorem actual_quartic_gate1 : KoblitzRound12Boundary.program1[140]?=some (.mul 2 139) := by decide +kernel
def sep1_2 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^3*u^1*v^0 + h^2*u^1*v^1
theorem sep1_2_boundary (v:ZMod 2) : sep1_2 0 1 v=1 := by simp [sep1_2]
def sep1_3 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^2*u^2*v^0 + h^3*u^1*v^0 + h^1*u^2*v^1 + h^1*u^1*v^2 + h^1*u^0*v^3
theorem sep1_3_boundary (v:ZMod 2) : sep1_3 0 1 v=1 := by simp [sep1_3]
def sep2_0 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^2*u^1*v^1
theorem sep2_0_boundary (v:ZMod 2) : sep2_0 0 1 v=1 := by simp [sep2_0]
def sep2_1 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^2*u^2*v^0 + h^3*u^1*v^0 + h^1*u^0*v^3 + h^2*u^1*v^1
theorem sep2_1_boundary (v:ZMod 2) : sep2_1 0 1 v=1 := by simp [sep2_1]
theorem actual_Q22_unit2 : (KoblitzRound16Boundary.point2.contains 30)=true := by decide +kernel
theorem actual_quartic_gate2 : KoblitzRound12Boundary.program2[140]?=some (.mul 2 139) := by decide +kernel
def sep2_2 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^2*u^2*v^0 + h^3*u^1*v^0 + h^2*u^1*v^1
theorem sep2_2_boundary (v:ZMod 2) : sep2_2 0 1 v=1 := by simp [sep2_2]
def sep2_3 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^2*u^2*v^0 + h^1*u^2*v^1 + h^1*u^0*v^3 + h^2*u^1*v^1
theorem sep2_3_boundary (v:ZMod 2) : sep2_3 0 1 v=1 := by simp [sep2_3]
def sep3_0 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^2*u^2*v^0 + h^3*u^1*v^0 + h^1*u^2*v^1 + h^1*u^1*v^2
theorem sep3_0_boundary (v:ZMod 2) : sep3_0 0 1 v=1 := by simp [sep3_0]
def sep3_1 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^2*u^2*v^0 + h^1*u^1*v^2 + h^2*u^1*v^1
theorem sep3_1_boundary (v:ZMod 2) : sep3_1 0 1 v=1 := by simp [sep3_1]
theorem actual_Q22_unit3 : (KoblitzRound16Boundary.point3.contains 30)=true := by decide +kernel
theorem actual_quartic_gate3 : KoblitzRound12Boundary.program3[140]?=some (.mul 2 139) := by decide +kernel
def sep3_2 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^2*u^2*v^0 + h^2*u^1*v^1
theorem sep3_2_boundary (v:ZMod 2) : sep3_2 0 1 v=1 := by simp [sep3_2]
def sep3_3 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^3*u^1*v^0 + h^1*u^2*v^1 + h^1*u^1*v^2
theorem sep3_3_boundary (v:ZMod 2) : sep3_3 0 1 v=1 := by simp [sep3_3]
end KoblitzRound17Survivors
#print axioms KoblitzRound17Survivors.sep0_0_boundary
#print axioms KoblitzRound17Survivors.sep0_1_boundary
#print axioms KoblitzRound17Survivors.sep0_2_boundary
#print axioms KoblitzRound17Survivors.sep0_3_boundary
#print axioms KoblitzRound17Survivors.actual_Q22_unit0
#print axioms KoblitzRound17Survivors.actual_quartic_gate0
#print axioms KoblitzRound17Survivors.sep1_0_boundary
#print axioms KoblitzRound17Survivors.sep1_1_boundary
#print axioms KoblitzRound17Survivors.sep1_2_boundary
#print axioms KoblitzRound17Survivors.sep1_3_boundary
#print axioms KoblitzRound17Survivors.actual_Q22_unit1
#print axioms KoblitzRound17Survivors.actual_quartic_gate1
#print axioms KoblitzRound17Survivors.sep2_0_boundary
#print axioms KoblitzRound17Survivors.sep2_1_boundary
#print axioms KoblitzRound17Survivors.sep2_2_boundary
#print axioms KoblitzRound17Survivors.sep2_3_boundary
#print axioms KoblitzRound17Survivors.actual_Q22_unit2
#print axioms KoblitzRound17Survivors.actual_quartic_gate2
#print axioms KoblitzRound17Survivors.sep3_0_boundary
#print axioms KoblitzRound17Survivors.sep3_1_boundary
#print axioms KoblitzRound17Survivors.sep3_2_boundary
#print axioms KoblitzRound17Survivors.sep3_3_boundary
#print axioms KoblitzRound17Survivors.actual_Q22_unit3
#print axioms KoblitzRound17Survivors.actual_quartic_gate3
