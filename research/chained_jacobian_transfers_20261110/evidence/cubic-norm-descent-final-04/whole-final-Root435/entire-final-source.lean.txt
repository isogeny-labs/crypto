import Std.Tactic
import Init.GrindInstances.Ring.Fin

/-! A concrete arithmetic obstruction in the original cubic representation.
The coefficients below are the schoolbook square reduced by z^3=d. The
theorems do not construct a field, a Jacobian or a polarized identification,
and assert nothing about other supplied-source models or other twists. -/
set_option autoImplicit false
set_option maxRecDepth 8192
namespace CubicNormDescent
open Lean.Grind

def norm {R : Type} [CommRing R] (d a b c : R) : R :=
  a^3 + d*b^3 + d^2*c^3 - 3*d*a*b*c

/-- The exact schoolbook coefficient convention used by the original Kf.mul. -/
def square0 {R : Type} [CommRing R] (d a b c : R) : R :=
  a*a + d*(b*c+c*b)
def square1 {R : Type} [CommRing R] (d a b c : R) : R :=
  a*b+b*a+d*c*c
def square2 {R : Type} [CommRing R] (a b c : R) : R :=
  a*c+b*b+c*a

/-- Expansion of the determinant of multiplication by a+b*z+c*z^2 in
the basis 1,z,z^2, using the relation z^3=d. -/
theorem determinant_formula {R : Type} [CommRing R] (d a b c : R) :
    a*(a*a-d*c*b)-d*c*(b*a-d*c*c)+d*b*(b*b-a*c)=norm d a b c := by
  unfold norm
  grind

theorem norm_square {R : Type} [CommRing R] (d a b c : R) :
    norm d (square0 d a b c) (square1 d a b c) (square2 a b c) =
      (norm d a b c)^2 := by
  unfold norm square0 square1 square2
  grind

theorem norm_of_constant_square {R : Type} [CommRing R]
    (d a b c D : R)
    (h0 : square0 d a b c = D) (h1 : square1 d a b c = 0)
    (h2 : square2 a b c = 0) : (norm d a b c)^2 = D^3 := by
  have h := norm_square d a b c
  rw [h0, h1, h2] at h
  unfold norm at h ⊢
  grind

/-- For a unit D, a square root of D^3 yields a base-ring square root of D. -/
theorem recover_base_square_root {R : Type} [CommRing R]
    (D invD u : R) (hinv : invD*D=1) (hu : u^2=D^3) :
    (u*invD)^2=D := by
  grind

/-- This is the already known finite residue calculation, now stated in the
actual coefficient ring used by the algebraic implication. Kernel reduction
checks only223 base residues, never the223^3 coefficient triples. -/
theorem base_nonroots :
    ∀ x : Fin 223, x^2 ≠ (12 : Fin 223) ∧ x^2 ≠ (24 : Fin 223) := by
  decide

theorem actual_constant_units :
    (93 : Fin 223)*12=1 ∧ (158 : Fin 223)*24=1 := by decide

theorem no_cubic_square_root12 (a b c : Fin 223) :
    ¬ (square0 3 a b c=12 ∧ square1 3 a b c=0 ∧ square2 a b c=0) := by
  intro ⟨h0,h1,h2⟩
  have hn := norm_of_constant_square 3 a b c 12 h0 h1 h2
  have hb := recover_base_square_root 12 93 (norm 3 a b c)
    actual_constant_units.1 hn
  exact (base_nonroots (norm 3 a b c * 93)).1 hb

theorem no_cubic_square_root24 (a b c : Fin 223) :
    ¬ (square0 3 a b c=24 ∧ square1 3 a b c=0 ∧ square2 a b c=0) := by
  intro ⟨h0,h1,h2⟩
  have hn := norm_of_constant_square 3 a b c 24 h0 h1 h2
  have hb := recover_base_square_root 24 158 (norm 3 a b c)
    actual_constant_units.2 hn
  exact (base_nonroots (norm 3 a b c * 158)).2 hb

/-- A square constant remains allowed: the obstruction does not reject all
models merely because their square root is represented in the cubic algebra. -/
theorem square_constant_control :
    square0 (3 : Fin 223) 2 0 0=4 ∧ square1 (3 : Fin 223) 2 0 0=0 ∧
    square2 (2 : Fin 223) 0 0=0 := by decide

/-- The original Kf.mul square's five unreduced/reduced natural-number
temporaries, specialized only to q223,c3. Native word-overflow and binary
execution correspondence are separate from this exact arithmetic model. -/
def originalSquare223 (a b c : Fin 223) : Fin 223 × Fin 223 × Fin 223 :=
  let t0 := (a.val*a.val)%223
  let t1 := (a.val*b.val+b.val*a.val)%223
  let t2 := (a.val*c.val+b.val*b.val+c.val*a.val)%223
  let t3 := (b.val*c.val+c.val*b.val)%223
  let t4 := (c.val*c.val)%223
  (Fin.ofNat 223 (t0+3*t3),Fin.ofNat 223 (t1+3*t4),Fin.ofNat 223 t2)

theorem original_square_coordinates (a b c : Fin 223) :
    originalSquare223 a b c =
      (square0 3 a b c,square1 3 a b c,square2 a b c) := by
  have three : (3 : Fin 223).val = 3 := rfl
  apply Prod.ext
  · apply Fin.ext
    simp [originalSquare223,square0,Fin.add_def,Fin.mul_def,Nat.add_mod,Nat.mul_mod,three]
  · apply Prod.ext
    · have reassociate : square1 (3 : Fin 223) a b c = a*b+b*a+3*(c*c) := by
        unfold square1
        grind
      rw [reassociate]
      apply Fin.ext
      simp [originalSquare223,Fin.add_def,Fin.mul_def,Nat.add_mod,Nat.mul_mod,three]
    · apply Fin.ext
      simp [originalSquare223,square2,Fin.add_def,Fin.mul_def,Nat.add_mod,Nat.mul_mod]

theorem original_square_ne12 (a b c : Fin 223) :
    originalSquare223 a b c ≠ (12,0,0) := by
  intro h
  rw [original_square_coordinates] at h
  exact no_cubic_square_root12 a b c
    ⟨congrArg Prod.fst h,congrArg (fun x => x.2.1) h,congrArg (fun x => x.2.2) h⟩

theorem original_square_ne24 (a b c : Fin 223) :
    originalSquare223 a b c ≠ (24,0,0) := by
  intro h
  rw [original_square_coordinates] at h
  exact no_cubic_square_root24 a b c
    ⟨congrArg Prod.fst h,congrArg (fun x => x.2.1) h,congrArg (fun x => x.2.2) h⟩

end CubicNormDescent
#print axioms CubicNormDescent.norm_square
#print axioms CubicNormDescent.determinant_formula
#print axioms CubicNormDescent.norm_of_constant_square
#print axioms CubicNormDescent.recover_base_square_root
#print axioms CubicNormDescent.base_nonroots
#print axioms CubicNormDescent.actual_constant_units
#print axioms CubicNormDescent.no_cubic_square_root12
#print axioms CubicNormDescent.no_cubic_square_root24
#print axioms CubicNormDescent.square_constant_control
#print axioms CubicNormDescent.original_square_coordinates
#print axioms CubicNormDescent.original_square_ne12
#print axioms CubicNormDescent.original_square_ne24
