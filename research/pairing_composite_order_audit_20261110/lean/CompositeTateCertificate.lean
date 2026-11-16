import CompositeTateThree
import CompositeTateFive

/-! Exact literal composite-order root hypothesis from Galbraith26.3.3.
The named domains are the actual rational torsion kernel and actual quotient
of the curve group, and the target is actual field units modulo powers.
No assumed canonical-pairing implementation is used to prove impossibility. -/
namespace CompositeTateCertificateAudit
open CompositeTateObstructionAudit
private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩

def SourceConditions (q : ℕ) [Fact (Nat.Prime q)] (n : ℕ)
    (E : WeierstrassCurve.Affine (ZMod q)) : Prop :=
  E.Δ≠0 ∧ Nat.Coprime n q ∧ n∣Nat.card E.Point ∧
    ∃ z : (ZMod q)ˣ,z^n=1 ∧ z≠1

theorem ternary_source_conditions :
    SourceConditions 3 4 CompositeTateThreeAudit.curve := by
  have h := CompositeTateThreeAudit.source_hypotheses
  exact ⟨CompositeTateThreeAudit.curve_nonsingular,h.1,h.2.1,
    CompositeTateThreeAudit.root,h.2.2⟩

theorem quinary_source_conditions :
    SourceConditions 5 6 CompositeTateFiveAudit.curve := by
  have h := CompositeTateFiveAudit.source_hypotheses
  exact ⟨CompositeTateFiveAudit.curve_nonsingular,h.1,h.2.1,
    CompositeTateFiveAudit.root,h.2.2⟩

theorem ternary_no_pairing : ¬PairingExists CompositeTateThreeAudit.curve 4 :=
  CompositeTateThreeAudit.no_left_nondegenerate_bilinear_pairing

theorem quinary_no_pairing : ¬PairingExists CompositeTateFiveAudit.curve 6 :=
  CompositeTateFiveAudit.no_left_nondegenerate_bilinear_pairing

theorem literal_nontrivial_root_nondegeneracy_claim_false :
    ¬∀ (q : ℕ) [Fact (Nat.Prime q)] (n : ℕ)
      (E : WeierstrassCurve.Affine (ZMod q)),
      SourceConditions q n E→PairingExists E n := by
  intro h
  exact ternary_no_pairing (h 3 4 CompositeTateThreeAudit.curve ternary_source_conditions)

theorem composite_tate_audit_certificate :
    SourceConditions 3 4 CompositeTateThreeAudit.curve ∧
    Nat.card CompositeTateThreeAudit.curve.Point=4 ∧
    addOrderOf CompositeTateThreeAudit.P=4 ∧
    Nat.card CompositeTateThreeAudit.Left=4 ∧
    Nat.card CompositeTateThreeAudit.Right=4 ∧
    (¬∃ z : CompositeTateThreeAudit.U,IsPrimitiveRoot z 4) ∧
    (∀ z : CompositeTateThreeAudit.Target,z^2=1) ∧
    ¬PairingExists CompositeTateThreeAudit.curve 4 ∧
    SourceConditions 5 6 CompositeTateFiveAudit.curve ∧
    Nat.card CompositeTateFiveAudit.curve.Point=6 ∧
    addOrderOf CompositeTateFiveAudit.P=3 ∧
    Nat.card CompositeTateFiveAudit.Left=6 ∧
    Nat.card CompositeTateFiveAudit.Right=6 ∧
    (¬∃ z : CompositeTateFiveAudit.U,IsPrimitiveRoot z 6) ∧
    (∀ z : CompositeTateFiveAudit.Target,z^2=1) ∧
    ¬PairingExists CompositeTateFiveAudit.curve 6 ∧
    ¬ (∀ (q : ℕ) [Fact (Nat.Prime q)] (n : ℕ)
      (E : WeierstrassCurve.Affine (ZMod q)),
      SourceConditions q n E→PairingExists E n) := by
  exact ⟨ternary_source_conditions,CompositeTateThreeAudit.curve_cardinality,
    CompositeTateThreeAudit.P_order,CompositeTateThreeAudit.left_cardinality,
    CompositeTateThreeAudit.right_cardinality,CompositeTateThreeAudit.primitive_root_absent,
    CompositeTateThreeAudit.target_square,ternary_no_pairing,
    quinary_source_conditions,CompositeTateFiveAudit.curve_cardinality,
    CompositeTateFiveAudit.P_order,CompositeTateFiveAudit.left_cardinality,
    CompositeTateFiveAudit.right_cardinality,CompositeTateFiveAudit.primitive_root_absent,
    CompositeTateFiveAudit.target_square,quinary_no_pairing,
    literal_nontrivial_root_nondegeneracy_claim_false⟩

#print axioms ternary_no_pairing
#print axioms quinary_no_pairing
#print axioms literal_nontrivial_root_nondegeneracy_claim_false
#print axioms composite_tate_audit_certificate
end CompositeTateCertificateAudit
