import CollisionProbability

namespace GeneralTwoStepCollisionAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit

theorem general_degree_two_collision_certificate :
    ∀ (G I : Type*) [AddCommGroup G] [Fintype G] [Fintype I]
      [DecidableEq G] [DecidableEq I] (p : I → ℝ)
      (hp0 : ∀ i, 0 ≤ p i) (hp : ∑ i, p i = 1) (g : I → G),
      TwoStepCollisionAudit.DegreeTwoGeneric g →
      (∀ v : G, expectation p (fun h : Hash G I => pairCount g h v) =
        1 - TwoStepCollisionAudit.secondMoment p - (TwoStepCollisionAudit.secondMoment p)^2 +
          TwoStepCollisionAudit.fourthMoment p) ∧
      (∀ z : JointSample G I, (jointLaw p hp0 hp z).toReal =
        (∏ x : G, p (z.1 x)) / (Fintype.card G : ℝ)^2) ∧
      (jointLaw p hp0 hp).toOuterMeasure (freshEvent g : Set (JointSample G I)) =
        ENNReal.ofReal ((1 - TwoStepCollisionAudit.secondMoment p -
          (TwoStepCollisionAudit.secondMoment p)^2 + TwoStepCollisionAudit.fourthMoment p) /
            Fintype.card G) := by
  intro G I _ _ _ _ _ p hp0 hp g hg
  refine ⟨?_, ?_, actual_fresh_collision_measure p hp0 hp g hg⟩
  · intro v
    rw [expected_fixed_terminal p hp g hg, TwoStepCollisionAudit.degree_two_label_coefficient p hp]
  · intro z
    exact jointLaw_real_mass p hp0 hp z

#print axioms general_degree_two_collision_certificate
end
end GeneralTwoStepCollisionAudit
