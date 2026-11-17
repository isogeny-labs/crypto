import Mathlib

/-! Exact instances, not the general tower theorem. The kernel checks the
literal partition and every coordinate certificate for the non-cartesian
rational F17 two-level tower from the original hand argument. -/
namespace VietaSpeedAudit
open MvPolynomial
abbrev P := MvPolynomial (Fin 2) (ZMod 17)
noncomputable def x : P := X 0
noncomputable def y : P := X 1
noncomputable def f0 : P := x^2 + 1 - 2*x*y
noncomputable def f1 : P := y^2 + 1 - 8*y
noncomputable def e10 : P := (8*x+6)*(11*y+16)
noncomputable def e12 : P := (9*x+12)*(11*y+16)
noncomputable def e2 : P := (12*x+11)*(6*y+2)
noncomputable def e9 : P := (5*x+7)*(6*y+2)

theorem partition : e10+e12+e2+e9 = 1 := by
  unfold e10 e12 e2 e9
  have h17 : (17 : P) = 0 := CharP.cast_eq_zero P 17
  linear_combination (19+18*x+17*x*y+18*y)*h17
theorem coordinate10x : (x-10)*e10 = (3*y+9)*f0+6*x*f1 := by
  unfold e10 f0 f1
  have h17 : (17 : P) = 0 := CharP.cast_eq_zero P 17
  linear_combination ((-57 : P) + (-39 : P)*y + (-70 : P)*x + (-44 : P)*x*y + (7 : P)*x^2 + (5 : P)*x^2*y)*h17
theorem coordinate10y : (y-11)*e10 = (3*x+15)*f1 := by
  unfold e10 f1
  have h17 : (17 : P) = 0 := CharP.cast_eq_zero P 17
  linear_combination ((-63 : P) + (-30 : P)*y + (3 : P)*y^2 + (-83 : P)*x + (-48 : P)*x*y + (5 : P)*x*y^2)*h17
theorem coordinate12x : (x-12)*e12 = (14*y+8)*f0+11*x*f1 := by
  unfold e12 f0 f1
  have h17 : (17 : P) = 0 := CharP.cast_eq_zero P 17
  linear_combination ((-136 : P) + (-94 : P)*y + (-91 : P)*x + (-56 : P)*x*y + x*y^2 + (8 : P)*x^2 + (5 : P)*x^2*y)*h17
theorem coordinate12y : (y-11)*e12 = (14*x+13)*f1 := by
  unfold e12 f1
  have h17 : (17 : P) = 0 := CharP.cast_eq_zero P 17
  linear_combination ((-125 : P) + (-68 : P)*y + (7 : P)*y^2 + (-94 : P)*x + (-49 : P)*x*y + (5 : P)*x*y^2)*h17
theorem coordinate2x : (x-2)*e2 = (4*y+7)*f0+8*x*f1 := by
  unfold e2 f0 f1
  have h17 : (17 : P) = 0 := CharP.cast_eq_zero P 17
  linear_combination ((-3 : P) + (-8 : P)*y + (-2 : P)*x + x^2 + (4 : P)*x^2*y)*h17
theorem coordinate2y : (y-14)*e2 = (4*x+15)*f1 := by
  unfold e2 f1
  have h17 : (17 : P) = 0 := CharP.cast_eq_zero P 17
  linear_combination ((-19 : P) + (-46 : P)*y + (3 : P)*y^2 + (-20 : P)*x + (-56 : P)*x*y + (4 : P)*x*y^2)*h17
theorem coordinate9x : (x-9)*e9 = (13*y+10)*f0+9*x*f1 := by
  unfold e9 f0 f1
  have h17 : (17 : P) = 0 := CharP.cast_eq_zero P 17
  linear_combination ((-8 : P) + (-23 : P)*y + (-5 : P)*x + (-8 : P)*x*y + x*y^2 + x^2*y)*h17
theorem coordinate9y : (y-14)*e9 = (13*x+8)*f1 := by
  unfold e9 f1
  have h17 : (17 : P) = 0 := CharP.cast_eq_zero P 17
  linear_combination ((-12 : P) + (-30 : P)*y + (2 : P)*y^2 + (-9 : P)*x + (-18 : P)*x*y + x*y^2)*h17

theorem coprime_witness : (x^2+1)-(9*x)*(2*x)=1 := by
  have h17 : (17 : P) = 0 := CharP.cast_eq_zero P 17
  linear_combination (-x^2)*h17

-- Finite statements use kernel reduction, never native_decide.
theorem upper_fiber_exact : ∀ a : ZMod 17,
    a^2+1-8*a=0 ↔ a=11 ∨ a=14 := by decide
theorem lower11_fiber_exact : ∀ a : ZMod 17,
    a^2+1-2*a*11=0 ↔ a=10 ∨ a=12 := by decide
theorem lower14_fiber_exact : ∀ a : ZMod 17,
    a^2+1-2*a*14=0 ↔ a=2 ∨ a=9 := by decide
theorem fiber_roots_distinct : (11 : ZMod 17) ≠ 14 ∧
    (10 : ZMod 17) ≠ 12 ∧ (2 : ZMod 17) ≠ 9 := by decide
theorem leaf_denominators_nonzero : (2*(10 : ZMod 17)) ≠ 0 ∧
    (2*(12 : ZMod 17)) ≠ 0 ∧ (2*(2 : ZMod 17)) ≠ 0 ∧
    (2*(9 : ZMod 17)) ≠ 0 := by decide
theorem nonsplit_control : ∀ a : ZMod 17, a^2 ≠ 3 := by decide
theorem repeated_tower_fiber : ∀ a : ZMod 17, a^2=0 ↔ a=0 := by decide

-- Actual quotient ring: a nonzero nilpotent survives, unlike a point set.
noncomputable def nilpoly : Polynomial (ZMod 17) := Polynomial.X^2
theorem nilpotent_root_square_zero : (AdjoinRoot.root nilpoly)^2=0 := by
  change (AdjoinRoot.mk nilpoly Polynomial.X)^2=0
  rw [← map_pow]
  exact AdjoinRoot.mk_self
theorem nilpotent_root_nonzero : AdjoinRoot.root nilpoly ≠ 0 := by
  let phi : AdjoinRoot nilpoly →+* DualNumber (ZMod 17) :=
    AdjoinRoot.lift (algebraMap (ZMod 17) (DualNumber (ZMod 17)))
      DualNumber.eps (by simp [nilpoly])
  intro h
  have heps : (DualNumber.eps : DualNumber (ZMod 17)) = 0 := by
    calc
      _ = phi (AdjoinRoot.root nilpoly) := (AdjoinRoot.lift_root _).symm
      _ = phi 0 := congrArg phi h
      _ = 0 := map_zero phi
  have hsnd := congrArg TrivSqZeroExt.snd heps
  have h10 : (1 : ZMod 17) = 0 := by simpa using hsnd
  exact (by decide : (1 : ZMod 17) ≠ 0) h10

#print axioms partition
#print axioms coordinate10x
#print axioms coordinate10y
#print axioms coordinate12x
#print axioms coordinate12y
#print axioms coordinate2x
#print axioms coordinate2y
#print axioms coordinate9x
#print axioms coordinate9y
#print axioms coprime_witness
#print axioms upper_fiber_exact
#print axioms lower11_fiber_exact
#print axioms lower14_fiber_exact
#print axioms fiber_roots_distinct
#print axioms leaf_denominators_nonzero
#print axioms nonsplit_control
#print axioms repeated_tower_fiber
#print axioms nilpotent_root_square_zero
#print axioms nilpotent_root_nonzero
end VietaSpeedAudit
