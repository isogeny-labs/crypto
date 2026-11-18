import SourcePowers

namespace BKGeneralFrobeniusAudit.SourceNineteen
noncomputable section
private instance : Fact (Nat.Prime 19) := ⟨by decide⟩
private instance : Module.Finite (ZMod 19) CubeField := cubic_monic.finite_adjoinRoot
private instance : DecidableEq CubeField := Classical.decEq _

/-- An actual embedding of the constructed degree-three field into the closure. -/
def cubeEmbedding : CubeField →ₐ[ZMod 19] Closure := IsAlgClosed.lift

instance cubeAlgebra : Algebra CubeField Closure := cubeEmbedding.toRingHom.toAlgebra

instance cubeTower : IsScalarTower (ZMod 19) CubeField Closure :=
  IsScalarTower.of_algHom cubeEmbedding

theorem source_extension_three :
    ∀ P : (curve.baseChange Closure).Point, (3 : ℕ) • P = 0 →
      P ∈ Set.range (WeierstrassCurve.Affine.Point.map (W' := curve)
        (Algebra.ofId CubeField Closure)) :=
  (source_all_extensions (K := CubeField) 3 cube_field_card).mpr (by decide)

theorem source_minimal_positive_degree {K : Type*} [Field K] [Fintype K]
    [Algebra (ZMod 19) K] [Algebra K Closure] [IsScalarTower (ZMod 19) K Closure]
    [DecidableEq K] (k : ℕ) (hk : 0 < k) (hc : Fintype.card K = 19^k)
    (h : ∀ P : (curve.baseChange Closure).Point, (3 : ℕ) • P = 0 →
      P ∈ Set.range (WeierstrassCurve.Affine.Point.map (W' := curve)
        (Algebra.ofId K Closure))) : 3 ≤ k :=
  Nat.le_of_dvd hk ((source_all_extensions k hc).mp h)

def sourceMatrix : Matrix (Fin 2) (Fin 2) (ZMod 3) :=
  LinearMap.toMatrix sourceBasis sourceBasis (linearFrobenius curve 3)

theorem sourceMatrix_cube : sourceMatrix^3 = 1 := by
  rw [sourceMatrix, LinearMap.toMatrix_pow, source_frobenius_cube, LinearMap.toMatrix_one]

theorem sourceMatrix_det : sourceMatrix.det = (19 : ZMod 3) := by
  have h : sourceMatrix.det^3 = 1 := by
    rw [← Matrix.det_pow, sourceMatrix_cube, Matrix.det_one]
  have hh := FiniteField.pow_card sourceMatrix.det
  have h1 : sourceMatrix.det = 1 := by simpa only [ZMod.card] using hh.symm.trans h
  have h19 : (19 : ZMod 3) = 1 := by decide +kernel
  exact h1.trans h19.symm

theorem cubic_three_torsion_card : Nat.card (Torsion (curve.baseChange CubeField) 3) = 9 := by
  have hc := fixedPower_card (K := CubeField) (L := Closure) curve 3 3
    (by simpa only [ZMod.card] using cube_field_card)
  have hk : fixedPower (L := Closure) curve 3 3 = ⊤ := by
    simp [fixedPower, source_frobenius_cube]
  have hh : Nat.card (fixedPower (L := Closure) curve 3 3) =
      Nat.card (Torsion (curve.baseChange Closure) 3) := by
    rw [hk]
    exact Nat.card_congr (Submodule.topEquiv :
      (⊤ : Submodule (ZMod 3) (Torsion (curve.baseChange Closure) 3)) ≃ₗ[ZMod 3]
        Torsion (curve.baseChange Closure) 3).toEquiv
  rw [geometric_three_card] at hh
  exact hc.symm.trans hh

#print axioms source_extension_three
#print axioms source_minimal_positive_degree
#print axioms sourceMatrix_det
#print axioms cubic_three_torsion_card
end
end BKGeneralFrobeniusAudit.SourceNineteen
