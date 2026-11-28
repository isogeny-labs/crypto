import TwoChoiceCertificate

namespace RhoNonzeroSupportAudit
noncomputable section
open scoped BigOperators
open RhoTwoChoiceCollisionAudit WalkWeightedGeneralAudit StaticHashCollisionAudit RhoFirstCollisionAudit

theorem masked_sum {A : Type*} [Fintype A] (P : A → Prop) [DecidablePred P] (F : A → ℝ) :
    (∑ a, if P a then F a else 0) = ∑ a : {a // P a}, F a.val := by
  classical
  rw [← Finset.sum_filter]
  exact Finset.sum_subtype _ (by simp) F

/-- Removing the zero contribution and renormalizing cannot decrease a
nonnegative statistic's masked mean. Empty conditioning populations stay zero. -/
theorem masked_mean_le_conditional {A : Type*} [Fintype A]
    (P : A → Prop) [DecidablePred P] (F : A → ℝ) (hF : ∀ a, 0 ≤ F a) :
    finiteMean (fun a => if P a then F a else 0) ≤
      finiteMean (fun a : {a // P a} => F a.val) := by
  classical
  unfold finiteMean
  rw [masked_sum]
  rcases isEmpty_or_nonempty {a // P a} with h | h
  · let : IsEmpty {a // P a} := h
    simp
  · let : Nonempty {a // P a} := h
    apply div_le_div_of_nonneg_left
    · exact Finset.sum_nonneg (fun a _ => hF a.val)
    · exact Nat.cast_pos.mpr Fintype.card_pos
    · exact_mod_cast Fintype.card_subtype_le P

/-- Finite conditional-expectation bound, with discarded mass explicit. -/
theorem mean_le_conditioned_add_discard {A : Type*} [Fintype A]
    (P : A → Prop) [DecidablePred P] (F : A → ℝ) (B : ℝ)
    (hF : ∀ a, 0 ≤ F a) (hB : ∀ a, F a ≤ B) :
    finiteMean F ≤ finiteMean (fun a : {a // P a} => F a.val) +
      B * finiteMean (fun a => if ¬P a then (1 : ℝ) else 0) := by
  classical
  have hp (a : A) : F a ≤ (if P a then F a else 0) +
      B * (if ¬P a then (1 : ℝ) else 0) := by
    by_cases h : P a <;> simp [h, hB a]
  have h := finiteMean_mono hp
  rw [finiteMean_add, finiteMean_mul] at h
  linarith [masked_mean_le_conditional P F hF]

def Good {p : ℕ} (z : TwoSupport p) : Prop := z.1 ≠ 0 ∧ z.1 + z.2.val ≠ 0

instance goodDecidable {p : ℕ} (z : TwoSupport p) : Decidable (Good z) :=
  inferInstanceAs (Decidable (z.1 ≠ 0 ∧ z.1 + z.2.val ≠ 0))

abbrev NonzeroSupport (p : ℕ) := {z : TwoSupport p // Good z}

variable {p : ℕ} [Fact p.Prime]

theorem discarded_mass : finiteMean (fun z : TwoSupport p =>
    if ¬Good z then (1 : ℝ) else 0) = 2 / p := by
  classical
  have hi (z : TwoSupport p) : (if ¬Good z then (1 : ℝ) else 0) =
      (if z.1 = 0 then 1 else 0) + (if z.1 + z.2.val = 0 then 1 else 0) := by
    by_cases h0 : z.1 = 0
    · simp [Good, h0, z.2.property]
    · by_cases h1 : z.1 + z.2.val = 0 <;> simp [Good, h0, h1]
  simp_rw [hi]
  rw [finiteMean_add]
  have h0 := relation_mass (p := p) 1 (by omega) (Fact.out : p.Prime).one_lt (0 : ZMod p)
  have h1 := relation_mass (p := p) 1 (by omega) (Fact.out : p.Prime).one_lt (1 : ZMod p)
  simp only [Nat.cast_one, one_mul, zero_mul, add_zero] at h0 h1
  rw [h0, h1]
  ring

def tableMean (z : TwoSupport p) : ℝ := expectation fair
  (fun h : Hash (ZMod p) (Fin 2) =>
    (firstCollision p (staticStep (twoUpdate z.1 z.2.val) h) 0 : ℝ))

theorem tableMean_nonneg (z : TwoSupport p) : 0 ≤ tableMean z := by
  apply Finset.sum_nonneg
  intro h _
  exact mul_nonneg (weight_nonnegative fair fair_nonneg h) (Nat.cast_nonneg _)

theorem tableMean_le (z : TwoSupport p) : tableMean z ≤ p := by
  have hp (h : Hash (ZMod p) (Fin 2)) :
      (firstCollision p (staticStep (twoUpdate z.1 z.2.val) h) 0 : ℝ) ≤ p := by
    exact_mod_cast firstCollision_le p _ 0
  have h := weighted_mono hp
  change tableMean z ≤ ∑ h : Hash (ZMod p) (Fin 2), weight fair h * (p : ℝ) at h
  rw [← Finset.sum_mul, weight_sum fair fair_sum, one_mul] at h
  exact h

/-- The actual nonzero-support mean; its normalized PMF is only asserted when
the conditioning population is nonempty (prime p>2). -/
def nonzeroMean (p : ℕ) [Fact p.Prime] : ℝ :=
  finiteMean (fun z : NonzeroSupport p => tableMean z.val)

theorem nonzeroMean_nonneg : 0 ≤ nonzeroMean p :=
  finiteMean_nonneg (fun z => tableMean_nonneg z.val)

theorem all_mean_le_nonzero_add_two : ensembleMean p ≤ nonzeroMean p + 2 := by
  rw [ensembleMean_coordinates]
  change finiteMean tableMean ≤ nonzeroMean p + 2
  have h := mean_le_conditioned_add_discard (Good (p := p)) tableMean (p : ℝ)
    tableMean_nonneg tableMean_le
  rw [discarded_mass] at h
  have hp : (p : ℝ) ≠ 0 := Nat.cast_ne_zero.mpr (Fact.out : p.Prime).ne_zero
  have hs : (p : ℝ) * (2 / p) = 2 := by field_simp
  rw [hs] at h
  exact h

#print axioms discarded_mass
#print axioms all_mean_le_nonzero_add_two
end
end RhoNonzeroSupportAudit
