import CubicTwistCount
import PrimeFieldTraceCertificate

namespace QuadraticTwistCountAudit
noncomputable section
open WeierstrassCurve.Affine SmallPrimeTraceGeometryAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance : Fact (Nat.Prime 7) := ⟨by decide⟩
abbrev F7 := ZMod 7

def sourceCurve : WeierstrassCurve.Affine F7 := cubicModel 0 1 0
def scaledCurve : WeierstrassCurve.Affine F7 := twistModel 0 1 0 3
theorem source_delta : sourceCurve.Δ≠0 := by decide +kernel
theorem scaled_delta : scaledCurve.Δ≠0 := by decide +kernel

instance sourceElliptic : sourceCurve.IsElliptic :=
  SmallCharacteristicJAudit.ellipticOfNonzero _ source_delta
instance scaledElliptic : scaledCurve.IsElliptic :=
  SmallCharacteristicJAudit.ellipticOfNonzero _ scaled_delta

theorem three_nonsquare : ¬IsSquare (3:F7) := by
  change ¬∃ y : F7,(3:F7)=y*y
  decide +kernel

def specialChange : WeierstrassCurve.VariableChange F7 := ⟨Units.mk0 4 (by decide),0,0,0⟩

theorem special_change : specialChange • sourceCurve=scaledCurve := by
  ext <;> decide +kernel

theorem source_j : sourceCurve.j=(1728:F7) := by
  rw [WeierstrassCurve.j_eq]
  decide +kernel

/-- A direct literal negation in the actual admissible-variable-change model. -/
theorem literal_nonsquare_no_change_false :
    ¬∀ a b d : F7,(cubicModel 0 a b).Δ≠0 → d≠0 → ¬IsSquare d →
      ¬∃ C : WeierstrassCurve.VariableChange F7,C • cubicModel 0 a b=twistModel 0 a b d := by
  intro h
  exact h 1 0 3 source_delta (by decide) three_nonsquare ⟨specialChange,special_change⟩

def pointEquationEquiv (W : WeierstrassCurve.Affine F7) (hΔ : W.Δ≠0) :
    W.Point ≃ Option {xy : F7×F7 // equation W xy} where
  toFun
    | .zero => none
    | .some x y h => some ⟨(x,y),(nonsingular_iff_equation W hΔ (x,y)).mp h⟩
  invFun
    | none => .zero
    | some p => .some p.val.1 p.val.2 ((nonsingular_iff_equation W hΔ p.val).mpr p.property)
  left_inv := by intro P; cases P <;> rfl
  right_inv := by
    intro P
    cases P with
    | none => rfl
    | some p => rcases p with ⟨⟨x,y⟩,h⟩; rfl

instance sourcePointFintype : Fintype sourceCurve.Point :=
  Fintype.ofEquiv (Option {xy : F7×F7 // equation sourceCurve xy})
    (pointEquationEquiv sourceCurve source_delta).symm
instance scaledPointFintype : Fintype scaledCurve.Point :=
  Fintype.ofEquiv (Option {xy : F7×F7 // equation scaledCurve xy})
    (pointEquationEquiv scaledCurve scaled_delta).symm

theorem scaling_equation : ∀ x y : F7,equation sourceCurve (x,y) → equation scaledCurve (4*x,y) := by
  decide +kernel
theorem rescaling_equation : ∀ x y : F7,equation scaledCurve (x,y) → equation sourceCurve (2*x,y) := by
  decide +kernel

def forwardPoint : sourceCurve.Point → scaledCurve.Point
  | .zero => .zero
  | .some x y h => .some (4*x) y ((nonsingular_iff_equation scaledCurve scaled_delta (4*x,y)).mpr
      (scaling_equation x y ((nonsingular_iff_equation sourceCurve source_delta (x,y)).mp h)))

def backwardPoint : scaledCurve.Point → sourceCurve.Point
  | .zero => .zero
  | .some x y h => .some (2*x) y ((nonsingular_iff_equation sourceCurve source_delta (2*x,y)).mpr
      (rescaling_equation x y ((nonsingular_iff_equation scaledCurve scaled_delta (x,y)).mp h)))

theorem forward_left_inverse : Function.LeftInverse backwardPoint forwardPoint := by
  intro P
  cases P with
  | zero => rfl
  | some x y h =>
    simp only [forwardPoint,backwardPoint,Point.some.injEq]
    constructor
    · rw [←mul_assoc,show (2:F7)*4=1 by decide +kernel,one_mul]
    · exact True.intro

theorem forward_right_inverse : Function.RightInverse backwardPoint forwardPoint := by
  intro P
  cases P with
  | zero => rfl
  | some x y h =>
    simp only [forwardPoint,backwardPoint,Point.some.injEq]
    constructor
    · rw [←mul_assoc,show (4:F7)*2=1 by decide +kernel,one_mul]
    · exact True.intro

theorem forward_add : ∀ P Q : sourceCurve.Point,
    forwardPoint (P+Q)=forwardPoint P+forwardPoint Q := by
  decide +kernel

def specialAddEquiv : sourceCurve.Point ≃+ scaledCurve.Point where
  toFun := forwardPoint
  invFun := backwardPoint
  left_inv := forward_left_inverse
  right_inv := forward_right_inverse
  map_add' := forward_add

theorem source_count : Nat.card sourceCurve.Point=8 :=
  PrimeFieldTraceAudit.eight_actual_control.2.2.1

theorem scaled_count : Nat.card scaledCurve.Point=8 :=
  (Nat.card_congr specialAddEquiv.toEquiv).symm.trans source_count

theorem two_nonsquare_five : ¬IsSquare (2:ZMod 5) := by
  change ¬∃ y : ZMod 5,(2:ZMod 5)=y*y
  decide +kernel

theorem deleting_nonsquare_false :
    ¬∀ a b c d : ZMod 3,(cubicModel a b c).Δ≠0 → d≠0 →
      Nat.card (cubicModel a b c).Point+Nat.card (twistModel a b c d).Point=
        2*(Fintype.card (ZMod 3)+1) := by
  intro h
  have hd : (cubicModel (2:ZMod 3) 0 1).Δ≠0 := by decide +kernel
  have hc : Nat.card (cubicModel (2:ZMod 3) 0 1).Point=5 :=
    SmallCharacteristicJAudit.finite_control_point_card
  have hh := h 2 0 1 1 hd one_ne_zero
  rw [twist_one,hc] at hh
  norm_num only [ZMod.card] at hh

theorem deleting_nonsingularity_false :
    ¬∀ a b c d : ZMod 5,¬IsSquare d →
      Nat.card (cubicModel a b c).Point+Nat.card (twistModel a b c d).Point=
        2*(Fintype.card (ZMod 5)+1) := by
  intro h
  have hc : Nat.card (cubicModel (0:ZMod 5) 0 0).Point=5 :=
    PrimeFieldTraceAudit.singular_point_count
  have hh := h 0 0 0 2 two_nonsquare_five
  rw [twist_zero_coefficients,hc] at hh
  norm_num only [ZMod.card] at hh

/-- Homogeneous inverse coordinate maps over every scalar ring of
characteristic seven; these are literal polynomial identities. -/
theorem universal_projective_identity {R : Type*} [CommRing R] [CharP R 7] (x y z : R) :
    2*(4*x)=x ∧ 4*(2*x)=x ∧
    y^2*z-(4*x)^3-2*(4*x)*z^2=y^2*z-x^3-x*z^2 ∧
    y^2*z-(2*x)^3-(2*x)*z^2=y^2*z-x^3-2*x*z^2 := by
  have h7 : (7:R)=0 := CharP.cast_eq_zero R 7
  refine ⟨?_,?_,?_,?_⟩
  · linear_combination x*h7
  · linear_combination x*h7
  · linear_combination -(9*x^3+x*z^2)*h7
  · linear_combination -(x^3)*h7

#print axioms literal_nonsquare_no_change_false
#print axioms specialAddEquiv
#print axioms scaled_count
#print axioms deleting_nonsquare_false
#print axioms deleting_nonsingularity_false
end
end QuadraticTwistCountAudit
