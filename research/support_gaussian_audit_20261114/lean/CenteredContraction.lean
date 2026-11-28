module

public import SpectralContraction
public import CenteredCell
public import Mathlib.Analysis.SpecialFunctions.Trigonometric.Bounds
public import Mathlib.Data.Finset.Max

@[expose] public section

namespace SupportGaussianAudit

open WalkCharacterAudit HildebrandCellAudit WeightedCycleCouplingAudit

theorem positive_weight_lower_bound {I : Type*} [Fintype I] [Nonempty I]
    (q : PMF I) (hq : ∀ i, 0<weight q i) : ∃ ω : ℝ, 0<ω ∧ ∀ i, ω ≤ weight q i := by
  classical
  let S : Finset ℝ := Finset.univ.image (weight q)
  have hs : S.Nonempty := by
    obtain ⟨i⟩ := (inferInstance : Nonempty I)
    exact ⟨weight q i,Finset.mem_image.mpr ⟨i,Finset.mem_univ i,rfl⟩⟩
  refine ⟨S.min' hs,?_,?_⟩
  · obtain ⟨i,_hi,he⟩ := Finset.mem_image.mp (S.min'_mem hs)
    rw [← he]
    exact hq i
  · intro i
    exact S.min'_le (weight q i) (Finset.mem_image.mpr ⟨i,Finset.mem_univ i,rfl⟩)

theorem centered_character_cosine {p : ℕ} [Fact p.Prime] (j x : ZMod p) :
    (frequencyCharacter p j x).re=Real.cos (2*Real.pi*(centered (x*j) : ℝ)/(p : ℝ)) := by
  rw [frequency_character_apply]
  have he : ZMod.stdAddChar (x*j)=Complex.exp
      (2*(Real.pi : ℂ)*Complex.I*(centered (x*j) : ℂ)/(p : ℂ)) := by
    simpa only [centered_cast] using ZMod.stdAddChar_coe (N := p) (centered (x*j))
  rw [he]
  have ha : 2*(Real.pi : ℂ)*Complex.I*(centered (x*j) : ℂ)/(p : ℂ) =
      ((2*Real.pi*(centered (x*j) : ℝ)/(p : ℝ) : ℝ) : ℂ)*Complex.I := by
    push_cast
    ring
  rw [ha,Complex.exp_ofReal_mul_I_re]

theorem centered_cosine_bound {p : ℕ} [Fact p.Prime] (j x : ZMod p) :
    (frequencyCharacter p j x).re ≤ 1-8*((centered (x*j) : ℝ)/(p : ℝ))^2 := by
  have hp : (0 : ℝ)<p := by exact_mod_cast (Fact.out : p.Prime).pos
  have hl : -(p : ℝ)<2*(centered (x*j) : ℝ) := by exact_mod_cast (centered_range (x*j)).1
  have hr : 2*(centered (x*j) : ℝ) ≤ p := by exact_mod_cast (centered_range (x*j)).2
  have hθ : |2*Real.pi*(centered (x*j) : ℝ)/(p : ℝ)| ≤ Real.pi := by
    apply abs_le.mpr
    constructor
    · apply (le_div_iff₀ hp).mpr
      have h := mul_le_mul_of_nonneg_left hl.le Real.pi_pos.le
      nlinarith
    · apply (div_le_iff₀ hp).mpr
      have h := mul_le_mul_of_nonneg_left hr Real.pi_pos.le
      nlinarith
  have hc := Real.cos_le_one_sub_mul_cos_sq hθ
  have he : 2/Real.pi^2*(2*Real.pi*(centered (x*j) : ℝ)/(p : ℝ))^2 =
      8*((centered (x*j) : ℝ)/(p : ℝ))^2 := by
    field_simp [Real.pi_ne_zero,hp.ne']
    ring
  rw [he] at hc
  rw [centered_character_cosine]
  exact hc

theorem centered_character_difference_bound {p : ℕ} [Fact p.Prime] (j x y : ZMod p) :
    16*((centered ((x-y)*j) : ℝ)/(p : ℝ))^2 ≤
      Complex.normSq (frequencyCharacter p j x-frequencyCharacter p j y) := by
  rw [character_difference_normSq]
  have h := centered_cosine_bound j (x-y)
  linarith

noncomputable def centeredRadius {I : Type*} [Fintype I] {p : ℕ}
    (a : I → ZMod p) (r : I) (j : ZMod p) : ℝ :=
  ∑ i, ((centered ((a r-a i)*j) : ℝ)/(p : ℝ))^2

theorem centered_radius_nonneg {I : Type*} [Fintype I] {p : ℕ}
    (a : I → ZMod p) (r : I) (j : ZMod p) : 0 ≤ centeredRadius a r j :=
  Finset.sum_nonneg (fun _ _ => sq_nonneg _)

theorem centered_eigenvalue_bound {I : Type*} [Fintype I] {p : ℕ} [Fact p.Prime]
    (q : PMF I) (a : I → ZMod p) (r : I) (j : ZMod p) {ω : ℝ}
    (hω : 0 ≤ ω) (hw : ∀ i, ω ≤ weight q i) :
    Complex.normSq (characterEigenvalue q a (frequencyCharacter p j)) ≤ 1-8*ω^2*centeredRadius a r j := by
  let z : I → ℂ := fun i => frequencyCharacter p j (a i)
  have hv := unit_phase_variance q z (fun i => character_unit_normSq _ _)
  have hn (i k : I) : 0 ≤ weight q i*weight q k*Complex.normSq (z i-z k) :=
    mul_nonneg (mul_nonneg (weight_nonneg q i) (weight_nonneg q k)) (Complex.normSq_nonneg _)
  have hrow : (∑ k, weight q r*weight q k*Complex.normSq (z r-z k)) ≤
      ∑ i, ∑ k, weight q i*weight q k*Complex.normSq (z i-z k) :=
    Finset.single_le_sum (fun i _ => Finset.sum_nonneg (fun k _ => hn i k)) (Finset.mem_univ r)
  have hlow : (∑ i, 16*ω^2*((centered ((a r-a i)*j) : ℝ)/(p : ℝ))^2) ≤
      ∑ i, weight q r*weight q i*Complex.normSq (z r-z i) := by
    apply Finset.sum_le_sum
    intro i _
    have hwprod : ω^2 ≤ weight q r*weight q i := by
      simpa only [pow_two] using mul_le_mul (hw r) (hw i) hω (weight_nonneg q r)
    calc
      _ = ω^2*(16*((centered ((a r-a i)*j) : ℝ)/(p : ℝ))^2) := by ring
      _ ≤ (weight q r*weight q i)*(16*((centered ((a r-a i)*j) : ℝ)/(p : ℝ))^2) :=
        mul_le_mul_of_nonneg_right hwprod (by positivity)
      _ ≤ _ := mul_le_mul_of_nonneg_left (centered_character_difference_bound j (a r) (a i))
        (mul_nonneg (weight_nonneg q r) (weight_nonneg q i))
  rw [← Finset.mul_sum] at hlow
  change 1-Complex.normSq (characterEigenvalue q a (frequencyCharacter p j)) = _ at hv
  change _ ≤ 1-8*ω^2*(∑ i, ((centered ((a r-a i)*j) : ℝ)/(p : ℝ))^2)
  have h := hlow.trans hrow
  linarith

theorem centered_eigenvalue_power_bound {I : Type*} [Fintype I] {p : ℕ} [Fact p.Prime]
    (q : PMF I) (a : I → ZMod p) (r : I) (j : ZMod p) {ω : ℝ}
    (hω : 0 ≤ ω) (hw : ∀ i, ω ≤ weight q i) (t : ℕ) :
    (Complex.normSq (characterEigenvalue q a (frequencyCharacter p j)))^t ≤
      Real.exp (-(8*ω^2*(t : ℝ))*centeredRadius a r j) := by
  let A : ℝ := 8*ω^2*centeredRadius a r j
  have he : 1-A ≤ Real.exp (-A) := by
    simpa only [sub_eq_add_neg,add_comm] using Real.add_one_le_exp (-A)
  have hb := (centered_eigenvalue_bound q a r j hω hw).trans he
  have hp := pow_le_pow_left₀ (Complex.normSq_nonneg _) hb t
  rw [← Real.exp_nat_mul] at hp
  calc
    _ ≤ Real.exp ((t : ℝ)*(-A)) := hp
    _ = _ := by congr 1; unfold A; ring

theorem actual_centered_moment_bound {I : Type*} [Fintype I] {p : ℕ} [Fact p.Prime]
    (q : PMF I) (a : I → ZMod p) (r : I) (j : ZMod p) {ω : ℝ}
    (hω : 0 ≤ ω) (hw : ∀ i, ω ≤ weight q i) (t : ℕ) (x : ZMod p) :
    Complex.normSq (complexExpectation (kernelLaw q (fun i y => y+a i) t x) (frequencyCharacter p j)) ≤
      Real.exp (-(8*ω^2*(t : ℝ))*centeredRadius a r j) := by
  rw [actual_character_squared_moment]
  exact centered_eigenvalue_power_bound q a r j hω hw t

/-- The decay constant is chosen from the fixed positive label law before any
modulus, support, frequency or time is selected. -/
theorem uniform_weight_centered_decay {I : Type*} [Fintype I] [Nonempty I]
    (q : PMF I) (hq : ∀ i, 0<weight q i) :
    ∃ ω : ℝ, 0<ω ∧ (∀ i, ω ≤ weight q i) ∧
      ∀ (p : ℕ) [Fact p.Prime] (a : I → ZMod p) (r : I) (j : ZMod p)
        (t : ℕ) (x : ZMod p),
      Complex.normSq (complexExpectation (kernelLaw q (fun i y => y+a i) t x)
        (frequencyCharacter p j)) ≤ Real.exp (-(8*ω^2*(t : ℝ))*centeredRadius a r j) := by
  obtain ⟨ω,hω,hw⟩ := positive_weight_lower_bound q hq
  refine ⟨ω,hω,hw,?_⟩
  intro p _ a r j t x
  exact actual_centered_moment_bound q a r j hω.le hw t x

end SupportGaussianAudit
