import HeightRadius

/-! Choose a finite coefficient radius from a height ratio. -/

namespace XedniUniformConstant

open XedniUniformQuotient XedniFreeTorsionQuotient XedniHeightRadius

variable {A : Type*} [AddCommGroup A] [Fintype A]

set_option linter.style.haveILetI false in
/-- An annihilating integer `c` larger than the norm ratio determines a
coefficient radius and quotient modulus, with no remaining numerical gap
hypothesis. -/
theorem bounded_relation_with_constant
    (d r c : ℕ) (points : Fin r → LiftGroup (A := A) d)
    (freeNorm : (Fin d → ℤ) → ℝ) (m U : ℝ)
    (hdegree : d < r) (hcpos : 0 < c)
    (hzero : freeNorm 0 = 0)
    (hadd : ∀ x y, freeNorm (x + y) ≤ freeNorm x + freeNorm y)
    (hsmul : ∀ (a : ℤ) x,
      freeNorm (a • x) ≤ (a.natAbs : ℝ) * freeNorm x)
    (hpoint : ∀ i, freeNorm (points i).1 ≤ U)
    (hU : 0 ≤ U)
    (hscale : ∀ (N : ℕ) u,
      freeNorm (N • u) = (N : ℝ) * freeNorm u)
    (hkillc : ∀ a : A, c • a = 0)
    (hmin : ∀ u, u ≠ 0 → m ≤ freeNorm u)
    (hratio : (2 : ℝ) * r * U < (c : ℝ) * m) :
    ∃ coeff, (∃ i, coeff i ≠ 0) ∧
      Bounded (2 * (Fintype.card A * c ^ d + 1)) coeff ∧
      eval points coeff = 0 := by
  let K := Fintype.card A * c ^ d + 1
  let N := c * K
  have hKpos : 0 < K := by dsimp [K]; omega
  have hNpos : 0 < N := by dsimp [N]; positivity
  haveI : NeZero N := ⟨by omega⟩
  have hkillN : ∀ a : A, N • a = 0 := by
    intro a
    dsimp [N]
    rw [mul_comm c K, mul_smul]
    simp [hkillc]
  have hgap : ((r : ℝ) * K * U) + ((r : ℝ) * K * U) < (N : ℝ) * m := by
    have hKreal : (0 : ℝ) < K := by exact_mod_cast hKpos
    have hmul := mul_lt_mul_of_pos_right hratio hKreal
    dsimp [N]
    push_cast
    nlinarith [hmul]
  have hrel := bounded_relation_from_point_heights d r c K N points freeNorm m U
    hdegree (le_refl _) (by dsimp [K]; omega) hzero hadd hsmul hpoint
    hU (hscale N) hkillN hmin hgap
  simpa [K] using hrel

/-- A positive minimum norm permits an explicit finite coefficient bound
for every fixed rank-gap configuration and finite torsion group. The bound
depends on the declared upper and lower norm constants. -/
theorem exists_uniform_bounded_relation
    (d r : ℕ) (points : Fin r → LiftGroup (A := A) d)
    (freeNorm : (Fin d → ℤ) → ℝ) (m U : ℝ)
    (hdegree : d < r) (hm : 0 < m)
    (hzero : freeNorm 0 = 0)
    (hadd : ∀ x y, freeNorm (x + y) ≤ freeNorm x + freeNorm y)
    (hsmul : ∀ (a : ℤ) x,
      freeNorm (a • x) ≤ (a.natAbs : ℝ) * freeNorm x)
    (hpoint : ∀ i, freeNorm (points i).1 ≤ U)
    (hU : 0 ≤ U)
    (hscale : ∀ (N : ℕ) u,
      freeNorm (N • u) = (N : ℝ) * freeNorm u)
    (hmin : ∀ u, u ≠ 0 → m ≤ freeNorm u) :
    ∃ c : ℕ, 0 < c ∧
      ∃ coeff, (∃ i, coeff i ≠ 0) ∧
        Bounded (2 * (Fintype.card A * c ^ d + 1)) coeff ∧
        eval points coeff = 0 := by
  obtain ⟨s, hs⟩ := exists_nat_gt (((2 : ℝ) * r * U) / m)
  let c := Fintype.card A * (s + 1)
  have hT : 0 < Fintype.card A := Fintype.card_pos
  have hcpos : 0 < c := by dsimp [c]; positivity
  have hkillc : ∀ a : A, c • a = 0 := by
    intro a
    dsimp [c]
    rw [mul_smul]
    exact card_nsmul_eq_zero
  have hsratio : (2 : ℝ) * r * U < (s : ℝ) * m := by
    exact (div_lt_iff₀ hm).mp hs
  have hsc : s ≤ c := by
    have hT1 : 1 ≤ Fintype.card A := hT
    have hs1 : s + 1 ≤ c := by
      calc
        s + 1 = 1 * (s + 1) := by simp
        _ ≤ Fintype.card A * (s + 1) := Nat.mul_le_mul_right _ hT1
        _ = c := rfl
    omega
  have hratio : (2 : ℝ) * r * U < (c : ℝ) * m :=
    lt_of_lt_of_le hsratio
      (mul_le_mul_of_nonneg_right (by exact_mod_cast hsc) (le_of_lt hm))
  refine ⟨c, hcpos, ?_⟩
  exact bounded_relation_with_constant d r c points freeNorm m U hdegree hcpos
    hzero hadd hsmul hpoint hU hscale hkillc hmin hratio

#print axioms XedniUniformConstant.bounded_relation_with_constant
#print axioms XedniUniformConstant.exists_uniform_bounded_relation

end XedniUniformConstant
