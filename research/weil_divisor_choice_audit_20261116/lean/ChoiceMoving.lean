import ChoiceEvaluation
import AuxiliaryRepresentatives
import AuxiliaryExistence
import PicardCriterion

namespace WeilDivisorChoiceAudit
noncomputable section
open WeierstrassCurve.Affine
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open EllipticPicardBridgeAudit WeilAuxiliaryRatioAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

/-- Actual moving by a point difference avoiding any supplied finite set. -/
theorem actual_moving_representative (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (D : W.Point →₀ ℤ) (hD : divisorDegree W D=0) (T : Finset W.Point) :
    ∃ (S : W.Point) (h : W.FunctionFieldˣ),
      Disjoint (shiftedRepresentative W S (divisorPointSum W D)).support T ∧
      functionDivisor W h h.ne_zero=shiftedRepresentative W S (divisorPointSum W D)-D := by
  classical
  let P:=divisorPointSum W D
  obtain ⟨S,hS⟩ := exists_point_outside W hΔ (T ∪ T.image (fun R=>R-P))
  have hST : S∉T := fun hm=>hS (Finset.mem_union_left _ hm)
  have hPT : P+S∉T := by
    intro hm
    apply hS
    apply Finset.mem_union_right
    apply Finset.mem_image.mpr
    exact ⟨P+S,hm,by abel⟩
  have hd : Disjoint (shiftedRepresentative W S P).support T := by
    apply Finset.disjoint_left.mpr
    intro R hR hT
    have hn : R≠P+S := fun he=>hPT (he ▸ hT)
    have hs : R≠S := fun he=>hST (he ▸ hT)
    exact (Finsupp.mem_support_iff.mp hR) (by simp [shiftedRepresentative,Ne.symm hn,Ne.symm hs])
  have hp : shiftedRepresentative W S P-D ∈ principalPointDivisors W := by
    apply (principal_iff_degree_and_point_sum W hΔ _).mpr
    constructor
    · rw [map_sub,shifted_representative_degree,hD,sub_self]
    · rw [map_sub,shiftedRepresentative,map_sub,point_sum_single,point_sum_single,
        one_zsmul,one_zsmul]
      change (P+S)-S-P=0
      abel
  obtain ⟨u,hu⟩:=hp
  exact ⟨S,u.toMul,hd,hu⟩

#print axioms actual_moving_representative
end
end WeilDivisorChoiceAudit
