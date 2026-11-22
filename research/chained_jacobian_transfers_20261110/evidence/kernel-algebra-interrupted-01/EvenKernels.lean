import Init.Grind.Ring.Basic

/-! Denominator clearing and finite geometric-kernel scalar certificates.
Actual elliptic/Jacobian torsion, cyclicity, division-polynomial evaluation,
Frobenius characteristic polynomial and coordinate-field identifications
are written/native bridges, not axioms of these statements. -/
set_option autoImplicit false
namespace EvenKernels
open Lean.Grind

theorem cubic_homogenization {R : Type} [CommRing R]
    (a b c e x d : R) :
    a*d^3+b*(x*d)*d^2+c*(x*d)^2*d+e*(x*d)^3 =
      d^3*(a+b*x+c*x^2+e*x^3) := by grind

theorem geometric_half_coordinates :
    (2 : Fin 7)*4=1 ∧ (2 : Fin 7)*55=5 ∧
    ∀ q : Fin 7,
      (2 : Fin 7)*4*q=q ∧ ((2 : Fin 7)*55*q=q ↔ q=0) := by decide

theorem geometric_wrong_half_counts :
    (7 : Nat)*6=42 ∧ (49 : Nat)-7=42 ∧
    (7 : Nat)*7=49 ∧ (49 : Nat)*49=2401 := by decide

theorem first_kernel_x_period :
    (5 : Nat)^6%7=1 ∧ (5 : Nat)^3%7=6 ∧
    ∀ n : Fin 2,
      (5 : Nat)^(n.val+1)%7≠1 ∧ (5 : Nat)^(n.val+1)%7≠6 := by decide

theorem first_primitive_x_period :
    (33 : Nat)^21%49=48 ∧
    ∀ n : Fin 20,
      (33 : Nat)^(n.val+1)%49≠1 ∧ (33 : Nat)^(n.val+1)%49≠48 := by decide

end EvenKernels
#print axioms EvenKernels.cubic_homogenization
#print axioms EvenKernels.geometric_half_coordinates
#print axioms EvenKernels.geometric_wrong_half_counts
#print axioms EvenKernels.first_kernel_x_period
#print axioms EvenKernels.first_primitive_x_period
