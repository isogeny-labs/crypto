import EmbeddingCriterion
import Mathlib.FieldTheory.Finite.GaloisField

namespace BKActualGaloisFieldAudit
noncomputable section
open BKActualThreeAudit BKActualEmbeddingAudit
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance galoisDecidableEq (k : ℕ) : DecidableEq (GaloisField 5 k) :=
  Classical.decEq _
variable {L : Type*} [Field L] [Algebra F L] [DecidableEq L] [IsAlgClosed L]

theorem actual_galois_field_criterion (k : ℕ) (hk : 0<k) :
    ∃ ι : GaloisField 5 k→ₐ[F]L,
      (∀ P : (geomCurve (L:=L)).Point,(3:ℕ) • P=0 →
        P∈Set.range (WeierstrassCurve.Affine.Point.map (W':=curve) ι)) ↔ 3∣5^k-1 := by
  let K := GaloisField 5 k
  let : Fintype K := Fintype.ofFinite K
  let ι : K→ₐ[F]L := IsAlgClosed.lift (R:=F) (S:=K) (M:=L)
  let : Algebra K L := ι.toRingHom.toAlgebra
  let : IsScalarTower F K L := IsScalarTower.of_algHom ι
  have hc : Fintype.card K=5^k := by
    rw [←Nat.card_eq_fintype_card]
    exact GaloisField.card 5 k (Nat.ne_of_gt hk)
  refine ⟨ι,?_⟩
  have hh := actual_extension_torsion_criterion (L:=L) (K:=K) k hc
  have hm : (fun Q : (curve.baseChange K).Point =>
      WeierstrassCurve.Affine.Point.map (W':=curve) (Algebra.ofId K L) Q)=
      (fun Q => WeierstrassCurve.Affine.Point.map (W':=curve) ι Q) := by
    funext Q
    cases Q <;> rfl
  simpa only [hm] using hh

#print axioms actual_galois_field_criterion
end
end BKActualGaloisFieldAudit
