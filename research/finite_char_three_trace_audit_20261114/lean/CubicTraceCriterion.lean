import CubicThreeTorsion

namespace FiniteCharThreeTraceAudit
noncomputable section
open QuadraticTwistCountAudit SmallPrimeTraceGeometryAudit SupersingularTraceClassesAudit
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
variable {K : Type*} [Field K] [Fintype K] [CharP K 3]

theorem three_divides_field_card : 3∣Fintype.card K := by
  obtain ⟨n,_,hq⟩ := FiniteField.card K 3
  rw [hq]
  exact dvd_pow_self 3 n.pos.ne'

theorem source_iff_three_div_trace (W : WeierstrassCurve.Affine K) :
    sourceSupersingular W ↔ (3:ℤ)∣fieldTrace W := by
  obtain ⟨n,_,hq⟩ := FiniteField.card K 3
  have hc : Int.gcd (fieldTrace W) (Fintype.card K:ℤ)=1 ↔ ¬(3:ℤ)∣fieldTrace W := by
    rw [Int.gcd_def,Int.natAbs_natCast,hq]
    change Nat.Coprime (fieldTrace W).natAbs (3^n.val) ↔ ¬(3:ℤ)∣fieldTrace W
    rw [Nat.coprime_pow_right_iff n.pos,Nat.coprime_comm,Nat.prime_three.coprime_iff_not_dvd]
    exact not_congr (Int.natCast_dvd (m:=3)).symm
  change ¬Int.gcd (fieldTrace W) (Fintype.card K:ℤ)=1 ↔ (3:ℤ)∣fieldTrace W
  rw [hc,not_not]

theorem source_iff_count_mod_three (W : WeierstrassCurve.Affine K) :
    sourceSupersingular W ↔ Nat.card W.Point%3=1 := by
  rw [source_iff_three_div_trace W,Int.dvd_iff_emod_eq_zero]
  have hq : (Fintype.card K:ℤ)%3=0 :=
    Int.emod_eq_zero_of_dvd (Int.natCast_dvd_natCast.mpr (three_divides_field_card (K:=K)))
  unfold fieldTrace
  omega

theorem source_false_of_three_div_count (W : WeierstrassCurve.Affine K)
    (hd : 3∣Nat.card W.Point) : ¬sourceSupersingular W := by
  rw [source_iff_count_mod_three W]
  have hm := Nat.dvd_iff_mod_eq_zero.mp hd
  omega

variable [DecidableEq K]

theorem no_three_divisor_when_b2_zero (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (hb : W.b₂=0) : ¬3∣Nat.card W.Point := by
  intro hd
  obtain ⟨P,hP,_,hk⟩ := BKLinearAudit.actual_rational_prime_torsion W 3 (by decide) hd
  exact hP (SmallCharacteristicJAudit.only_zero_when_b2_zero W hΔ hb P hk)

/-- The exact source predicate for every cubic over every finite field of
characteristic three. Trace divisibility is derived; Hasse is not assumed. -/
theorem cubic_source_iff_a_zero (a b c : K) (hΔ : (cubicModel a b c).Δ≠0) :
    sourceSupersingular (cubicModel a b c) ↔ a=0 := by
  obtain ⟨d,hd⟩ := FiniteField.exists_nonsquare (F:=K) char_three_odd
  have hd0 : d≠0 := by
    intro h
    apply hd
    rw [h]
    exact ⟨0,by simp⟩
  have hΔt : (twistModel a b c d).Δ≠0 := by
    rw [twist_discriminant]
    exact mul_ne_zero (pow_ne_zero 6 hd0) hΔ
  constructor
  · intro hs
    by_contra ha
    rcases ordinary_either_three_point a b c d hΔ ha hd with hp | hp
    · exact source_false_of_three_div_count _ (three_point_dvd_count _ hp) hs
    · have hst : sourceSupersingular (twistModel a b c d) :=
        (actual_twist_source_iff char_three_odd a b c d hΔ hd).mpr hs
      exact source_false_of_three_div_count _ (three_point_dvd_count _ hp) hst
  · intro ha
    subst a
    apply (source_iff_count_mod_three _).mpr
    have hn := no_three_divisor_when_b2_zero (cubicModel (0:K) b c) hΔ
      (by simp [cubicModel,WeierstrassCurve.b₂])
    have hnt := no_three_divisor_when_b2_zero (twistModel (0:K) b c d) hΔt
      (by simp [twistModel,cubicModel,WeierstrassCurve.b₂])
    have hc := actual_twist_count_sum char_three_odd (0:K) b c d hΔ hd
    have hq := Nat.dvd_iff_mod_eq_zero.mp (three_divides_field_card (K:=K))
    have hn0 : Nat.card (cubicModel (0:K) b c).Point%3≠0 := by
      intro h
      exact hn (Nat.dvd_iff_mod_eq_zero.mpr h)
    have hnt0 : Nat.card (twistModel (0:K) b c d).Point%3≠0 := by
      intro h
      exact hnt (Nat.dvd_iff_mod_eq_zero.mpr h)
    omega

#print axioms source_iff_three_div_trace
#print axioms source_iff_count_mod_three
#print axioms cubic_source_iff_a_zero
end
end FiniteCharThreeTraceAudit
