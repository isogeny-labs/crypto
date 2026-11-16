import SplitCertificate
import RamifiedImports

namespace EllipticRamifiedNormProjectionAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- The actual nonzero prime ideal (X-a), independent of any elliptic model. -/
def finiteBasePlace (a : K) : IsDedekindDomain.HeightOneSpectrum K[X] where
  asIdeal:=Ideal.span {X-C a}
  isPrime:=Ideal.isPrime_span_singleton_of_prime (prime_X_sub_C a)
  ne_bot:=by rw [ne_eq,Ideal.span_singleton_eq_bot];exact X_sub_C_ne_zero a

def finiteBaseValuation (a : K) : Valuation (RatFunc K) ℤᵐ⁰ :=
  (finiteBasePlace a).valuation (RatFunc K)

def finiteBaseOrder (a : K) (c : RatFunc K) : ℤ := -WithZero.log (finiteBaseValuation a c)

theorem finite_base_prime_mem (a : K) (p : K[X]) :
    p∈(finiteBasePlace a).asIdeal ↔ p.eval a=0 := by
  change p∈Ideal.span {X-C a} ↔ _
  rw [Ideal.mem_span_singleton,dvd_iff_isRoot,IsRoot.def]

theorem finite_base_polynomial_unit (a : K) (p : K[X]) (hp : p.eval a≠0) :
    finiteBaseValuation a (algebraMap K[X] (RatFunc K) p)=1 := by
  apply (IsDedekindDomain.HeightOneSpectrum.valuation_eq_one_iff_notMem (v:=finiteBasePlace a)).mpr
  rwa [finite_base_prime_mem]

theorem finite_base_polynomial_parameter (a : K) :
    finiteBaseValuation a (algebraMap K[X] (RatFunc K) (X-C a))=WithZero.exp (-1 : ℤ) := by
  rw [finiteBaseValuation,IsDedekindDomain.HeightOneSpectrum.valuation_of_algebraMap]
  exact (finiteBasePlace a).intValuation_singleton (X_sub_C_ne_zero a) rfl

theorem finite_base_parameter_value (a : K) :
    finiteBaseValuation a (baseVerticalUnit a : RatFunc K)=WithZero.exp (-1 : ℤ) :=
  finite_base_polynomial_parameter a

/-- Complete normalized polynomial valuation, using actual rootMultiplicity/cofactor decomposition. -/
theorem finite_base_polynomial_value (a : K) (p : K[X]) (hp : p≠0) :
    finiteBaseValuation a (algebraMap K[X] (RatFunc K) p)=
      WithZero.exp (-(p.rootMultiplicity a : ℤ)) := by
  obtain ⟨q,hp,hq⟩:=p.exists_eq_pow_rootMultiplicity_mul_and_not_dvd hp a
  have hqv : finiteBaseValuation a (algebraMap K[X] (RatFunc K) q)=1 := by
    apply finite_base_polynomial_unit
    simpa only [dvd_iff_isRoot,IsRoot.def] using hq
  conv_lhs => rw [hp]
  rw [map_mul,map_pow,map_mul,map_pow,finite_base_polynomial_parameter,hqv,mul_one]
  rw [←WithZero.exp_nsmul]
  congr 1
  simp

theorem finite_base_valuation_surjective (a : K) : Function.Surjective (finiteBaseValuation a) := by
  intro b
  refine WithZero.expRecOn b ?_ (fun n=>?_)
  · exact ⟨0,map_zero _⟩
  · refine ⟨(baseVerticalUnit a : RatFunc K)^(-n),?_⟩
    rw [map_zpow₀,finite_base_parameter_value,←WithZero.exp_zsmul]
    congr 1
    simp

#print axioms finite_base_polynomial_value
#print axioms finite_base_parameter_value
#print axioms finite_base_valuation_surjective
end
end EllipticRamifiedNormProjectionAudit
