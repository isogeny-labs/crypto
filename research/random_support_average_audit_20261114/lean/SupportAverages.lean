module

public import FieldGaussian

@[expose] public section

namespace RandomSupportAverageAudit

open Real SupportGaussianAudit InjectiveBoxAudit HildebrandCellAudit
open WalkCharacterAudit WeightedCycleCouplingAudit CycleCouplingAudit

variable {p d : ℕ} [Fact p.Prime]

def allSupportEquiv : (ZMod p × Sample (Choice p) d) ≃ Sample (ZMod p) (d+1) where
  toFun z := ⟨supportsFromSample z.1 z.2,supports_from_sample_injective z.1 z.2⟩
  invFun f := (f 0,sampleFromSupports ⟨fun i => f i,rfl,f.injective⟩)
  left_inv z := by
    apply Prod.ext
    · rfl
    · exact (sourceEquiv z.1).left_inv z.2
  right_inv f := by
    apply Function.Embedding.ext
    intro i
    cases i using Fin.cases with
    | zero => rfl
    | succ i =>
      change f 0+(f i.succ-f 0)=f i.succ
      ring

noncomputable def fullSupportLaw (p d : ℕ) [Fact p.Prime] (h : d ≤ p-1) :
    PMF (Sample (ZMod p) (d+1)) :=
  sampleLaw (ZMod p) (d+1) (by
    rw [ZMod.card]
    have hp := (Fact.out : p.Prime).pos
    omega)

theorem uniform_product_expectation {X Y : Type*} [Fintype X] [Nonempty X]
    [Fintype Y] [Nonempty Y] (F : X × Y → ℝ) :
    expectation (uniformLaw (X × Y)) F =
      expectation (uniformLaw X) (fun x => expectation (uniformLaw Y) (fun y => F (x,y))) := by
  unfold expectation
  simp_rw [uniform_real]
  rw [Fintype.card_prod,Nat.cast_mul,Fintype.sum_prod_type]
  apply Finset.sum_congr rfl
  intro x _
  rw [Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro y _
  ring

theorem full_support_expectation (h : d ≤ p-1) (F : Sample (ZMod p) (d+1) → ℝ) :
    expectation (fullSupportLaw p d h) F =
      expectation (uniformLaw (ZMod p)) (fun a₀ =>
        expectation (sampleLaw (Choice p) d (by rw [choice_card]; exact h))
          (fun e => F (allSupportEquiv (a₀,e)))) := by
  let : Nonempty (Sample (Choice p) d) := sample_nonempty (by rw [choice_card]; exact h)
  let : Nonempty (Sample (ZMod p) (d+1)) :=
    ⟨allSupportEquiv (0,Classical.choice (inferInstance : Nonempty (Sample (Choice p) d)))⟩
  have he := uniform_map_equiv (allSupportEquiv (p := p) (d := d))
  have hx := expectation_map (uniformLaw (ZMod p × Sample (Choice p) d)) allSupportEquiv F
  rw [he,uniform_product_expectation] at hx
  exact hx

theorem expectation_mono {X : Type*} [Fintype X] (q : PMF X) {F H : X → ℝ}
    (h : ∀ x, F x ≤ H x) : expectation q F ≤ expectation q H :=
  Finset.sum_le_sum (fun x _ => mul_le_mul_of_nonneg_left (h x) (ENNReal.toReal_nonneg))

theorem expectation_constant {X : Type*} [Fintype X] (q : PMF X) (c : ℝ) :
    expectation q (fun _ => c)=c := by
  unfold expectation
  rw [← Finset.sum_mul]
  change (∑ x,weight q x)*c=c
  rw [weight_sum,one_mul]

theorem full_gaussian_average (h : d ≤ p-1) (j : ZMod p) (hj : j ≠ 0)
    {c : ℝ} (hc : 0<c) :
    expectation (fullSupportLaw p d h) (fun a => exp (-c*centeredRadius (fun i => a i) 0 j)) ≤
      (sqrt (Real.pi/(c/(p : ℝ)^2)))^d/((p-1).descFactorial d : ℝ) := by
  rw [full_support_expectation]
  calc
    _ ≤ expectation (uniformLaw (ZMod p)) (fun _ =>
        (sqrt (Real.pi/(c/(p : ℝ)^2)))^d/((p-1).descFactorial d : ℝ)) := by
      apply expectation_mono
      intro a₀
      exact relative_gaussian_average h a₀ j hj hc
    _ = _ := expectation_constant _ _

theorem full_moment_average (h : d ≤ p-1) (q : PMF (Fin (d+1)))
    {ω : ℝ} (hω : 0<ω) (hw : ∀ i, ω ≤ weight q i)
    (j : ZMod p) (hj : j ≠ 0) (t : ℕ) (ht : 0<t) (x : ZMod p) :
    expectation (fullSupportLaw p d h) (fun a =>
      Complex.normSq (complexExpectation (kernelLaw q (fun i y => y+a i) t x) (frequencyCharacter p j))) ≤
      (sqrt (Real.pi/((8*ω^2*(t : ℝ))/(p : ℝ)^2)))^d/((p-1).descFactorial d : ℝ) := by
  calc
    _ ≤ expectation (fullSupportLaw p d h) (fun a =>
        exp (-(8*ω^2*(t : ℝ))*centeredRadius (fun i => a i) 0 j)) := by
      apply expectation_mono
      intro a
      exact actual_centered_moment_bound q (fun i => a i) 0 j hω.le hw t x
    _ ≤ _ := full_gaussian_average h j hj (by positivity)

end RandomSupportAverageAudit
