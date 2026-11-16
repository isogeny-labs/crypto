import BinaryFrobenius

namespace BKFrobeniusBridgeAudit
noncomputable section
open ThreeTorsionRankAudit.General FrobeniusPointDescentAudit Matrix Module
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance : Fact (Nat.Prime 7) := ⟨by decide⟩
abbrev ExcludedClosure := TorsionBoundaryAudit.K
private instance : DecidableEq ExcludedClosure := Classical.decEq _
private instance : CharP ExcludedClosure 7 := charP_of_injective_algebraMap' (ZMod 7) 7
private instance : NeZero (3:ExcludedClosure) := by
  constructor
  intro h
  have hd : 7 ∣ 3 := (CharP.cast_eq_zero_iff ExcludedClosure 7 3).mp h
  norm_num at hd

theorem actual_excluded_boundary :
    ∃ b : Basis (Fin 2) (ZMod 3) (ThreeTorsion (TorsionBoundaryAudit.bkCurve.baseChange ExcludedClosure)),
      frobeniusMatrix TorsionBoundaryAudit.bkCurve b≠1 ∧ (7:ZMod 3)^1=1 := by
  let b := actualGeometricBasis (L:=ExcludedClosure) TorsionBoundaryAudit.bkCurve
    TorsionBoundaryAudit.bkCurve_nonsingular
  refine ⟨b,?_,by decide +kernel⟩
  intro hm
  obtain ⟨_,_,_,_,_,P,hP,hn⟩ := bk_nonfixed_control
  have hf := (matrix_power_identity_iff TorsionBoundaryAudit.bkCurve b 1).mp
    (by simpa only [pow_one] using hm)
  exact hn (by simpa only [Function.iterate_one] using hf P hP)

/-- Actual-object bridge. The generic determinantq is deliberately a premise;
the binary specialization proves it, and the F7 boundary is an actual control. -/
theorem actual_frobenius_bridge_certificate :
    (∀ (F L : Type) [Field F] [Fintype F] [Field L] [Algebra F L]
      [DecidableEq F] [DecidableEq L] [IsAlgClosed L] [NeZero (3:L)]
      (W : WeierstrassCurve.Affine F), W.Δ≠0 →
      ∃ b : Basis (Fin 2) (ZMod 3) (ThreeTorsion (W.baseChange L)),
        (frobeniusMatrix W b).det≠0 ∧
        (∀ c : Basis (Fin 2) (ZMod 3) (ThreeTorsion (W.baseChange L)),
          (frobeniusMatrix W b).det=(frobeniusMatrix W c).det) ∧
        (∀ P : ThreeTorsion (W.baseChange L),
          frobeniusMatrix W b *ᵥ b.repr P=b.repr (linearFrobenius W P)) ∧
        (∀ k : ℕ, (frobeniusMatrix W b)^k=1 ↔
          ∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 → (pointFrobenius W)^[k] P=P) ∧
        (3 ∣ Nat.card W.Point → ∃ v : Fin 2 → ZMod 3,
          v≠0 ∧ frobeniusMatrix W b *ᵥ v=v) ∧
        (∀ (K : Type) [Field K] [Fintype K] [Algebra F K] [Algebra K L]
          [IsScalarTower F K L] [DecidableEq K] (k : ℕ),
          Fintype.card K=Fintype.card F^k →
          ((∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
            P∈Set.range (pointInclusion F K L W)) ↔
              (frobeniusMatrix W b)^k=1) ∧
          (3 ∣ Nat.card W.Point → (Fintype.card F : ZMod 3)≠1 →
            (frobeniusMatrix W b).det=(Fintype.card F : ZMod 3) →
            ((∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
              P∈Set.range (pointInclusion F K L W)) ↔
                3 ∣ Fintype.card F^k-1)))) ∧
    (CharP Binary.L 2 ∧ Binary.curve.Δ≠0 ∧ Nat.card Binary.curve.Point=3 ∧
      Nat.card (ThreeTorsion (Binary.curve.baseChange Binary.L))=9 ∧
      Binary.matrix.det=2 ∧ Binary.matrix^2=1 ∧ Binary.matrix≠1 ∧
      ∀ (K : Type) [Field K] [Fintype K] [Algebra Binary.F K] [Algebra K Binary.L]
        [IsScalarTower Binary.F K Binary.L] [DecidableEq K] (k : ℕ),
        Fintype.card K=2^k →
        ((∀ P : (Binary.curve.baseChange Binary.L).Point, (3:ℕ) • P=0 →
          P∈Set.range (pointInclusion Binary.F K Binary.L Binary.curve)) ↔ 3 ∣ 2^k-1)) ∧
    (∃ b : Basis (Fin 2) (ZMod 3) (ThreeTorsion (TorsionBoundaryAudit.bkCurve.baseChange ExcludedClosure)),
      frobeniusMatrix TorsionBoundaryAudit.bkCurve b≠1 ∧ (7:ZMod 3)^1=1) := by
  refine ⟨?_,?_,actual_excluded_boundary⟩
  · intro F L _ _ _ _ _ _ _ _ W hΔ
    let b := actualGeometricBasis (L:=L) W hΔ
    refine ⟨b,matrix_det_ne_zero W b,determinant_basis_independent W b,
      matrix_coordinates W b,matrix_power_identity_iff W b,
      actual_rational_fixed_vector W b,?_⟩
    intro K _ _ _ _ _ _ k hc
    exact ⟨actual_extension_iff_matrix_power W b k hc,
      fun hN hq hdet => actual_extension_criterion_of_det W b hN hq hdet k hc⟩
  · refine ⟨inferInstance,Binary.discriminant_ne_zero,Binary.rational_cardinality,
      Binary.geometric_cardinality,?_,Binary.matrix_square,Binary.matrix_nonidentity,?_⟩
    · simpa only [ZMod.card,Nat.cast_ofNat] using Binary.actual_determinant
    · intro K _ _ _ _ _ _ k hc
      exact Binary.actual_extension_criterion k hc

#print axioms actual_excluded_boundary
#print axioms actual_frobenius_bridge_certificate
end
end BKFrobeniusBridgeAudit
