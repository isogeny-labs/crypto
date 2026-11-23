import GeneralEvaluation
import TangentCertificate
namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open NegationPullbackAudit EllipticTranslationAudit GeneralTranslationAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- Actual degree-zero torsion divisor, with integer coefficients. -/
def torsionDivisor (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) : W.Point →₀ ℤ :=
  (n : ℤ) • Finsupp.single P 1 - (n : ℤ) • Finsupp.single 0 1

theorem chosen_torsion_divisor (W : WeierstrassCurve.Affine K) (P : W.Point)
    (n : ℕ) (hn : n • P=0) :
    functionDivisor W (millerFunction W P n) (miller_function_nonzero W P n)=torsionDivisor W P n := by
  rw [function_divisor_miller,millerDivisor,hn]
  unfold torsionDivisor
  rw [sub_smul,one_smul]
  abel

omit [DecidableEq K] in
theorem divisor_neg_single (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℤ) :
    divisorNeg W (Finsupp.single P n)=Finsupp.single (-P) n := by
  change Finsupp.equivMapDomain (pointNegEquiv W) (Finsupp.single P n)=_
  rw [Finsupp.equivMapDomain_eq_mapDomain,Finsupp.mapDomain_single]
  rfl

/-- Numerator of the Miller-order auxiliary ratio as an actual rational function of S. -/
def auxiliaryNumerator (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) (f g : W.FunctionField) : W.FunctionField :=
  generalTranslation W hΔ Q f * fieldNeg W g

def auxiliaryDenominator (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P : W.Point) (f g : W.FunctionField) : W.FunctionField :=
  f * generalTranslation W hΔ (-P) (fieldNeg W g)

/-- This order is the inverse of Galbraith26.1, as retained in the source footnote. -/
def auxiliaryQuotient (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f g : W.FunctionField) : W.FunctionField :=
  auxiliaryNumerator W hΔ Q f g / auxiliaryDenominator W hΔ P f g

theorem auxiliary_numerator_nonzero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (Q : W.Point) {f g : W.FunctionField} (hf : f≠0) (hg : g≠0) :
    auxiliaryNumerator W hΔ Q f g≠0 :=
  mul_ne_zero (general_translation_nonzero W hΔ Q hf) (field_neg_nonzero W hg)

theorem auxiliary_denominator_nonzero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P : W.Point) {f g : W.FunctionField} (hf : f≠0) (hg : g≠0) :
    auxiliaryDenominator W hΔ P f g≠0 :=
  mul_ne_zero hf (general_translation_nonzero W hΔ (-P) (field_neg_nonzero W hg))

/-- Actual geometric divisor cancellation, before using any constant-kernel theorem. -/
theorem auxiliary_same_divisor (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (n : ℕ) {f g : W.FunctionField} (hf : f≠0) (hg : g≠0)
    (hDf : functionDivisor W f hf=torsionDivisor W P n)
    (hDg : functionDivisor W g hg=torsionDivisor W Q n) :
    functionDivisor W (auxiliaryNumerator W hΔ Q f g) (auxiliary_numerator_nonzero W hΔ Q hf hg)=
      functionDivisor W (auxiliaryDenominator W hΔ P f g) (auxiliary_denominator_nonzero W hΔ P hf hg) := by
  unfold auxiliaryNumerator auxiliaryDenominator
  rw [function_divisor_mul W (general_translation_nonzero W hΔ Q hf) (field_neg_nonzero W hg),
    function_divisor_mul W hf (general_translation_nonzero W hΔ (-P) (field_neg_nonzero W hg)),
    general_translation_function_divisor W hΔ Q hf,function_divisor_pullback W g hg,
    general_translation_function_divisor W hΔ (-P) (field_neg_nonzero W hg),
    function_divisor_pullback W g hg,hDf,hDg]
  simp only [torsionDivisor,map_sub,map_zsmul,divisor_translation_single,divisor_neg_single,
    neg_zero,zero_add,neg_neg]
  have he : -Q+P=P+ -Q := add_comm _ _
  rw [he]
  abel

/-- Derived scalar identity for actual functions; geometric scope is essential here. -/
theorem auxiliary_scalar [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (n : ℕ) {f g : W.FunctionField} (hf : f≠0) (hg : g≠0)
    (hDf : functionDivisor W f hf=torsionDivisor W P n)
    (hDg : functionDivisor W g hg=torsionDivisor W Q n) :
    ∃ c : K, c≠0 ∧ auxiliaryQuotient W hΔ P Q f g=constantHom W c := by
  obtain ⟨c,hc,he⟩ := (same_divisor_iff_scalar W hΔ
    (auxiliary_numerator_nonzero W hΔ Q hf hg)
    (auxiliary_denominator_nonzero W hΔ P hf hg)).mp
    (auxiliary_same_divisor W hΔ P Q n hf hg hDf hDg)
  refine ⟨c,hc,?_⟩
  exact (div_eq_iff (auxiliary_denominator_nonzero W hΔ P hf hg)).mpr he

/-- Every-n actual chosen Miller quotient is scalar, including zero/degenerate points. -/
theorem chosen_auxiliary_scalar [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (n : ℕ) (hP : n • P=0) (hQ : n • Q=0) :
    ∃ c : K, c≠0 ∧ auxiliaryQuotient W hΔ P Q
      (millerFunction W P n) (millerFunction W Q n)=constantHom W c :=
  auxiliary_scalar W hΔ P Q n (miller_function_nonzero W P n) (miller_function_nonzero W Q n)
    (chosen_torsion_divisor W P n hP) (chosen_torsion_divisor W Q n hQ)

#print axioms auxiliary_same_divisor
#print axioms auxiliary_scalar
#print axioms chosen_auxiliary_scalar
end
end WeilAuxiliaryRatioAudit
