import SupportCounting

namespace RhoTwoChoiceCollisionAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit StaticHashCollisionAudit RhoFirstCollisionAudit

theorem weighted_mono {A : Type*} [Fintype A] [DecidableEq A]
    {F H : (A → Fin 2) → ℝ} (h : ∀ w, F w ≤ H w) :
    expectation fair F ≤ expectation fair H :=
  Finset.sum_le_sum (fun w _ =>
    mul_le_mul_of_nonneg_left (h w) (weight_nonnegative fair fair_nonneg w))

theorem iid_survival_lower {p : ℕ} (T W : ℕ) (a δ : ZMod p) :
    goodMass W T - (if shortRelation T W a δ then (1 : ℝ) else 0) ≤
      iidSurvival fair (twoUpdate a δ) 0 T := by
  classical
  have hp (w : Fin T → Fin 2) :
      (if ¬Hit W 0 w then (1 : ℝ) else 0) -
          (if shortRelation T W a δ then 1 else 0) ≤
        (if SelfAvoiding (twoUpdate a δ) 0 w then 1 else 0) := by
    by_cases hs : shortRelation T W a δ
    · simp only [hs, ite_true]
      split_ifs <;> norm_num
    · by_cases hw : Hit W 0 w
      · simp only [hs, hw, not_true_eq_false, ite_false, sub_zero]
        split_ifs <;> norm_num
      · have hv := balanced_no_relation_survives a δ w hw hs
        simp [hs, hw, hv]
  have h := weighted_mono hp
  change (∑ w : Fin T → Fin 2, weight fair w *
    ((if ¬Hit W 0 w then (1 : ℝ) else 0) -
      (if shortRelation T W a δ then 1 else 0))) ≤ _ at h
  simp only [mul_sub, Finset.sum_sub_distrib, ← Finset.sum_mul,
    weight_sum fair fair_sum, one_mul] at h
  exact h

variable {p : ℕ} [Fact p.Prime]

theorem averaged_survival_lower (T W : ℕ) (hTp : T < p) :
    goodMass W T - (T : ℝ) * (2 * W + 1) / p ≤
      finiteMean (fun z : TwoSupport p => iidSurvival fair (twoUpdate z.1 z.2.val) 0 T) := by
  have h := finiteMean_mono (fun z : TwoSupport p => iid_survival_lower T W z.1 z.2.val)
  rw [finiteMean_sub, finiteMean_const] at h
  linarith [short_relation_mass_le (p := p) T W hTp]

theorem averaged_survival_half (m : ℕ) (hm : 0 < m)
    (hmp : 4 * m^2 * (4 * m + 1) ≤ p) :
    1 / 2 ≤ finiteMean (fun z : TwoSupport p =>
      iidSurvival fair (twoUpdate z.1 z.2.val) 0 (m^2)) := by
  have hTp : m^2 < p := by nlinarith [sq_pos_of_pos hm]
  have h := averaged_survival_lower (p := p) (m^2) (2 * m) hTp
  have hg := goodMass_three_quarters m hm
  have hp : (0 : ℝ) < p := Nat.cast_pos.mpr (Fact.out : p.Prime).pos
  have hrel : (m^2 : ℕ) * (2 * (2 * m) + 1 : ℝ) / p ≤ 1 / 4 := by
    apply (div_le_iff₀ hp).mpr
    have hcast : (4 : ℝ) * (m : ℝ)^2 * (4 * m + 1) ≤ p := by exact_mod_cast hmp
    push_cast
    nlinarith
  push_cast at h hrel
  linarith

/-- Actual least-repeat mean under uniform ordered distinct increments and
the actual independent uniform labels on vertices. -/
def ensembleMean (p : ℕ) [Fact p.Prime] : ℝ :=
  finiteMean (fun f : Fin 2 ↪ ZMod p => expectation fair
    (fun h : Hash (ZMod p) (Fin 2) =>
      (firstCollision p (fun x => x + f (h x)) 0 : ℝ)))

theorem ensembleMean_actual_law : ensembleMean p =
    ∑ f : Fin 2 ↪ ZMod p, (uniformLaw (Fin 2 ↪ ZMod p) f).toReal *
      ∑ h : Hash (ZMod p) (Fin 2), (hashLaw fair fair_nonneg fair_sum h).toReal *
        (firstCollision p (fun x => x + f (h x)) 0 : ℝ) := by
  unfold ensembleMean
  rw [finiteMean_actual_law]
  simp only [hashLaw_real_mass]
  rfl

theorem ensembleMean_coordinates : ensembleMean p =
    finiteMean (fun z : TwoSupport p => expectation fair
      (fun h : Hash (ZMod p) (Fin 2) =>
        (firstCollision p (staticStep (twoUpdate z.1 z.2.val) h) 0 : ℝ))) := by
  unfold ensembleMean
  rw [← finiteMean_equiv (supportEquiv p)]
  congr 1
  funext z
  congr 1
  funext h
  congr 2
  funext x
  change x + (z.1 + ((h x).val : ZMod p) * z.2.val) =
    x + z.1 + ((h x).val : ZMod p) * z.2.val
  ring

theorem collision_mean_tail_lower (T : ℕ) (a δ : ZMod p) :
    (T : ℝ) * iidSurvival fair (twoUpdate a δ) 0 T ≤
      expectation fair (fun h : Hash (ZMod p) (Fin 2) =>
        (firstCollision p (staticStep (twoUpdate a δ) h) 0 : ℝ)) := by
  classical
  rw [← first_collision_tail_mass p fair fair_sum]
  have hp (h : Hash (ZMod p) (Fin 2)) :
      (T : ℝ) * (if T < firstCollision p (staticStep (twoUpdate a δ) h) 0 then 1 else 0) ≤
        (firstCollision p (staticStep (twoUpdate a δ) h) 0 : ℝ) := by
    split_ifs with hT
    · simpa only [mul_one] using (by exact_mod_cast hT.le :
        (T : ℝ) ≤ (firstCollision p (staticStep (twoUpdate a δ) h) 0 : ℝ))
    · simp only [mul_zero]
      positivity
  have h := weighted_mono hp
  unfold expectation at h ⊢
  rw [Finset.mul_sum]
  convert h using 1
  apply Finset.sum_congr rfl
  intro h _
  ring

theorem finiteMean_mul {A : Type*} [Fintype A] (c : ℝ) (F : A → ℝ) :
    finiteMean (fun a => c * F a) = c * finiteMean F := by
  simp only [finiteMean, ← Finset.mul_sum]
  ring

theorem ensemble_mean_lower (m : ℕ) (hm : 0 < m)
    (hmp : 4 * m^2 * (4 * m + 1) ≤ p) : (m : ℝ)^2 / 2 ≤ ensembleMean p := by
  rw [ensembleMean_coordinates]
  have h := finiteMean_mono (fun z : TwoSupport p =>
    collision_mean_tail_lower (p := p) (m^2) z.1 z.2.val)
  rw [finiteMean_mul] at h
  have hs := averaged_survival_half (p := p) m hm hmp
  have hp : 0 ≤ (m : ℝ)^2 := sq_nonneg _
  push_cast at h
  nlinarith

#print axioms ensembleMean_actual_law
#print axioms ensemble_mean_lower
end
end RhoTwoChoiceCollisionAudit
