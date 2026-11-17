import ChoiceCertificate
import LawsImports

namespace WeilGeometricLawsAudit
noncomputable section
open WeierstrassCurve.Affine
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open WeilDivisorChoiceAudit EllipticPicardBridgeAudit WeilAuxiliaryRatioAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

/-- Actual integer torsion kernel in the actual point group. -/
def torsionPoints (W : WeierstrassCurve.Affine K) (n : ℤ) : AddSubgroup W.Point where
  carrier:={P|n • P=0}
  zero_mem':=smul_zero n
  add_mem':=by
    intro P Q hP hQ
    change n • P=0 at hP
    change n • Q=0 at hQ
    change n • (P+Q)=0
    rw [smul_add,hP,hQ,zero_add]
  neg_mem':=by
    intro P hP
    change n • P=0 at hP
    change n • (-P)=0
    rw [smul_neg,hP,neg_zero]

def selectedPair (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (P Q : torsionPoints W n) : DivisorPair W n :=
  Classical.choose (actual_pair_exists W hΔ n P.val Q.val P.property Q.property)

theorem selected_pair_classes (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (P Q : torsionPoints W n) :
    divisorPointSum W (selectedPair W hΔ n P Q).D=P.val ∧
      divisorPointSum W (selectedPair W hΔ n P Q).E=Q.val :=
  Classical.choose_spec (actual_pair_exists W hΔ n P.val Q.val P.property Q.property)

/-- Actual geometric source26.1 quotient on actual torsion points. -/
def geometricPairing (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (P Q : torsionPoints W n) : Kˣ := pairValue W n (selectedPair W hΔ n P Q)

theorem geometric_pairing_of_data (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (P Q : torsionPoints W n) (A : DivisorPair W n)
    (hD : divisorPointSum W A.D=P.val) (hE : divisorPointSum W A.E=Q.val) :
    geometricPairing W hΔ n P Q=pairValue W n A :=
  pair_value_representative_choice W hΔ n _ A
    ((selected_pair_classes W hΔ n P Q).1.trans hD.symm)
    ((selected_pair_classes W hΔ n P Q).2.trans hE.symm)

theorem geometric_pairing_root (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (P Q : torsionPoints W n) : geometricPairing W hΔ n P Q^n=1 :=
  pair_value_root W hΔ n _

theorem geometric_pairing_swap (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (P Q : torsionPoints W n) : geometricPairing W hΔ n Q P=(geometricPairing W hΔ n P Q)⁻¹ := by
  rw [geometric_pairing_of_data W hΔ n Q P (swapPair W n (selectedPair W hΔ n P Q))
    (selected_pair_classes W hΔ n P Q).2 (selected_pair_classes W hΔ n P Q).1,pair_value_swap]
  rfl

/-- Actual proper data with any supplied opposite divisor/function. -/
theorem pair_with_right (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (E : W.Point →₀ ℤ) (he : divisorDegree W E=0) (g : W.FunctionFieldˣ)
    (hg : functionDivisor W g g.ne_zero=n • E) (P : torsionPoints W n) :
    ∃ A : DivisorPair W n,A.E=E ∧ A.g=g ∧ divisorPointSum W A.D=P.val := by
  obtain ⟨S,h,hd,hh⟩:=actual_moving_representative W hΔ (pointRepresentative W P.val)
    (point_representative_degree W P.val) E.support
  let D:=shiftedRepresentative W S (divisorPointSum W (pointRepresentative W P.val))
  have hdeg : divisorDegree W D=0 := shifted_representative_degree W S _
  have hsum : divisorPointSum W D=P.val := by
    have hs:=function_divisor_point_sum_zero W hΔ h.ne_zero
    rw [hh,map_sub,sub_eq_zero] at hs
    simpa only [D,pointRepresentative,map_sub,point_sum_single,one_zsmul,sub_zero] using hs
  obtain ⟨f,hf⟩:=actual_multiple_principal W hΔ n D hdeg (by rw [hsum];exact P.property)
  exact ⟨⟨⟨⟨D,E⟩,⟨f,g⟩⟩,hdeg,he,hd,hf,hg⟩,rfl,rfl,hsum⟩

#print axioms selected_pair_classes
#print axioms geometric_pairing_of_data
#print axioms geometric_pairing_root
#print axioms geometric_pairing_swap
#print axioms pair_with_right
end
end WeilGeometricLawsAudit
