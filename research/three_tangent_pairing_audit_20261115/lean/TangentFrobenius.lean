import TangentCoordinates
import ShiftDescent

namespace ThreeTangentPairingAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General Module Matrix
  BKFrobeniusBridgeAudit ThreeTorsionCharacterAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {F L : Type*} [Field F] [Fintype F] [Field L] [Algebra F L]
  [DecidableEq F] [DecidableEq L] [IsAlgClosed L] [NeZero (3:L)]

/-- No rational fixed point, supplied determinant or normalized-family premise. -/
theorem actual_general_three_determinant (W : WeierstrassCurve.Affine F) (hΔ : W.Δ≠0)
    (b : Basis (Fin 2) (ZMod 3) (Torsion (L:=L) W)) :
    (frobeniusMatrix W b).det=(Fintype.card F : ZMod 3) := by
  have hd : (W.baseChange L).Δ≠0 := BKThreeNormalizationAudit.base_discriminant_nonzero W hΔ
  let A := frobeniusMatrix W b
  let z := tangentPair (W.baseChange L) (b 0).val (b 1).val
  have hz : z^3=1 := tangentPair_cube (W.baseChange L) hd (b 0) (b 1)
  have hn : z≠1 := basis_pairing_nontrivial (W.baseChange L) hd b
  have h0 : A 0 0 • b 0+A 1 0 • b 1=linearFrobenius W (b 0) := by
    simp only [A,frobeniusMatrix,LinearMap.toMatrix_apply]
    simpa only [Fin.sum_univ_two] using b.sum_repr (linearFrobenius W (b 0))
  have h1 : A 0 1 • b 0+A 1 1 • b 1=linearFrobenius W (b 1) := by
    simp only [A,frobeniusMatrix,LinearMap.toMatrix_apply]
    simpa only [Fin.sum_univ_two] using b.sum_repr (linearFrobenius W (b 1))
  have he : z^A.det.val=z^Fintype.card F := by
    have hc := tangentPair_frobenius W (b 0).val (b 1).val
    change tangentPair (W.baseChange L) (linearFrobenius W (b 0)).val
      (linearFrobenius W (b 1)).val=z^Fintype.card F at hc
    rw [←h0,←h1,tangentPair_coordinates (W.baseChange L) hd] at hc
    simpa only [Matrix.det_fin_two,A,z,mul_comm] using hc
  exact primitive_exponent z hz hn A.det (Fintype.card F) he

#print axioms actual_general_three_determinant
end
end ThreeTangentPairingAudit
