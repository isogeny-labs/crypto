import AuxiliaryRepresentatives

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit GeneralTranslationAudit EllipticTranslationAudit
open BKThreeNormalizationAudit ThreeTangentPairingAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- The exact Galbraith26.1 order, using actual proper divisor evaluations. -/
def sourceAuxiliaryValue (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q S : W.Point) (f g : W.FunctionFieldˣ) (h : AuxiliaryAdmissible W P Q f g S) : Kˣ :=
  evaluateDivisor W (pointRepresentative W P) (generalTranslatedUnit W hΔ (-S) g)
      (point_evaluation_domain W hΔ P Q S f g h) /
    evaluateDivisor W (shiftedRepresentative W S Q) f (shifted_evaluation_domain W P Q S f g h)

theorem source_auxiliary_inverse (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q S : W.Point) (f g : W.FunctionFieldˣ) (h : AuxiliaryAdmissible W P Q f g S) :
    sourceAuxiliaryValue W hΔ P Q S f g h=(auxiliaryValue W P Q f g S h)⁻¹ := by
  have hp : pointOrder W (generalTranslatedUnit W hΔ (-S) g) P=0 := by
    rw [general_translated_unit_coe,general_translation_order]
    simpa only [sub_eq_add_neg] using h.2.2.2
  have hz : pointOrder W (generalTranslatedUnit W hΔ (-S) g) 0=0 := by
    rw [general_translated_unit_coe,general_translation_order,zero_add]
    exact h.2.1
  have hq : pointOrder W f (Q+S)=0 := by simpa only [add_comm] using h.1
  have vp : localUnitValue W P (generalTranslatedUnit W hΔ (-S) g) hp=
      localUnitValue W (P-S) g h.2.2.2 := by
    rw [local_unit_value_translation W hΔ P (-S) g
      (by simpa only [sub_eq_add_neg] using h.2.2.2)]
    simp only [sub_eq_add_neg]
  have vz : localUnitValue W 0 (generalTranslatedUnit W hΔ (-S) g) hz=
      localUnitValue W (-S) g h.2.1 := by
    rw [local_unit_value_translation W hΔ 0 (-S) g
      (by simpa only [zero_add] using h.2.1)]
    simp only [zero_add]
  dsimp only [sourceAuxiliaryValue,pointRepresentative,shiftedRepresentative]
  rw [evaluate_point_difference W P 0 (generalTranslatedUnit W hΔ (-S) g) hp hz
      (point_evaluation_domain W hΔ P Q S f g h),
    evaluate_point_difference W (Q+S) S f hq h.2.2.1
      (shifted_evaluation_domain W P Q S f g h),vp,vz]
  unfold auxiliaryValue
  have hcomm : Q+S=S+Q := add_comm _ _
  simp only [hcomm]
  change localUnitValue W (P-S) g h.2.2.2/localUnitValue W (-S) g h.2.1/
    (localUnitValue W (S+Q) f h.1/localUnitValue W S f h.2.2.1)=
    (localUnitValue W (S+Q) f h.1*localUnitValue W (-S) g h.2.1/
      (localUnitValue W S f h.2.2.1*localUnitValue W (P-S) g h.2.2.2))⁻¹
  simp only [div_eq_mul_inv,mul_inv,inv_inv]
  ac_rfl

/-- Positive n and the actual principal divisor make the source representatives disjoint. -/
theorem admissible_representatives_disjoint (W : WeierstrassCurve.Affine K)
    (P Q S : W.Point) (n : ℕ) (hn : n≠0) (f g : W.FunctionFieldˣ)
    (hDf : functionDivisor W f f.ne_zero=torsionDivisor W P n)
    (h : AuxiliaryAdmissible W P Q f g S) :
    Disjoint (pointRepresentative W P).support (shiftedRepresentative W S Q).support := by
  apply Finset.disjoint_left.mpr
  intro R hP hE
  have he : pointOrder W f R=(n : ℤ)*pointRepresentative W P R := by
    have hh := congrArg (fun D : W.Point →₀ ℤ => D R) hDf
    change pointOrder W f R=((n : ℤ) • Finsupp.single P (1 : ℤ)-
      (n : ℤ) • Finsupp.single 0 (1 : ℤ) : W.Point →₀ ℤ) R at hh
    change pointOrder W f R=(n : ℤ)*((Finsupp.single P (1 : ℤ)) R-(Finsupp.single 0 (1 : ℤ)) R)
    simpa only [Finsupp.sub_apply,Finsupp.smul_apply,zsmul_eq_mul,Int.cast_id,mul_sub] using hh
  have hne : pointOrder W f R≠0 := by
    rw [he]
    exact mul_ne_zero (Int.natCast_ne_zero.mpr hn) (Finsupp.mem_support_iff.mp hP)
  have hu := (mem_support_units_iff W _ f).mp
    ((disjoint_principal_support_iff W f _).mp (shifted_evaluation_domain W P Q S f g h)) R hE
  exact hne hu

/-- Exact source ratio equals the inverse tangent pairing, on actual admissible representatives. -/
theorem source_tangent_inverse [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (x y u v : K)
    (hP : W.Nonsingular x y) (hQ : W.Nonsingular u v)
    (htP : (3 : ℕ) • Point.some x y hP=0) (htQ : (3 : ℕ) • Point.some u v hQ=0)
    (hx : x≠u) (S : W.Point)
    (h : AuxiliaryAdmissible W (.some x y hP) (.some u v hQ)
      (Units.mk0 (tangentFunction W x y) (tangent_function_nonzero W x y))
      (Units.mk0 (tangentFunction W u v) (tangent_function_nonzero W u v)) S) :
    (sourceAuxiliaryValue W hΔ (.some x y hP) (.some u v hQ) S
      (Units.mk0 (tangentFunction W x y) (tangent_function_nonzero W x y))
      (Units.mk0 (tangentFunction W u v) (tangent_function_nonzero W u v)) h : K)=
      (tangentPair W (.some x y hP) (.some u v hQ))⁻¹ := by
  have hn : tangentPair W (.some x y hP) (.some u v hQ)≠0 := by
    obtain ⟨ha,hb⟩ := tangentPair_outside_domain W hΔ x y u v hP hQ htP htQ hx
    change (if x=u then 1 else -tangentValue W x y u v/tangentValue W u v x y)≠0
    rw [ite_eq_right hx]
    exact div_ne_zero (neg_ne_zero.mpr ha) hb
  have hv := auxiliary_value_of_scalar W hΔ (.some x y hP) (.some u v hQ) _ _ S h _ hn
    (tangent_auxiliary_quotient W hΔ x y u v hP hQ htP htQ hx)
  rw [source_auxiliary_inverse,hv]
  rfl

#print axioms source_auxiliary_inverse
#print axioms admissible_representatives_disjoint
#print axioms source_tangent_inverse
end
end WeilAuxiliaryRatioAudit
