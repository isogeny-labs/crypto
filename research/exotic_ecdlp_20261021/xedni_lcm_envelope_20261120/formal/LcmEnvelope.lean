import TorsionEnvelope

/-! The least common multiple of all admissible cardinalities is a common
annihilator, often much smaller than the factorial size bound. -/

namespace XedniLcmEnvelope

/-- LCM of the positive integers at most `T`, with value one at zero. -/
def universalExponent : ℕ → ℕ
  | 0 => 1
  | T + 1 => Nat.lcm (universalExponent T) (T + 1)

theorem universalExponent_pos (T : ℕ) : 0 < universalExponent T := by
  induction T with
  | zero => simp [universalExponent]
  | succ T ih =>
      simpa [universalExponent] using Nat.lcm_pos ih (Nat.succ_pos T)

theorem dvd_universalExponent (T n : ℕ) (hnpos : 0 < n) (hle : n ≤ T) :
    n ∣ universalExponent T := by
  induction T with
  | zero => omega
  | succ T ih =>
      by_cases h : n ≤ T
      · exact (ih h).trans (by
          simpa [universalExponent] using
            Nat.dvd_lcm_left (universalExponent T) (T + 1))
      · have hn : n = T + 1 := by omega
        subst n
        simpa [universalExponent] using
          Nat.dvd_lcm_right (universalExponent T) (T + 1)

/-- The envelope is the least positive common multiple of all admissible
group cardinalities, in the divisibility order. -/
theorem universalExponent_dvd_of_all (T M : ℕ)
    (hall : ∀ n, 0 < n → n ≤ T → n ∣ M) :
    universalExponent T ∣ M := by
  induction T with
  | zero => simp [universalExponent]
  | succ T ih =>
      apply Nat.lcm_dvd
      · exact ih (fun n hn hle => hall n hn (Nat.le_succ_of_le hle))
      · exact hall (T + 1) (Nat.succ_pos T) le_rfl

theorem universalExponent_kills_bounded_group
    (A : Type*) [AddCommGroup A] [Fintype A]
    (T : ℕ) (hcard : Fintype.card A ≤ T) (a : A) :
    universalExponent T • a = 0 := by
  have hdiv : Fintype.card A ∣ universalExponent T :=
    dvd_universalExponent T (Fintype.card A) Fintype.card_pos hcard
  obtain ⟨k, hk⟩ := hdiv
  rw [hk, mul_smul]
  simp

/-- The varying-family obstruction with a universal LCM annihilator derived
only from a cardinality envelope. -/
theorem no_uniform_varying_height_lifts_lcm
    {r q : ℕ} [Fact q.Prime]
    (A : (Fin r → ZMod q) → Type*)
    [∀ t, AddCommGroup (A t)] [∀ t, Fintype (A t)]
    (d : (Fin r → ZMod q) → ℕ)
    (D T sNat : ℕ)
    (red : ∀ t, XedniFreeTorsionQuotient.LiftGroup (A := A t) (d t) →+ ZMod q)
    (lifts : ∀ t, Fin r → XedniFreeTorsionQuotient.LiftGroup (A := A t) (d t))
    (s : Fin r → ZMod q) (w : ZMod q)
    (freeNorm : ∀ t, (Fin (d t) → ℤ) → ℝ) (m U : ℝ)
    (hcompat : ∀ t i,
      red t (lifts t i) = XedniObstruction.xedniSampleMap s w t i)
    (hdegree : ∀ t, d t < r) (hrank : ∀ t, d t ≤ D)
    (htorsion : ∀ t, Fintype.card (A t) ≤ T)
    (hzero : ∀ t, freeNorm t 0 = 0)
    (hadd : ∀ t x y,
      freeNorm t (x + y) ≤ freeNorm t x + freeNorm t y)
    (hsmul : ∀ t (a : ℤ) x,
      freeNorm t (a • x) ≤ (a.natAbs : ℝ) * freeNorm t x)
    (hpoint : ∀ t i, freeNorm t (lifts t i).1 ≤ U)
    (hU : 0 ≤ U)
    (hscale : ∀ t (N : ℕ) u,
      freeNorm t (N • u) = (N : ℝ) * freeNorm t u)
    (hmin : ∀ t u, u ≠ 0 → m ≤ freeNorm t u)
    (hm : 0 < m)
    (hsratio : ((2 : ℝ) * r * U) / m < (sNat : ℝ))
    (hbudget :
      (2 * (2 * (T * (universalExponent T * (sNat + 1)) ^ D + 1)) + 1) ^ r - 1 < q) :
    False := by
  let c := universalExponent T * (sNat + 1)
  have hcpos : 0 < c := by
    dsimp [c]
    exact Nat.mul_pos (universalExponent_pos T) (Nat.succ_pos sNat)
  have hkillc : ∀ t (a : A t), c • a = 0 := by
    intro t a
    dsimp [c]
    rw [mul_smul]
    exact universalExponent_kills_bounded_group (A t) T (htorsion t) ((sNat + 1) • a)
  have hsratio' : (2 : ℝ) * r * U < (sNat : ℝ) * m :=
    (div_lt_iff₀ hm).mp hsratio
  have hsle : sNat ≤ c := by
    have hfactor : 1 ≤ universalExponent T := universalExponent_pos T
    have hsucc : sNat + 1 ≤ c := by
      calc
        sNat + 1 = 1 * (sNat + 1) := by simp
        _ ≤ universalExponent T * (sNat + 1) :=
          Nat.mul_le_mul_right _ hfactor
        _ = c := rfl
    omega
  have hratio : (2 : ℝ) * r * U < (c : ℝ) * m :=
    lt_of_lt_of_le hsratio'
      (mul_le_mul_of_nonneg_right (by exact_mod_cast hsle) (le_of_lt hm))
  exact XedniVaryingFamily.no_uniform_varying_height_lifts A d D T c
    red lifts s w freeNorm m U hcompat hdegree hrank htorsion hcpos
    hzero hadd hsmul hpoint hU hscale hkillc hmin hratio hbudget

#print axioms XedniLcmEnvelope.universalExponent_pos
#print axioms XedniLcmEnvelope.dvd_universalExponent
#print axioms XedniLcmEnvelope.universalExponent_dvd_of_all
#print axioms XedniLcmEnvelope.universalExponent_kills_bounded_group
#print axioms XedniLcmEnvelope.no_uniform_varying_height_lifts_lcm

end XedniLcmEnvelope
