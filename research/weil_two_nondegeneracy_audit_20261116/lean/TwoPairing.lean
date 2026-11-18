import TwoVertical

namespace WeilTwoNondegeneracyAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit FunctionDivisorFinitenessAudit MillerRecursionAudit
open GeometricPrincipalDivisorAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open WeilGeometricLawsAudit WeilDivisorChoiceAudit EllipticPicardBridgeAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

theorem geometric_two_of_auxiliary (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : torsionPoints W 2) (f g : W.FunctionFieldˣ)
    (hf : functionDivisor W f f.ne_zero=torsionDivisor W P.val 2)
    (hg : functionDivisor W g g.ne_zero=torsionDivisor W Q.val 2)
    (S : W.Point) (h : AuxiliaryAdmissible W P.val Q.val f g S) :
    geometricPairing W hΔ 2 P Q=sourceAuxiliaryValue W hΔ P.val Q.val S f g h := by
  have hD : functionDivisor W f f.ne_zero=(2 : ℤ) • pointRepresentative W P.val := by
    rw [hf];norm_num [torsionDivisor,pointRepresentative,smul_sub]
  have hE:=shifted_function_divisor W hΔ Q.val S 2 g hg
  let A : DivisorPair W 2:=⟨⟨⟨pointRepresentative W P.val,shiftedRepresentative W S Q.val⟩,
    ⟨f,generalTranslatedUnit W hΔ (-S) g⟩⟩,point_representative_degree W P.val,
    shifted_representative_degree W S Q.val,
    admissible_representatives_disjoint W P.val Q.val S 2 (by decide) f g hf h,hD,hE⟩
  have hp : divisorPointSum W A.D=P.val := by
    simp only [A,DivisorPair.D,pointRepresentative,map_sub,point_sum_single,one_zsmul,sub_zero]
  have hq : divisorPointSum W A.E=Q.val := by
    simp only [A,DivisorPair.E,shiftedRepresentative,map_sub,point_sum_single,one_zsmul,add_sub_cancel_right]
  rw [geometric_pairing_of_data W hΔ 2 P Q A hp hq]
  rfl

theorem geometric_two_distinct_x (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (x y u v : K) (hP : W.Nonsingular x y) (hQ : W.Nonsingular u v)
    (P Q : torsionPoints W 2) (hp : P.val=Point.some x y hP)
    (hq : Q.val=Point.some u v hQ) (hx : x≠u) :
    (geometricPairing W hΔ 2 P Q : K)= -1 := by
  have htP : Point.some x y hP+Point.some x y hP=0 := by
    have ht:=P.property
    change (2 : ℤ) • P.val=0 at ht
    simpa only [hp,two_zsmul] using ht
  have htQ : Point.some u v hQ+Point.some u v hQ=0 := by
    have ht:=Q.property
    change (2 : ℤ) • Q.val=0 at ht
    simpa only [hq,two_zsmul] using ht
  let f:=Units.mk0 (verticalFunction W x) (vertical_nonzero W x)
  let g:=Units.mk0 (verticalFunction W u) (vertical_nonzero W u)
  have hf : functionDivisor W f f.ne_zero=torsionDivisor W P.val 2 := by
    rw [hp];exact two_vertical_divisor W hP htP
  have hg : functionDivisor W g g.ne_zero=torsionDivisor W Q.val 2 := by
    rw [hq];exact two_vertical_divisor W hQ htQ
  obtain ⟨S,hS⟩:=exists_auxiliary_admissible W hΔ P.val Q.val f g
  rw [geometric_two_of_auxiliary W hΔ P Q f g hf hg S hS,source_auxiliary_inverse]
  have he : auxiliaryQuotient W hΔ P.val Q.val f g=constantHom W (-1) := by
    rw [hp,hq];exact two_vertical_auxiliary W hΔ x y u v hP hQ htP htQ hx
  rw [auxiliary_value_of_scalar W hΔ P.val Q.val f g S hS (-1) (neg_ne_zero.mpr one_ne_zero) he]
  simp

theorem geometric_two_left_nondegenerate [NeZero (2 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P : torsionPoints W 2)
    (h : ∀ Q : torsionPoints W 2,geometricPairing W hΔ 2 P Q=1) : P=0 := by
  rcases P with ⟨P,hP⟩
  cases P with
  | zero => rfl
  | some x y hN =>
    obtain ⟨u,v,hQ,hux,htQ⟩:=exists_other_two_point W hΔ x
    let Q : torsionPoints W 2:=⟨.some u v hQ,by
      change (2 : ℤ) • Point.some u v hQ=0
      simpa only [two_zsmul] using htQ⟩
    have hv:=geometric_two_distinct_x W hΔ x y u v hN hQ ⟨.some x y hN,hP⟩ Q rfl rfl hux.symm
    have he:=congrArg (fun z : Kˣ=>(z : K)) (h Q)
    rw [hv,Units.val_one] at he
    have hz : (2 : K)=0 := by linear_combination -he
    exact (NeZero.ne (2 : K) hz).elim

theorem geometric_two_right_nondegenerate [NeZero (2 : K)]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (Q : torsionPoints W 2)
    (h : ∀ P : torsionPoints W 2,geometricPairing W hΔ 2 P Q=1) : Q=0 := by
  apply geometric_two_left_nondegenerate W hΔ Q
  intro P
  rw [geometric_pairing_swap,h P,inv_one]

#print axioms geometric_two_of_auxiliary
#print axioms geometric_two_distinct_x
#print axioms geometric_two_left_nondegenerate
#print axioms geometric_two_right_nondegenerate
end
end WeilTwoNondegeneracyAudit
