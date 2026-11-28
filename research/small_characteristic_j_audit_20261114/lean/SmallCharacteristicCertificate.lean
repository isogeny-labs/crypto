import GeometricJ

namespace SmallCharacteristicJAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

def finiteControl : WeierstrassCurve.Affine (ZMod 3) := ⟨0,2,0,0,1⟩

theorem finite_control_delta : finiteControl.Δ=1 := by decide +kernel

private instance : finiteControl.IsElliptic := ellipticOfNonzero _ (by
  rw [finite_control_delta]
  exact one_ne_zero)

theorem finite_control_j : finiteControl.j=1 := by
  rw [WeierstrassCurve.j_eq,finite_control_delta]
  norm_num [finiteControl,WeierstrassCurve.c₄,WeierstrassCurve.b₂,WeierstrassCurve.b₄]
  decide +kernel

theorem finite_control_point_card : Nat.card finiteControl.Point=5 := by
  rw [Nat.card_congr finiteControl.nonsingularPointEquiv]
  change Nat.card (Option {xy : ZMod 3×ZMod 3 // finiteControl.Nonsingular xy.1 xy.2})=5
  let : DecidablePred (fun xy : ZMod 3×ZMod 3 => finiteControl.Nonsingular xy.1 xy.2) := fun xy =>
    decidable_of_iff (xy.2^2=xy.1^3+2*xy.1^2+1) (by
      change xy.2^2=xy.1^3+2*xy.1^2+1 ↔ finiteControl.Nonsingular xy.1 xy.2
      rw [←finiteControl.equation_iff_nonsingular_of_Δ_ne_zero (delta_nonzero finiteControl)]
      simp [WeierstrassCurve.Affine.equation_iff,finiteControl])
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem finite_control_trivial (P : finiteControl.Point) (ht : (3:ℕ) • P=0) : P=0 := by
  by_contra hn
  have ho : addOrderOf P=3 := addOrderOf_eq_prime ht hn
  have hd := addOrderOf_dvd_natCard P
  rw [ho,finite_control_point_card] at hd
  norm_num at hd

theorem finite_control_torsion_card : Nat.card (ThreeTorsion finiteControl)=1 :=
  (killed_card_one_iff 3).mpr finite_control_trivial

abbrev ControlClosure := AlgebraicClosure (ZMod 3)
private instance : DecidableEq ControlClosure := Classical.decEq _

theorem finite_control_geometric : Nat.card (ThreeTorsion (finiteControl.baseChange ControlClosure))=3 := by
  have h := three_geometric_j (L:=ControlClosure) finiteControl
  simpa only [finite_control_j,one_ne_zero,ite_false] using h

theorem deleting_algclosed_false :
    ¬(finiteControl.j=0 ↔ ∀ P : finiteControl.Point,(3:ℕ) • P=0 → P=0) := by
  intro h
  have hz := h.mpr finite_control_trivial
  rw [finite_control_j] at hz
  exact one_ne_zero hz

theorem unrestricted_j_trivial_false :
    ¬∀ (K : Type) [Field K] [DecidableEq K] [CharP K 3]
      (W : WeierstrassCurve.Affine K) [W.IsElliptic],
      W.j=0 ↔ ∀ P : W.Point,(3:ℕ) • P=0 → P=0 := by
  intro h
  exact deleting_algclosed_false (h (ZMod 3) finiteControl)

theorem small_characteristic_j_certificate :
    (∀ (K : Type) [Field K] [DecidableEq K] [IsAlgClosed K] [CharP K 2]
      (W : WeierstrassCurve.Affine K) [W.IsElliptic],
      Nat.card (TwoTorsionRankAudit.TwoTorsion W)=(if W.j=0 then 1 else 2) ∧
      Module.finrank (ZMod 2) (TwoTorsionRankAudit.TwoTorsion W)=(if W.j=0 then 0 else 1) ∧
      (W.j=0 ↔ ∀ P : W.Point,(2:ℕ) • P=0 → P=0)) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [IsAlgClosed K] [CharP K 3]
      (W : WeierstrassCurve.Affine K) [W.IsElliptic],
      Nat.card (ThreeTorsion W)=(if W.j=0 then 1 else 3) ∧
      Module.finrank (ZMod 3) (ThreeTorsion W)=(if W.j=0 then 0 else 1) ∧
      (W.j=0 ↔ ∀ P : W.Point,(3:ℕ) • P=0 → P=0)) ∧
    (∀ (F L : Type) [Field F] [Field L] [Algebra F L] [DecidableEq F]
      [DecidableEq L] [IsAlgClosed L] [CharP F 2]
      (W : WeierstrassCurve.Affine F) [W.IsElliptic],
      Nat.card (TwoTorsionRankAudit.TwoTorsion (W.baseChange L))=(if W.j=0 then 1 else 2)) ∧
    (∀ (F L : Type) [Field F] [Field L] [Algebra F L] [DecidableEq F]
      [DecidableEq L] [IsAlgClosed L] [CharP F 3]
      (W : WeierstrassCurve.Affine F) [W.IsElliptic],
      Nat.card (ThreeTorsion (W.baseChange L))=(if W.j=0 then 1 else 3)) ∧
    (finiteControl.Δ=1 ∧ finiteControl.j=1 ∧ Nat.card finiteControl.Point=5 ∧
      Nat.card (ThreeTorsion finiteControl)=1 ∧
      Nat.card (ThreeTorsion (finiteControl.baseChange ControlClosure))=3 ∧
      ¬(finiteControl.j=0 ↔ ∀ P : finiteControl.Point,(3:ℕ) • P=0 → P=0)) ∧
    (¬∀ (K : Type) [Field K] [DecidableEq K] [CharP K 3]
      (W : WeierstrassCurve.Affine K) [W.IsElliptic],
      W.j=0 ↔ ∀ P : W.Point,(3:ℕ) • P=0 → P=0) := by
  refine ⟨?_,?_,?_,?_,?_,unrestricted_j_trivial_false⟩
  · intro K _ _ _ _ W _
    exact ⟨two_j_card W,two_j_finrank W,two_j_trivial W⟩
  · intro K _ _ _ _ W _
    exact ⟨three_j_card W,three_j_finrank W,three_j_trivial W⟩
  · intro F L _ _ _ _ _ _ _ W _
    exact two_geometric_j W
  · intro F L _ _ _ _ _ _ _ W _
    exact three_geometric_j W
  · exact ⟨finite_control_delta,finite_control_j,finite_control_point_card,
      finite_control_torsion_card,finite_control_geometric,deleting_algclosed_false⟩

#print axioms finite_control_point_card
#print axioms deleting_algclosed_false
#print axioms small_characteristic_j_certificate
end
end SmallCharacteristicJAudit
