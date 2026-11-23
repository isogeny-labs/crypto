import MixedCertificate
import SourcePowers
import RationalDomainImports

namespace WeilRationalDomainAudit
noncomputable section
open WeierstrassCurve.Affine WeilGeometricLawsAudit

section Scalar
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

theorem geometric_pairing_zsmul_left (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (m : ℤ) (P Q : torsionPoints W m) (z : ℤ) :
    geometricPairing W hΔ m (z • P) Q=(geometricPairing W hΔ m P Q)^z := by
  cases z with
  | ofNat n =>
    simpa only [Int.ofNat_eq_natCast,natCast_zsmul,zpow_natCast] using
      WeilMixedLevelAudit.geometric_pairing_nsmul_left W hΔ m P Q n
  | negSucc n =>
    rw [negSucc_zsmul,geometric_pairing_neg_left,zpow_negSucc,
      WeilMixedLevelAudit.geometric_pairing_nsmul_left]

theorem geometric_pairing_zsmul_right (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (m : ℤ) (P Q : torsionPoints W m) (z : ℤ) :
    geometricPairing W hΔ m P (z • Q)=(geometricPairing W hΔ m P Q)^z := by
  rw [geometric_pairing_swap,geometric_pairing_zsmul_left,
    geometric_pairing_swap,inv_zpow,inv_inv]

theorem actual_cyclic_pullback_trivial {G : Type*} [AddGroup G] [IsAddCyclic G]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (m : ℤ)
    (f : G →+ torsionPoints W m) (P Q : G) :
    geometricPairing W hΔ m (f P) (f Q)=1 := by
  obtain ⟨g,hg⟩:=exists_zsmul_surjective G
  obtain ⟨a,rfl⟩:=hg P
  obtain ⟨b,rfl⟩:=hg Q
  rw [map_zsmul,map_zsmul,geometric_pairing_zsmul_left,
    geometric_pairing_zsmul_right,geometric_pairing_alternating,one_zpow,one_zpow]
end Scalar

section Transport
variable {F L : Type*} [Field F] [DecidableEq F] [Field L] [DecidableEq L] [Algebra F L]

def natIntegerTorsionEquiv (W : WeierstrassCurve.Affine F) (n : ℕ) :
    BKGeneralFrobeniusAudit.Torsion W n ≃+ torsionPoints W (n : ℤ) where
  toFun P:=⟨P.val,(natCast_zsmul P.val n).trans P.property⟩
  invFun P:=⟨P.val,(natCast_zsmul P.val n).symm.trans P.property⟩
  left_inv _:=rfl
  right_inv _:=rfl
  map_add' _ _:=rfl

def curvePointEquiv {W V : WeierstrassCurve.Affine F} (h : W=V) : W.Point ≃+ V.Point := by
  cases h
  exact AddEquiv.refl _

omit [DecidableEq F] in
theorem curve_self_base_change (W : WeierstrassCurve.Affine F) : W.baseChange F=W := by
  change W.map (RingHom.id F)=W
  exact WeierstrassCurve.map_id W

def fieldPointInclusion (W : WeierstrassCurve.Affine F) : W.Point →+ (W.baseChange L).Point :=
  (Point.map (W':=W) (Algebra.ofId F L)).comp
    (curvePointEquiv (curve_self_base_change W).symm).toAddMonoidHom

theorem field_point_inclusion_injective (W : WeierstrassCurve.Affine F) :
    Function.Injective (fieldPointInclusion (L:=L) W) :=
  (Point.map_injective (W':=W) (Algebra.ofId F L)).comp
    (curvePointEquiv (curve_self_base_change W).symm).injective

def integerTorsionInclusion (W : WeierstrassCurve.Affine F) (n : ℤ) :
    torsionPoints W n →+ torsionPoints (W.baseChange L) n where
  toFun P:=⟨fieldPointInclusion W P.val,by
    change n • fieldPointInclusion W P.val=0
    rw [←map_zsmul,P.property,map_zero]⟩
  map_zero':=by apply Subtype.ext;exact map_zero _
  map_add' P Q:=by apply Subtype.ext;exact map_add _ P.val Q.val

theorem integer_torsion_inclusion_injective (W : WeierstrassCurve.Affine F) (n : ℤ) :
    Function.Injective (integerTorsionInclusion (L:=L) W n) := by
  intro P Q h
  apply Subtype.ext
  apply field_point_inclusion_injective (L:=L) W
  exact congrArg (fun X : torsionPoints (W.baseChange L) n=>X.val) h
end Transport

#print axioms geometric_pairing_zsmul_left
#print axioms actual_cyclic_pullback_trivial
#print axioms integer_torsion_inclusion_injective
end
end WeilRationalDomainAudit
