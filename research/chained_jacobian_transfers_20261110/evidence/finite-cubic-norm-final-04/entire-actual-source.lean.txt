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

/-! Universal finite q223 coefficient norm and explicit inverse. The only finite
certificates range over223 base elements; there is no223^3 triple enumeration.
No Rust compiler, field typeclass, Jacobian or cost correspondence is asserted. -/
namespace CubicFiniteNorm22308
open Lean.Grind
attribute [local instance] Semiring.natCast

def coefficientProduct (x y : Fin 223 × Fin 223 × Fin 223) :
    Fin 223 × Fin 223 × Fin 223 :=
  (x.1*y.1+3*(x.2.1*y.2.2+x.2.2*y.2.1),
   x.1*y.2.1+x.2.1*y.1+3*x.2.2*y.2.2,
   x.1*y.2.2+x.2.1*y.2.1+x.2.2*y.1)

def cofactor (x : Fin 223 × Fin 223 × Fin 223) :
    Fin 223 × Fin 223 × Fin 223 :=
  (x.1^2-3*x.2.1*x.2.2,3*x.2.2^2-x.1*x.2.1,x.2.1^2-x.1*x.2.2)

def coefficientNorm (x : Fin 223 × Fin 223 × Fin 223) : Fin 223 :=
  x.1^3+3*x.2.1^3+9*x.2.2^3-9*x.1*x.2.1*x.2.2

def scale (s : Fin 223) (x : Fin 223 × Fin 223 × Fin 223) :
    Fin 223 × Fin 223 × Fin 223 := (s*x.1,s*x.2.1,s*x.2.2)

/-- The base certificate is checked only on223 base elements. -/
theorem base_unit_certificate :
    ∀ x : Fin 223, x=0 ∨ x^221*x=1 := by decide

theorem base_cube_zero : ∀ x : Fin 223, x^3=0 → x=0 := by decide

/-- A low-degree polynomial identity over an arbitrary commutative ring. -/
theorem cofactor_cubic_relation {R : Type} [CommRing R] (a b c : R) :
    (3*c^2-a*b)^3-3*(b^2-a*c)^3 =
      (a^3+3*b^3+9*c^3-9*a*b*c)*(3*c^3-b^3) := by
  grind

theorem unit_cube_ratio {R : Type} [CommRing R] (x y u : R)
    (hc : x^3=3*y^3) (hu : u*y=1) : (u*x)^3=3 := by
  grind

theorem zero_norm_cofactor_cubes (x : Fin 223 × Fin 223 × Fin 223)
    (hn : coefficientNorm x=0) : (cofactor x).2.1^3=3*(cofactor x).2.2^3 := by
  rcases x with ⟨a,b,c⟩
  have h := cofactor_cubic_relation a b c
  change a^3+3*b^3+9*c^3-9*a*b*c=0 at hn
  rw [hn] at h
  change (3*c^2-a*b)^3=3*(b^2-a*c)^3
  grind

theorem zero_norm_second_cofactor (x : Fin 223 × Fin 223 × Fin 223)
    (hn : coefficientNorm x=0) : (cofactor x).2.2=0 := by
  rcases base_unit_certificate (cofactor x).2.2 with hz | hi
  · exact hz
  · exfalso
    exact CubicConstructorFrobenius22301.no_base_cubic_root_three
      ((cofactor x).2.2^221*(cofactor x).2.1)
      (unit_cube_ratio (cofactor x).2.1 (cofactor x).2.2
        ((cofactor x).2.2^221) (zero_norm_cofactor_cubes x hn) hi)

theorem zero_norm_first_cofactor (x : Fin 223 × Fin 223 × Fin 223)
    (hn : coefficientNorm x=0) : (cofactor x).2.1=0 := by
  have h := zero_norm_cofactor_cubes x hn
  rw [zero_norm_second_cofactor x hn] at h
  apply base_cube_zero (cofactor x).2.1
  simpa using h

theorem zero_cofactors_cube_relation {R : Type} [CommRing R] (a b c : R)
    (h1 : 3*c^2-a*b=0) (h2 : b^2-a*c=0) : b^3=3*c^3 := by
  grind

theorem zero_norm_coordinate_cube_relation (a b c : Fin 223)
    (hn : coefficientNorm (a,b,c)=0) : b^3=3*c^3 := by
  have h1 := zero_norm_first_cofactor (a,b,c) hn
  have h2 := zero_norm_second_cofactor (a,b,c) hn
  exact zero_cofactors_cube_relation a b c h1 h2

theorem zero_norm_third_coordinate (a b c : Fin 223)
    (hn : coefficientNorm (a,b,c)=0) : c=0 := by
  rcases base_unit_certificate c with hz | hi
  · exact hz
  · exfalso
    exact CubicConstructorFrobenius22301.no_base_cubic_root_three (c^221*b)
      (unit_cube_ratio b c (c^221) (zero_norm_coordinate_cube_relation a b c hn) hi)

theorem zero_norm_all_coordinates (x : Fin 223 × Fin 223 × Fin 223)
    (hn : coefficientNorm x=0) : x=(0,0,0) := by
  rcases x with ⟨a,b,c⟩
  have hc := zero_norm_third_coordinate a b c hn
  have hb3 := zero_norm_coordinate_cube_relation a b c hn
  rw [hc] at hb3
  have hb : b=0 := base_cube_zero b (by simpa using hb3)
  have ha3 : a^3=0 := by
    change a^3+3*b^3+9*c^3-9*a*b*c=0 at hn
    rw [hb,hc] at hn
    grind
  have ha := base_cube_zero a ha3
  simp only [ha,hb,hc]

theorem norm_zero_iff (x : Fin 223 × Fin 223 × Fin 223) :
    coefficientNorm x=0 ↔ x=(0,0,0) := by
  constructor
  · exact zero_norm_all_coordinates x
  · rintro rfl
    decide

theorem nonzero_input_nonzero_norm (x : Fin 223 × Fin 223 × Fin 223)
    (hx : x≠(0,0,0)) : coefficientNorm x≠0 := by
  intro h
  exact hx ((norm_zero_iff x).mp h)

theorem derived_norm_unit (x : Fin 223 × Fin 223 × Fin 223)
    (hx : x≠(0,0,0)) : (coefficientNorm x)^221*coefficientNorm x=1 := by
  rcases base_unit_certificate (coefficientNorm x) with hz | hi
  · exact False.elim (nonzero_input_nonzero_norm x hx hz)
  · exact hi

theorem coefficient_cofactor_product (x : Fin 223 × Fin 223 × Fin 223) :
    coefficientProduct x (cofactor x)=(coefficientNorm x,0,0) := by
  rcases x with ⟨a,b,c⟩
  apply Prod.ext
  · simp only [coefficientProduct,cofactor,coefficientNorm]
    grind
  · apply Prod.ext <;> simp only [coefficientProduct,cofactor] <;> grind

theorem coefficient_product_scale (x y : Fin 223 × Fin 223 × Fin 223)
    (s : Fin 223) : coefficientProduct x (scale s y)=scale s (coefficientProduct x y) := by
  rcases x with ⟨a,b,c⟩
  rcases y with ⟨d,e,f⟩
  apply Prod.ext
  · simp only [coefficientProduct,scale]
    grind
  · apply Prod.ext <;> simp only [coefficientProduct,scale] <;> grind

theorem coefficient_product_comm (x y : Fin 223 × Fin 223 × Fin 223) :
    coefficientProduct x y=coefficientProduct y x := by
  rcases x with ⟨a,b,c⟩
  rcases y with ⟨d,e,f⟩
  apply Prod.ext
  · simp only [coefficientProduct]
    grind
  · apply Prod.ext <;> simp only [coefficientProduct] <;> grind

/-- The norm unit is derived, not supplied as an assumption. -/
theorem every_nonzero_coefficient_inverse (x : Fin 223 × Fin 223 × Fin 223)
    (hx : x≠(0,0,0)) :
    coefficientProduct x (scale ((coefficientNorm x)^221) (cofactor x))=(1,0,0) ∧
    coefficientProduct (scale ((coefficientNorm x)^221) (cofactor x)) x=(1,0,0) := by
  have hleft : coefficientProduct x
      (scale ((coefficientNorm x)^221) (cofactor x))=(1,0,0) := by
    rw [coefficient_product_scale,coefficient_cofactor_product]
    apply Prod.ext
    · exact derived_norm_unit x hx
    · apply Prod.ext <;> simp only [scale] <;> grind
  constructor
  · exact hleft
  · rw [coefficient_product_comm]
    exact hleft

theorem zero_and_unit_controls :
    coefficientNorm (0,0,0)=0 ∧ coefficientNorm (1,0,0)=1 ∧
    coefficientProduct (1,0,0) (1,0,0)=(1,0,0) := by decide

/-- A reducible parameter gives a genuine nonzero zero-norm tuple. -/
theorem reducible_parameter_control :
    ((222,1,0) : Fin 223 × Fin 223 × Fin 223)≠(0,0,0) ∧
    ((222 : Fin 223)^3+1*(1 : Fin 223)^3+1^2*(0 : Fin 223)^3-
      3*1*222*1*0)=0 := by decide

end CubicFiniteNorm22308
#print axioms CubicFiniteNorm22308.base_unit_certificate
#print axioms CubicFiniteNorm22308.base_cube_zero
#print axioms CubicFiniteNorm22308.cofactor_cubic_relation
#print axioms CubicFiniteNorm22308.unit_cube_ratio
#print axioms CubicFiniteNorm22308.zero_norm_cofactor_cubes
#print axioms CubicFiniteNorm22308.zero_norm_second_cofactor
#print axioms CubicFiniteNorm22308.zero_norm_first_cofactor
#print axioms CubicFiniteNorm22308.zero_cofactors_cube_relation
#print axioms CubicFiniteNorm22308.zero_norm_coordinate_cube_relation
#print axioms CubicFiniteNorm22308.zero_norm_third_coordinate
#print axioms CubicFiniteNorm22308.zero_norm_all_coordinates
#print axioms CubicFiniteNorm22308.norm_zero_iff
#print axioms CubicFiniteNorm22308.nonzero_input_nonzero_norm
#print axioms CubicFiniteNorm22308.derived_norm_unit
#print axioms CubicFiniteNorm22308.coefficient_cofactor_product
#print axioms CubicFiniteNorm22308.coefficient_product_scale
#print axioms CubicFiniteNorm22308.coefficient_product_comm
#print axioms CubicFiniteNorm22308.every_nonzero_coefficient_inverse
#print axioms CubicFiniteNorm22308.zero_and_unit_controls
#print axioms CubicFiniteNorm22308.reducible_parameter_control
