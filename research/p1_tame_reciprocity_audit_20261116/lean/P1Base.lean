import FinitePlaceCertificate
import P1Imports

namespace P1TameReciprocityAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit EllipticFinitePlaceProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

/-- A local auxiliary model. Global nonsingularity is not asserted. -/
def auxiliaryModel (a : K) : WeierstrassCurve.Affine K :=
  ⟨0,-3*a,1,3*a^2,-a^3⟩

theorem auxiliary_nonsingular (a : K) : (auxiliaryModel a).Nonsingular a 0 := by
  rw [nonsingular_iff',equation_iff]
  constructor
  · dsimp [auxiliaryModel]
    ring
  · right
    simp [auxiliaryModel]

theorem auxiliary_split (a : K) :
    (Point.some a 0 (auxiliary_nonsingular a) : (auxiliaryModel a).Point) ≠
      -Point.some a 0 (auxiliary_nonsingular a) := by
  intro h
  rw [Point.neg_some] at h
  have hy := (Point.some.inj h).2
  simp [negY,auxiliaryModel] at hy

def p1FiniteResidue (a : K) : FiniteBaseRing a →+* K :=
  finitePlaceEvaluation (auxiliaryModel a) (auxiliary_nonsingular a)

def p1FiniteUnitResidue (a : K) : (FiniteBaseRing a).unitGroup →* Kˣ :=
  finitePlaceUnitEvaluation (auxiliaryModel a) (auxiliary_nonsingular a)

def p1FiniteTame (a : K) (f g : (RatFunc K)ˣ) : Kˣ :=
  finitePlaceTameSymbol (auxiliaryModel a) (auxiliary_nonsingular a) f g

theorem p1_finite_residue_realized (W : WeierstrassCurve.Affine K) {a b : K}
    (h : W.Nonsingular a b) (r : FiniteBaseRing a) :
    p1FiniteResidue a r=finitePlaceEvaluation W h r :=
  finite_place_eval_model_independent _ _ (auxiliary_nonsingular a) h r

theorem p1_finite_unit_realized (W : WeierstrassCurve.Affine K) {a b : K}
    (h : W.Nonsingular a b) (r : (FiniteBaseRing a).unitGroup) :
    p1FiniteUnitResidue a r=finitePlaceUnitEvaluation W h r :=
  finite_place_unit_model_independent _ _ (auxiliary_nonsingular a) h r

theorem p1_finite_tame_realized (W : WeierstrassCurve.Affine K) {a b : K}
    (h : W.Nonsingular a b) (f g : (RatFunc K)ˣ) :
    p1FiniteTame a f g=finitePlaceTameSymbol W h f g :=
  finite_place_tame_model_independent _ _ (auxiliary_nonsingular a) h f g

theorem p1_finite_residue_constant (a c : K) :
    p1FiniteResidue a (finiteBaseConstant a c)=c :=
  finite_place_eval_constant _ (auxiliary_nonsingular a) c

theorem p1_finite_residue_surjective (a : K) : Function.Surjective (p1FiniteResidue a) :=
  finite_place_eval_surjective _ (auxiliary_nonsingular a)

theorem p1_finite_residue_kernel (a : K) :
    RingHom.ker (p1FiniteResidue a)=IsLocalRing.maximalIdeal (FiniteBaseRing a) :=
  finite_place_eval_kernel _ (auxiliary_nonsingular a)

theorem infinity_residue_model_independent (W V : WeierstrassCurve.Affine K)
    (r : BaseInfinityRing K) : baseEvaluation W r=baseEvaluation V r :=
  base_evaluation_model_independent W V r

theorem infinity_unit_model_independent (W V : WeierstrassCurve.Affine K)
    (r : (BaseInfinityRing K).unitGroup) : baseUnitEvaluation W r=baseUnitEvaluation V r := by
  apply Units.ext
  exact infinity_residue_model_independent W V _

def p1InfinityResidue : BaseInfinityRing K →+* K := baseEvaluation (auxiliaryModel (0 : K))

def p1InfinityUnitResidue : (BaseInfinityRing K).unitGroup →* Kˣ :=
  baseUnitEvaluation (auxiliaryModel (0 : K))

def p1InfinityTame (f g : (RatFunc K)ˣ) : Kˣ := baseTameSymbol (auxiliaryModel (0 : K)) f g

theorem p1_infinity_tame_realized (W : WeierstrassCurve.Affine K) (f g : (RatFunc K)ˣ) :
    p1InfinityTame f g=baseTameSymbol W f g := by
  rw [p1InfinityTame,baseTameSymbol,baseTameSymbol,infinity_unit_model_independent (auxiliaryModel 0) W]

#print axioms auxiliary_nonsingular
#print axioms auxiliary_split
#print axioms p1_finite_residue_realized
#print axioms p1_finite_unit_realized
#print axioms p1_finite_tame_realized
#print axioms p1_finite_residue_kernel
#print axioms infinity_residue_model_independent
#print axioms p1_infinity_tame_realized
end
end P1TameReciprocityAudit
