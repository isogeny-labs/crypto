import BinaryBasis

namespace TwoTorsionRankAudit
noncomputable section

/-- Actual geometric points/rank, full characteristic2classification and
the direct dropped-coprimality control. The qualified source node is preserved. -/
theorem actual_two_torsion_rank_certificate :
    (∀ (K : Type) [Field K] [DecidableEq K] [NeZero (2 : K)]
      (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
      Finite (TwoTorsion W) ∧ Nat.card (TwoTorsion W)=Nat.card (Root W)+1 ∧
      Nat.card (TwoTorsion W)≤4) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [NeZero (2 : K)] [IsAlgClosed K]
      (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
      Nat.card (TwoTorsion W)=4 ∧ ∃ A B : TwoTorsion W,BinaryBasis A B) ∧
    (∀ (K L : Type) [Field K] [DecidableEq K] [NeZero (2 : K)]
      [Field L] [DecidableEq L] [IsAlgClosed L]
      (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (f : K →+* L),
      Nat.card (TwoTorsion (W.map f))=4) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [CharP K 2] [IsAlgClosed K]
      (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
      Nat.card (TwoTorsion W)=if W.a₁=0 then 1 else 2) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [CharP K 2],
      (ordinaryModel K).Δ=1 ∧ (supersingularModel K).Δ=1 ∧
      Nat.card (TwoTorsion (ordinaryModel K))=2 ∧
      Nat.card (TwoTorsion (supersingularModel K))=1 ∧ addOrderOf (ordinaryPoint K)=2) ∧
    (∀ (K L : Type) [Field K] [DecidableEq K] [CharP K 2]
      [Field L] [DecidableEq L] [CharP L 2] (f : K →+* L) (Q : (ordinaryModel L).Point),
      2 • Q=0 → ∃ P : (ordinaryModel K).Point,2 • P=0 ∧ Q=ordinaryMap f P) ∧
    (Nat.Prime 2 ∧ Nat.card (ordinaryModel Base).Point=4 ∧
      2∣Nat.card (ordinaryModel Base).Point ∧ 2^2∣Nat.card (ordinaryModel Base).Point ∧
      ¬2∣2-1 ∧ Nat.gcd 2 2≠1 ∧
      (∀ k : ℕ,0<k → Nat.card (BinaryField k)=2^k ∧ ¬2∣2^k-1)) ∧
    (Nat.card (TwoTorsion (ordinaryModel Geometric))=2 ∧
      (∀ Q : (ordinaryModel Geometric).Point,2 • Q=0 →
        ∃ P : (ordinaryModel Base).Point,2 • P=0 ∧ Q=ordinaryMap baseEmbedding P) ∧
      ¬ ((∀ Q : (ordinaryModel Geometric).Point,2 • Q=0 →
        ∃ P : (ordinaryModel Base).Point,2 • P=0 ∧ Q=ordinaryMap baseEmbedding P) ↔ 2∣2-1)) := by
  refine ⟨?_,?_,?_,?_,?_,?_,?_,actual_geometric_card,actual_geometric_torsion_in_base,
    concrete_geometric_point_criterion_false⟩
  · intro K _ _ _ W hΔ
    exact ⟨torsion_finite W hΔ,torsion_card_eq W hΔ,torsion_card_le_four W hΔ⟩
  · intro K _ _ _ _ W hΔ
    exact ⟨geometric_two_torsion_card W hΔ,actual_geometric_binary_basis W hΔ⟩
  · intro K L _ _ _ _ _ _ W hΔ f
    exact geometric_scalar_extension_card W hΔ f
  · intro K _ _ _ _ W hΔ
    exact char_two_geometric_classification W hΔ
  · intro K _ _ _
    exact ⟨ordinary_delta,supersingular_delta,ordinary_torsion_card,
      supersingular_torsion_card,ordinaryPoint_order⟩
  · intro K L _ _ _ _ _ _ f Q hQ
    exact ordinary_all_torsion_in_image f Q hQ
  · obtain ⟨hp,hc,hdiv,hsq,hnot,hgcd,hfields,_⟩ := characteristic_point_inclusion_boundary
    exact ⟨hp,hc,hdiv,hsq,hnot,hgcd,hfields⟩

#print axioms actual_two_torsion_rank_certificate
end
end TwoTorsionRankAudit
