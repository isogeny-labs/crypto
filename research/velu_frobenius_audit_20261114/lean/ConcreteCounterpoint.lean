import ActualCounterpoint
import CyclicStability

namespace VeluFrobeniusAudit
noncomputable section
open WeierstrassCurve.Affine ThreeTorsionRankAudit FrobeniusPointDescentAudit
variable {K : Type*} [Field K] [DecidableEq K] [Algebra (ZMod 5) K] [CharP K 5]

def mixedPoint (a : K) (ha : a ^ 2 = 2) : (curve (K := K)).Point :=
  rationalPoint + antiPoint a ha

theorem mixedPoint_nonzero (a : K) (ha : a ^ 2 = 2) : mixedPoint a ha ≠ 0 := by
  intro hz
  have he := add_eq_zero_iff_eq_neg.mp hz
  simp only [rationalPoint, antiPoint, Point.neg_some, Point.some.injEq] at he
  exact zero_ne_one he.1

theorem mixedPoint_counter (a : K) (ha : a ^ 2 = 2) :
    addOrderOf (mixedPoint a ha) = 3 ∧
      pointFrobenius baseCurve (mixedPoint a ha) ∉ AddSubgroup.zmultiples (mixedPoint a ha) :=
  by
    have hh := mixed_point_outside_cyclic (G := (curve (K := K)).Point)
      (pointFrobenius (L := K) baseCurve) (rationalPoint (K := K)) (antiPoint a ha)
      rationalPoint_three (antiPoint_three a ha) rationalPoint_nonzero (antiPoint_nonzero a ha)
      (mixedPoint_nonzero a ha) rationalPoint_frobenius (antiPoint_frobenius a ha)
    simpa only [mixedPoint] using hh

theorem mixedPoint_no_scalar (a : K) (ha : a ^ 2 = 2) :
    ¬∃ k : ℤ, k • mixedPoint a ha = pointFrobenius baseCurve (mixedPoint a ha) := by
  intro hk
  exact (mixedPoint_counter a ha).2 (AddSubgroup.mem_zmultiples_iff.mpr hk)

theorem mixedPoint_not_rational (a : K) (ha : a ^ 2 = 2) :
    ¬∃ Q : baseCurve.Point, Point.map (W' := baseCurve) (Algebra.ofId (ZMod 5) K) Q =
      mixedPoint a ha := by
  intro hQ
  have hfix := (point_frobenius_fixed_iff baseCurve (mixedPoint a ha)).mpr hQ
  apply (mixedPoint_counter a ha).2
  rw [hfix]
  exact AddSubgroup.mem_zmultiples _

theorem antiPoint_stable (a : K) (ha : a ^ 2 = 2) :
    ∀ Q ∈ AddSubgroup.zmultiples (antiPoint a ha),
      pointFrobenius baseCurve Q ∈ AddSubgroup.zmultiples (antiPoint a ha) := by
  apply (cyclic_stable_iff_scalar _ _).mpr
  refine ⟨-1, ?_⟩
  simp [antiPoint_frobenius a ha]

theorem antiPoint_not_rational (a : K) (ha : a ^ 2 = 2) :
    ¬∃ Q : baseCurve.Point, Point.map (W' := baseCurve) (Algebra.ofId (ZMod 5) K) Q =
      antiPoint a ha := by
  intro hQ
  have hf := (point_frobenius_fixed_iff baseCurve (antiPoint a ha)).mpr hQ
  rw [antiPoint_frobenius] at hf
  have h2 : (2 : ℕ) • antiPoint a ha = 0 := by
    rw [two_nsmul]
    exact (congrArg (fun X => antiPoint a ha + X) hf.symm).trans (add_neg_cancel _)
  have hz := antiPoint_three a ha
  rw [show (3 : ℕ) = 2 + 1 by rfl, add_nsmul, h2, zero_add, one_nsmul] at hz
  exact antiPoint_nonzero a ha hz

abbrev F25 := GaloisField 5 2
instance field25Fintype : Fintype F25 := Fintype.ofFinite _
instance field25DecidableEq : DecidableEq F25 := Classical.decEq _

theorem field25_card : Fintype.card F25 = 25 := by
  rw [← Nat.card_eq_fintype_card]
  exact (GaloisField.card 5 2 (by decide)).trans (by norm_num)

theorem field25_degree : Module.finrank (ZMod 5) F25 = 2 := GaloisField.finrank 5 (by decide)

theorem field25_square_two : ∃ a : F25, a ^ 2 = 2 := by
  have hF : ringChar F25 ≠ 2 := by rw [ringChar.eq F25 5]; decide
  have h2 : (2 : F25) ≠ 0 := (CharP.cast_eq_zero_iff F25 5 2).not.mpr (by decide)
  have hs : IsSquare (2 : F25) := (FiniteField.isSquare_iff hF h2).mpr (by
    rw [field25_card]
    norm_num only [Nat.reduceSub, Nat.reduceDiv]
    have hn : (4095 : F25) = 0 := (CharP.cast_eq_zero_iff F25 5 4095).mpr (by decide)
    linear_combination hn)
  obtain ⟨a, ha⟩ := hs
  exact ⟨a, by simpa only [pow_two] using ha.symm⟩

theorem finite_curve_counter :
    Fintype.card F25 = 25 ∧ Module.finrank (ZMod 5) F25 = 2 ∧
      (curve (K := F25)).Δ ≠ 0 ∧
      ∃ P : (curve (K := F25)).Point, addOrderOf P = 3 ∧
        pointFrobenius baseCurve P ∉ AddSubgroup.zmultiples P ∧
        (¬∃ k : ℤ, k • P = pointFrobenius baseCurve P) ∧
        (¬∃ Q : baseCurve.Point, Point.map (W' := baseCurve) (Algebra.ofId (ZMod 5) F25) Q = P) := by
  obtain ⟨a, ha⟩ := field25_square_two
  exact ⟨field25_card, field25_degree, curve_delta_ne, mixedPoint a ha,
    (mixedPoint_counter a ha).1, (mixedPoint_counter a ha).2,
    mixedPoint_no_scalar a ha, mixedPoint_not_rational a ha⟩

theorem finite_stable_irrational_control :
    ∃ B : (curve (K := F25)).Point, addOrderOf B = 3 ∧ pointFrobenius baseCurve B = -B ∧
      (∀ Q ∈ AddSubgroup.zmultiples B, pointFrobenius baseCurve Q ∈ AddSubgroup.zmultiples B) ∧
      (¬∃ Q : baseCurve.Point, Point.map (W' := baseCurve) (Algebra.ofId (ZMod 5) F25) Q = B) := by
  obtain ⟨a, ha⟩ := field25_square_two
  let _ : Fact (Nat.Prime 3) := ⟨by decide +kernel⟩
  exact ⟨antiPoint a ha, addOrderOf_eq_prime (antiPoint_three a ha) (antiPoint_nonzero a ha),
    antiPoint_frobenius a ha, antiPoint_stable a ha, antiPoint_not_rational a ha⟩

#print axioms mixedPoint_counter
#print axioms finite_curve_counter
#print axioms finite_stable_irrational_control
end
end VeluFrobeniusAudit
