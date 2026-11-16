import FiniteTwoTorsion

namespace FiniteCharTwoTraceAudit
noncomputable section
open SmallPrimeTraceGeometryAudit SupersingularTraceClassesAudit TwoTorsionRankAudit

theorem prime_power_trace_parity (q N n : ℕ) (hn : 0<n) (hq : q=2^n) :
    Int.gcd ((q:ℤ)+1-N) (q:ℤ)≠1 ↔ ¬2∣N := by
  let t : ℤ := (q:ℤ)+1-N
  have hc : Int.gcd t (q:ℤ)=1 ↔ ¬(2:ℤ)∣t := by
    rw [Int.gcd_def,Int.natAbs_natCast,hq]
    change Nat.Coprime t.natAbs (2^n) ↔ ¬(2:ℤ)∣t
    rw [Nat.coprime_pow_right_iff hn,Nat.coprime_comm,Nat.prime_two.coprime_iff_not_dvd]
    exact not_congr (Int.natCast_dvd (m:=2)).symm
  have hqd : (2:ℕ)∣q := by
    rw [hq]
    exact dvd_pow_self 2 (Nat.ne_of_gt hn)
  have hqm : (q:ℤ)%2=0 := Int.emod_eq_zero_of_dvd (Int.natCast_dvd_natCast.mpr hqd)
  have ht : (2:ℤ)∣t ↔ ¬2∣N := by
    rw [Int.dvd_iff_emod_eq_zero,Nat.dvd_iff_mod_eq_zero]
    dsimp [t]
    omega
  change ¬Int.gcd t (q:ℤ)=1 ↔ ¬2∣N
  rw [hc,not_not]
  exact ht

variable {K : Type*} [Field K] [Fintype K] [DecidableEq K] [CharP K 2]

omit [DecidableEq K] in
theorem source_iff_odd_actual_count (W : WeierstrassCurve.Affine K) :
    sourceSupersingular W ↔ ¬2∣Nat.card W.Point := by
  obtain ⟨n,_,hq⟩ := FiniteField.card K 2
  exact prime_power_trace_parity _ _ n.val n.pos hq

theorem source_iff_a1_zero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    sourceSupersingular W ↔ W.a₁=0 := by
  rw [source_iff_odd_actual_count W,two_dvd_actual_count_iff W hΔ,not_not]

theorem source_iff_j_zero (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    sourceSupersingular W ↔ W.j=0 := by
  rw [W.j_eq_zero_iff_of_char_two]
  exact source_iff_a1_zero W (SmallCharacteristicJAudit.delta_nonzero W)

theorem source_iff_rational_two_card_one (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) :
    sourceSupersingular W ↔ Nat.card (TwoTorsion W)=1 := by
  rw [source_iff_a1_zero W hΔ,finite_two_card W hΔ]
  by_cases ha : W.a₁=0 <;> simp [ha]

theorem finite_j_two_card (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    Nat.card (TwoTorsion W)=if W.j=0 then 1 else 2 := by
  rw [finite_two_card W (SmallCharacteristicJAudit.delta_nonzero W)]
  simp only [W.j_eq_zero_iff_of_char_two]

theorem finite_j_two_finrank (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    Module.finrank (ZMod 2) (TwoTorsion W)=if W.j=0 then 0 else 1 := by
  have : Finite W.Point := BKLinearAudit.actual_rational_point_finite W
  have hc := Module.natCard_eq_pow_finrank (K:=ZMod 2) (V:=TwoTorsion W)
  have hn : Nat.card (ZMod 2)=2 := by simp
  rw [hn,finite_j_two_card W] at hc
  by_cases hj : W.j=0
  · rw [ite_eq_left hj] at hc ⊢
    exact Nat.pow_right_injective (by decide : 2≤2)
      (show 2^Module.finrank (ZMod 2) (TwoTorsion W)=2^0 from hc.symm)
  · rw [ite_eq_right hj] at hc ⊢
    exact Nat.pow_right_injective (by decide : 2≤2)
      (show 2^Module.finrank (ZMod 2) (TwoTorsion W)=2^1 from hc.symm)

section FiniteExtension
variable {L : Type*} [Field L] [Fintype L] [Algebra K L] [DecidableEq L]

theorem finite_extension_source_iff (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    sourceSupersingular (W.baseChange L) ↔ sourceSupersingular W := by
  have : CharP L 2 := charP_of_injective_algebraMap' K 2
  have : (W.baseChange L).IsElliptic := SmallCharacteristicJAudit.ellipticOfNonzero _
    (BKThreeNormalizationAudit.base_discriminant_nonzero W (SmallCharacteristicJAudit.delta_nonzero W))
  rw [source_iff_j_zero,source_iff_j_zero]
  change (W.map (algebraMap K L)).j=0 ↔ W.j=0
  rw [WeierstrassCurve.map_j]
  exact map_eq_zero (algebraMap K L)
end FiniteExtension

section Scalar
variable {L : Type*} [Field L] [Algebra K L] [DecidableEq L] [IsAlgClosed L]

theorem source_iff_geometric_two_card_one (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    sourceSupersingular W ↔ Nat.card (TwoTorsion (W.baseChange L))=1 := by
  rw [source_iff_j_zero W,SmallCharacteristicJAudit.two_geometric_j W]
  by_cases hj : W.j=0 <;> simp [hj]

theorem rational_geometric_two_card_equal (W : WeierstrassCurve.Affine K) [W.IsElliptic] :
    Nat.card (TwoTorsion (W.baseChange L))=Nat.card (TwoTorsion W) := by
  rw [SmallCharacteristicJAudit.two_geometric_j W,
    finite_two_card W (SmallCharacteristicJAudit.delta_nonzero W)]
  simp only [W.j_eq_zero_iff_of_char_two]
end Scalar

#print axioms prime_power_trace_parity
#print axioms source_iff_a1_zero
#print axioms source_iff_j_zero
#print axioms source_iff_geometric_two_card_one
#print axioms rational_geometric_two_card_equal
#print axioms finite_j_two_finrank
#print axioms finite_extension_source_iff
end
end FiniteCharTwoTraceAudit
