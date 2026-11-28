import VaryingFamily

/-! A cardinality cap gives one annihilator for every selected torsion group. -/

namespace XedniTorsionEnvelope

/-- The factorial of a size cap annihilates any finite abelian group under
that cap, since its actual cardinality divides the factorial. -/
theorem factorial_kills_bounded_group
    (A : Type*) [AddCommGroup A] [Fintype A]
    (T : ℕ) (hcard : Fintype.card A ≤ T) (a : A) :
    T.factorial • a = 0 := by
  have hdiv : Fintype.card A ∣ T.factorial :=
    Nat.dvd_factorial Fintype.card_pos hcard
  obtain ⟨k, hk⟩ := hdiv
  rw [hk, mul_smul]
  simp

/-- A torsion-cardinality cap and a chosen integer above the norm ratio
construct a common annihilator for the varying-family height obstruction. -/
theorem no_uniform_varying_height_lifts_factorial
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
      (2 * (2 * (T * (T.factorial * (sNat + 1)) ^ D + 1)) + 1) ^ r - 1 < q) :
    False := by
  let c := T.factorial * (sNat + 1)
  have hcpos : 0 < c := by dsimp [c]; positivity
  have hkillc : ∀ t (a : A t), c • a = 0 := by
    intro t a
    dsimp [c]
    rw [mul_smul]
    exact factorial_kills_bounded_group (A t) T (htorsion t) ((sNat + 1) • a)
  have hsratio' : (2 : ℝ) * r * U < (sNat : ℝ) * m :=
    (div_lt_iff₀ hm).mp hsratio
  have hsle : sNat ≤ c := by
    have hfactorial : 1 ≤ T.factorial := Nat.factorial_pos T
    have hsucc : sNat + 1 ≤ c := by
      calc
        sNat + 1 = 1 * (sNat + 1) := by simp
        _ ≤ T.factorial * (sNat + 1) :=
          Nat.mul_le_mul_right _ hfactorial
        _ = c := rfl
    omega
  have hratio : (2 : ℝ) * r * U < (c : ℝ) * m :=
    lt_of_lt_of_le hsratio'
      (mul_le_mul_of_nonneg_right (by exact_mod_cast hsle) (le_of_lt hm))
  exact XedniVaryingFamily.no_uniform_varying_height_lifts A d D T c
    red lifts s w freeNorm m U hcompat hdegree hrank htorsion hcpos
    hzero hadd hsmul hpoint hU hscale hkillc hmin hratio hbudget

#print axioms XedniTorsionEnvelope.factorial_kills_bounded_group
#print axioms XedniTorsionEnvelope.no_uniform_varying_height_lifts_factorial

end XedniTorsionEnvelope
