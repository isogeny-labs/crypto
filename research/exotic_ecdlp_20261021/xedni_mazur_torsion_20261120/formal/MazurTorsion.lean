import LcmEnvelope

/-! The arithmetic consequences of the rational elliptic-curve torsion
classification, stated with the classification isomorphism as an input. -/

namespace XedniMazurTorsion

/-- The fifteen group shapes in Mazur's rational torsion classification. -/
def MazurShape (A : Type*) [AddCommGroup A] : Prop :=
  (∃ n : ℕ, ((1 ≤ n ∧ n ≤ 10) ∨ n = 12) ∧ Nonempty (A ≃+ ZMod n)) ∨
  (∃ v : ℕ, 1 ≤ v ∧ v ≤ 4 ∧ Nonempty (A ≃+ (ZMod 2 × ZMod (2 * v))))

theorem zmod_killed_of_dvd (n c : ℕ) (h : n ∣ c) (x : ZMod n) :
    c • x = 0 := by
  obtain ⟨k, hk⟩ := h
  rw [hk, mul_smul]
  simp [nsmul_eq_mul]

/-- Every group on Mazur's list is annihilated by 2520, the LCM of its
possible element orders. -/
theorem mazur_shape_killed (A : Type*) [AddCommGroup A]
    (hshape : MazurShape A) (a : A) :
    2520 • a = 0 := by
  rcases hshape with ⟨n, hn, ⟨e⟩⟩ | ⟨v, hvpos, hvle, ⟨e⟩⟩
  · have hdiv : n ∣ 2520 := by
      rcases hn with h | h
      · obtain ⟨hpos, hle⟩ := h
        interval_cases n <;> norm_num at *
      · subst n
        norm_num
    apply e.injective
    rw [map_zero, map_nsmul]
    exact zmod_killed_of_dvd n 2520 hdiv (e a)
  · have hdiv : 2 * v ∣ 2520 := by
      interval_cases v <;> norm_num at *
    apply e.injective
    rw [map_zero, map_nsmul]
    apply Prod.ext
    · simpa using zmod_killed_of_dvd 2 2520 (by norm_num) (e a).1
    · simpa using zmod_killed_of_dvd (2 * v) 2520 hdiv (e a).2

/-- The same classification gives the uniform cardinality cap 16. -/
theorem mazur_shape_card_le (A : Type*) [AddCommGroup A] [Fintype A]
    (hshape : MazurShape A) : Fintype.card A ≤ 16 := by
  rcases hshape with ⟨n, hn, ⟨e⟩⟩ | ⟨v, hvpos, hvle, ⟨e⟩⟩
  · have hnpos : 0 < n := by rcases hn with ⟨h, _⟩ | h <;> omega
    have : NeZero n := ⟨by omega⟩
    have hcard : Fintype.card A = n := by
      simpa using Fintype.card_congr e.toEquiv
    rw [hcard]
    rcases hn with ⟨_, hle⟩ | h <;> omega
  · have : NeZero (2 * v) := ⟨by omega⟩
    have hcard : Fintype.card A = 2 * (2 * v) := by
      simpa using Fintype.card_congr e.toEquiv
    rw [hcard]
    omega

/-- Apply the varying-family height obstruction using Mazur's exact
rational torsion shapes instead of a cardinality-only annihilator. -/
theorem no_uniform_varying_height_lifts_mazur
    {r q : ℕ} [Fact q.Prime]
    (A : (Fin r → ZMod q) → Type*)
    [∀ t, AddCommGroup (A t)] [∀ t, Fintype (A t)]
    (d : (Fin r → ZMod q) → ℕ)
    (D sNat : ℕ)
    (red : ∀ t, XedniFreeTorsionQuotient.LiftGroup (A := A t) (d t) →+ ZMod q)
    (lifts : ∀ t, Fin r → XedniFreeTorsionQuotient.LiftGroup (A := A t) (d t))
    (s : Fin r → ZMod q) (w : ZMod q)
    (freeNorm : ∀ t, (Fin (d t) → ℤ) → ℝ) (m U : ℝ)
    (hshape : ∀ t, MazurShape (A t))
    (hcompat : ∀ t i,
      red t (lifts t i) = XedniObstruction.xedniSampleMap s w t i)
    (hdegree : ∀ t, d t < r) (hrank : ∀ t, d t ≤ D)
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
      (2 * (2 * (16 * (2520 * (sNat + 1)) ^ D + 1)) + 1) ^ r - 1 < q) :
    False := by
  let c := 2520 * (sNat + 1)
  have hcpos : 0 < c := by dsimp [c]; positivity
  have htorsion : ∀ t, Fintype.card (A t) ≤ 16 :=
    fun t => mazur_shape_card_le (A t) (hshape t)
  have hkillc : ∀ t (a : A t), c • a = 0 := by
    intro t a
    dsimp [c]
    rw [mul_smul]
    exact mazur_shape_killed (A t) (hshape t) ((sNat + 1) • a)
  have hsratio' : (2 : ℝ) * r * U < (sNat : ℝ) * m :=
    (div_lt_iff₀ hm).mp hsratio
  have hsle : sNat ≤ c := by omega
  have hratio : (2 : ℝ) * r * U < (c : ℝ) * m :=
    lt_of_lt_of_le hsratio'
      (mul_le_mul_of_nonneg_right (by exact_mod_cast hsle) (le_of_lt hm))
  exact XedniVaryingFamily.no_uniform_varying_height_lifts A d D 16 c
    red lifts s w freeNorm m U hcompat hdegree hrank htorsion hcpos
    hzero hadd hsmul hpoint hU hscale hkillc hmin hratio hbudget

#print axioms XedniMazurTorsion.zmod_killed_of_dvd
#print axioms XedniMazurTorsion.mazur_shape_killed
#print axioms XedniMazurTorsion.mazur_shape_card_le
#print axioms XedniMazurTorsion.no_uniform_varying_height_lifts_mazur

end XedniMazurTorsion
