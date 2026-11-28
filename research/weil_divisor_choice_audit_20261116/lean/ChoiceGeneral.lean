import ChoiceChange

namespace WeilDivisorChoiceAudit
noncomputable section
open WeierstrassCurve.Affine
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open WeilAuxiliaryRatioAudit EllipticPicardBridgeAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

/-- Arbitrary two-coordinate representative independence, without cross-disjointness premises. -/
theorem pair_value_representative_choice (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℤ) (A B : DivisorPair W n)
    (hD : divisorPointSum W A.D=divisorPointSum W B.D)
    (hE : divisorPointSum W A.E=divisorPointSum W B.E) : pairValue W n A=pairValue W n B := by
  classical
  obtain ⟨S,h,hM,hh⟩:=actual_moving_representative W hΔ A.D A.degD (A.E.support ∪ B.E.support)
  let M:=shiftedRepresentative W S (divisorPointSum W A.D)
  have hdeg : divisorDegree W M=0 := shifted_representative_degree W S _
  have hMA : Disjoint M.support A.E.support := Finset.disjoint_of_subset_right Finset.subset_union_left hM
  have hMB : Disjoint M.support B.E.support := Finset.disjoint_of_subset_right Finset.subset_union_right hM
  have hsum : divisorPointSum W M=divisorPointSum W A.D := by
    have hs:=function_divisor_point_sum_zero W hΔ h.ne_zero
    rw [hh,map_sub,sub_eq_zero] at hs
    exact hs
  have hp : M-B.D ∈ principalPointDivisors W := by
    apply (principal_iff_degree_and_point_sum W hΔ _).mpr
    exact ⟨by rw [map_sub,hdeg,B.degD,sub_self],by rw [map_sub,hsum,hD,sub_self]⟩
  obtain ⟨u,hu⟩:=hp
  let C:=replaceLeft W n A M hdeg hMA h hh
  let C':=replaceLeft W n B M hdeg hMB u.toMul hu
  have hC : pairValue W n C=pairValue W n A := corrected_left_invariance W hΔ n A M hdeg hMA h hh
  have hC' : pairValue W n C'=pairValue W n B := corrected_left_invariance W hΔ n B M hdeg hMB u.toMul hu
  have he : pairValue W n (swapPair W n C)=pairValue W n (swapPair W n C') :=
    pair_value_left_choice W hΔ n _ _ rfl hE
  rw [pair_value_swap,pair_value_swap] at he
  exact hC.symm.trans ((inv_injective he).trans hC')

/-- An actual n-principal function is derived from degree zero and actual n-torsion point sum. -/
theorem actual_multiple_principal (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℤ) (D : W.Point →₀ ℤ) (hdeg : divisorDegree W D=0)
    (ht : n • divisorPointSum W D=0) :
    ∃ f : W.FunctionFieldˣ,functionDivisor W f f.ne_zero=n • D := by
  have hp : n • D ∈ principalPointDivisors W :=
    (principal_iff_degree_and_point_sum W hΔ _).mpr
      ⟨by rw [map_zsmul,hdeg,smul_zero],by rw [map_zsmul];exact ht⟩
  obtain ⟨u,hu⟩:=hp
  exact ⟨u.toMul,hu⟩

/-- Every actual torsion-point pair has proper disjoint divisor representatives and functions. -/
theorem actual_pair_exists (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℤ) (P Q : W.Point) (hP : n • P=0) (hQ : n • Q=0) :
    ∃ A : DivisorPair W n,divisorPointSum W A.D=P ∧ divisorPointSum W A.E=Q := by
  let D:=pointRepresentative W P
  have hd : divisorDegree W D=0 := point_representative_degree W P
  have hsumD : divisorPointSum W D=P := by simp only [D,pointRepresentative,map_sub,point_sum_single,one_zsmul,sub_zero]
  obtain ⟨S,h,hE,hh⟩:=actual_moving_representative W hΔ (pointRepresentative W Q)
    (point_representative_degree W Q) D.support
  let E:=shiftedRepresentative W S (divisorPointSum W (pointRepresentative W Q))
  have he : divisorDegree W E=0 := shifted_representative_degree W S _
  have hsumE : divisorPointSum W E=Q := by
    have hs:=function_divisor_point_sum_zero W hΔ h.ne_zero
    rw [hh,map_sub,sub_eq_zero] at hs
    simpa only [E,pointRepresentative,map_sub,point_sum_single,one_zsmul,sub_zero] using hs
  obtain ⟨f,hf⟩:=actual_multiple_principal W hΔ n D hd (by rw [hsumD];exact hP)
  obtain ⟨g,hg⟩:=actual_multiple_principal W hΔ n E he (by rw [hsumE];exact hQ)
  exact ⟨⟨⟨⟨D,E⟩,⟨f,g⟩⟩,hd,he,hE.symm,hf,hg⟩,hsumD,hsumE⟩

#print axioms pair_value_representative_choice
#print axioms actual_multiple_principal
#print axioms actual_pair_exists
end
end WeilDivisorChoiceAudit
