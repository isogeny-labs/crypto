import DoublingDegreeImports
import FrobeniusDegreeCertificate

namespace EllipticDoublingDegreeAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

def doublingNumerator (W : WeierstrassCurve.Affine K) : K[X] :=
  X^4-C W.b₄*X^2-C (2*W.b₆)*X-C W.b₈

def doublingDenominator (W : WeierstrassCurve.Affine K) : K[X] :=
  C 4*X^3+C W.b₂*X^2+C (2*W.b₄)*X+C W.b₆

def doublingRational (W : WeierstrassCurve.Affine K) : RatFunc K :=
  algebraMap K[X] (RatFunc K) (doublingNumerator W) /
    algebraMap K[X] (RatFunc K) (doublingDenominator W)

theorem doubling_numerator_degree (W : WeierstrassCurve.Affine K) :
    (doublingNumerator W).natDegree=4 := by
  unfold doublingNumerator
  compute_degree!

theorem doubling_denominator_degree_le (W : WeierstrassCurve.Affine K) :
    (doublingDenominator W).natDegree≤3 := by
  unfold doublingDenominator
  compute_degree!

theorem actual_fiber_exists [IsAlgClosed K] (W : WeierstrassCurve.Affine K) (x : K) :
    ∃ y : K, W.Equation x y := by
  have hd : (fiberPolynomial W x).natDegree=2 := by
    unfold fiberPolynomial
    compute_degree!
  obtain ⟨y,hy⟩ := IsAlgClosed.exists_root (fiberPolynomial W x) (by
    rw [degree_eq_natDegree (fiber_polynomial_nonzero W x),hd]
    norm_num)
  exact ⟨y,(equation_fiber_root W x y).mpr hy⟩

theorem doubling_coprime (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    IsCoprime (doublingNumerator W) (doublingDenominator W) := by
  apply (Polynomial.isCoprime_iff_aeval_ne_zero_of_isAlgClosed
    (k:=K) (K:=AlgebraicClosure K) (doublingNumerator W) (doublingDenominator W)).mpr
  intro x
  let V := W.map (algebraMap K (AlgebraicClosure K))
  have hv : V.Δ≠0 := by
    change (W.map (algebraMap K (AlgebraicClosure K))).Δ≠0
    rw [WeierstrassCurve.map_Δ]
    simpa only [map_zero] using (algebraMap K (AlgebraicClosure K)).injective.ne hΔ
  obtain ⟨y,hy⟩ := actual_fiber_exists V x
  have hns := (equation_iff_nonsingular_of_Δ_ne_zero hv).mp hy
  have h := den_duplication_ne_zero_or_num_duplication_ne_zero hns
  simpa only [doublingNumerator,doublingDenominator,aeval_sub,aeval_add,aeval_mul,
    map_pow,aeval_X,aeval_C,V,WeierstrassCurve.map_b₂,WeierstrassCurve.map_b₄,
    WeierstrassCurve.map_b₆,WeierstrassCurve.map_b₈,map_mul,map_ofNat] using h.symm

theorem doubling_denominator_ne_zero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    doublingDenominator W≠0 := by
  intro hz
  have hu : IsUnit (doublingNumerator W) := by simpa only [hz,isCoprime_zero_right] using doubling_coprime W hΔ
  have hd := natDegree_eq_zero_of_isUnit hu
  rw [doubling_numerator_degree] at hd
  norm_num at hd

theorem doubling_reduced_degrees (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    (doublingRational W).num.natDegree=4 ∧ (doublingRational W).denom.natDegree≤3 := by
  classical
  have hc := doubling_coprime W hΔ
  have hg : gcd (doublingNumerator W) (doublingDenominator W)=1 :=
    (normalize_gcd _ _).symm.trans (normalize_eq_one.mpr ((gcd_isUnit_iff _ _).mpr hc))
  have hl : (doublingDenominator W).leadingCoeff⁻¹≠0 :=
    inv_ne_zero (leadingCoeff_ne_zero.mpr (doubling_denominator_ne_zero W hΔ))
  constructor
  · rw [doublingRational,RatFunc.num_div,hg]
    simp only [EuclideanDomain.div_one]
    rw [natDegree_C_mul hl]
    exact doubling_numerator_degree W
  · rw [doublingRational,RatFunc.denom_div _ (doubling_denominator_ne_zero W hΔ),hg]
    simp only [EuclideanDomain.div_one]
    rw [natDegree_C_mul hl]
    exact doubling_denominator_degree_le W

theorem doubling_rational_degree (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Module.finrank (IntermediateField.adjoin K {doublingRational W}) (RatFunc K)=4 := by
  rw [RatFunc.finrank_eq_max_natDegree]
  obtain ⟨hn,hd⟩ := doubling_reduced_degrees W hΔ
  rw [hn,Nat.max_eq_left (by omega)]

theorem doubling_rational_transcendental (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    Transcendental K (doublingRational W) := by
  apply RatFunc.transcendental_of_ne_C
  intro h
  have hz := (RatFunc.eq_C_iff _).mp h
  have hd := (doubling_reduced_degrees W hΔ).1
  omega

#print axioms doubling_coprime
#print axioms doubling_rational_degree
end
end EllipticDoublingDegreeAudit
