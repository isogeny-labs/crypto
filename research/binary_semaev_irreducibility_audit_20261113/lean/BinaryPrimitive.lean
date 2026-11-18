import BinaryFunctionField
import PrimitiveBridge
import Mathlib.FieldTheory.IsAlgClosed.AlgebraicClosure

namespace BinarySemaevIrreducibilityAudit
open Polynomial SemaevIrreducibilityAudit
noncomputable section
variable {K : Type*} [Field K] [CharP K 2]

def binaryLiteralLeading : TwoVar K := (coefficientX+coefficientY)^2
def binaryLiteralMiddle : TwoVar K := coefficientX*coefficientY
def binaryLiteralTrailing (b : K) : TwoVar K := (coefficientX*coefficientY)^2+coefficientParameter b
def binaryLiteral (b : K) : ThreeVar K :=
  C binaryLiteralLeading*X^2+C binaryLiteralMiddle*X+C (binaryLiteralTrailing b)

theorem binaryLiteralLeading_diagonal :
    (binaryLiteralLeading : TwoVar K)=diagonal^2 := by
  unfold binaryLiteralLeading diagonal coefficientX coefficientY
  rw [CharTwo.sub_eq_add]
  ring

omit [CharP K 2] in
theorem diagonal_not_dvd_binary_middle :
    ¬ (diagonal : TwoVar K)∣binaryLiteralMiddle := by
  intro hd
  have he : (binaryLiteralMiddle : TwoVar K).eval (X : Polynomial K)=0 :=
    Polynomial.dvd_iff_isRoot.mp hd
  have hv : (binaryLiteralMiddle : TwoVar K).eval (X : Polynomial K)=X^2 := by
    simp [binaryLiteralMiddle,coefficientX,coefficientY,pow_two]
  rw [hv] at he
  exact pow_ne_zero 2 Polynomial.X_ne_zero he

omit [CharP K 2] in
theorem diagonal_binary_middle_relPrime :
    IsRelPrime (diagonal : TwoVar K) binaryLiteralMiddle := by
  intro d hd hm
  obtain ⟨e,he⟩ := hd
  have hir : Irreducible (diagonal : TwoVar K) :=
    Polynomial.irreducible_X_sub_C (X : Polynomial K)
  rcases hir.isUnit_or_isUnit he with hu | hu
  · exact hu
  · have hh : (diagonal : TwoVar K)∣d := hu.dvd_mul_right.mp (dvd_of_eq he)
    exact (diagonal_not_dvd_binary_middle (dvd_trans hh hm)).elim

theorem binaryLiteral_coeff_two (b : K) :
    (binaryLiteral b).coeff 2=(diagonal : TwoVar K)^2 := by
  simp only [binaryLiteral,coeff_add,coeff_C_mul,coeff_X_pow,coeff_X,coeff_C]
  norm_num [binaryLiteralLeading_diagonal]

omit [CharP K 2] in
theorem binaryLiteral_coeff_one (b : K) :
    (binaryLiteral b).coeff 1=binaryLiteralMiddle := by simp [binaryLiteral]

theorem binaryLiteral_primitive (b : K) : (binaryLiteral b).IsPrimitive := by
  intro r hr
  have hd := (Polynomial.C_dvd_iff_dvd_coeff r (binaryLiteral b)).mp hr
  have hr2 : r∣(diagonal : TwoVar K)^2 := by simpa only [binaryLiteral_coeff_two] using hd 2
  have hr1 : r∣binaryLiteralMiddle := by simpa only [binaryLiteral_coeff_one] using hd 1
  have hrel : IsRelPrime ((diagonal : TwoVar K)^2) binaryLiteralMiddle := by
    simpa only [pow_two] using
      (diagonal_binary_middle_relPrime (K:=K)).mul_left diagonal_binary_middle_relPrime
  exact hrel hr2 hr1

def baseCoefficientMap {L : Type*} [Field L] (f : K →+* L) : TwoVar K →+* Outer L :=
  (coefficientEmbedding (K:=L)).comp
    (Polynomial.mapRingHom (Polynomial.mapRingHom f))

omit [CharP K 2] in
theorem baseCoefficientMap_injective {L : Type*} [Field L] (f : K →+* L) :
    Function.Injective (baseCoefficientMap f) := by
  exact coefficientEmbedding_injective.comp
    (Polynomial.map_injective _ (Polynomial.map_injective _ f.injective))

omit [CharP K 2] in
theorem binaryLiteral_map {L : Type*} [Field L] [CharP L 2] (f : K →+* L) (b : K) :
    (binaryLiteral b).map (baseCoefficientMap f)=binaryQuadratic (f b) := by
  simp [binaryLiteral,binaryLiteralLeading,binaryLiteralMiddle,binaryLiteralTrailing,
    baseCoefficientMap,coefficientEmbedding,binaryQuadratic,binaryLeading,binaryMiddle,
    binaryTrailing,coefficientX,coefficientY,coefficientParameter,genericX,genericY,parameter]

theorem binaryQuadratic_irreducible_algClosed [IsAlgClosed K] (b : K) :
    Irreducible (binaryQuadratic b) := by
  obtain ⟨s,hs⟩ := IsAlgClosed.exists_pow_nat_eq b (n:=2) (by decide)
  rw [← hs]
  exact binary_function_field_irreducible s

/-- The literal ordinary-binary polynomial is irreducible over every
characteristic-two field, without a square-root or nonsingularity premise. -/
theorem binaryLiteral_irreducible (b : K) : Irreducible (binaryLiteral b) := by
  apply (binaryLiteral_primitive b).irreducible_of_irreducible_map_of_injective
    (baseCoefficientMap_injective (algebraMap K (AlgebraicClosure K)))
  rw [binaryLiteral_map]
  exact binaryQuadratic_irreducible_algClosed _

#print axioms binaryLiteral_irreducible
end
end BinarySemaevIrreducibilityAudit
