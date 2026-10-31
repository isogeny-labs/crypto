import Std.Tactic
set_option maxRecDepth 10000
set_option maxHeartbeats 3000000
namespace PureZeroComponents
variable {R : Type} [Lean.Grind.CommRing R]
def D (a b c d : R) : R :=
  256*d^3-192*a*c*d^2-128*b^2*d^2+144*b*c^2*d-27*c^4+
  144*a^2*b*d^2-6*a^2*c^2*d-80*a*b^2*c*d+18*a*b*c^3+
  16*b^4*d-4*b^3*c^2-27*a^4*d^2+18*a^3*b*c*d-
  4*a^3*c^3-4*a^2*b^3*d+a^2*b^2*c^2
def C0 (a b c d : R) : R := -64*d^3+c^4-8*b*c^2*d+16*b^2*d^2
def C1 (a b c d : R) : R := -24*c^3+32*b*c*d-192*a*d^2-8*a*b*c^2+32*a*b^2*d
def C2 (a b c d : R) : R := 192*c^2+256*b*d-64*b^3+32*a*b*c-192*a^2*d+16*a^2*b^2
def C3 (a b c : R) : R := -512*c+256*a*b-64*a^3
theorem component_certificate (a b c d : R) :
    2048*D a b c d =
    (-8192)*C0 a b c d +
    (-64*c+96*a*b-24*a^3)*C1 a b c d +
    (112*c^2-512*b*d+8*a*b*c+48*a^2*d)*C2 a b c d +
    (137*c^3-588*b*c*d+2*b^3*c+792*a*d^2+3*a*b*c^2)*C3 a b c := by
  unfold D C0 C1 C2 C3
  grobner
#print axioms component_certificate

-- The scalar is required nonzero: no field-characteristic exception is hidden.
theorem coefficients_force_discriminant_zero {F : Type} [Lean.Grind.Field F]
    (a b c d : F) (h0 : C0 a b c d=0) (h1 : C1 a b c d=0)
    (h2 : C2 a b c d=0) (h3 : C3 a b c=0) (hchar : (2048:F)≠0) :
    D a b c d=0 := by
  have h := component_certificate a b c d
  rw [h0,h1,h2,h3] at h
  have hz : (2048:F)*D a b c d=0 := by
    simpa only [Lean.Grind.Semiring.mul_zero, Lean.Grind.Semiring.add_zero] using h
  have hinv := Lean.Grind.Field.inv_mul_cancel hchar
  calc
    D a b c d = 1*D a b c d := (Lean.Grind.Semiring.one_mul _).symm
    _ = ((2048:F)⁻¹*2048)*D a b c d := by rw [hinv]
    _ = (2048:F)⁻¹*(2048*D a b c d) := Lean.Grind.Semiring.mul_assoc _ _ _
    _ = 0 := by rw [hz, Lean.Grind.Semiring.mul_zero]
#print axioms coefficients_force_discriminant_zero
end PureZeroComponents
