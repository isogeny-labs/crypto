import LawsAlternating

namespace WeilGeometricLawsAudit
noncomputable section
open WeierstrassCurve.Affine
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit FinitePointEvaluationAudit
open WeilDivisorChoiceAudit EllipticPicardBridgeAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

def raiseLevel (W : WeierstrassCurve.Affine K) (n m : ℤ)
    (Q : torsionPoints W n) : torsionPoints W (n*m) :=
  ⟨Q.val,by
    change (n*m) • Q.val=0
    rw [mul_comm n m,mul_smul,Q.property,smul_zero]⟩

def lowerLevel (W : WeierstrassCurve.Affine K) (n m : ℤ)
    (P : torsionPoints W (n*m)) : torsionPoints W n :=
  ⟨m • P.val,by change n • (m • P.val)=0;rw [smul_smul];exact P.property⟩

/-- Actual source26.2.3(5) level compatibility from genuine functions and divisors. -/
theorem geometric_pairing_level (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n m : ℤ)
    (P : torsionPoints W (n*m)) (Q : torsionPoints W n) :
    geometricPairing W hΔ (n*m) P (raiseLevel W n m Q)=
      geometricPairing W hΔ n (lowerLevel W n m P) Q := by
  let A:=selectedPair W hΔ (n*m) P (raiseLevel W n m Q)
  have hP : divisorPointSum W A.D=P.val := (selected_pair_classes W hΔ (n*m) P (raiseLevel W n m Q)).1
  have hQ : divisorPointSum W A.E=Q.val := (selected_pair_classes W hΔ (n*m) P (raiseLevel W n m Q)).2
  obtain ⟨g,hg⟩:=actual_multiple_principal W hΔ n A.E A.degE (by rw [hQ];exact Q.property)
  have hgm : functionDivisor W (↑(g^m) : W.FunctionField) (g^m).ne_zero=(n*m) • A.E := by
    rw [unit_function_divisor_zpow,hg,smul_smul,mul_comm m n]
  let H : DivisorPair W (n*m) := ⟨⟨⟨A.D,A.E⟩,⟨A.f,g^m⟩⟩,A.degD,A.degE,A.disjoint,A.divF,hgm⟩
  have hdf : functionDivisor W A.f A.f.ne_zero=n • (m • A.D) := by rw [A.divF,smul_smul]
  let L : DivisorPair W n := ⟨⟨⟨m • A.D,A.E⟩,⟨A.f,g⟩⟩,
    by rw [map_zsmul,A.degD,smul_zero],A.degE,
    Finset.disjoint_of_subset_left Finsupp.support_smul A.disjoint,hdf,hg⟩
  have hsum : divisorPointSum W L.D=(lowerLevel W n m P).val := by
    change divisorPointSum W (m • A.D)=m • P.val
    rw [map_zsmul,hP]
  have hdg:=domain_of_multiple W A.E A.D n g hg A.disjoint
  have hdgm:=domain_of_multiple W A.E A.D (n*m) (g^m) hgm A.disjoint
  have he:=proper_evaluation_function_zpow W A.D g m hdg hdgm
  simp only [proper_evaluation_helper] at he
  rw [geometric_pairing_of_data W hΔ (n*m) P (raiseLevel W n m Q) H hP hQ,
    geometric_pairing_of_data W hΔ n (lowerLevel W n m P) Q L hsum hQ,
    pair_value_helper,pair_value_helper]
  change (exponentEvaluation W (g^m) A.D).toMul/(exponentEvaluation W A.f A.E).toMul=
    (exponentEvaluation W g (m • A.D)).toMul/(exponentEvaluation W A.f A.E).toMul
  rw [he,(exponentEvaluation W g).map_zsmul m A.D,toMul_zsmul]

#print axioms geometric_pairing_level
end
end WeilGeometricLawsAudit
