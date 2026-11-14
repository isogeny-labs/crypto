import GeneralThreeTrace
import SmallPrimeTraceCertificate

namespace FiniteCharThreeTraceAudit
noncomputable section
open QuadraticTwistCountAudit SmallPrimeTraceGeometryAudit SupersingularTraceClassesAudit
open WeierstrassCurve.Affine ThreeTorsionRankAudit.General
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
abbrev F3 := ZMod 3

/-- Reuse the preceding actual ordinary F3 model and its certified #E=5. -/
def ordinaryCurve : WeierstrassCurve.Affine F3 := cubicModel 2 0 1
def ordinaryTwist : WeierstrassCurve.Affine F3 := twistModel 2 0 1 2
def ssCurve : WeierstrassCurve.Affine F3 := cubicModel 0 2 1
def ssTwist : WeierstrassCurve.Affine F3 := twistModel 0 2 1 2

theorem ordinary_delta : ordinaryCurve.Δ≠0 := by decide +kernel
theorem ordinary_twist_delta : ordinaryTwist.Δ≠0 := by decide +kernel
theorem ss_delta : ssCurve.Δ≠0 := by decide +kernel
theorem ss_twist_delta : ssTwist.Δ≠0 := by decide +kernel

instance ordinaryElliptic : ordinaryCurve.IsElliptic :=
  SmallCharacteristicJAudit.ellipticOfNonzero _ ordinary_delta
instance ordinaryTwistElliptic : ordinaryTwist.IsElliptic :=
  SmallCharacteristicJAudit.ellipticOfNonzero _ ordinary_twist_delta
instance ssElliptic : ssCurve.IsElliptic := SmallCharacteristicJAudit.ellipticOfNonzero _ ss_delta
instance ssTwistElliptic : ssTwist.IsElliptic := SmallCharacteristicJAudit.ellipticOfNonzero _ ss_twist_delta

theorem two_nonsquare : ¬IsSquare (2:F3) := by
  change ¬∃ y : F3,(2:F3)=y*y
  decide +kernel

theorem ordinary_count : Nat.card ordinaryCurve.Point=5 :=
  SmallCharacteristicJAudit.finite_control_point_card
theorem ordinary_rational_three_card : Nat.card (ThreeTorsion ordinaryCurve)=1 :=
  SmallCharacteristicJAudit.finite_control_torsion_card
theorem ordinary_j : ordinaryCurve.j=1 := SmallCharacteristicJAudit.finite_control_j

theorem ordinary_twist_count : Nat.card ordinaryTwist.Point=3 := by
  have h := actual_twist_count_sum (char_three_odd (K:=F3)) 2 0 1 2 ordinary_delta two_nonsquare
  change Nat.card ordinaryCurve.Point+Nat.card ordinaryTwist.Point=2*(Fintype.card F3+1) at h
  rw [ordinary_count] at h
  norm_num only [ZMod.card] at h
  omega

def twistPoint : ordinaryTwist.Point := .some 1 1
  ((ordinaryTwist.equation_iff_nonsingular_of_Δ_ne_zero ordinary_twist_delta).mp (by
    rw [WeierstrassCurve.Affine.equation_iff]
    decide +kernel))
theorem twist_point_killed : (3:ℕ) • twistPoint=0 := by decide +kernel
theorem twist_point_order : addOrderOf twistPoint=3 :=
  addOrderOf_eq_prime twist_point_killed (Point.some_ne_zero _)

theorem ordinary_twist_rational_three_card : Nat.card (ThreeTorsion ordinaryTwist)=3 := by
  rcases ThreeTorsionCharThreeAudit.char_three_card_one_or_three ordinaryTwist ordinary_twist_delta with h | h
  · obtain ⟨hs,_⟩ := Nat.card_eq_one_iff_unique.mp h
    have hz := hs.elim (⟨twistPoint,twist_point_killed⟩ : ThreeTorsion ordinaryTwist) 0
    exact False.elim (Point.some_ne_zero _ (congrArg Subtype.val hz))
  · exact h

theorem ordinary_source_false : ¬sourceSupersingular ordinaryCurve := by
  rw [source_iff_count_mod_three,ordinary_count]
  decide +kernel
theorem ordinary_twist_source_false : ¬sourceSupersingular ordinaryTwist := by
  rw [source_iff_count_mod_three,ordinary_twist_count]
  decide +kernel

theorem rational_replacement_false :
    ¬(sourceSupersingular ordinaryCurve ↔ Nat.card (ThreeTorsion ordinaryCurve)=1) := by
  intro h
  exact ordinary_source_false (h.mpr ordinary_rational_three_card)

/-- Reuse the preceding actual supersingular F3 curve of point order/count7. -/
theorem ss_count : Nat.card ssCurve.Point=7 := sextic_count
theorem ss_twist_count : Nat.card ssTwist.Point=1 := by
  have h := actual_twist_count_sum (char_three_odd (K:=F3)) 0 2 1 2 ss_delta two_nonsquare
  change Nat.card ssCurve.Point+Nat.card ssTwist.Point=2*(Fintype.card F3+1) at h
  rw [ss_count] at h
  norm_num only [ZMod.card] at h
  omega
theorem ss_source : sourceSupersingular ssCurve :=
  (cubic_source_iff_a_zero 0 2 1 ss_delta).mpr rfl
theorem ss_twist_source : sourceSupersingular ssTwist :=
  (actual_twist_source_iff (char_three_odd (K:=F3)) 0 2 1 2 ss_delta two_nonsquare).mpr ss_source
theorem ss_trace : fieldTrace ssCurve= -3 := by simp [fieldTrace,ss_count]
theorem ss_twist_trace : fieldTrace ssTwist=3 := by simp [fieldTrace,ss_twist_count]

theorem trace_zero_strengthening_false :
    ¬∀ W : WeierstrassCurve.Affine F3,W.Δ≠0 → sourceSupersingular W → fieldTrace W=0 := by
  intro h
  have hh := h ssCurve ss_delta ss_source
  rw [ss_trace] at hh
  contradiction

/-- Every finite characteristic-three field, with actual normalization,
paired ordinary torsion, geometric criteria and explicit negative controls. -/
theorem finite_char_three_trace_certificate :
    (∀ (K : Type) [Field K] [Fintype K] [DecidableEq K] [CharP K 3]
      (W : WeierstrassCurve.Affine K) [W.IsElliptic],
      (squareCompleted W).Δ=W.Δ ∧ Nonempty ((squareCompleted W).Point ≃+ W.Point) ∧
      Nat.card (squareCompleted W).Point=Nat.card W.Point ∧
      (sourceSupersingular W ↔ (3:ℤ)∣fieldTrace W) ∧
      (sourceSupersingular W ↔ Nat.card W.Point%3=1) ∧
      (sourceSupersingular W ↔ W.b₂=0) ∧ (sourceSupersingular W ↔ W.j=0) ∧
      (sourceSupersingular W → Nat.card (ThreeTorsion W)=1)) ∧
    (∀ (K L : Type) [Field K] [Field L] [Fintype K] [DecidableEq K] [DecidableEq L]
      [CharP K 3] [Algebra K L] [IsAlgClosed L]
      (W : WeierstrassCurve.Affine K) [W.IsElliptic],
      (sourceSupersingular W ↔ Nat.card (ThreeTorsion (W.baseChange L))=1) ∧
      (sourceSupersingular W ↔ Module.finrank (ZMod 3) (ThreeTorsion (W.baseChange L))=0) ∧
      Nat.card (ThreeTorsion (W.baseChange L))=(if W.j=0 then 1 else 3) ∧
      Module.finrank (ZMod 3) (ThreeTorsion (W.baseChange L))=(if W.j=0 then 0 else 1)) ∧
    (∀ (K L : Type) [Field K] [Field L] [Fintype K] [Fintype L] [DecidableEq K] [DecidableEq L]
      [CharP K 3] [Algebra K L] (W : WeierstrassCurve.Affine K) [W.IsElliptic],
      sourceSupersingular (W.baseChange L) ↔ sourceSupersingular W) ∧
    (∀ (K : Type) [Field K] [Fintype K] [DecidableEq K] [CharP K 3],
      ∀ a b c d : K,(cubicModel a b c).Δ≠0 → a≠0 → ¬IsSquare d →
      (∃ P : (cubicModel a b c).Point,P≠0 ∧ (3:ℕ) • P=0) ∨
      (∃ P : (twistModel a b c d).Point,P≠0 ∧ (3:ℕ) • P=0)) ∧
    (ordinaryCurve.Δ≠0 ∧ ordinaryTwist.Δ≠0 ∧ ordinaryCurve.j=1 ∧
      Nat.card ordinaryCurve.Point=5 ∧ Nat.card ordinaryTwist.Point=3 ∧
      Nat.card (ThreeTorsion ordinaryCurve)=1 ∧ Nat.card (ThreeTorsion ordinaryTwist)=3 ∧
      addOrderOf twistPoint=3 ∧ ¬sourceSupersingular ordinaryCurve ∧ ¬sourceSupersingular ordinaryTwist ∧
      ¬(sourceSupersingular ordinaryCurve ↔ Nat.card (ThreeTorsion ordinaryCurve)=1)) ∧
    (ssCurve.Δ≠0 ∧ ssTwist.Δ≠0 ∧ sourceSupersingular ssCurve ∧ sourceSupersingular ssTwist ∧
      Nat.card ssCurve.Point=7 ∧ Nat.card ssTwist.Point=1 ∧ fieldTrace ssCurve= -3 ∧ fieldTrace ssTwist=3) ∧
    (¬∀ W : WeierstrassCurve.Affine F3,W.Δ≠0 → sourceSupersingular W → fieldTrace W=0) := by
  refine ⟨?_,?_,?_,?_,?_,?_,trace_zero_strengthening_false⟩
  · intro K _ _ _ _ W _
    have hΔ := SmallCharacteristicJAudit.delta_nonzero W
    exact ⟨completed_delta W,⟨completedEquiv W hΔ⟩,completed_card W hΔ,
      source_iff_three_div_trace W,source_iff_count_mod_three W,source_iff_b2_zero W hΔ,
      source_iff_j_zero W,source_implies_rational_three_card_one W hΔ⟩
  · intro K L _ _ _ _ _ _ _ _ W _
    exact ⟨source_iff_geometric_three_card_one W,source_iff_geometric_three_rank_zero W,
      SmallCharacteristicJAudit.three_geometric_j W,geometric_three_rank W⟩
  · intro K L _ _ _ _ _ _ _ _ W _
    exact finite_extension_source_iff W
  · intro K _ _ _ _ a b c d hΔ ha hd
    exact ordinary_either_three_point a b c d hΔ ha hd
  · exact ⟨ordinary_delta,ordinary_twist_delta,ordinary_j,ordinary_count,ordinary_twist_count,
      ordinary_rational_three_card,ordinary_twist_rational_three_card,twist_point_order,
      ordinary_source_false,ordinary_twist_source_false,rational_replacement_false⟩
  · exact ⟨ss_delta,ss_twist_delta,ss_source,ss_twist_source,ss_count,ss_twist_count,ss_trace,ss_twist_trace⟩

#print axioms ordinary_twist_rational_three_card
#print axioms rational_replacement_false
#print axioms trace_zero_strengthening_false
#print axioms finite_char_three_trace_certificate
end
end FiniteCharThreeTraceAudit
