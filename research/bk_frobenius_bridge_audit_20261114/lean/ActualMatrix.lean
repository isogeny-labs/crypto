import ActualFrobenius
import Mathlib.LinearAlgebra.Determinant

namespace BKFrobeniusBridgeAudit
noncomputable section
open ThreeTorsionRankAudit.General FrobeniusPointDescentAudit Module Matrix
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {F L : Type*} [Field F] [Fintype F] [Field L] [Algebra F L]
  [DecidableEq L]

def frobeniusMatrix (W : WeierstrassCurve.Affine F)
    (b : Basis (Fin 2) (ZMod 3) (Torsion (L:=L) W)) : Matrix (Fin 2) (Fin 2) (ZMod 3) :=
  LinearMap.toMatrix b b (linearFrobenius W)

theorem matrix_coordinates (W : WeierstrassCurve.Affine F)
    (b : Basis (Fin 2) (ZMod 3) (Torsion (L:=L) W)) (P : Torsion (L:=L) W) :
    frobeniusMatrix W b *ᵥ b.repr P=b.repr (linearFrobenius W P) :=
  LinearMap.toMatrix_mulVec_repr b b _ P

theorem matrix_power_identity_iff (W : WeierstrassCurve.Affine F)
    (b : Basis (Fin 2) (ZMod 3) (Torsion (L:=L) W)) (k : ℕ) :
    (frobeniusMatrix W b)^k=1 ↔
      ∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 → (pointFrobenius W)^[k] P=P := by
  rw [frobeniusMatrix,LinearMap.toMatrix_pow,← LinearMap.toMatrix_one b,
    (LinearMap.toMatrix b b).injective.eq_iff]
  exact linear_power_identity_iff W k

theorem determinant_basis_independent (W : WeierstrassCurve.Affine F)
    (b c : Basis (Fin 2) (ZMod 3) (Torsion (L:=L) W)) :
    (frobeniusMatrix W b).det=(frobeniusMatrix W c).det :=
  LinearMap.det_toMatrix_eq_det_toMatrix b c _

theorem actual_frobenius_injective (W : WeierstrassCurve.Affine F) :
    Function.Injective (linearFrobenius (L:=L) W) := by
  intro P Q h
  apply Subtype.ext
  exact WeierstrassCurve.Affine.Point.map_injective
    (f:=FiniteField.frobeniusAlgHom F L) (congrArg Subtype.val h)

theorem actual_frobenius_bijective [NeZero (3:L)]
    (W : WeierstrassCurve.Affine F) (hΔ : (W.baseChange L).Δ ≠ 0) :
    Function.Bijective (linearFrobenius (L:=L) W) := by
  let : Finite (Torsion (L:=L) W) := torsion_finite (W.baseChange L) hΔ
  exact ⟨actual_frobenius_injective W,Finite.surjective_of_injective (actual_frobenius_injective W)⟩

theorem matrix_det_ne_zero (W : WeierstrassCurve.Affine F)
    (b : Basis (Fin 2) (ZMod 3) (Torsion (L:=L) W)) :
    (frobeniusMatrix W b).det ≠ 0 := by
  intro hz
  obtain ⟨v,hv,hker⟩ := Matrix.exists_mulVec_eq_zero_iff.mpr hz
  let P := b.equivFun.symm v
  have hp : linearFrobenius W P=0 := by
    apply b.equivFun.injective
    rw [map_zero]
    change (fun i => b.repr (linearFrobenius W P) i)=0
    rw [← matrix_coordinates]
    have hpv : (b.repr P : Fin 2 → ZMod 3)=v := b.equivFun.apply_symm_apply v
    rw [hpv]
    exact hker
  have hp0 : P=0 := actual_frobenius_injective W (hp.trans (map_zero _).symm)
  apply hv
  have hh := congrArg b.equivFun hp0
  simpa only [P,LinearEquiv.apply_symm_apply,map_zero] using hh

def actualGeometricBasis [IsAlgClosed L] [NeZero (3:L)]
    (W : WeierstrassCurve.Affine F) (hΔ : W.Δ ≠ 0) :
    Basis (Fin 2) (ZMod 3) (Torsion (L:=L) W) := by
  apply geometricThreeBasis
  change (W.map (algebraMap F L)).Δ ≠ 0
  rw [WeierstrassCurve.map_Δ]
  exact map_ne_zero (algebraMap F L) |>.mpr hΔ

#print axioms matrix_power_identity_iff
#print axioms determinant_basis_independent
#print axioms actual_frobenius_bijective
#print axioms matrix_det_ne_zero
#print axioms actualGeometricBasis
end
end BKFrobeniusBridgeAudit
