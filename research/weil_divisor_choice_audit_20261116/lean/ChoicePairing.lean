import ChoiceMoving

namespace WeilDivisorChoiceAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit WeilAuxiliaryRatioAudit EllipticPicardBridgeAudit
open EllipticGlobalReciprocityAudit
variable {K : Type*} [Field K]

local instance choiceFunctionMonoid (W : WeierstrassCurve.Affine K) : Monoid W.FunctionField :=
  (inferInstance : Field W.FunctionField).toMonoid

/-- Every proper-domain guard is derived from the actual divisor equality. -/
theorem domain_of_multiple (W : WeierstrassCurve.Affine K) (D E : W.Point →₀ ℤ)
    (n : ℤ) (f : W.FunctionFieldˣ) (hf : functionDivisor W f f.ne_zero=n • D)
    (hd : Disjoint E.support D.support) :
    Disjoint E.support (functionDivisor W f f.ne_zero).support := by
  apply Finset.disjoint_left.mpr
  intro P hE hF
  have hz : D P=0 := Finsupp.notMem_support_iff.mp (fun hm=>Finset.disjoint_left.mp hd hE hm)
  apply Finsupp.mem_support_iff.mp hF
  rw [hf,Finsupp.smul_apply,hz,smul_zero]

/-- Actual principal divisor, named to keep dependent structure elaboration bounded. -/
def unitDivisor (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ) : W.Point →₀ ℤ :=
  functionDivisor W (f : W.FunctionField) f.ne_zero

/-- Actual inputs, bundled as ordinary products rather than a costly dependent recursor. -/
abbrev PairInputs (W : WeierstrassCurve.Affine K) :=
  ((W.Point →₀ ℤ) × (W.Point →₀ ℤ)) × (W.FunctionFieldˣ × W.FunctionFieldˣ)

def PairCondition (W : WeierstrassCurve.Affine K) (n : ℤ) (A : PairInputs W) : Prop :=
  divisorDegree W A.1.1=0 ∧ divisorDegree W A.1.2=0 ∧
  Disjoint A.1.1.support A.1.2.support ∧
  unitDivisor W A.2.1=n • A.1.1 ∧ unitDivisor W A.2.2=n • A.1.2

def DivisorPair (W : WeierstrassCurve.Affine K) (n : ℤ) :=
  {A : PairInputs W // PairCondition W n A}

namespace DivisorPair
variable {W : WeierstrassCurve.Affine K} {n : ℤ}
def D (A : DivisorPair W n) : W.Point →₀ ℤ := A.val.1.1
def E (A : DivisorPair W n) : W.Point →₀ ℤ := A.val.1.2
def f (A : DivisorPair W n) : W.FunctionFieldˣ := A.val.2.1
def g (A : DivisorPair W n) : W.FunctionFieldˣ := A.val.2.2
theorem degD (A : DivisorPair W n) : divisorDegree W A.D=0 := A.property.1
theorem degE (A : DivisorPair W n) : divisorDegree W A.E=0 := A.property.2.1
theorem disjoint (A : DivisorPair W n) : Disjoint A.D.support A.E.support := A.property.2.2.1
theorem divF (A : DivisorPair W n) : functionDivisor W A.f A.f.ne_zero=n • A.D := A.property.2.2.2.1
theorem divG (A : DivisorPair W n) : functionDivisor W A.g A.g.ne_zero=n • A.E := A.property.2.2.2.2
end DivisorPair

def pairValue (W : WeierstrassCurve.Affine K) (n : ℤ) (A : DivisorPair W n) : Kˣ :=
  evaluateDivisor W A.D A.g (domain_of_multiple W A.E A.D n A.g A.divG A.disjoint)/
    evaluateDivisor W A.E A.f (domain_of_multiple W A.D A.E n A.f A.divF A.disjoint.symm)

theorem pair_value_helper (W : WeierstrassCurve.Affine K) (n : ℤ) (A : DivisorPair W n) :
    pairValue W n A=(exponentEvaluation W A.g A.D).toMul/(exponentEvaluation W A.f A.E).toMul := by
  simp only [pairValue,proper_evaluation_helper]

def swapPair (W : WeierstrassCurve.Affine K) (n : ℤ) (A : DivisorPair W n) : DivisorPair W n :=
  ⟨⟨⟨A.E,A.D⟩,⟨A.g,A.f⟩⟩,A.degE,A.degD,A.disjoint.symm,A.divG,A.divF⟩

theorem pair_value_swap (W : WeierstrassCurve.Affine K) (n : ℤ) (A : DivisorPair W n) :
    pairValue W n (swapPair W n A)=(pairValue W n A)⁻¹ := by
  rw [pair_value_helper,pair_value_helper]
  change (exponentEvaluation W A.f A.E).toMul/(exponentEvaluation W A.g A.D).toMul=_
  rw [inv_div]


variable [IsAlgClosed K]

/-- Roots-of-unity membership follows from full actual geometric reciprocity. -/
theorem pair_value_root (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℤ) (A : DivisorPair W n) : pairValue W n A^n=1 := by
  have hfg : Disjoint (functionDivisor W A.f A.f.ne_zero).support
      (functionDivisor W A.g A.g.ne_zero).support := by
    rw [A.divF]
    exact Finset.disjoint_of_subset_left Finsupp.support_smul
      (domain_of_multiple W A.E A.D n A.g A.divG A.disjoint)
  have hr:=actual_geometric_disjoint_evaluation W hΔ A.f A.g hfg
  rw [proper_evaluation_helper,proper_evaluation_helper,A.divG,A.divF,
    (exponentEvaluation W A.f).map_zsmul n A.E,
    (exponentEvaluation W A.g).map_zsmul n A.D,toMul_zsmul,toMul_zsmul] at hr
  rw [pair_value_helper,div_zpow,hr]
  exact div_self' _

/-- The source formula is independent of the choice of functions with these actual divisors. -/
theorem pair_value_function_choice (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℤ) (A B : DivisorPair W n) (hD : A.D=B.D) (hE : A.E=B.E) :
    pairValue W n A=pairValue W n B := by
  have hg : functionDivisor W A.g A.g.ne_zero=functionDivisor W B.g B.g.ne_zero := by
    rw [A.divG,B.divG,hE]
  have hf : functionDivisor W A.f A.f.ne_zero=functionDivisor W B.f B.f.ne_zero := by
    rw [A.divF,B.divF,hD]
  have hAg := domain_of_multiple W A.E A.D n A.g A.divG A.disjoint
  have hBg := domain_of_multiple W B.E A.D n B.g B.divG (by rw [←hE];exact A.disjoint)
  have hAf := domain_of_multiple W A.D A.E n A.f A.divF A.disjoint.symm
  have hBf := domain_of_multiple W B.D A.E n B.f B.divF (by rw [←hD];exact A.disjoint.symm)
  have hvG := geometric_degree_zero_choice W hΔ A.D A.degD A.g B.g hAg hBg hg
  have hvF := geometric_degree_zero_choice W hΔ A.E A.degE A.f B.f hAf hBf hf
  simp only [proper_evaluation_helper] at hvG hvF
  rw [pair_value_helper,pair_value_helper,hvG,hvF,hD,hE]

#print axioms domain_of_multiple
#print axioms pair_value_root
#print axioms pair_value_function_choice
end
end WeilDivisorChoiceAudit
