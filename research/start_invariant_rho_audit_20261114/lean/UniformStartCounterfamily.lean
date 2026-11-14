import HashTranslation

namespace StartInvariantRhoAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit RhoFirstCollisionAudit RhoUniformCollisionAudit
variable (p : ℕ) [NeZero p]

/-- Exactly the earlier joint static-hash and independent uniform ordered-start
PMF. The second starting point remains present in that probability space. -/
def jointMean {I : Type*} [Fintype I] [DecidableEq I] (w : I → ℝ)
    (hw0 : ∀ i, 0 ≤ w i) (hw : ∑ i, w i = 1) (g : I → ZMod p) : ℝ :=
  ∑ z : GeneralTwoStepCollisionAudit.JointSample (ZMod p) I,
    (GeneralTwoStepCollisionAudit.jointLaw w hw0 hw z).toReal *
      (firstCollision p (step g z.1) z.2.1 : ℝ)

theorem joint_mean_eq {I : Type*} [Fintype I] [DecidableEq I] (w : I → ℝ)
    (hw0 : ∀ i, 0 ≤ w i) (hw : ∑ i, w i = 1) (g : I → ZMod p) :
    jointMean p w hw0 hw g =
      expectation w (fun h : Hash (ZMod p) I => (firstCollision p (step g h) 0 : ℝ)) := by
  unfold jointMean
  rw [Fintype.sum_prod_type]
  simp only [GeneralTwoStepCollisionAudit.jointLaw_real_mass, ZMod.card]
  rw [Finset.sum_comm]
  have hs (xy : ZMod p × ZMod p) :
      (∑ h : Hash (ZMod p) I, weight w h / (p : ℝ)^2 *
        (firstCollision p (step g h) xy.1 : ℝ)) =
      expectation w (fun h : Hash (ZMod p) I => (firstCollision p (step g h) 0 : ℝ)) /
        (p : ℝ)^2 := by
    calc
      _ = expectation w
          (fun h : Hash (ZMod p) I => (firstCollision p (step g h) xy.1 : ℝ)) /
            (p : ℝ)^2 := by
        unfold expectation
        rw [Finset.sum_div]
        apply Finset.sum_congr rfl
        intro h _
        ring
      _ = _ := by rw [expected_firstCollision_start p g w xy.1]
  simp_rw [hs]
  simp only [Finset.sum_const, Finset.card_univ, Fintype.card_prod, ZMod.card,
    nsmul_eq_mul, Nat.cast_mul]
  have hp0 : (p : ℝ) ≠ 0 := by exact_mod_cast NeZero.ne p
  field_simp

theorem sidon_all_start_bounds (h : Hash (ZMod p) (Fin 20)) (a : ZMod p) :
    p ≤ 951 * firstCollision p (SidonRhoCounterfamilyAudit.transition p h) a ∧
      firstCollision p (SidonRhoCounterfamilyAudit.transition p h) a ≤ p := by
  change p ≤ 951 * firstCollision p (step (SidonRhoCounterfamilyAudit.increments p) h) a ∧
    firstCollision p (step (SidonRhoCounterfamilyAudit.increments p) h) a ≤ p
  rw [firstCollision_from_zero]
  exact SidonRhoCounterfamilyAudit.firstCollision_bounds p (shift a h)

def uniformMean : ℝ := jointMean p (uniformWeights 20) (weights_nonnegative 20)
  (weights_sum 20 (Nat.succ_pos 19)) (SidonRhoCounterfamilyAudit.increments p)

theorem uniformMean_eq : uniformMean p = SidonRhoCounterfamilyAudit.meanCollision p :=
  joint_mean_eq p (uniformWeights 20) (weights_nonnegative 20)
    (weights_sum 20 (Nat.succ_pos 19)) (SidonRhoCounterfamilyAudit.increments p)

theorem mean_under_same_local_law : uniformMean p =
    ∑ z : GeneralTwoStepCollisionAudit.JointSample (ZMod p) (Fin 20),
      (SidonRhoCounterfamilyAudit.localLaw p z).toReal *
        (firstCollision p (SidonRhoCounterfamilyAudit.transition p z.1) z.2.1 : ℝ) := rfl

theorem uniform_bounds : (p : ℝ) ≤ 951 * uniformMean p ∧ uniformMean p ≤ p := by
  rw [uniformMean_eq]
  exact ⟨SidonRhoCounterfamilyAudit.mean_lower p, SidonRhoCounterfamilyAudit.mean_upper p⟩

theorem uniform_start_unbounded (C : ℕ) :
    ∃ (q : ℕ) (hq : Nat.Prime q), 1902 < q ∧
      (C : ℝ) * q < (@uniformMean q ⟨hq.ne_zero⟩)^2 := by
  obtain ⟨q, hq, hqB, hc⟩ := SidonRhoCounterfamilyAudit.arbitrarily_large_collision_ratio C
  let : NeZero q := ⟨hq.ne_zero⟩
  exact ⟨q, hq, hqB, by simpa only [uniformMean_eq] using hc⟩

theorem uniform_start_bound_false :
    ¬∃ C : ℝ, ∀ (q : ℕ) (hq : Nat.Prime q), 1902 < q →
      TwoStepCollisionAudit.DegreeTwoGeneric (SidonRhoCounterfamilyAudit.increments q) →
        (@uniformMean q ⟨hq.ne_zero⟩)^2 ≤ C * q := by
  rintro ⟨C, hC⟩
  apply SidonRhoCounterfamilyAudit.universal_degree_two_collision_bound_false
  refine ⟨C, ?_⟩
  intro q hq hqB hg
  let : NeZero q := ⟨hq.ne_zero⟩
  simpa only [uniformMean_eq] using hC q hq hqB hg

theorem concrete_uniform_ratio : (4 : ℝ) * 10000019 < (uniformMean 10000019)^2 := by
  rw [uniformMean_eq]
  exact SidonRhoCounterfamilyAudit.concrete_collision_ratio

theorem start_invariant_rho_certificate :
    (∀ (q : ℕ) [NeZero q] (I : Type*) [Fintype I] [DecidableEq I] (w : I → ℝ)
      (hw0 : ∀ i, 0 ≤ w i) (hw : ∑ i, w i = 1) (g : I → ZMod q),
      jointMean q w hw0 hw g =
        expectation w (fun h : Hash (ZMod q) I => (firstCollision q (step g h) 0 : ℝ)) ∧
      (∀ (a : ZMod q) (φ : ℕ → ℝ),
        expectation w (fun h : Hash (ZMod q) I => φ (firstCollision q (step g h) a)) =
          expectation w (fun h : Hash (ZMod q) I => φ (firstCollision q (step g h) 0)))) ∧
    (∀ (q : ℕ) [NeZero q], 1902 < q →
      TwoStepCollisionAudit.DegreeTwoGeneric (SidonRhoCounterfamilyAudit.increments q) ∧
      (∀ (h : Hash (ZMod q) (Fin 20)) (a : ZMod q),
        q ≤ 951 * firstCollision q (SidonRhoCounterfamilyAudit.transition q h) a ∧
          firstCollision q (SidonRhoCounterfamilyAudit.transition q h) a ≤ q) ∧
      uniformMean q =
        ∑ z : GeneralTwoStepCollisionAudit.JointSample (ZMod q) (Fin 20),
          (SidonRhoCounterfamilyAudit.localLaw q z).toReal *
            (firstCollision q (SidonRhoCounterfamilyAudit.transition q z.1) z.2.1 : ℝ) ∧
      uniformMean q = SidonRhoCounterfamilyAudit.meanCollision q ∧
      (q : ℝ) ≤ 951 * uniformMean q ∧ uniformMean q ≤ q ∧
      (SidonRhoCounterfamilyAudit.localLaw q).toOuterMeasure
        (GeneralTwoStepCollisionAudit.freshEvent (SidonRhoCounterfamilyAudit.increments q) :
          Set (GeneralTwoStepCollisionAudit.JointSample (ZMod q) (Fin 20))) =
        ENNReal.ofReal ((7581 : ℝ) / (8000 * q))) ∧
    (∀ C : ℕ, ∃ (q : ℕ) (hq : Nat.Prime q), 1902 < q ∧
      (C : ℝ) * q < (@uniformMean q ⟨hq.ne_zero⟩)^2) ∧
    (¬∃ C : ℝ, ∀ (q : ℕ) (hq : Nat.Prime q), 1902 < q →
      TwoStepCollisionAudit.DegreeTwoGeneric (SidonRhoCounterfamilyAudit.increments q) →
        (@uniformMean q ⟨hq.ne_zero⟩)^2 ≤ C * q) ∧
    Nat.Prime 10000019 ∧ (4 : ℝ) * 10000019 < (uniformMean 10000019)^2 := by
  refine ⟨?_, ?_, uniform_start_unbounded, uniform_start_bound_false,
    SidonRhoCounterfamilyAudit.concrete_prime, concrete_uniform_ratio⟩
  · intro q _ I _ _ w hw0 hw g
    exact ⟨joint_mean_eq q w hw0 hw g, firstCollision_statistic_start q g w⟩
  · intro q _ hq
    exact ⟨SidonRhoCounterfamilyAudit.generic q hq, sidon_all_start_bounds q,
      mean_under_same_local_law q, uniformMean_eq q, (uniform_bounds q).1,
      (uniform_bounds q).2, SidonRhoCounterfamilyAudit.actual_local_probability q hq⟩

#print axioms joint_mean_eq
#print axioms sidon_all_start_bounds
#print axioms uniform_start_bound_false
#print axioms start_invariant_rho_certificate
end
end StartInvariantRhoAudit
