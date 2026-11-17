import GeneralBasis
import FrobeniusFiniteExtension
import Mathlib.LinearAlgebra.Matrix.ToLin

namespace BKFrobeniusBridgeAudit
noncomputable section
open ThreeTorsionRankAudit.General FrobeniusPointDescentAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {F L : Type*} [Field F] [Fintype F] [Field L] [Algebra F L]
  [DecidableEq L]

abbrev Torsion (W : WeierstrassCurve.Affine F) := ThreeTorsion (W.baseChange L)

/-- Restriction of the actual coordinate q-Frobenius to the actual subgroup. -/
def torsionFrobenius (W : WeierstrassCurve.Affine F) : Torsion (L:=L) W →+ Torsion (L:=L) W where
  toFun P := ⟨pointFrobenius W P.val,point_frobenius_preserves_torsion W 3 P.val P.property⟩
  map_zero' := by apply Subtype.ext; exact map_zero _
  map_add' P Q := by apply Subtype.ext; exact map_add _ P.val Q.val

def linearFrobenius (W : WeierstrassCurve.Affine F) : Torsion (L:=L) W →ₗ[ZMod 3] Torsion (L:=L) W :=
  (torsionFrobenius W).toZModLinearMap 3

theorem linear_frobenius_val (W : WeierstrassCurve.Affine F) (P : Torsion (L:=L) W) :
    (linearFrobenius W P).val=pointFrobenius W P.val := rfl

theorem linear_power_val (W : WeierstrassCurve.Affine F) (k : ℕ) (P : Torsion (L:=L) W) :
    (((linearFrobenius W)^k) P).val=(pointFrobenius W)^[k] P.val := by
  induction k with
  | zero => rfl
  | succ k ih =>
    rw [pow_succ']
    change (linearFrobenius W (((linearFrobenius W)^k) P)).val=_
    rw [linear_frobenius_val,ih,Function.iterate_succ_apply']

theorem linear_power_identity_iff (W : WeierstrassCurve.Affine F) (k : ℕ) :
    (linearFrobenius (L:=L) W)^k=1 ↔
      ∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 → (pointFrobenius W)^[k] P=P := by
  constructor
  · intro h P hP
    have hp := congrArg (fun f : Torsion (L:=L) W →ₗ[ZMod 3] Torsion (L:=L) W => (f ⟨P,hP⟩).val) h
    simpa only [linear_power_val,Module.End.one_apply] using hp
  · intro h
    ext P
    exact (linear_power_val W k P).trans (h P.val P.property)

#print axioms linear_power_val
#print axioms linear_power_identity_iff
end
end BKFrobeniusBridgeAudit
