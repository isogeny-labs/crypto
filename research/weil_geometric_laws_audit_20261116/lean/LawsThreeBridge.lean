import LawsAlternating
import AuxiliaryControls

namespace WeilGeometricLawsAudit
noncomputable section
open WeierstrassCurve.Affine
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit EllipticPicardBridgeAudit
open WeilDivisorChoiceAudit WeilAuxiliaryRatioAudit GeneralTranslationAudit
open ThreeTorsionRankAudit.General ThreeTangentPairingAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

def fromThree (W : WeierstrassCurve.Affine K) (P : ThreeTorsion W) : torsionPoints W 3 :=
  ⟨P.val,by change (3 : ℤ) • P.val=0;exact (natCast_zsmul P.val 3).trans P.property⟩

/-- General source quotient agrees with the credited actual geometric three-torsion construction. -/
theorem geometric_pairing_three (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : ThreeTorsion W) :
    geometricPairing W hΔ 3 (fromThree W P) (fromThree W Q)=divisorWeilThree W hΔ P Q := by
  obtain ⟨hD,hE,hd,hf,hg⟩:=divisor_weil_three_representatives W hΔ P Q
  let S:=sourcePairingPoint W hΔ P Q
  let A : DivisorPair W 3 := ⟨⟨⟨pointRepresentative W P.val,shiftedRepresentative W S Q.val⟩,
    ⟨chosenFunctionUnit W P.val 3,generalTranslatedUnit W hΔ (-S) (chosenFunctionUnit W Q.val 3)⟩⟩,
    hD,hE,hd,hf,hg⟩
  have hp : divisorPointSum W A.D=(fromThree W P).val := by
    change divisorPointSum W (pointRepresentative W P.val)=P.val
    simp only [pointRepresentative,map_sub,point_sum_single,one_zsmul,sub_zero]
  have hq : divisorPointSum W A.E=(fromThree W Q).val := by
    change divisorPointSum W (shiftedRepresentative W S Q.val)=Q.val
    simp only [shiftedRepresentative,map_sub,point_sum_single,one_zsmul,add_sub_cancel_right]
  rw [geometric_pairing_of_data W hΔ 3 (fromThree W P) (fromThree W Q) A hp hq]
  rfl

def toThree (W : WeierstrassCurve.Affine K) (P : torsionPoints W 3) : ThreeTorsion W :=
  ⟨P.val,(natCast_zsmul P.val 3).symm.trans P.property⟩

omit [IsAlgClosed K] in
theorem from_three_to_three (W : WeierstrassCurve.Affine K) (P : torsionPoints W 3) :
    fromThree W (toThree W P)=P := Subtype.ext rfl

/-- Full level-three nondegeneracy transfers across actual equality, with the coprime guard. -/
theorem geometric_pairing_three_left_nondegenerate [NeZero (3 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P : torsionPoints W 3)
    (h : ∀ Q : torsionPoints W 3,geometricPairing W hΔ 3 P Q=1) : P=0 := by
  have hp : toThree W P=0 := divisor_weil_three_left_nondegenerate W hΔ (toThree W P) (by
    intro Q
    rw [←geometric_pairing_three,from_three_to_three]
    exact h _)
  exact Subtype.ext (congrArg Subtype.val hp)

theorem geometric_pairing_three_right_nondegenerate [NeZero (3 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (Q : torsionPoints W 3)
    (h : ∀ P : torsionPoints W 3,geometricPairing W hΔ 3 P Q=1) : Q=0 := by
  have hq : toThree W Q=0 := divisor_weil_three_right_nondegenerate W hΔ (toThree W Q) (by
    intro P
    rw [←geometric_pairing_three,from_three_to_three]
    exact h _)
  exact Subtype.ext (congrArg Subtype.val hq)

private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : DecidableEq ThreeTorsionRankAudit.General.BinaryClosure:=Classical.decEq _

/-- A credited actual characteristic-two nontrivial positive for the new general quotient. -/
theorem actual_binary_three_positive :
    ThreeTorsionRankAudit.General.binaryCurve.Δ≠0 ∧
    ∃ P Q : torsionPoints ThreeTorsionRankAudit.General.binaryCurve 3,
      geometricPairing ThreeTorsionRankAudit.General.binaryCurve ThreeTorsionRankAudit.General.binary_discriminant 3 P Q≠1 := by
  obtain ⟨hc,h3,hd,P,Q,hpow,hval,hinv,hne⟩:=binary_inverse_convention_counter
  refine ⟨hd,fromThree _ P,fromThree _ Q,?_⟩
  rw [geometric_pairing_three]
  exact hne

#print axioms geometric_pairing_three
#print axioms actual_binary_three_positive
end
end WeilGeometricLawsAudit
