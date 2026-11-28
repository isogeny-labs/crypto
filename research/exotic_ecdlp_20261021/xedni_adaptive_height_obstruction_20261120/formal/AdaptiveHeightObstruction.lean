import UniformConstant
import XedniDependentLift

/-! Uniform point-height bounds obstruct an all-sample adaptive lift rule. -/

namespace XedniAdaptiveHeightObstruction

open XedniFreeTorsionQuotient

variable {A : Type*} [AddCommGroup A] [Fintype A]

/-- A compatible lift chosen after seeing each offset tuple cannot have the
same low-rank, pointwise height bounds for every tuple when its complete
bounded-relation coefficient box is smaller than the subgroup order. -/
theorem no_uniform_adaptive_height_lifts
    (d r q c : ℕ) [Fact q.Prime]
    (red : LiftGroup (A := A) d →+ ZMod q)
    (s : Fin r → ZMod q) (w : ZMod q)
    (lifts : (Fin r → ZMod q) → Fin r → LiftGroup (A := A) d)
    (freeNorm : (Fin d → ℤ) → ℝ) (m U : ℝ)
    (hcompat : ∀ t i,
      red (lifts t i) = XedniObstruction.xedniSampleMap s w t i)
    (hdegree : d < r) (hcpos : 0 < c)
    (hzero : freeNorm 0 = 0)
    (hadd : ∀ x y, freeNorm (x + y) ≤ freeNorm x + freeNorm y)
    (hsmul : ∀ (a : ℤ) x,
      freeNorm (a • x) ≤ (a.natAbs : ℝ) * freeNorm x)
    (hpoint : ∀ t i, freeNorm (lifts t i).1 ≤ U)
    (hU : 0 ≤ U)
    (hscale : ∀ (N : ℕ) u,
      freeNorm (N • u) = (N : ℝ) * freeNorm u)
    (hkillc : ∀ a : A, c • a = 0)
    (hmin : ∀ u, u ≠ 0 → m ≤ freeNorm u)
    (hratio : (2 : ℝ) * r * U < (c : ℝ) * m)
    (hbudget : (2 * (2 * (Fintype.card A * c ^ d + 1)) + 1) ^ r - 1 < q) :
    False := by
  let B := 2 * (Fintype.card A * c ^ d + 1)
  have hpow : 2 * B + 1 ≤ (2 * B + 1) ^ r := by
    calc
      2 * B + 1 = (2 * B + 1) ^ 1 := by simp
      _ ≤ (2 * B + 1) ^ r := pow_le_pow_right₀ (by omega) (by omega)
  have hB : B < q := by dsimp [B] at *; omega
  have hwitness (t : Fin r → ZMod q) :
      ∃ coeff, XedniObstruction.Nontrivial coeff ∧
        XedniObstruction.Relation (lifts t) coeff ∧
        XedniObstruction.Bounded B coeff := by
    obtain ⟨coeff, hn, hb, hr⟩ :=
      XedniUniformConstant.bounded_relation_with_constant d r c
        (lifts t) freeNorm m U hdegree hcpos hzero hadd hsmul
        (hpoint t) hU hscale hkillc hmin hratio
    refine ⟨coeff, hn, ?_, hb⟩
    simpa [XedniObstruction.Relation, XedniUniformQuotient.eval] using hr
  have hcount := XedniObstruction.dependent_lift_explicit_count
    red s w lifts hcompat B hB (fun t _ => by
      obtain ⟨coeff, hn, hr, hb⟩ := hwitness t
      exact ⟨coeff, hn, hr, hb⟩)
  have hall : XedniObstruction.dependentLiftSamples lifts = Finset.univ := by
    apply Finset.eq_univ_iff_forall.mpr
    intro t
    simp only [XedniObstruction.dependentLiftSamples, Finset.mem_filter,
      Finset.mem_univ, true_and]
    obtain ⟨coeff, hn, hr, _⟩ := hwitness t
    exact ⟨coeff, hn, hr⟩
  rw [hall] at hcount
  have hcard : (Finset.univ : Finset (Fin r → ZMod q)).card =
      Nat.card (Fin r → ZMod q) := Fintype.card_eq_nat_card
  have hpos : 0 < Nat.card (Fin r → ZMod q) := Nat.card_pos
  have hcardq : Nat.card (ZMod q) = q := by simp
  have hbound' : Nat.card (Fin r → ZMod q) * q ≤
      Nat.card (Fin r → ZMod q) * ((2 * B + 1) ^ r - 1) := by
    simpa [hcard, hcardq, mul_comm] using hcount
  have hstrict : Nat.card (Fin r → ZMod q) * ((2 * B + 1) ^ r - 1) <
      Nat.card (Fin r → ZMod q) * q :=
    Nat.mul_lt_mul_of_pos_left hbudget hpos
  exact (lt_irrefl _ (lt_of_le_of_lt hbound' hstrict))

#print axioms XedniAdaptiveHeightObstruction.no_uniform_adaptive_height_lifts

end XedniAdaptiveHeightObstruction
