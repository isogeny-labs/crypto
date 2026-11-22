import TateMillerRepresentative

namespace TateMillerEvaluationAudit
noncomputable section
open WeierstrassCurve.Affine MillerFinitePointAudit MillerRecursionAudit
open WeilRationalDomainAudit WeilAuxiliaryRatioAudit FinitePointEvaluationAudit
open MillerBasechangeAudit FunctionDivisorFinitenessAudit
private instance : Fact (Nat.Prime 19):=⟨by decide⟩
abbrev G:=BKGeneralFrobeniusAudit.SourceNineteen.generator

theorem pNonsingular : Curve.Nonsingular (12 : ZMod 19) 13 := by
  apply (equation_iff_nonsingular_of_Δ_ne_zero BKGeneralFrobeniusAudit.SourceNineteen.curve_nonsingular).mp
  rw [BKGeneralFrobeniusAudit.SourceNineteen.equation_iff]
  decide +kernel
theorem sNonsingular : Curve.Nonsingular (2 : ZMod 19) 4 := by
  apply (equation_iff_nonsingular_of_Δ_ne_zero BKGeneralFrobeniusAudit.SourceNineteen.curve_nonsingular).mp
  rw [BKGeneralFrobeniusAudit.SourceNineteen.equation_iff]
  decide +kernel
theorem aNonsingular : Curve.Nonsingular (3 : ZMod 19) 6 := by
  apply (equation_iff_nonsingular_of_Δ_ne_zero BKGeneralFrobeniusAudit.SourceNineteen.curve_nonsingular).mp
  rw [BKGeneralFrobeniusAudit.SourceNineteen.equation_iff]
  decide +kernel
def P : Curve.Point:=.some 12 13 pNonsingular
def S : Curve.Point:=.some 2 4 sNonsingular
def A : Curve.Point:=.some 3 6 aNonsingular

theorem actual_coordinates : P=(6 : ℕ) • G ∧ S=(3 : ℕ) • G ∧ S+G=A := by decide +kernel
theorem actual_P_three : (3 : ℕ) • P=0 := by
  rw [actual_coordinates.1,smul_smul]
  exact BKGeneralFrobeniusAudit.SourceNineteen.generator_controls.1
theorem actual_P_nonzero : P≠0 := by decide +kernel

abbrev f:=chosenFunctionUnit Curve P 3
def readout : ZMod 19:=(shortLine Curve 12 13 3 6/shortLine Curve 12 13 2 4)^6
def directReadout : ZMod 19:=(shortLine Curve 12 13 0 5)^6

theorem actual_expression_controls :
    shortLine Curve 12 13 2 4=7 ∧ shortLine Curve 12 13 3 6=15 ∧
    shortLine Curve 12 13 0 5=15 ∧ readout=11 ∧ directReadout=11 ∧
    readout^3=1 ∧ readout≠1 ∧ readout=directReadout := by
  dsimp only [readout,directReadout,shortLine,Curve,BKGeneralFrobeniusAudit.SourceNineteen.curve]
  decide +kernel

theorem source_unit_S : pointOrder Curve f S=0 :=
  actual_three_short_unit Curve BKGeneralFrobeniusAudit.SourceNineteen.curve_nonsingular
    rfl rfl rfl pNonsingular actual_P_three sNonsingular (by decide +kernel)
theorem source_unit_A : pointOrder Curve f A=0 :=
  actual_three_short_unit Curve BKGeneralFrobeniusAudit.SourceNineteen.curve_nonsingular
    rfl rfl rfl pNonsingular actual_P_three aNonsingular (by decide +kernel)

def sourceValue : (ZMod 19)ˣ:=
  evaluateDivisor Curve (shiftedDivisor Curve A S) f (shifted_disjoint Curve f A S source_unit_A source_unit_S)

theorem actual_source_readout : (sourceValue : ZMod 19)^6=readout := by
  have he : sourceValue=localUnitValue Curve A f source_unit_A/localUnitValue Curve S f source_unit_S:=
    shifted_evaluation Curve f A S source_unit_A source_unit_S
  have ha : (localUnitValue Curve A f source_unit_A : ZMod 19)=shortLine Curve 12 13 3 6:=
    actual_three_short_value Curve BKGeneralFrobeniusAudit.SourceNineteen.curve_nonsingular
      rfl rfl rfl pNonsingular actual_P_three aNonsingular source_unit_A
  have hb : (localUnitValue Curve S f source_unit_S : ZMod 19)=shortLine Curve 12 13 2 4:=
    actual_three_short_value Curve BKGeneralFrobeniusAudit.SourceNineteen.curve_nonsingular
      rfl rfl rfl pNonsingular actual_P_three sNonsingular source_unit_S
  calc
    (sourceValue : ZMod 19)^6=
        ((localUnitValue Curve A f source_unit_A : ZMod 19)/
          (localUnitValue Curve S f source_unit_S : ZMod 19))^6:=
      by simpa only [Units.val_div_eq_div_val] using
        congrArg (fun u : (ZMod 19)ˣ=>(u : ZMod 19)^6) he
    _=readout:=by rw [ha,hb];rfl

theorem chosen_direct_agrees : (sourceValue : ZMod 19)^6=directReadout :=
  actual_source_readout.trans actual_expression_controls.2.2.2.2.2.2.2

theorem actual_source_not_cube : ¬∃ u : (ZMod 19)ˣ,u^3=sourceValue := by
  rintro ⟨u,hu⟩
  have hv : (sourceValue : ZMod 19)^6=1 := by
    calc
      (sourceValue : ZMod 19)^6=(u : ZMod 19)^(3*6) := by
        rw [←hu,Units.val_pow_eq_pow_val,pow_mul]
      _=1 := by
        simpa only [ZMod.card] using FiniteField.pow_card_sub_one_eq_one (u : ZMod 19) u.ne_zero
  exact actual_expression_controls.2.2.2.2.2.2.1 (actual_source_readout.symm.trans hv)

theorem actual_geometric_source_support :
    Disjoint (shiftedDivisor (extendedCurve (L:=Closure) Curve)
        (pointBaseChange (L:=Closure) Curve A) (pointBaseChange (L:=Closure) Curve S)).support
      (functionDivisor (extendedCurve (L:=Closure) Curve) (extendedChosenUnit (L:=Closure) Curve P 3)
        (extendedChosenUnit (L:=Closure) Curve P 3).ne_zero).support := by
  classical
  exact actual_geometric_shifted_support Curve P A S 3 actual_P_three
    (by decide +kernel) (by decide +kernel) (by decide +kernel) (by decide +kernel)

#print axioms actual_coordinates
#print axioms actual_P_three
#print axioms actual_P_nonzero
#print axioms actual_expression_controls
#print axioms source_unit_S
#print axioms source_unit_A
#print axioms actual_source_readout
#print axioms chosen_direct_agrees
#print axioms actual_source_not_cube
#print axioms actual_geometric_source_support
end
end TateMillerEvaluationAudit
