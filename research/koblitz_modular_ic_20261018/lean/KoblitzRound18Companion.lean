import KoblitzRound17Boundary
set_option maxRecDepth 100000
set_option maxHeartbeats 0
namespace KoblitzRound18Companion
def sep0_4 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^2*u^2*v^0 + h^3*u^1*v^0 + h^1*u^2*v^1 + h^1*u^1*v^2 + h^1*u^0*v^3
theorem sep0_4_boundary (v:ZMod 2) : sep0_4 0 1 v=1 := by simp [sep0_4]
theorem actual_Q24_unit0 : (KoblitzRound17Boundary.point0.contains 64)=true := by decide +kernel
theorem actual_quartic_gate0 : KoblitzRound12Boundary.program0[320]?=some (.mul 8 319) := by decide +kernel
def sep1_4 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^1*u^3*v^0 + h^2*u^2*v^0 + h^3*u^1*v^0 + h^1*u^2*v^1 + h^1*u^1*v^2 + h^1*u^0*v^3
theorem sep1_4_boundary (v:ZMod 2) : sep1_4 0 1 v=1 := by simp [sep1_4]
theorem actual_Q24_unit1 : (KoblitzRound17Boundary.point1.contains 64)=true := by decide +kernel
theorem actual_quartic_gate1 : KoblitzRound12Boundary.program1[320]?=some (.mul 8 319) := by decide +kernel
def sep2_4 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^2*u^2*v^0 + h^3*u^1*v^0 + h^1*u^0*v^3
theorem sep2_4_boundary (v:ZMod 2) : sep2_4 0 1 v=1 := by simp [sep2_4]
theorem actual_Q24_unit2 : (KoblitzRound17Boundary.point2.contains 63)=true := by decide +kernel
theorem actual_quartic_gate2 : KoblitzRound12Boundary.program2[320]?=some (.mul 8 319) := by decide +kernel
def sep3_4 (h u v:ZMod 2) : ZMod 2 := h^0*u^4*v^0 + h^2*u^2*v^0 + h^3*u^1*v^0 + h^1*u^0*v^3
theorem sep3_4_boundary (v:ZMod 2) : sep3_4 0 1 v=1 := by simp [sep3_4]
theorem actual_Q24_unit3 : (KoblitzRound17Boundary.point3.contains 63)=true := by decide +kernel
theorem actual_quartic_gate3 : KoblitzRound12Boundary.program3[320]?=some (.mul 8 319) := by decide +kernel
end KoblitzRound18Companion
#print axioms KoblitzRound18Companion.sep0_4_boundary
#print axioms KoblitzRound18Companion.actual_Q24_unit0
#print axioms KoblitzRound18Companion.actual_quartic_gate0
#print axioms KoblitzRound18Companion.sep1_4_boundary
#print axioms KoblitzRound18Companion.actual_Q24_unit1
#print axioms KoblitzRound18Companion.actual_quartic_gate1
#print axioms KoblitzRound18Companion.sep2_4_boundary
#print axioms KoblitzRound18Companion.actual_Q24_unit2
#print axioms KoblitzRound18Companion.actual_quartic_gate2
#print axioms KoblitzRound18Companion.sep3_4_boundary
#print axioms KoblitzRound18Companion.actual_Q24_unit3
#print axioms KoblitzRound18Companion.actual_quartic_gate3
