import CoordinateDescent

namespace PairingGeometricTorsionAudit
noncomputable section
open PairingMillerCurveAudit PairingMillerTorsionAudit PairingGeometricCoordinateAudit
variable {K : Type*} [Field K] [Algebra (ZMod 5) K] [DecidableEq K]
variable (f : F25 →ₐ[ZMod 5] K)

abbrev extendedCurve := BKActualThreeAudit.geomCurve (L:=K)

def pointMap : curve.Point →+ (extendedCurve (K:=K)).Point :=
  WeierstrassCurve.Affine.Point.map (W':=BKActualThreeAudit.curve) f

theorem pointMap_injective : Function.Injective (pointMap f) :=
  WeierstrassCurve.Affine.Point.map_injective (W':=BKActualThreeAudit.curve) f

theorem actual_torsion_descends (R : (extendedCurve (K:=K)).Point)
    (hR : (3:ℕ) • R=0) :
    ∃ U : curve.Point,(3:ℕ) • U=0 ∧ pointMap f U=R := by
  cases R with
  | zero => exact ⟨0,by simp,by change pointMap f 0=0;exact map_zero _⟩
  | some x y h =>
    have hx:=BKActualThreeAudit.three_coordinate_classification h hR
    have he:=BKActualThreeAudit.actual_equation h
    obtain ⟨a,ha⟩:=cubic_coordinate_descends f x hx
    obtain ⟨b,hb⟩:=y_coordinate_descends f x y he hx
    have hab : curve.Nonsingular a b := by
      apply (BKActualThreeAudit.curve.baseChange_nonsingular f.injective a b).mp
      simpa only [ha,hb] using h
    let U : curve.Point := .some a b hab
    have hu : pointMap f U=WeierstrassCurve.Affine.Point.some x y h := by
      change WeierstrassCurve.Affine.Point.some (f a) (f b) _ = _
      rw [WeierstrassCurve.Affine.Point.some.injEq]
      exact ⟨ha,hb⟩
    refine ⟨U,?_,hu⟩
    apply pointMap_injective f
    rw [map_nsmul,hu,hR,map_zero]

def extendedTorsion : AddSubgroup (extendedCurve (K:=K)).Point :=
  (nsmulAddMonoidHom 3 : (extendedCurve (K:=K)).Point →+ (extendedCurve (K:=K)).Point).ker

def torsionMap : threeTorsion →+ extendedTorsion (K:=K) :=
  ((pointMap f).comp threeTorsion.subtype).codRestrict (extendedTorsion (K:=K))
    (fun U=>by
      change (3:ℕ) • pointMap f U.val=0
      have hU : (3:ℕ) • U.val=0 := U.property
      rw [←map_nsmul,hU,map_zero])

theorem torsionMap_bijective : Function.Bijective (torsionMap f) := by
  constructor
  · intro U V h
    apply Subtype.ext
    exact pointMap_injective f (congrArg Subtype.val h)
  · intro R
    obtain ⟨U,hU,hUR⟩:=actual_torsion_descends f R.val R.property
    exact ⟨⟨U,hU⟩,Subtype.ext hUR⟩

def torsionEquivalence : threeTorsion ≃+ extendedTorsion (K:=K) :=
  AddEquiv.ofBijective (torsionMap f) (torsionMap_bijective f)

include f in
theorem all_extension_torsion_cardinality : Nat.card (extendedTorsion (K:=K))=9 := by
  rw [←Nat.card_congr (torsionEquivalence f).toEquiv]
  exact actual_torsion_cardinality

theorem actual_extension_torsion_basis :
    ∃ e : Index ≃+ extendedTorsion (K:=K),
      ∀ v : Index,(e v).val=pointMap f (indexedPoint v) := by
  refine ⟨PairingMillerTorsionAudit.torsionEquiv.trans (torsionEquivalence f),?_⟩
  intro v
  rfl

#print axioms actual_torsion_descends
#print axioms all_extension_torsion_cardinality
#print axioms actual_extension_torsion_basis
end
end PairingGeometricTorsionAudit
