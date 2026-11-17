import PairingFixture
import RelationYield
import Mathlib.RingTheory.MvPolynomial.Homogeneous
import Mathlib.RingTheory.MvPolynomial.Ideal

/-! Actual prime-field S3 encoding, its leading homogeneous component, and a
finite useful-row fixture. Closure degrees in the native audit are a different
quantity from homogeneous regularity and are not asserted by these lemmas. -/
namespace S3System

def s3 {R : Type*} [CommRing R] (a b x y z : R) : R :=
  (x-y)^2*z^2 - 2*((x+y)*(x*y+a)+2*b)*z + (x*y-a)^2 - 4*b*(x+y)

theorem twist_scaling_identity {R : Type*} [CommRing R] (a b d x y z : R) :
    s3 (d^2*a) (d^3*b) (d*x) (d*y) (d*z) = d^4*s3 a b x y z := by
  unfold s3
  ring

section LeadingForm
variable {R : Type*} [CommRing R]
open MvPolynomial

noncomputable abbrev x : MvPolynomial (Fin 2) R := X 0
noncomputable abbrev y : MvPolynomial (Fin 2) R := X 1

noncomputable def equation (a b z : R) : MvPolynomial (Fin 2) R := s3 (C a) (C b) x y (C z)

theorem equation_expansion (a b z : R) :
    equation a b z = x^2*y^2 +
      C (-2*z) * (x^2*y + x*y^2) +
      (C (z^2) * (x^2 + C (-2) * (x*y) + y^2) + C (-2*a)*(x*y)) +
      C (-2*a*z-4*b) * (x+y) + C (a^2-4*b*z) := by
  simp only [equation, s3, map_sub, map_mul, map_neg, map_pow, map_ofNat]
  ring

theorem s3_leading_component (a b z : R) :
    homogeneousComponent 4 (equation a b z) = x^2*y^2 := by
  have hx := isHomogeneous_X R (0 : Fin 2)
  have hy := isHomogeneous_X R (1 : Fin 2)
  have hx2 := isHomogeneous_X_pow (R := R) (0 : Fin 2) 2
  have hy2 := isHomogeneous_X_pow (R := R) (1 : Fin 2) 2
  have h4 : (x^2*y^2 : MvPolynomial (Fin 2) R).IsHomogeneous 4 := hx2.mul hy2
  have h3 : (C (-2*z)*(x^2*y+x*y^2)).IsHomogeneous 3 :=
    ((hx2.mul hy).add (hx.mul hy2)).C_mul _
  have h2 : (C (z^2)*(x^2+C (-2)*(x*y)+y^2)+C (-2*a)*(x*y)).IsHomogeneous 2 :=
    ((hx2.add ((hx.mul hy).C_mul _)).add hy2).C_mul _ |>.add ((hx.mul hy).C_mul _)
  have h1 : (C (-2*a*z-4*b)*(x+y)).IsHomogeneous 1 := (hx.add hy).C_mul _
  have h0 : (C (a^2-4*b*z) : MvPolynomial (Fin 2) R).IsHomogeneous 0 :=
    isHomogeneous_C _ _
  rw [equation_expansion]
  simp only [map_add, homogeneousComponent_of_mem h4, homogeneousComponent_of_mem h3,
    homogeneousComponent_of_mem h2, homogeneousComponent_of_mem h1,
    homogeneousComponent_of_mem h0]
  norm_num

theorem twist_leading_component_equal (a b d z zt : R) :
    homogeneousComponent 4 (equation (d^2*a) (d^3*b) zt) =
      homogeneousComponent 4 (equation a b z) := by
  rw [s3_leading_component, s3_leading_component]

theorem quadratic_base_leading (i : Fin 2) (u v : R) :
    homogeneousComponent 2 ((X i-C u)*(X i-C v)) = (X i : MvPolynomial (Fin 2) R)^2 := by
  have h : (X i-C u)*(X i-C v) =
      (X i : MvPolynomial (Fin 2) R)^2 + C (-u-v)*X i + C (u*v) := by
    simp only [map_sub,map_mul,map_neg]
    ring
  have h2 := isHomogeneous_X_pow (R := R) i 2
  have h1 := (isHomogeneous_X R i).C_mul (-u-v)
  have h0 := isHomogeneous_C (Fin 2) (u*v)
  rw [h]
  simp only [map_add,homogeneousComponent_of_mem h2,homogeneousComponent_of_mem h1,
    homogeneousComponent_of_mem h0]
  norm_num

theorem cubic_base_leading (i : Fin 2) (u v w : R) :
    homogeneousComponent 3 ((X i-C u)*(X i-C v)*(X i-C w)) =
      (X i : MvPolynomial (Fin 2) R)^3 := by
  have h : (X i-C u)*(X i-C v)*(X i-C w) =
      (X i : MvPolynomial (Fin 2) R)^3 + C (-u-v-w)*X i^2 +
      C (u*v+u*w+v*w)*X i + C (-u*v*w) := by
    simp only [map_add,map_sub,map_mul,map_neg]
    ring
  have h3 := isHomogeneous_X_pow (R := R) i 3
  have h2 := (isHomogeneous_X_pow (R := R) i 2).C_mul (-u-v-w)
  have h1 := (isHomogeneous_X R i).C_mul (u*v+u*w+v*w)
  have h0 := isHomogeneous_C (Fin 2) (-u*v*w)
  rw [h]
  rw [map_add,map_add,map_add,homogeneousComponent_of_mem h3,homogeneousComponent_of_mem h2,
    homogeneousComponent_of_mem h1,homogeneousComponent_of_mem h0]
  norm_num

end LeadingForm

/-- Divisibility by the three ACTUAL leading monomials x^b,y^b,x^2*y^2. -/
def headKilled (b i j : ℕ) : Prop := b ≤ i ∨ b ≤ j ∨ (2 ≤ i ∧ 2 ≤ j)

theorem high_degree_killed (b i j : ℕ) (_hb : 2 ≤ b) (hd : b < i+j) :
    headKilled b i j := by
  unfold headKilled
  omega

theorem degree_b_survives (b : ℕ) (hb : 2 ≤ b) :
    (b-1)+1=b ∧ ¬headKilled b (b-1) 1 := by
  unfold headKilled
  omega

/-- The standard-monomial first-empty degree of this leading-form ideal.
This is not a theorem identifying it with an F4/F5 runtime or closure degree. -/
theorem homogeneous_cutoff_exact (b : ℕ) (hb : 2 ≤ b) :
    (∀ i j, i+j=b+1 → headKilled b i j) ∧
      (∃ i j, i+j=b ∧ ¬headKilled b i j) := by
  constructor
  · intro i j hij
    exact high_degree_killed b i j hb (by omega)
  · exact ⟨b-1,1,degree_b_survives b hb⟩

noncomputable def exponent (i j : ℕ) : Fin 2 →₀ ℕ :=
  Finsupp.single 0 i + Finsupp.single 1 j

theorem exponent_le_iff (i j m n : ℕ) :
    exponent i j ≤ exponent m n ↔ i ≤ m ∧ j ≤ n := by
  constructor
  · intro h
    exact ⟨by simpa [exponent] using h 0, by simpa [exponent] using h 1⟩
  · rintro ⟨hi,hj⟩ v
    fin_cases v
    · simpa [exponent] using hi
    · simpa [exponent] using hj

noncomputable def headIdeal (R : Type*) [CommRing R] (b : ℕ) :
    Ideal (MvPolynomial (Fin 2) R) :=
  Ideal.span ((fun e => MvPolynomial.monomial e (1 : R)) ''
    {exponent b 0, exponent 0 b, exponent 2 2})

theorem headIdeal_generators {R : Type*} [CommRing R] (b : ℕ) :
    headIdeal R b = Ideal.span ({x^2*y^2,x^b,y^b} : Set (MvPolynomial (Fin 2) R)) := by
  have hm (i j : ℕ) : MvPolynomial.monomial (exponent i j) (1 : R) =
      (x^i*y^j : MvPolynomial (Fin 2) R) := by
    simp only [exponent,x,y,MvPolynomial.X_pow_eq_monomial,
      MvPolynomial.monomial_mul_monomial,one_mul]
  unfold headIdeal
  simp only [Set.image_insert_eq,Set.image_singleton,hm,pow_zero,mul_one,one_mul]
  congr 1
  ext f
  simp only [Set.mem_insert_iff,Set.mem_singleton_iff]
  tauto

theorem size2_leading_ideal {R : Type*} [CommRing R] (a b z u v : R) :
    Ideal.span ({MvPolynomial.homogeneousComponent 4 (equation a b z),
      MvPolynomial.homogeneousComponent 2 ((x-MvPolynomial.C u)*(x-MvPolynomial.C v)),
      MvPolynomial.homogeneousComponent 2 ((y-MvPolynomial.C u)*(y-MvPolynomial.C v))} :
      Set (MvPolynomial (Fin 2) R)) = headIdeal R 2 := by
  rw [s3_leading_component,quadratic_base_leading,quadratic_base_leading,headIdeal_generators]

theorem size3_leading_ideal {R : Type*} [CommRing R] (a b z u v w : R) :
    Ideal.span ({MvPolynomial.homogeneousComponent 4 (equation a b z),
      MvPolynomial.homogeneousComponent 3 ((x-MvPolynomial.C u)*(x-MvPolynomial.C v)*(x-MvPolynomial.C w)),
      MvPolynomial.homogeneousComponent 3 ((y-MvPolynomial.C u)*(y-MvPolynomial.C v)*(y-MvPolynomial.C w))} :
      Set (MvPolynomial (Fin 2) R)) = headIdeal R 3 := by
  rw [s3_leading_component,cubic_base_leading,cubic_base_leading,headIdeal_generators]

theorem monomial_member_iff {R : Type*} [CommRing R] [Nontrivial R]
    (b i j : ℕ) : MvPolynomial.monomial (exponent i j) (1 : R) ∈ headIdeal R b ↔
      headKilled b i j := by
  classical
  rw [headIdeal, MvPolynomial.mem_ideal_span_monomial_image]
  simp [MvPolynomial.support_monomial, exponent_le_iff, headKilled]

theorem actual_head_ideal_cutoff {R : Type*} [CommRing R] [Nontrivial R]
    (b : ℕ) (hb : 2 ≤ b) :
    (∀ i j, i+j=b+1 → MvPolynomial.monomial (exponent i j) (1 : R) ∈ headIdeal R b) ∧
      (∃ i j, i+j=b ∧ MvPolynomial.monomial (exponent i j) (1 : R) ∉ headIdeal R b) := by
  simpa only [monomial_member_iff] using homogeneous_cutoff_exact b hb

namespace F43
open WeierstrassCurve.Affine TwistEmbedding
abbrev k := PairingFixture.k
abbrev E : WeierstrassCurve.Affine k := short 1 0
abbrev T : WeierstrassCurve.Affine k := short 4 0
instance : T.IsElliptic := by
  constructor
  apply isUnit_iff_ne_zero.mpr
  decide

def ep (x y : k) (h : y^2=x^3+x) : E.Point :=
  Point.some x y ((equation_iff_nonsingular_of_Δ_ne_zero
    (WeierstrassCurve.isUnit_Δ E).ne_zero).mp (by simpa [equation_iff,E,short] using h))
def tp (x y : k) (h : y^2=x^3+4*x) : T.Point :=
  Point.some x y ((equation_iff_nonsingular_of_Δ_ne_zero
    (WeierstrassCurve.isUnit_Δ T).ne_zero).mp (by simpa [equation_iff,T,short] using h))

def P : E.Point := ep 31 18 (by decide)
def U : T.Point := tp 24 26 (by decide)
def mainBase : Fin 2 → E.Point := ![ep 4 5 (by decide),ep 13 19 (by decide)]
def twistBase : Fin 2 → T.Point := ![tp 15 9 (by decide),tp 17 6 (by decide)]

/-- This particular quadratic twist is ALSO base-field isomorphic to E.
The map is x↦41x,y↦11y; no scalar-label table constructs it. -/
noncomputable def toTwist : E.Point →+ T.Point :=
  TwistGroupMap.liftHom (4 : k) 0 21 (by decide) (35 : k) (by decide)

theorem toTwist_injective : Function.Injective toTwist :=
  TwistEmbedding.lift_injective (4 : k) 0 21 (by decide) (35 : k) (by decide)

theorem source_image : toTwist P = U := by
  change TwistEmbedding.lift (4 : k) 0 21 (by decide) (35 : k) (by decide) P = U
  simp only [TwistEmbedding.lift,P,U,ep,tp]
  congr 1 <;> decide +kernel

theorem scalar_reduction (A B : E.Point) (n : ℤ) :
    toTwist B = n • toTwist A ↔ B = n • A :=
  RelationYield.scalar_solution_iff toTwist toTwist_injective A B n

def signed {G : Type*} [AddCommGroup G] (s : Fin 2) (A : G) : G :=
  if s=0 then A else -A

theorem main_first_target_has_no_decomposition :
    ∀ i j s t : Fin 2, signed s (mainBase i) + signed t (mainBase j) ≠ P := by
  decide +kernel

theorem twist_first_relation : twistBase 0 + twistBase 1 = U := by decide +kernel
theorem twist_second_relation : -(twistBase 0 + twistBase 0) = U := by decide +kernel

theorem independent_row_determinant : (-2 : ZMod 11)*1-0*1 ≠ 0 := by decide

theorem recovered_base_logs : ∀ a b : ZMod 11,
    (-2*a=1 ∧ a+b=1) ↔ (a=5 ∧ b=7) := by decide +kernel

end F43

#print axioms twist_scaling_identity
#print axioms equation_expansion
#print axioms s3_leading_component
#print axioms twist_leading_component_equal
#print axioms quadratic_base_leading
#print axioms cubic_base_leading
#print axioms high_degree_killed
#print axioms degree_b_survives
#print axioms homogeneous_cutoff_exact
#print axioms exponent_le_iff
#print axioms headIdeal_generators
#print axioms size2_leading_ideal
#print axioms size3_leading_ideal
#print axioms monomial_member_iff
#print axioms actual_head_ideal_cutoff
#print axioms F43.toTwist_injective
#print axioms F43.source_image
#print axioms F43.scalar_reduction
#print axioms F43.main_first_target_has_no_decomposition
#print axioms F43.twist_first_relation
#print axioms F43.twist_second_relation
#print axioms F43.independent_row_determinant
#print axioms F43.recovered_base_logs
end S3System
