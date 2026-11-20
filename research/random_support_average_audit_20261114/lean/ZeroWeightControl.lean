module

public import AsymptoticMixing

@[expose] public section

namespace RandomSupportAverageAudit

open Real SupportGaussianAudit InjectiveBoxAudit HildebrandCellAudit
open WalkCharacterAudit WeightedCycleCouplingAudit CycleCouplingAudit FiniteFourierUpperBoundAudit

theorem pure_label_walk {I G : Type*} [Fintype I] [Fintype G] [AddCommGroup G]
    (i₀ : I) (a : I → G) (t : ℕ) (x : G) :
    kernelLaw (PMF.pure i₀) (fun i y => y+a i) t x=PMF.pure (x+t • a i₀) := by
  induction t generalizing x with
  | zero => simp [kernelLaw]
  | succ t ih =>
    rw [kernelLaw,PMF.pure_bind,ih]
    congr 1
    rw [succ_nsmul]
    abel

theorem pointmass_distance_lower (p : ℕ) [Fact p.Prime] (x : ZMod p) :
    (1/2 : ℝ) ≤ totalVariation (PMF.pure x) (uniformLaw (ZMod p)) := by
  classical
  have he := event_difference_le_total_variation (PMF.pure x) (uniformLaw (ZMod p)) ({x} : Finset (ZMod p))
  have hm : eventMass (PMF.pure x) ({x} : Finset (ZMod p))=1 := by simp [eventMass,weight,PMF.pure_apply]
  rw [hm,uniform_event_mass] at he
  simp only [Finset.card_singleton,Nat.cast_one,ZMod.card] at he
  have hp : (2 : ℝ) ≤ p := by exact_mod_cast (Fact.out : p.Prime).two_le
  have hh : 1/(p : ℝ) ≤ (1/2 : ℝ) := (div_le_div_iff₀ (by linarith : (0 : ℝ)<p) (by norm_num)).mpr (by linarith)
  have ha : (1/2 : ℝ) ≤ abs (1-1/(p : ℝ)) := (by linarith : (1/2 : ℝ) ≤ 1-1/(p : ℝ)).trans (le_abs_self _)
  exact ha.trans he

theorem zero_weight_average_lower {p d : ℕ} [Fact p.Prime] (h : d ≤ p-1)
    (t : ℕ) (x : ZMod p) :
    (1/2 : ℝ) ≤ expectedDistance p d h (PMF.pure (0 : Fin (d+1))) t x := by
  calc
    _ = expectation (fullSupportLaw p d h) (fun _ => (1/2 : ℝ)) := (expectation_constant _ _).symm
    _ ≤ _ := by
      apply expectation_mono
      intro a
      rw [pure_label_walk]
      exact pointmass_distance_lower p _

theorem strict_weight_deletion_false (d : ℕ) :
    ¬ ∃ γ : ℝ, 0<γ ∧ ∃ N : ℕ, d+1 ≤ N ∧
      ∀ (p : ℕ) [Fact p.Prime] (h : d ≤ p-1), N ≤ p → ∀ x : ZMod p,
      expectedDistance p d h (PMF.pure (0 : Fin (d+1))) (sourceTime d γ p) x<(1/4 : ℝ) := by
  rintro ⟨γ,_hγ,N,hN,hall⟩
  obtain ⟨p,hp,hprime⟩ := Nat.exists_infinite_primes (max N (d+1))
  let : Fact p.Prime := ⟨hprime⟩
  have hpN : N ≤ p := (le_max_left _ _).trans hp
  have hdp : d ≤ p-1 := by omega
  have hl := zero_weight_average_lower hdp (sourceTime d γ p) (0 : ZMod p)
  have hh := hall p hdp hpN 0
  linarith

end RandomSupportAverageAudit
