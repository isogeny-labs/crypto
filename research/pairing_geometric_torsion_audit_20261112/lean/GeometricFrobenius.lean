import AlgebraicClosure
import BaseDescent

namespace PairingGeometricFrobeniusAudit
noncomputable section
open PairingMillerCurveAudit PairingMillerTorsionAudit PairingGeometricTorsionAudit
  PairingGeometricClosureAudit PairingMillerBaseDescentAudit FrobeniusPointDescentAudit
variable {K : Type*} [Field K] [Algebra (ZMod 5) K] [DecidableEq K]
variable (f : F25 →ₐ[ZMod 5] K)

theorem pointMap_frobenius (R : curve.Point) :
    pointFrobenius BKActualThreeAudit.curve (pointMap f R)=
      pointMap f (pointFrobenius BKActualThreeAudit.curve R) := by
  cases R with
  | zero => rfl
  | some x y h =>
    change WeierstrassCurve.Affine.Point.some _ _ _ =
      WeierstrassCurve.Affine.Point.some _ _ _
    rw [WeierstrassCurve.Affine.Point.some.injEq]
    change (f x)^Fintype.card (ZMod 5)=f (x^Fintype.card (ZMod 5)) ∧
      (f y)^Fintype.card (ZMod 5)=f (y^Fintype.card (ZMod 5))
    exact ⟨(map_pow f x _).symm,(map_pow f y _).symm⟩

theorem actual_extension_frobenius_matrix (v : Index) :
    pointFrobenius BKActualThreeAudit.curve (pointMap f (indexedPoint v))=
      pointMap f (indexedPoint (v.1,-v.2)) := by
  rw [pointMap_frobenius,indexed_frobenius]

theorem actual_extension_rational_line (v : Index) :
    (∃ R : BKActualThreeAudit.curve.Point,
      WeierstrassCurve.Affine.Point.map (W':=BKActualThreeAudit.curve)
        (Algebra.ofId (ZMod 5) K) R=pointMap f (indexedPoint v)) ↔ v.2=0 := by
  rw [←point_frobenius_fixed_iff,actual_extension_frobenius_matrix,
    (pointMap_injective f).eq_iff,←indexed_frobenius,←base_rational_iff_frobenius]
  exact base_rational_indexed_iff v

variable [IsAlgClosed K]

variable (K) in
abbrev geometricGroup := extendedTorsion (K:=K)

variable (K) in
abbrev geometricBasis := actualGeometricBasis (K:=K)

variable (K) in
abbrev geometricEmbedding := chosenEmbedding (K:=K)

variable (K) in
abbrev geometricCurve := extendedCurve (K:=K)

variable (K) in
def baseInclusion : BKActualThreeAudit.curve.Point →+ (geometricCurve K).Point :=
  WeierstrassCurve.Affine.Point.map (W':=BKActualThreeAudit.curve)
    (Algebra.ofId (ZMod 5) K)

theorem actual_geometric_torsion_and_frobenius_certificate :
    Nat.card (geometricGroup K)=9 ∧
    (∀ v : Index,(geometricBasis K v).val=
      pointMap (geometricEmbedding K) (indexedPoint v)) ∧
    (∀ R : (geometricCurve K).Point,(3:ℕ) • R=0 →
      ∃ U : curve.Point,(3:ℕ) • U=0 ∧ pointMap (geometricEmbedding K) U=R) ∧
    (∀ v : Index,pointFrobenius BKActualThreeAudit.curve
      (geometricBasis K v).val=
        (geometricBasis K (v.1,-v.2)).val) ∧
    (∀ v : Index,(∃ R : BKActualThreeAudit.curve.Point,
      baseInclusion K R=(geometricBasis K v).val) ↔ v.2=0) := by
  refine ⟨actual_geometric_torsion_cardinality,actual_geometric_basis_values,
    actual_torsion_descends chosenEmbedding,?_,?_⟩
  · intro v
    simpa only [actual_geometric_basis_values] using
      actual_extension_frobenius_matrix (chosenEmbedding (K:=K)) v
  · intro v
    change (∃ R : BKActualThreeAudit.curve.Point,
      WeierstrassCurve.Affine.Point.map (W':=BKActualThreeAudit.curve)
        (Algebra.ofId (ZMod 5) K) R=
          pointMap (chosenEmbedding (K:=K)) (indexedPoint v)) ↔ v.2=0
    exact actual_extension_rational_line (chosenEmbedding (K:=K)) v

#print axioms pointMap_frobenius
#print axioms actual_geometric_torsion_and_frobenius_certificate
end
end PairingGeometricFrobeniusAudit
