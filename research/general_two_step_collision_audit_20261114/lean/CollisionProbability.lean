import ExpectedCollision

namespace GeneralTwoStepCollisionAudit
noncomputable section
open scoped BigOperators ENNReal
open WalkWeightedGeneralAudit WalkBoundaryAudit
variable {G I : Type*} [AddCommGroup G] [Fintype G] [Fintype I]
  [DecidableEq G] [DecidableEq I]

abbrev JointSample (G I : Type*) := (G → I) × (G × G)

/-- The actual static product hash law and independent uniform ordered starts. -/
def jointLaw (p : I → ℝ) (hp0 : ∀ i, 0 ≤ p i) (hp : ∑ i, p i = 1) : PMF (JointSample G I) :=
  PMF.ofFintype (fun z => hashLaw p hp0 hp z.1 * uniformPMF (G × G) z.2) (by
    have hu : (∑ xy : G × G, uniformPMF (G × G) xy) = 1 := by
      simpa only [tsum_fintype] using (uniformPMF (G × G)).tsum_coe
    have hh : (∑ h : Hash G I, hashLaw p hp0 hp h) = 1 := by
      simpa only [tsum_fintype] using (hashLaw p hp0 hp (G := G)).tsum_coe
    rw [Fintype.sum_prod_type]
    simp only [← Finset.mul_sum, hu, mul_one, hh])

omit [DecidableEq I] in
@[simp] theorem jointLaw_mass (p : I → ℝ) (hp0 : ∀ i, 0 ≤ p i) (hp : ∑ i, p i = 1)
    (z : JointSample G I) :
    jointLaw p hp0 hp z = hashLaw p hp0 hp z.1 * uniformPMF (G × G) z.2 := rfl

omit [DecidableEq I] in
theorem jointLaw_real_mass (p : I → ℝ) (hp0 : ∀ i, 0 ≤ p i) (hp : ∑ i, p i = 1)
    (z : JointSample G I) :
    (jointLaw p hp0 hp z).toReal = weight p z.1 / (Fintype.card G : ℝ)^2 := by
  rw [jointLaw_mass, ENNReal.toReal_mul, hashLaw_real_mass]
  simp only [uniformPMF_apply, ENNReal.toReal_inv, ENNReal.toReal_mul, ENNReal.toReal_natCast,
    Fintype.card_prod, Nat.cast_mul, pow_two, div_eq_mul_inv]

def freshEvent (g : I → G) : Finset (JointSample G I) :=
  Finset.univ.filter (fun z => Fresh g z.1 z.2)

def freshProbability (p : I → ℝ) (hp0 : ∀ i, 0 ≤ p i) (hp : ∑ i, p i = 1)
    (g : I → G) : ℝ :=
  ∑ z : JointSample G I, (jointLaw p hp0 hp z).toReal *
    (if Fresh g z.1 z.2 then 1 else 0)

theorem fresh_probability_real (p : I → ℝ) (hp0 : ∀ i, 0 ≤ p i) (hp : ∑ i, p i = 1)
    (g : I → G) (hg : TwoStepCollisionAudit.DegreeTwoGeneric g) :
    freshProbability p hp0 hp g = TwoStepCollisionAudit.labelCoefficient p / Fintype.card G := by
  unfold freshProbability
  rw [Fintype.sum_prod_type]
  simp only [jointLaw_real_mass]
  have hf (h : Hash G I) :
      (∑ xy : G × G, (weight p h / (Fintype.card G : ℝ)^2) *
        (if Fresh g h xy then (1 : ℝ) else 0)) =
          (weight p h * globalCount g h) / (Fintype.card G : ℝ)^2 := by
    unfold globalCount
    rw [Finset.mul_sum, Finset.sum_div]
    apply Finset.sum_congr rfl
    intro xy _
    ring
  simp_rw [hf]
  rw [← Finset.sum_div]
  change expectation p (fun h : Hash G I => globalCount g h) / (Fintype.card G : ℝ)^2 = _
  rw [expected_global_count p hp g hg]
  have hN : (Fintype.card G : ℝ) ≠ 0 := by exact_mod_cast Fintype.card_ne_zero
  field_simp

omit [DecidableEq I] in
theorem fresh_probability_eq_measure (p : I → ℝ) (hp0 : ∀ i, 0 ≤ p i)
    (hp : ∑ i, p i = 1) (g : I → G) :
    freshProbability p hp0 hp g =
      ((jointLaw p hp0 hp).toOuterMeasure (freshEvent g : Set (JointSample G I))).toReal := by
  rw [PMF.toOuterMeasure_apply_finset,
    ENNReal.toReal_sum (fun z _ => (jointLaw p hp0 hp).apply_ne_top z)]
  unfold freshProbability freshEvent
  rw [Finset.sum_filter]
  apply Finset.sum_congr rfl
  intro z _
  by_cases hz : Fresh g z.1 z.2 <;> simp [hz]

theorem actual_fresh_collision_probability (p : I → ℝ) (hp0 : ∀ i, 0 ≤ p i)
    (hp : ∑ i, p i = 1) (g : I → G) (hg : TwoStepCollisionAudit.DegreeTwoGeneric g) :
    ((jointLaw p hp0 hp).toOuterMeasure (freshEvent g : Set (JointSample G I))).toReal =
      (1 - TwoStepCollisionAudit.secondMoment p - (TwoStepCollisionAudit.secondMoment p)^2 +
        TwoStepCollisionAudit.fourthMoment p) / Fintype.card G := by
  rw [← fresh_probability_eq_measure, fresh_probability_real p hp0 hp g hg,
    TwoStepCollisionAudit.degree_two_label_coefficient p hp]

theorem actual_fresh_collision_measure (p : I → ℝ) (hp0 : ∀ i, 0 ≤ p i)
    (hp : ∑ i, p i = 1) (g : I → G) (hg : TwoStepCollisionAudit.DegreeTwoGeneric g) :
    (jointLaw p hp0 hp).toOuterMeasure (freshEvent g : Set (JointSample G I)) =
      ENNReal.ofReal ((1 - TwoStepCollisionAudit.secondMoment p -
        (TwoStepCollisionAudit.secondMoment p)^2 + TwoStepCollisionAudit.fourthMoment p) /
          Fintype.card G) := by
  have hu : (jointLaw p hp0 hp).toOuterMeasure (Set.univ : Set (JointSample G I)) = 1 := by
    rw [PMF.toOuterMeasure_apply]
    simp only [Set.indicator_univ, PMF.tsum_coe]
  have hl : (jointLaw p hp0 hp).toOuterMeasure (freshEvent g : Set (JointSample G I)) ≤ 1 :=
    ((jointLaw p hp0 hp).toOuterMeasure.mono (Set.subset_univ _)).trans_eq hu
  have hn : (jointLaw p hp0 hp).toOuterMeasure (freshEvent g : Set (JointSample G I)) ≠ ∞ :=
    ne_top_of_le_ne_top ENNReal.one_ne_top hl
  rw [← ENNReal.ofReal_toReal hn, actual_fresh_collision_probability p hp0 hp g hg]

#print axioms jointLaw_real_mass
#print axioms actual_fresh_collision_probability
#print axioms actual_fresh_collision_measure
end
end GeneralTwoStepCollisionAudit
