module

public import SupportAverages

@[expose] public section

namespace RandomSupportAverageAudit

open Real SupportGaussianAudit InjectiveBoxAudit HildebrandCellAudit
open WalkCharacterAudit WeightedCycleCouplingAudit CycleCouplingAudit FiniteFourierUpperBoundAudit

variable {p d : ℕ} [Fact p.Prime]

theorem frequency_index_injective : Function.Injective (frequencyCharacter p) := by
  intro j k h
  have he := congrArg (fun ψ : AddChar (ZMod p) ℂ => ψ 1) h
  simp only [frequency_character_apply,one_mul] at he
  exact ZMod.injective_stdAddChar he

theorem frequency_index_bijective : Function.Bijective (frequencyCharacter p) := by
  apply (Fintype.bijective_iff_injective_and_card _).mpr
  exact ⟨frequency_index_injective,AddChar.card_eq.symm⟩

theorem frequency_index_zero : frequencyCharacter p 0=0 := by
  ext x
  simp [frequency_character_apply]

theorem expectation_mul_constant {X : Type*} [Fintype X] (q : PMF X) (c : ℝ) (F : X → ℝ) :
    expectation q (fun x => c*F x)=c*expectation q F := by
  unfold expectation
  rw [Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro x _
  ring

theorem expectation_sum {X Y : Type*} [Fintype X] (q : PMF X)
    (S : Finset Y) (F : Y → X → ℝ) :
    expectation q (fun x => ∑ y ∈ S, F y x)=∑ y ∈ S, expectation q (F y) := by
  unfold expectation
  simp_rw [Finset.mul_sum]
  rw [Finset.sum_comm]

theorem expectation_square {X : Type*} [Fintype X] (q : PMF X) (F : X → ℝ) :
    (expectation q F)^2 ≤ expectation q (fun x => (F x)^2) := by
  have h := Finset.sum_sq_le_sum_mul_sum_of_sq_le_mul (Finset.univ : Finset X)
    (r := fun x => weight q x*F x) (f := weight q) (g := fun x => weight q x*(F x)^2)
    (fun x _ => weight_nonneg q x) (fun x _ => mul_nonneg (weight_nonneg q x) (sq_nonneg _))
    (fun x _ => le_of_eq (by ring))
  rw [weight_sum,one_mul] at h
  exact h

noncomputable def expectedDistance (p d : ℕ) [Fact p.Prime] (h : d ≤ p-1)
    (q : PMF (Fin (d+1))) (t : ℕ) (x : ZMod p) : ℝ :=
  expectation (fullSupportLaw p d h) (fun a =>
    totalVariation (kernelLaw q (fun i y => y+a i) t x) (uniformLaw (ZMod p)))

noncomputable def expectedSquaredDistance (p d : ℕ) [Fact p.Prime] (h : d ≤ p-1)
    (q : PMF (Fin (d+1))) (t : ℕ) (x : ZMod p) : ℝ :=
  expectation (fullSupportLaw p d h) (fun a =>
    (totalVariation (kernelLaw q (fun i y => y+a i) t x) (uniformLaw (ZMod p)))^2)

theorem expected_distance_nonneg (h : d ≤ p-1) (q : PMF (Fin (d+1)))
    (t : ℕ) (x : ZMod p) : 0 ≤ expectedDistance p d h q t x :=
  Finset.sum_nonneg (fun a _ => mul_nonneg ENNReal.toReal_nonneg
    (total_variation_nonneg (kernelLaw q (fun i y => y+a i) t x) (uniformLaw (ZMod p))))

theorem expected_distance_square_bound (h : d ≤ p-1) (q : PMF (Fin (d+1)))
    (t : ℕ) (x : ZMod p) :
    (expectedDistance p d h q t x)^2 ≤ expectedSquaredDistance p d h q t x :=
  expectation_square _ _

theorem finite_expected_squared_bound (h : d ≤ p-1) (q : PMF (Fin (d+1)))
    {ω : ℝ} (hω : 0<ω) (hw : ∀ i, ω ≤ weight q i)
    (t : ℕ) (ht : 0<t) (x : ZMod p) :
    expectedSquaredDistance p d h q t x ≤
      ((p-1 : ℕ) : ℝ)/4 *
        (sqrt (Real.pi/((8*ω^2*(t : ℝ))/(p : ℝ)^2)))^d/((p-1).descFactorial d : ℝ) := by
  classical
  let B : ℝ := (sqrt (Real.pi/((8*ω^2*(t : ℝ))/(p : ℝ)^2)))^d/((p-1).descFactorial d : ℝ)
  have hm (ψ : AddChar (ZMod p) ℂ) (hψ : ψ ≠ 0) :
      expectation (fullSupportLaw p d h) (fun a =>
        Complex.normSq (complexExpectation (kernelLaw q (fun i y => y+a i) t x) ψ)) ≤ B := by
    obtain ⟨j,hj⟩ := (frequency_index_bijective (p := p)).2 ψ
    have hj0 : j ≠ 0 := by
      intro hz
      apply hψ
      rw [← hj,hz,frequency_index_zero]
    rw [← hj]
    exact full_moment_average h q hω hw j hj0 t ht x
  have he : expectation (fullSupportLaw p d h) (fun a =>
      nontrivialEnergy (kernelLaw q (fun i y => y+a i) t x)) ≤ ((p-1 : ℕ) : ℝ)*B := by
    unfold nontrivialEnergy
    rw [expectation_sum]
    calc
      _ ≤ ∑ _ψ ∈ (Finset.univ.erase (0 : AddChar (ZMod p) ℂ)), B :=
        Finset.sum_le_sum (fun ψ hψ => hm ψ (Finset.mem_erase.mp hψ).1)
      _ = _ := by simp [AddChar.card_eq,ZMod.card]
  unfold expectedSquaredDistance
  calc
    _ ≤ expectation (fullSupportLaw p d h) (fun a =>
        (1/4 : ℝ)*nontrivialEnergy (kernelLaw q (fun i y => y+a i) t x)) :=
      expectation_mono _ (fun a => uniform_total_variation_bound _)
    _ = (1/4 : ℝ)*expectation (fullSupportLaw p d h) (fun a =>
        nontrivialEnergy (kernelLaw q (fun i y => y+a i) t x)) := expectation_mul_constant _ _ _
    _ ≤ (1/4 : ℝ)*(((p-1 : ℕ) : ℝ)*B) := mul_le_mul_of_nonneg_left he (by norm_num)
    _ = _ := by unfold B; ring

theorem finite_expected_distance_bound (h : d ≤ p-1) (q : PMF (Fin (d+1)))
    {ω : ℝ} (hω : 0<ω) (hw : ∀ i, ω ≤ weight q i)
    (t : ℕ) (ht : 0<t) (x : ZMod p) :
    (expectedDistance p d h q t x)^2 ≤
      ((p-1 : ℕ) : ℝ)/4 *
        (sqrt (Real.pi/((8*ω^2*(t : ℝ))/(p : ℝ)^2)))^d/((p-1).descFactorial d : ℝ) :=
  (expected_distance_square_bound h q t x).trans (finite_expected_squared_bound h q hω hw t ht x)

end RandomSupportAverageAudit
