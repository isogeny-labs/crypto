import ThreeCharacterControls

namespace ThreeTorsionCharacterAudit
noncomputable section
open Module Matrix ThreeTorsionRankAudit.General BKFrobeniusBridgeAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

/-- Actual family determinant and qualified criterion, with explicit deleted-
characteristic and singular controls. Arbitrary-curve normalization is separate. -/
theorem actual_three_character_certificate :
    (∀ (F L : Type) [Field F] [Fintype F] [Field L] [Algebra F L]
      [DecidableEq F] [DecidableEq L] [IsAlgClosed L] [NeZero (3:L)]
      (W : WeierstrassCurve.Affine F), W.a₂=0 → W.a₄=0 → W.a₆=0 → W.Δ≠0 →
      ∃ b : Basis (Fin 2) (ZMod 3) (ThreeTorsion (W.baseChange L)),
        (frobeniusMatrix W b).det=(Fintype.card F : ZMod 3) ∧
        ∀ (K : Type) [Field K] [Fintype K] [Algebra F K] [Algebra K L]
          [IsScalarTower F K L] [DecidableEq K],
          (Fintype.card F : ZMod 3)≠1 → ∀ k : ℕ, Fintype.card K=Fintype.card F^k →
          ((∀ P : (W.baseChange L).Point, (3:ℕ) • P=0 →
            P∈Set.range (pointInclusion F K L W)) ↔ 3 ∣ Fintype.card F^k-1)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [NeZero (3:K)] (a b : K)
      (hb : b≠0) (Q : (curve a b).Point), (3:ℕ) • Q=0 →
      (pointValue a b Q)^3=1 ∧
      pointValue a b (-Q)=(pointValue a b Q)⁻¹ ∧
      (pointValue a b Q=1 ↔ Q=0 ∨ Q=origin a b hb ∨ Q=-(origin a b hb)) ∧
      (pointValue a b Q≠1 → ∀ n : ℕ,
        pointValue a b (Q+n • origin a b hb)=pointValue a b Q)) ∧
    (CharP OrdinaryClosure 3 ∧ ordinaryBase.Δ≠0 ∧
      Nat.card (ThreeTorsion (curve (1:OrdinaryClosure) 1))=3 ∧
      Module.finrank (ZMod 3) (ThreeTorsion (curve (1:OrdinaryClosure) 1))=1 ∧
      (∀ (K : Type) [Field K] [Algebra (ZMod 3) K] [Algebra K OrdinaryClosure]
        [IsScalarTower (ZMod 3) K OrdinaryClosure] [DecidableEq K],
        ∀ P : (ordinaryBase.baseChange OrdinaryClosure).Point, (3:ℕ) • P=0 →
          P∈Set.range (pointInclusion (ZMod 3) K OrdinaryClosure ordinaryBase)) ∧
      (∀ k : ℕ, 0<k → ¬3 ∣ 3^k-1) ∧
      ¬((∀ P : (ordinaryBase.baseChange OrdinaryClosure).Point, (3:ℕ) • P=0 →
        P∈Set.range (pointInclusion (ZMod 3) (ZMod 3) OrdinaryClosure ordinaryBase)) ↔ 3 ∣ 3^1-1)) ∧
    (∀ (K : Type) [Field K],
      ((curve (1:K) 0).Equation 0 0 ∧ ¬(curve (1:K) 0).Nonsingular 0 0) ∧
      ((1:K)≠0 ∧ (curve (3:K) 1).Δ=0 ∧ (curve (3:K) 1).Nonsingular 0 0)) := by
  refine ⟨?_,?_,?_,?_⟩
  · intro F L _ _ _ _ _ _ _ _ W h2 h4 h6 hΔ
    have hb := third_coefficient_ne_zero W h2 h4 h6 hΔ
    let b := actualGeometricBasis (L:=L) W hΔ
    refine ⟨b,actual_tate_frobenius_determinant W h2 h4 h6 hb hΔ b,?_⟩
    intro K _ _ _ _ _ _ hq k hc
    exact actual_tate_extension_criterion W h2 h4 h6 hb hΔ hq k hc
  · intro K _ _ _ a b hb Q ht
    exact ⟨pointValue_cube a b Q ht,pointValue_neg a b Q ht,pointValue_kernel a b hb Q ht,
      fun hn n => pointValue_add_origin_nsmul a b hb Q ht hn n⟩
  · obtain ⟨hΔ,hcard,hfin⟩ := actual_char_three_rank_control
    exact ⟨inferInstance,hΔ,hcard,hfin,fun K _ _ _ _ _ => ordinary_all_torsion_in_extension,
      characteristic_three_cyclotomic_failure,deleting_coprimality_inclusion_false⟩
  · intro K _
    exact ⟨singular_origin K,nonzero_b_singular_curve K⟩

#print axioms actual_three_character_certificate
end
end ThreeTorsionCharacterAudit
