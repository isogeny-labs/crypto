module

public import WalkUpperBound

@[expose] public section

namespace FiniteFourierUpperBoundAudit

open WalkCharacterAudit HildebrandCellAudit WeightedCycleCouplingAudit

theorem two_point_full_distance :
    l1Distance (PMF.pure (0 : ZMod 2)) (uniformLaw (ZMod 2))=1 := by
  have hu : (Finset.univ : Finset (ZMod 2))={0,1} := by decide +kernel
  unfold l1Distance
  rw [hu]
  norm_num [weight,PMF.pure_apply,uniform_real]

theorem two_point_half_distance :
    totalVariation (PMF.pure (0 : ZMod 2)) (uniformLaw (ZMod 2))=1/2 := by
  rw [totalVariation,two_point_full_distance]

theorem two_point_spectral_energy : nontrivialEnergy (PMF.pure (0 : ZMod 2))=1 := by
  classical
  norm_num [nontrivialEnergy,complex_expectation_pure,AddChar.card_eq,ZMod.card]

theorem mixed_normalization_bound_false :
    ¬ (l1Distance (PMF.pure (0 : ZMod 2)) (uniformLaw (ZMod 2)))^2 ≤
      (1/4 : ℝ)*nontrivialEnergy (PMF.pure (0 : ZMod 2)) := by
  rw [two_point_full_distance,two_point_spectral_energy]
  norm_num

theorem correct_normalization_equality :
    (totalVariation (PMF.pure (0 : ZMod 2)) (uniformLaw (ZMod 2)))^2 =
      (1/4 : ℝ)*nontrivialEnergy (PMF.pure (0 : ZMod 2)) := by
  rw [two_point_half_distance,two_point_spectral_energy]
  norm_num

theorem quarter_constant_necessary (c : ℝ)
    (h : ∀ q : PMF (ZMod 2), (totalVariation q (uniformLaw (ZMod 2)))^2 ≤ c*nontrivialEnergy q) :
    (1/4 : ℝ) ≤ c := by
  have hp := h (PMF.pure 0)
  rw [two_point_half_distance,two_point_spectral_energy] at hp
  norm_num at hp
  exact hp

end FiniteFourierUpperBoundAudit
