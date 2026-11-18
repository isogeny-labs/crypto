module
public import UnitEvaluation

@[expose] public section
namespace FinitePointEvaluationAudit
noncomputable section
open MillerFinitePointAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
variable {K : Type*} [Field K]

/-- Common-support helper. Its product agrees with actual evaluations when D.support ⊆ T. -/
def divisorEvaluationOn (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
    (D : W.Point →₀ ℤ) : supportUnits W T →* Kˣ :=
  D.prod fun P n => supportPointEvaluation W T P ^ n

/-- Actual multiplicative divisor evaluation on functions unit at every support point. -/
def divisorEvaluation (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) :
    supportUnits W D.support →* Kˣ := divisorEvaluationOn W D.support D

theorem disjoint_principal_support_iff (W : WeierstrassCurve.Affine K)
    (f : W.FunctionFieldˣ) (D : W.Point →₀ ℤ) :
    Disjoint D.support (functionDivisor W f f.ne_zero).support ↔ f ∈ supportUnits W D.support := by
  rw [mem_support_units_iff]
  constructor
  · intro hd P hP
    have hn : P ∉ (functionDivisor W f f.ne_zero).support := Finset.disjoint_left.mp hd hP
    simpa only [Finsupp.mem_support_iff, function_divisor_apply, not_not] using hn
  · intro hu
    apply Finset.disjoint_left.mpr
    intro P hP hF
    have hn : pointOrder W f P ≠ 0 := by
      simpa only [Finsupp.mem_support_iff, function_divisor_apply] using hF
    exact hn (hu P hP)

/-- Source disjoint-support guard is converted to the actual local-unit domain. -/
def evaluateDivisor (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (f : W.FunctionFieldˣ) (hd : Disjoint D.support (functionDivisor W f f.ne_zero).support) : Kˣ :=
  divisorEvaluation W D ⟨f, (disjoint_principal_support_iff W f D).mp hd⟩

theorem disjoint_principal_mul (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (f g : W.FunctionFieldˣ)
    (hf : Disjoint D.support (functionDivisor W f f.ne_zero).support)
    (hg : Disjoint D.support (functionDivisor W g g.ne_zero).support) :
    Disjoint D.support (functionDivisor W (f * g) (f * g).ne_zero).support :=
  (disjoint_principal_support_iff W (f * g) D).mpr
    ((supportUnits W D.support).mul_mem
      ((disjoint_principal_support_iff W f D).mp hf)
      ((disjoint_principal_support_iff W g D).mp hg))

theorem evaluate_divisor_mul (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (f g : W.FunctionFieldˣ)
    (hf : Disjoint D.support (functionDivisor W f f.ne_zero).support)
    (hg : Disjoint D.support (functionDivisor W g g.ne_zero).support) :
    evaluateDivisor W D (f * g) (disjoint_principal_mul W D f g hf hg) =
      evaluateDivisor W D f hf * evaluateDivisor W D g hg :=
  (divisorEvaluation W D).map_mul
    ⟨f, (disjoint_principal_support_iff W f D).mp hf⟩
    ⟨g, (disjoint_principal_support_iff W g D).mp hg⟩

theorem divisor_evaluation_on_add (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
    (D E : W.Point →₀ ℤ) :
    divisorEvaluationOn W T (D + E) = divisorEvaluationOn W T D * divisorEvaluationOn W T E :=
  Finsupp.prod_add_index' (fun P => by
    apply MonoidHom.ext
    intro f
    change (supportPointEvaluation W T P f) ^ (0 : ℤ) = 1
    exact zpow_zero _) (fun P m n => by
    apply MonoidHom.ext
    intro f
    simp only [MonoidHom.zpow_apply, MonoidHom.mul_apply]
    exact zpow_add _ _ _)

theorem divisor_evaluation_on_apply (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
    (D : W.Point →₀ ℤ) (f : supportUnits W T) :
    divisorEvaluationOn W T D f = D.prod fun P n => supportPointEvaluation W T P f ^ n := by
  classical
  simp only [divisorEvaluationOn, Finsupp.prod, MonoidHom.finsetProd_apply, MonoidHom.zpow_apply]

theorem divisor_evaluation_constant (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
    (D : W.Point →₀ ℤ) (hD : D.support ⊆ T) (c : Kˣ) :
    divisorEvaluationOn W T D (supportConstantUnits W T c) = c ^ divisorDegree W D := by
  classical
  rw [divisor_evaluation_on_apply]
  calc
    D.prod (fun P n => supportPointEvaluation W T P (supportConstantUnits W T c) ^ n) =
        D.prod (fun _ n => c ^ n) := by
      apply Finsupp.prod_congr
      intro P hP
      rw [support_point_constant W T P (hD hP)]
    _ = c ^ divisorDegree W D := by
      change (∏ P ∈ D.support, c ^ D P) = c ^ (∑ P ∈ D.support, D P)
      have hs (S : Finset W.Point) : (∏ P ∈ S, c ^ D P) = c ^ (∑ P ∈ S, D P) := by
        induction S using Finset.induction_on with
        | empty => simp
        | @insert P S hP ih =>
          rw [Finset.prod_insert hP, Finset.sum_insert hP, ih, zpow_add]
      exact hs D.support

theorem divisor_evaluation_scalar (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
    (D : W.Point →₀ ℤ) (hD : D.support ⊆ T) (c : Kˣ) (f : supportUnits W T) :
    divisorEvaluationOn W T D (supportConstantUnits W T c * f) =
      c ^ divisorDegree W D * divisorEvaluationOn W T D f := by
  rw [map_mul, divisor_evaluation_constant W T D hD]

theorem degree_zero_scalar_invariance (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
    (D : W.Point →₀ ℤ) (hD : D.support ⊆ T) (hdeg : divisorDegree W D = 0)
    (c : Kˣ) (f : supportUnits W T) :
    divisorEvaluationOn W T D (supportConstantUnits W T c * f) = divisorEvaluationOn W T D f := by
  rw [divisor_evaluation_scalar W T D hD, hdeg, zpow_zero, one_mul]

#print axioms disjoint_principal_support_iff
#print axioms evaluate_divisor_mul
#print axioms degree_zero_scalar_invariance
end
end FinitePointEvaluationAudit
