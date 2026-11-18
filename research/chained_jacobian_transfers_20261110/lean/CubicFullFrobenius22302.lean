import Std.Tactic
import Init.GrindInstances.Ring.Fin

/-! Arithmetic certificates for the exact original Kf::new(223) parameters
and its Frobenius basis coefficients. No field/Jacobian construction,
Rust powmod/compiler/counter correspondence or runtime cost is asserted. -/
set_option autoImplicit false
set_option maxRecDepth 8192
namespace CubicConstructorFrobenius22301
open Lean.Grind
attribute [local instance] Semiring.natCast

theorem constructor_guard : (223 : Nat)%12=7 ∧ (223-1)/3=74 := by decide

theorem skipped_candidate_two : (2 : Nat)^74%223=1 := by decide

theorem selected_candidate_three : (3 : Nat)^74%223=183 ∧ (183 : Nat)≠1 := by decide

/-- Only223 base residues are reduced; no cubic coefficient population. -/
theorem no_base_cubic_root_three : ∀ x : Fin 223, x^3≠(3 : Fin 223) := by decide

theorem frobenius_coefficients :
    (3 : Fin 223)^74=183 ∧ (183 : Fin 223)^2=39 ∧
    (183 : Fin 223)^3=1 ∧ (39 : Fin 223)^3=1 ∧
    (183 : Fin 223)≠1 ∧ (39 : Fin 223)≠1 := by decide

theorem constructor_size : (223 : Nat)^3=11089567 := by decide

/-- Power grouping uses only the ordinary semiring laws. -/
theorem group_power {R : Type} [Semiring R] (z : R) (m n : Nat) :
    z^(m*n)=(z^m)^n := by
  induction n with
  | zero => simp only [Nat.mul_zero,Semiring.pow_zero]
  | succ n ih => rw [Nat.mul_succ,Semiring.pow_add,ih,Semiring.pow_succ]

/-- The actual characteristic hypothesis transports the computed base residue. -/
theorem coefficient_power {R : Type} [CommRing R] [IsCharP R 223] :
    (3 : R)^74=183 := by
  grind

/-- The relation z^3=3 gives the constructor's first Frobenius coefficient. -/
theorem generator_frobenius {R : Type} [CommRing R] [IsCharP R 223]
    (z : R) (hz : z^3=3) : z^223=183*z := by
  change z^(3*74+1)=183*z
  rw [Semiring.pow_succ,group_power,hz,coefficient_power]

/-- The second basis coefficient is39 under the same explicit relation. -/
theorem squared_generator_frobenius {R : Type} [CommRing R] [IsCharP R 223]
    (z : R) (hz : z^3=3) : (z^2)^223=39*z^2 := by
  rw [← group_power]
  change z^(223*2)=39*z^2
  rw [group_power,generator_frobenius z hz]
  grind

def basisFrob (x : Fin 223 × Fin 223 × Fin 223) : Fin 223 × Fin 223 × Fin 223 :=
  (x.1,183*x.2.1,39*x.2.2)

/-- A certificate for the concrete coefficient action, not a field construction. -/
theorem basis_frobenius_three (a b c : Fin 223) :
    basisFrob (basisFrob (basisFrob (a,b,c)))=(a,b,c) := by
  apply Prod.ext
  · rfl
  · apply Prod.ext <;> simp only [basisFrob] <;> grind

/-- Fixed vectors of the exact diagonal coefficient action are the base axis. -/
theorem fixed_coefficient_subspace (a b c : Fin 223) :
    basisFrob (a,b,c)=(a,b,c) ↔ b=0 ∧ c=0 := by
  constructor
  · intro h
    have hb : (183 : Fin 223)*b=b := congrArg (fun x => x.2.1) h
    have hc : (39 : Fin 223)*c=c := congrArg (fun x => x.2.2) h
    constructor <;> grind
  · rintro ⟨rfl,rfl⟩
    apply Prod.ext
    · rfl
    · apply Prod.ext <;> simp only [basisFrob] <;> grind

end CubicConstructorFrobenius22301
#print axioms CubicConstructorFrobenius22301.constructor_guard
#print axioms CubicConstructorFrobenius22301.skipped_candidate_two
#print axioms CubicConstructorFrobenius22301.selected_candidate_three
#print axioms CubicConstructorFrobenius22301.no_base_cubic_root_three
#print axioms CubicConstructorFrobenius22301.frobenius_coefficients
#print axioms CubicConstructorFrobenius22301.constructor_size
#print axioms CubicConstructorFrobenius22301.group_power
#print axioms CubicConstructorFrobenius22301.coefficient_power
#print axioms CubicConstructorFrobenius22301.generator_frobenius
#print axioms CubicConstructorFrobenius22301.squared_generator_frobenius
#print axioms CubicConstructorFrobenius22301.basis_frobenius_three
#print axioms CubicConstructorFrobenius22301.fixed_coefficient_subspace


/-! Additive full-expression transport. The exact frozen01 source is the
prefix. This constructs a coefficient embedding into a stated characteristic
ring and gives a full q-power formula; representation injectivity remains an
explicit input for the fixed-point converse. -/
namespace CubicFullFrobenius22302
open Lean.Grind
attribute [local instance] Semiring.natCast

theorem frobenius_add {R : Type} [CommRing R] [IsCharP R 223] (a b : R) :
    (a+b)^223=a^223+b^223 := by
  grind =>
    (try ring)
    done

/-- Kernel reduction of only the223 coefficient residues. -/
theorem base_fermat : ∀ a : Fin 223, a^223=a := by decide

def embed {R : Type} [CommRing R] (a : Fin 223) : R := (a.val : R)

theorem embed_injective {R : Type} [CommRing R] [IsCharP R 223] :
    Function.Injective (embed (R := R)) := by
  intro a b h
  apply Fin.ext
  exact (IsCharP.natCast_eq_iff_of_lt 223 a.isLt b.isLt).mp h

theorem embed_add {R : Type} [CommRing R] [IsCharP R 223] (a b : Fin 223) :
    embed (R := R) (a+b)=embed a+embed b := by
  unfold embed
  rw [Fin.val_add,IsCharP.natCast_emod 223,Semiring.natCast_add]

theorem embed_mul {R : Type} [CommRing R] [IsCharP R 223] (a b : Fin 223) :
    embed (R := R) (a*b)=embed a*embed b := by
  unfold embed
  rw [Fin.val_mul,IsCharP.natCast_emod 223,Semiring.natCast_mul]

theorem embed_pow {R : Type} [CommRing R] [IsCharP R 223] (a : Fin 223) (n : Nat) :
    embed (R := R) (a^n)=(embed a)^n := by
  induction n with
  | zero =>
    simp only [Semiring.pow_zero]
    change ((1 : Nat) : R)=1
    exact Semiring.natCast_one
  | succ n ih =>
    rw [Semiring.pow_succ,embed_mul,ih,Semiring.pow_succ]

theorem embedded_coefficients_fixed {R : Type} [CommRing R] [IsCharP R 223]
    (a : Fin 223) : (embed (R := R) a)^223=embed a := by
  rw [← embed_pow,base_fermat]

theorem embedded_action_constants {R : Type} [CommRing R] :
    embed (R := R) (183 : Fin 223)=(183 : R) ∧
    embed (R := R) (39 : Fin 223)=(39 : R) := by
  constructor
  · change ((183 : Nat) : R)=(183 : R)
    exact Semiring.natCast_eq_ofNat 183
  · change ((39 : Nat) : R)=(39 : R)
    exact Semiring.natCast_eq_ofNat 39

def encode {R : Type} [CommRing R] (z : R)
    (x : Fin 223 × Fin 223 × Fin 223) : R :=
  embed x.1+embed x.2.1*z+embed x.2.2*z^2

/-- The exact full expression, with canonical embedded base coefficients. -/
theorem full_expression {R : Type} [CommRing R] [IsCharP R 223]
    (z : R) (hz : z^3=3) (x : Fin 223 × Fin 223 × Fin 223) :
    (encode z x)^223=encode z (CubicConstructorFrobenius22301.basisFrob x) := by
  rcases x with ⟨a,b,c⟩
  change (embed a+embed b*z+embed c*z^2)^223=
    embed a+embed (183*b)*z+embed (39*c)*z^2
  rw [frobenius_add,frobenius_add,CommSemiring.mul_pow,CommSemiring.mul_pow,
    embedded_coefficients_fixed,embedded_coefficients_fixed,embedded_coefficients_fixed,
    CubicConstructorFrobenius22301.generator_frobenius z hz,
    CubicConstructorFrobenius22301.squared_generator_frobenius z hz,
    embed_mul,embed_mul]
  have constants := embedded_action_constants (R := R)
  rw [constants.1,constants.2]
  grind

theorem three_power_steps {R : Type} [CommRing R] [IsCharP R 223]
    (z : R) (hz : z^3=3) (x : Fin 223 × Fin 223 × Fin 223) :
    (((encode z x)^223)^223)^223=encode z x := by
  rcases x with ⟨a,b,c⟩
  rw [full_expression z hz,full_expression z hz,full_expression z hz]
  exact congrArg (encode z)
    (CubicConstructorFrobenius22301.basis_frobenius_three a b c)

/-- The converse states the exact still-needed basis independence input. -/
theorem fixed_expression_iff {R : Type} [CommRing R] [IsCharP R 223]
    (z : R) (hz : z^3=3) (hinj : Function.Injective (encode z)) (a b c : Fin 223) :
    (encode z (a,b,c))^223=encode z (a,b,c) ↔ b=0 ∧ c=0 := by
  rw [full_expression z hz]
  constructor
  · intro h
    exact (CubicConstructorFrobenius22301.fixed_coefficient_subspace a b c).mp (hinj h)
  · intro h
    exact congrArg (encode z)
      ((CubicConstructorFrobenius22301.fixed_coefficient_subspace a b c).mpr h)

end CubicFullFrobenius22302
#print axioms CubicFullFrobenius22302.frobenius_add
#print axioms CubicFullFrobenius22302.base_fermat
#print axioms CubicFullFrobenius22302.embed_injective
#print axioms CubicFullFrobenius22302.embed_add
#print axioms CubicFullFrobenius22302.embed_mul
#print axioms CubicFullFrobenius22302.embed_pow
#print axioms CubicFullFrobenius22302.embedded_coefficients_fixed
#print axioms CubicFullFrobenius22302.embedded_action_constants
#print axioms CubicFullFrobenius22302.full_expression
#print axioms CubicFullFrobenius22302.three_power_steps
#print axioms CubicFullFrobenius22302.fixed_expression_iff
