module

public import TVDistance

@[expose] public section

namespace FiniteFourierUpperBoundAudit

open WalkCharacterAudit HildebrandCellAudit WeightedCycleCouplingAudit

noncomputable def differenceEnergy {G : Type*} [Fintype G] [AddCommGroup G] (q r : PMF G) : ℝ :=
  ∑ ψ : AddChar G ℂ, Complex.normSq (complexExpectation q ψ-complexExpectation r ψ)

noncomputable def nontrivialEnergy {G : Type*} [Fintype G] [AddCommGroup G] (q : PMF G) : ℝ := by
  classical
  exact ∑ ψ ∈ (Finset.univ.erase (0 : AddChar G ℂ)), Complex.normSq (complexExpectation q ψ)

theorem difference_energy_parseval {G : Type*} [Fintype G] [AddCommGroup G] (q r : PMF G) :
    differenceEnergy q r=(Fintype.card G : ℝ)*(∑ x, (weight q x-weight r x)^2) := by
  have hp := finite_plancherel (fun x => (weight q x : ℂ)-(weight r x : ℂ))
  simpa [differenceEnergy,difference_fourier,Complex.normSq_apply,pow_two] using hp

theorem l1_fourier_bound {G : Type*} [Fintype G] [AddCommGroup G] (q r : PMF G) :
    (l1Distance q r)^2 ≤ differenceEnergy q r := by
  rw [difference_energy_parseval]
  exact l1_cauchy_bound q r

theorem total_variation_fourier_bound {G : Type*} [Fintype G] [AddCommGroup G] (q r : PMF G) :
    (totalVariation q r)^2 ≤ (1/4 : ℝ)*differenceEnergy q r := by
  have h := l1_fourier_bound q r
  unfold totalVariation
  nlinarith

theorem uniform_difference_energy {G : Type*} [Fintype G] [AddCommGroup G] (q : PMF G) :
    differenceEnergy q (uniformLaw G)=nontrivialEnergy q := by
  classical
  unfold differenceEnergy nontrivialEnergy
  calc
    _ = ∑ ψ ∈ (Finset.univ.erase (0 : AddChar G ℂ)),
        Complex.normSq (complexExpectation q ψ-complexExpectation (uniformLaw G) ψ) := by
      symm
      apply Finset.sum_subset (Finset.erase_subset _ _)
      intro ψ hψ hn
      have hz : ψ=0 := by
        by_contra hz
        exact hn (Finset.mem_erase.mpr ⟨hz,hψ⟩)
      subst ψ
      simp [complexExpectation,complex_weight_sum]
    _ = _ := by
      apply Finset.sum_congr rfl
      intro ψ hψ
      rw [uniform_character_coefficient,ite_eq_right (Finset.mem_erase.mp hψ).1,sub_zero]

theorem uniform_total_variation_bound {G : Type*} [Fintype G] [AddCommGroup G] (q : PMF G) :
    (totalVariation q (uniformLaw G))^2 ≤ (1/4 : ℝ)*nontrivialEnergy q := by
  have h := total_variation_fourier_bound q (uniformLaw G)
  rw [uniform_difference_energy] at h
  exact h

theorem uniform_event_mass {G : Type*} [Fintype G] [AddCommGroup G] (A : Finset G) :
    eventMass (uniformLaw G) A=(A.card : ℝ)/(Fintype.card G : ℝ) := by
  simp [eventMass,weight,uniform_real,div_eq_mul_inv]

noncomputable def walkSpectralEnergy {I G : Type*} [Fintype I] [Fintype G] [AddCommGroup G]
    (q : PMF I) (a : I → G) (t : ℕ) : ℝ := by
  classical
  exact ∑ ψ ∈ (Finset.univ.erase (0 : AddChar G ℂ)),
    (Complex.normSq (characterEigenvalue q a ψ))^t

theorem actual_walk_upper_bound {I G : Type*} [Fintype I] [Fintype G] [AddCommGroup G]
    (q : PMF I) (a : I → G) (t : ℕ) (x : G) :
    (totalVariation (kernelLaw q (fun i y => y+a i) t x) (uniformLaw G))^2 ≤
      (1/4 : ℝ)*walkSpectralEnergy q a t := by
  have h := uniform_total_variation_bound (kernelLaw q (fun i y => y+a i) t x)
  simpa only [nontrivialEnergy,walkSpectralEnergy,actual_character_squared_moment] using h

theorem actual_walk_event_upper_bound {I G : Type*} [Fintype I] [Fintype G] [AddCommGroup G]
    (q : PMF I) (a : I → G) (t : ℕ) (x : G) (A : Finset G) :
    |eventMass (kernelLaw q (fun i y => y+a i) t x) A-(A.card : ℝ)/(Fintype.card G : ℝ)|^2 ≤
      (1/4 : ℝ)*walkSpectralEnergy q a t := by
  rw [← uniform_event_mass A]
  have he := event_difference_le_total_variation (kernelLaw q (fun i y => y+a i) t x) (uniformLaw G) A
  calc
    _ ≤ (totalVariation (kernelLaw q (fun i y => y+a i) t x) (uniformLaw G))^2 :=
      pow_le_pow_left₀ (abs_nonneg _) he 2
    _ ≤ _ := actual_walk_upper_bound q a t x

end FiniteFourierUpperBoundAudit
