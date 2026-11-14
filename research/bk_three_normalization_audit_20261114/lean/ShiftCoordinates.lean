import ThreeCharacterFrobenius

namespace BKThreeNormalizationAudit
noncomputable section
variable {R : Type*} [CommRing R]

def shifted (W : WeierstrassCurve.Affine R) (r s t : R) : WeierstrassCurve.Affine R :=
  ⟨W.a₁+2*s,W.a₂-s*W.a₁+3*r-s^2,W.a₃+r*W.a₁+2*t,
    W.a₄-s*W.a₃+2*r*W.a₂-(t+r*s)*W.a₁+3*r^2-2*s*t,
    W.a₆+r*W.a₄+r^2*W.a₂+r^3-t*W.a₃-t^2-r*t*W.a₁⟩

theorem shifted_eq_variableChange (W : WeierstrassCurve.Affine R) (r s t : R) :
    shifted W r s t=WeierstrassCurve.VariableChange.mk 1 r s t • W := by
  simp only [WeierstrassCurve.variableChange_def,inv_one,Units.val_one,one_pow,one_mul,shifted]

theorem shifted_discriminant (W : WeierstrassCurve.Affine R) (r s t : R) :
    (shifted W r s t).Δ=W.Δ := by
  rw [shifted_eq_variableChange,WeierstrassCurve.variableChange_Δ]
  simp

theorem shifted_inverse (W : WeierstrassCurve.Affine R) (r s t : R) :
    shifted (shifted W r s t) (-r) (-s) (r*s-t)=W := by
  ext <;> simp only [shifted] <;> ring

theorem equation_identity (W : WeierstrassCurve.Affine R) (r s t x y : R) :
    W.polynomial.evalEval (x+r) (y+s*x+t)=(shifted W r s t).polynomial.evalEval x y := by
  simp only [WeierstrassCurve.Affine.evalEval_polynomial,shifted]
  ring

theorem shifted_equation (W : WeierstrassCurve.Affine R) (r s t x y : R) :
    (shifted W r s t).Equation x y ↔ W.Equation (x+r) (y+s*x+t) := by
  simp only [WeierstrassCurve.Affine.Equation,equation_identity]

theorem negY_identity (W : WeierstrassCurve.Affine R) (r s t x y : R) :
    W.negY (x+r) (y+s*x+t)=(shifted W r s t).negY x y+s*x+t := by
  simp only [WeierstrassCurve.Affine.negY,shifted]
  ring

theorem numerator_identity (W : WeierstrassCurve.Affine R) (r s t x y : R) :
    3*(x+r)^2+2*W.a₂*(x+r)+W.a₄-W.a₁*(y+s*x+t)=
      (3*x^2+2*(shifted W r s t).a₂*x+(shifted W r s t).a₄-(shifted W r s t).a₁*y)+
      s*(2*y+(shifted W r s t).a₁*x+(shifted W r s t).a₃) := by
  simp only [shifted]
  ring

theorem denominator_identity (W : WeierstrassCurve.Affine R) (r s t x y : R) :
    2*(y+s*x+t)+W.a₁*(x+r)+W.a₃=2*y+(shifted W r s t).a₁*x+(shifted W r s t).a₃ := by
  simp only [shifted]
  ring

theorem addX_identity (W : WeierstrassCurve.Affine R) (r s t x₁ x₂ l : R) :
    W.addX (x₁+r) (x₂+r) (l+s)=(shifted W r s t).addX x₁ x₂ l+r := by
  simp only [WeierstrassCurve.Affine.addX,shifted]
  ring

theorem addY_identity (W : WeierstrassCurve.Affine R) (r s t x₁ x₂ y₁ l : R) :
    W.addY (x₁+r) (x₂+r) (y₁+s*x₁+t) (l+s)=
      (shifted W r s t).addY x₁ x₂ y₁ l+s*(shifted W r s t).addX x₁ x₂ l+t := by
  simp only [WeierstrassCurve.Affine.addY,WeierstrassCurve.Affine.negAddY,
    WeierstrassCurve.Affine.negY,WeierstrassCurve.Affine.addX,shifted]
  ring

#print axioms equation_identity
#print axioms shifted_inverse
#print axioms addX_identity
#print axioms addY_identity
end
end BKThreeNormalizationAudit
