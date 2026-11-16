import S3FunctionField
import Mathlib.RingTheory.Polynomial.GaussLemma
import Mathlib.RingTheory.Polynomial.UniqueFactorization

namespace SemaevIrreducibilityAudit
open Polynomial
noncomputable section
variable {K : Type*} [Field K]

abbrev TwoVar (K : Type*) [Field K] := Polynomial (Polynomial K)
abbrev ThreeVar (K : Type*) [Field K] := Polynomial (TwoVar K)
def coefficientX : TwoVar K := C X
def coefficientY : TwoVar K := X
def coefficientParameter (a : K) : TwoVar K := C (C a)
def diagonal : TwoVar K := X-C X

def literalLeading (_a _b : K) : TwoVar K := (coefficientX-coefficientY)^2
def literalMiddle (a b : K) : TwoVar K :=
  -2*((coefficientX+coefficientY)*(coefficientX*coefficientY+coefficientParameter a)+
    2*coefficientParameter b)
def literalTrailing (a b : K) : TwoVar K :=
  (coefficientX*coefficientY-coefficientParameter a)^2-
    4*coefficientParameter b*(coefficientX+coefficientY)
def literalSource (a b : K) : ThreeVar K :=
  C (literalLeading a b)*X^2+C (literalMiddle a b)*X+C (literalTrailing a b)

/-- The actual injective coefficient map K[x,y] -> K(x)(y). -/
def coefficientEmbedding : TwoVar K →+* Outer K :=
  (algebraMap (Polynomial (Inner K)) (Outer K)).comp
    (Polynomial.mapRingHom (algebraMap (Polynomial K) (Inner K)))

theorem coefficientEmbedding_injective :
    Function.Injective (coefficientEmbedding : TwoVar K →+* Outer K) := by
  exact (RatFunc.algebraMap_injective (Inner K)).comp
    (Polynomial.map_injective (algebraMap (Polynomial K) (Inner K))
      (RatFunc.algebraMap_injective K))

theorem literalSource_map (a b : K) :
    (literalSource a b).map coefficientEmbedding=quadratic a b := by
  simp [literalSource,quadratic,coefficientEmbedding,literalLeading,literalMiddle,
    literalTrailing,leading,middle,trailing,coefficientX,coefficientY,
    coefficientParameter,genericX,genericY,parameter]
  norm_num only [map_ofNat]

theorem literalLeading_diagonal (a b : K) : literalLeading a b=(diagonal : TwoVar K)^2 := by
  unfold literalLeading diagonal coefficientX coefficientY
  ring

theorem diagonal_not_dvd_middle (a b : K) (h2 : (2:K)≠0) :
    ¬ (diagonal : TwoVar K)∣literalMiddle a b := by
  intro hd
  have he : (literalMiddle a b).eval (X : Polynomial K)=0 :=
    Polynomial.dvd_iff_isRoot.mp hd
  have hv : (literalMiddle a b).eval (X : Polynomial K)= -4*cubic a b := by
    simp [literalMiddle,coefficientX,coefficientY,coefficientParameter,cubic]
    ring
  have h4 : (4:K)≠0 := by
    have hh := pow_ne_zero 2 h2
    norm_num at hh
    exact hh
  have h4p : (4:Polynomial K)≠0 := by
    have hh : C (4:K)≠0 := by simpa using h4
    norm_num only [map_ofNat] at hh
    exact hh
  rw [hv] at he
  exact mul_ne_zero (neg_ne_zero.mpr h4p) (cubic_ne_zero a b) he

theorem diagonal_middle_relPrime (a b : K) (h2 : (2:K)≠0) :
    IsRelPrime (diagonal : TwoVar K) (literalMiddle a b) := by
  intro d hd hm
  obtain ⟨e,he⟩ := hd
  have hir : Irreducible (diagonal : TwoVar K) :=
    Polynomial.irreducible_X_sub_C (X : Polynomial K)
  rcases hir.isUnit_or_isUnit he with hu | hu
  · exact hu
  · have hh : (diagonal : TwoVar K)∣d := hu.dvd_mul_right.mp (dvd_of_eq he)
    exact (diagonal_not_dvd_middle a b h2 (dvd_trans hh hm)).elim

theorem literalSource_coeff_two (a b : K) :
    (literalSource a b).coeff 2=(diagonal : TwoVar K)^2 := by
  simp only [literalSource,coeff_add,coeff_C_mul,coeff_X_pow,coeff_X,coeff_C]
  norm_num [literalLeading_diagonal]

theorem literalSource_coeff_one (a b : K) :
    (literalSource a b).coeff 1=literalMiddle a b := by
  simp [literalSource]

theorem literalSource_isPrimitive (a b : K) (h2 : (2:K)≠0) :
    (literalSource a b).IsPrimitive := by
  intro r hr
  have hd := (Polynomial.C_dvd_iff_dvd_coeff r (literalSource a b)).mp hr
  have hr2 : r∣(diagonal : TwoVar K)^2 := by simpa only [literalSource_coeff_two] using hd 2
  have hr1 : r∣literalMiddle a b := by simpa only [literalSource_coeff_one] using hd 1
  have hrel : IsRelPrime ((diagonal : TwoVar K)^2) (literalMiddle a b) := by
    simpa only [pow_two] using
      (diagonal_middle_relPrime a b h2).mul_left (diagonal_middle_relPrime a b h2)
  exact hrel hr2 hr1

theorem actual_nested_source_irreducible (a b : K) (h2 : (2:K)≠0) :
    Irreducible (literalSource a b) := by
  apply (literalSource_isPrimitive a b h2).irreducible_of_irreducible_map_of_injective
    coefficientEmbedding_injective
  rw [literalSource_map]
  exact actual_function_field_quadratic_irreducible a b h2

#print axioms actual_nested_source_irreducible

end
end SemaevIrreducibilityAudit
