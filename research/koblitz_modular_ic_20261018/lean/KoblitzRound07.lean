import KoblitzRound06
import Mathlib.GroupTheory.FreeAbelianGroup

namespace KoblitzRound07

/-- Reindexing every factor is the invariant-coordinate norm argument. -/
theorem orbit_product_invariant {I R : Type*} [Fintype I] [CommMonoid R]
    (e : I ≃ I) (f : I → R) : (∏ i, f (e i)) = ∏ i, f i := by
  exact e.prod_comp f

theorem trace_commutes {I A B : Type*} [Fintype I]
    [AddCommGroup A] [AddCommGroup B] (h : A →+ B) (x : I → A) :
    h (∑ i, x i) = ∑ i, h (x i) := by simp

theorem weighted_trace_commutes {I A B : Type*} [Fintype I]
    [AddCommGroup A] [AddCommGroup B] (h : A →+ B) (n : ℤ) (x : I → A) :
    h (n • ∑ i, x i) = n • ∑ i, h (x i) := by simp

def antisymmetric {A : Type*} [AddCommGroup A] (p : A) : FreeAbelianGroup A :=
  FreeAbelianGroup.of ((8:ℤ) • p) - FreeAbelianGroup.of (-((8:ℤ) • p))

/-- The odd representative is odd as a formal divisor, before Picard reduction. -/
theorem antisymmetric_neg {A : Type*} [AddCommGroup A] (p : A) :
    antisymmetric (-p) = -antisymmetric p := by
  simp [antisymmetric, sub_eq_add_neg, add_comm]

theorem antisymmetric_class {A : Type*} [AddCommGroup A] (p : A)
    (h11 : (11:ℤ) • p = 0) :
    (8:ℤ) • p - (-((8:ℤ) • p)) = (5:ℤ) • p := by
  have h : (8:ℤ) • p - (-((8:ℤ) • p)) = (5:ℤ) • p + (11:ℤ) • p := by abel
  simpa [h11] using h

theorem antisymmetric_equivariant {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (f : A →+ B) (p : A) :
    FreeAbelianGroup.map f (antisymmetric p) = antisymmetric (f p) := by
  simp [antisymmetric]

theorem deck_line_identity {R : Type*} [CommRing R] (a b u p q : R) :
    -a*q+b*(u*p+q)=(b*u)*p+(b-a)*q := by ring

theorem requested_deck_line {R : Type*} [CommRing R] (a u p q : R)
    (h : a*u=1) : -a*q+a*(u*p+q)=p := by
  calc
    -a*q+a*(u*p+q)=(a*u)*p := by ring
    _=p := by rw [h]; simp

theorem odd_scalar_eigenvalues :
    (7:ZMod 11)^5 = -1 ∧ (7:ZMod 11)^9=8 ∧ (7:ZMod 11)^4 = -8 := by
  decide +kernel

theorem cusp_image_zero {A : Type*} [AddCommGroup A] (x : A)
    (h14 : (14:ℤ) • x=0) (h11 : (11:ℤ) • x=0) : x=0 := by
  have h : (4:ℤ) • ((14:ℤ) • x) - (5:ℤ) • ((11:ℤ) • x)=x := by abel
  rw [h14,h11] at h
  simpa using h.symm

theorem new_zero_degree : 55*60+5*660=6600 := by norm_num
theorem odd_divisor_degree : 10*660=6600 := by norm_num

/-- A row with a new nonzero entry is outside the span of rows zero there. -/
theorem new_row_independent {I C K : Type*} [Fintype I] [CommRing K] [Nontrivial K]
    (rows : I → C → K) (new : C → K) (column : C)
    (old_zero : ∀ i, rows i column=0) (new_one : new column=1)
    (weights : I → K) : (fun c => ∑ i, weights i * rows i c) ≠ new := by
  intro h
  have e := congrArg (fun v : C → K => v column) h
  simpa [old_zero,new_one] using e

/-- A graph projecting to its parameter has no hidden point identifications. -/
theorem graph_injective {A B : Type*} (f : A → B) :
    Function.Injective (fun x => (f x,x)) := by
  intro x y h
  exact congrArg Prod.snd h

/-- Exact integer lift for the binary supersingular ordinate shift. -/
theorem cofactor_coordinate_lift {R : Type*} [CommRing R] (x y X r c : R) :
    (y+r)^2+(y+r)-(X^3+X+c) =
    (y^2+y-(x^3+x+c))+(x^3+x+r^2+r-(X^3+X))+2*y*r := by ring

/-- The exposed rational-function identity suffices for curve preservation. -/
theorem cofactor_curve_preservation {K : Type*} [CommRing K] [CharP K 2]
    (x y X r c : K) (source : y^2+y=x^3+x+c)
    (coordinate : X^3+X=x^3+x+r^2+r) :
    (y+r)^2+(y+r)=X^3+X+c := by
  have h2 : (2:K)=0 := by simpa using (CharP.natCast_eq_natCast_mod K 2 2)
  linear_combination source-coordinate+y*r*h2

end KoblitzRound07

#print axioms KoblitzRound07.orbit_product_invariant
#print axioms KoblitzRound07.trace_commutes
#print axioms KoblitzRound07.weighted_trace_commutes
#print axioms KoblitzRound07.antisymmetric_neg
#print axioms KoblitzRound07.antisymmetric_class
#print axioms KoblitzRound07.antisymmetric_equivariant
#print axioms KoblitzRound07.deck_line_identity
#print axioms KoblitzRound07.requested_deck_line
#print axioms KoblitzRound07.odd_scalar_eigenvalues
#print axioms KoblitzRound07.cusp_image_zero
#print axioms KoblitzRound07.new_zero_degree
#print axioms KoblitzRound07.odd_divisor_degree
#print axioms KoblitzRound07.new_row_independent
#print axioms KoblitzRound07.graph_injective
#print axioms KoblitzRound07.cofactor_coordinate_lift
#print axioms KoblitzRound07.cofactor_curve_preservation
