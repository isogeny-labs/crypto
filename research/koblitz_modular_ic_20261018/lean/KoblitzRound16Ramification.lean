import KoblitzRound15Jets
namespace KoblitzRound16Ramification
theorem ramification_six (e x y:ℤ) (hy:2*y=e) (hx:3*x=2*e) : ∃ k:ℤ,e=6*k ∧ x=4*k ∧ y=3*k := by
 refine ⟨e/6,?_,?_,?_⟩ <;> omega
theorem no_low_ramification (e x y:ℤ) (he:0<e) (he6:e<6) (hy:2*y=e) (hx:3*x=2*e) : False := by omega
theorem actual_gates0 : KoblitzRound12Boundary.program0[12]?=some (.mul 1 1) ∧ KoblitzRound12Boundary.program0[16]?=some (.mul 15 15) ∧ KoblitzRound12Boundary.program0[17]?=some (.mul 0 16) ∧ KoblitzRound12Boundary.program0[18]?=some (.mul 17 17) ∧ KoblitzRound12Boundary.program0[19]?=some (.mul 0 18) := by decide +kernel
theorem actual_units0 : (KoblitzRound14Boundary.point0.contains 12)=true ∧ (KoblitzRound14Boundary.point0.contains 19)=true := by decide +kernel
theorem actual_gates1 : KoblitzRound12Boundary.program1[12]?=some (.mul 1 1) ∧ KoblitzRound12Boundary.program1[16]?=some (.mul 15 15) ∧ KoblitzRound12Boundary.program1[17]?=some (.mul 0 16) ∧ KoblitzRound12Boundary.program1[18]?=some (.mul 17 17) ∧ KoblitzRound12Boundary.program1[19]?=some (.mul 0 18) := by decide +kernel
theorem actual_units1 : (KoblitzRound14Boundary.point1.contains 12)=true ∧ (KoblitzRound14Boundary.point1.contains 19)=true := by decide +kernel
theorem actual_gates2 : KoblitzRound12Boundary.program2[12]?=some (.mul 1 1) ∧ KoblitzRound12Boundary.program2[16]?=some (.mul 15 15) ∧ KoblitzRound12Boundary.program2[17]?=some (.mul 0 16) ∧ KoblitzRound12Boundary.program2[18]?=some (.mul 17 17) ∧ KoblitzRound12Boundary.program2[19]?=some (.mul 0 18) := by decide +kernel
theorem actual_units2 : (KoblitzRound14Boundary.point2.contains 12)=true ∧ (KoblitzRound14Boundary.point2.contains 19)=true := by decide +kernel
theorem actual_gates3 : KoblitzRound12Boundary.program3[12]?=some (.mul 1 1) ∧ KoblitzRound12Boundary.program3[16]?=some (.mul 15 15) ∧ KoblitzRound12Boundary.program3[17]?=some (.mul 0 16) ∧ KoblitzRound12Boundary.program3[18]?=some (.mul 17 17) ∧ KoblitzRound12Boundary.program3[19]?=some (.mul 0 18) := by decide +kernel
theorem actual_units3 : (KoblitzRound14Boundary.point3.contains 12)=true ∧ (KoblitzRound14Boundary.point3.contains 19)=true := by decide +kernel
end KoblitzRound16Ramification
#print axioms KoblitzRound16Ramification.ramification_six
#print axioms KoblitzRound16Ramification.no_low_ramification
#print axioms KoblitzRound16Ramification.actual_gates0
#print axioms KoblitzRound16Ramification.actual_units0
#print axioms KoblitzRound16Ramification.actual_gates1
#print axioms KoblitzRound16Ramification.actual_units1
#print axioms KoblitzRound16Ramification.actual_gates2
#print axioms KoblitzRound16Ramification.actual_units2
#print axioms KoblitzRound16Ramification.actual_gates3
#print axioms KoblitzRound16Ramification.actual_units3
