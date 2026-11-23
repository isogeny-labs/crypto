import GaloisAuxiliary

namespace WeilGaloisCovarianceAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerBasechangeAudit FinitePointEvaluationAudit GeneralTranslationAudit MillerFinitePointAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit EllipticPicardBridgeAudit
open WeilAuxiliaryRatioAudit WeilDivisorChoiceAudit WeilGeometricLawsAudit
variable {K L : Type*} [Field K] [Field L] [Algebra K L]
variable [DecidableEq K] [DecidableEq L]

/-- The actual point homomorphism maps the actual integer torsion kernel. -/
def torsionBasechange (W : WeierstrassCurve.Affine K) (n : ℤ) :
    torsionPoints W n →+ torsionPoints (extendedCurve (L := L) W) n where
  toFun P := ⟨pointBaseChange (L := L) W P.val,by
    change n • pointBaseChange (L := L) W P.val=0
    rw [←map_zsmul,P.property,map_zero]⟩
  map_zero' := Subtype.ext (map_zero _)
  map_add' P Q := Subtype.ext (map_add (pointBaseChange W) P.val Q.val)

theorem torsion_basechange_coe (W : WeierstrassCurve.Affine K) (n : ℤ)
    (P : torsionPoints W n) :
    (torsionBasechange (L := L) W n P).val=pointBaseChange (L := L) W P.val := rfl

variable [IsAlgClosed K]

/-- Every proper chosen Miller auxiliary representative computes the selected quotient. -/
theorem geometric_pairing_source (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℕ) (hn : n≠0) (P Q : torsionPoints W (n : ℤ)) (S : W.Point)
    (h : AuxiliaryAdmissible W P.val Q.val (chosenFunctionUnit W P.val n)
      (chosenFunctionUnit W Q.val n) S) :
    geometricPairing W hΔ (n : ℤ) P Q=sourceAuxiliaryValue W hΔ P.val Q.val S
      (chosenFunctionUnit W P.val n) (chosenFunctionUnit W Q.val n) h := by
  have hp : n • P.val=0 := (natCast_zsmul P.val n).symm.trans P.property
  have hq : n • Q.val=0 := (natCast_zsmul Q.val n).symm.trans Q.property
  have hf : functionDivisor W (chosenFunctionUnit W P.val n) (chosenFunctionUnit W P.val n).ne_zero=
      (n : ℤ) • pointRepresentative W P.val := by
    rw [chosen_unit_divisor W P.val n hp]
    simp only [torsionDivisor,pointRepresentative,smul_sub]
  have hg:=shifted_function_divisor W hΔ Q.val S n (chosenFunctionUnit W Q.val n)
    (chosen_unit_divisor W Q.val n hq)
  have hd:=admissible_representatives_disjoint W P.val Q.val S n hn _ _
    (chosen_unit_divisor W P.val n hp) h
  let A : DivisorPair W (n : ℤ) := ⟨⟨⟨pointRepresentative W P.val,shiftedRepresentative W S Q.val⟩,
    ⟨chosenFunctionUnit W P.val n,generalTranslatedUnit W hΔ (-S) (chosenFunctionUnit W Q.val n)⟩⟩,
    point_representative_degree W P.val,shifted_representative_degree W S Q.val,hd,hf,hg⟩
  have hD : divisorPointSum W A.D=P.val := by
    change divisorPointSum W (pointRepresentative W P.val)=P.val
    simp only [pointRepresentative,map_sub,point_sum_single,one_zsmul,sub_zero]
  have hE : divisorPointSum W A.E=Q.val := by
    change divisorPointSum W (shiftedRepresentative W S Q.val)=Q.val
    simp only [shiftedRepresentative,map_sub,point_sum_single,one_zsmul,add_sub_cancel_right]
  rw [geometric_pairing_of_data W hΔ (n : ℤ) P Q A hD hE]
  rfl

theorem geometric_pairing_zero_level (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : torsionPoints W 0) : geometricPairing W hΔ 0 P Q=1 :=
  pair_value_zero W hΔ _

variable [IsAlgClosed L]

/-- Actual source26.1 pairing covariance for every natural level, including zero. -/
theorem geometric_pairing_basechange (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℕ) (P Q : torsionPoints W (n : ℤ)) :
    geometricPairing (extendedCurve (L := L) W) (extended_delta_nonzero W hΔ) (n : ℤ)
      (torsionBasechange W (n : ℤ) P) (torsionBasechange W (n : ℤ) Q)=
        Units.map (algebraMap K L).toMonoidHom (geometricPairing W hΔ (n : ℤ) P Q) := by
  by_cases hn : n=0
  · subst n
    change geometricPairing _ _ 0 _ _=Units.map (algebraMap K L).toMonoidHom
      (geometricPairing W hΔ 0 P Q)
    rw [geometric_pairing_zero_level,geometric_pairing_zero_level,map_one]
  obtain ⟨S,hS⟩:=exists_auxiliary_admissible W hΔ P.val Q.val
    (chosenFunctionUnit W P.val n) (chosenFunctionUnit W Q.val n)
  have ht : AuxiliaryAdmissible (extendedCurve (L := L) W)
      (torsionBasechange W (n : ℤ) P).val (torsionBasechange W (n : ℤ) Q).val
      (chosenFunctionUnit (extendedCurve (L := L) W) (pointBaseChange W P.val) n)
      (chosenFunctionUnit (extendedCurve (L := L) W) (pointBaseChange W Q.val) n)
      (pointBaseChange W S) := by
    simpa only [chosen_unit_basechange,torsion_basechange_coe] using
      auxiliary_admissible_basechange (L := L) W P.val Q.val S
        (chosenFunctionUnit W P.val n) (chosenFunctionUnit W Q.val n) hS
  rw [geometric_pairing_source _ _ n hn _ _ (pointBaseChange W S) ht,
    geometric_pairing_source W hΔ n hn P Q S hS]
  simpa only [chosen_unit_basechange,torsion_basechange_coe] using
    source_auxiliary_basechange (L := L) W hΔ P.val Q.val S
      (chosenFunctionUnit W P.val n) (chosenFunctionUnit W Q.val n) hS

#print axioms geometric_pairing_source
#print axioms geometric_pairing_zero_level
#print axioms geometric_pairing_basechange
end
end WeilGaloisCovarianceAudit
