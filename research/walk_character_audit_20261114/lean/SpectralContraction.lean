module

public import PhaseVariance

@[expose] public section

namespace WalkCharacterAudit

open WeightedCycleCouplingAudit
open scoped ComplexConjugate

theorem character_unit_normSq {G : Type*} [AddGroup G] [Finite G] (ψ : AddChar G ℂ) (x : G) :
    Complex.normSq (ψ x)=1 := by rw [Complex.normSq_eq_norm_sq,ψ.norm_apply,one_pow]

theorem actual_character_squared_moment {I G : Type*} [Fintype I] [Fintype G] [AddCommGroup G]
    (q : PMF I) (a : I → G) (ψ : AddChar G ℂ) (t : ℕ) (x : G) :
    Complex.normSq (complexExpectation (kernelLaw q (fun i y => y+a i) t x) ψ) =
      (Complex.normSq (characterEigenvalue q a ψ))^t := by
  rw [actual_character_walk,Complex.normSq_mul,map_pow,character_unit_normSq,mul_one]

theorem character_difference_normSq {G : Type*} [AddCommGroup G] [Finite G]
    (ψ : AddChar G ℂ) (x y : G) :
    Complex.normSq (ψ x-ψ y)=2-2*(ψ (x-y)).re := by
  have he : ψ (x-y)=ψ x*conj (ψ y) := by
    rw [sub_eq_add_neg,ψ.map_add_eq_mul,ψ.map_neg_eq_conj]
  have hx := character_unit_normSq ψ x
  have hy := character_unit_normSq ψ y
  rw [he]
  simp only [Complex.normSq_apply] at hx hy ⊢
  simp only [Complex.sub_re,Complex.sub_im,Complex.mul_re,Complex.conj_re,Complex.conj_im]
  nlinarith

theorem actual_character_pair_bound {I G : Type*} [Fintype I] [AddCommGroup G] [Finite G]
    (q : PMF I) (a : I → G) (ψ : AddChar G ℂ) (i j : I) (hij : i ≠ j) :
    Complex.normSq (characterEigenvalue q a ψ) ≤
      1-2*weight q i*weight q j*(1-(ψ (a i-a j)).re) := by
  have hc := two_phase_contraction q (fun i => ψ (a i))
    (fun i => character_unit_normSq ψ (a i)) i j hij
  change Complex.normSq (characterEigenvalue q a ψ) ≤
    1-weight q i*weight q j*Complex.normSq (ψ (a i)-ψ (a j)) at hc
  calc
    _ ≤ _ := hc
    _ = _ := by rw [character_difference_normSq]; ring

theorem actual_character_phase_alignment {I G : Type*} [Fintype I] [AddCommGroup G] [Finite G]
    (q : PMF I) (a : I → G) (ψ : AddChar G ℂ) (i j : I) (hij : i ≠ j)
    (hi : 0<weight q i) (hj : 0<weight q j) (δ : ℝ)
    (hl : 1-2*weight q i*weight q j*δ < Complex.normSq (characterEigenvalue q a ψ)) :
    1-δ<(ψ (a i-a j)).re := by
  have hc := actual_character_pair_bound q a ψ i j hij
  by_contra hn
  have hgap : δ ≤ 1-(ψ (a i-a j)).re := by linarith
  have hm := mul_le_mul_of_nonneg_left hgap (show 0 ≤ 2*weight q i*weight q j by positivity)
  linarith

theorem frequency_character_cosine {p : ℕ} [NeZero p] (j x : ZMod p) :
    (frequencyCharacter p j x).re = Real.cos (2*Real.pi*((x*j).val : ℝ)/(p : ℝ)) := by
  rw [frequency_character_apply]
  have he : ZMod.stdAddChar (x*j)=
      Complex.exp (2*(Real.pi : ℂ)*Complex.I*((x*j).val : ℂ)/(p : ℂ)) := by
    simpa only [Int.cast_natCast,ZMod.natCast_zmod_val] using
      ZMod.stdAddChar_coe (N := p) (((x*j).val : ℕ) : ℤ)
  rw [he]
  have ha : 2*(Real.pi : ℂ)*Complex.I*((x*j).val : ℂ)/(p : ℂ) =
      ((2*Real.pi*((x*j).val : ℝ)/(p : ℝ) : ℝ) : ℂ)*Complex.I := by
    push_cast
    ring
  rw [ha,Complex.exp_ofReal_mul_I_re]

theorem prime_frequency_strict {I : Type*} [Fintype I] {p : ℕ} [Fact p.Prime]
    (q : PMF I) (a : I → ZMod p) (ha : Function.Injective a) (j : ZMod p) (hj : j ≠ 0)
    (u v : I) (huv : u ≠ v) (hu : 0<weight q u) (hv : 0<weight q v) :
    Complex.normSq (characterEigenvalue q a (frequencyCharacter p j))<1 := by
  apply strict_two_phase_contraction q (fun i => frequencyCharacter p j (a i))
    (fun i => character_unit_normSq _ _) u v huv hu hv
  intro he
  exact huv (ha ((frequency_character_injective j hj) he))

theorem pure_character_eigenvalue {I G : Type*} [Fintype I] [AddGroup G]
    (i : I) (a : I → G) (ψ : AddChar G ℂ) : characterEigenvalue (PMF.pure i) a ψ=ψ (a i) :=
  complex_expectation_pure i (fun i => ψ (a i))

def twoSupports (p : ℕ) (b : Bool) : ZMod p := if b then 1 else 0

theorem two_supports_injective (p : ℕ) [Fact p.Prime] : Function.Injective (twoSupports p) := by
  intro b c h
  cases b <;> cases c <;> simp_all [twoSupports]

theorem zero_weight_counter (p : ℕ) [Fact p.Prime] :
    Function.Injective (twoSupports p) ∧
    characterEigenvalue (PMF.pure false) (twoSupports p) (frequencyCharacter p 1)=1 ∧
    ¬ Complex.normSq (characterEigenvalue (PMF.pure false) (twoSupports p) (frequencyCharacter p 1))<1 := by
  have he : characterEigenvalue (PMF.pure false) (twoSupports p) (frequencyCharacter p 1)=1 := by
    rw [pure_character_eigenvalue]
    simp [twoSupports]
  exact ⟨two_supports_injective p,he,by rw [he]; norm_num⟩

theorem pure_false_weights : weight (PMF.pure false) false=1 ∧ weight (PMF.pure false) true=0 := by
  norm_num [weight,PMF.pure_apply]

theorem fair_weights_positive (b : Bool) : 0<weight CycleCouplingAudit.fairCoin b := by
  norm_num [weight,CycleCouplingAudit.fairCoin,PMF.ofFintype_apply]

def evenSupports (b : Bool) : ZMod 4 := if b then 2 else 0

theorem composite_frequency_counter :
    Function.Injective evenSupports ∧ (2 : ZMod 4) ≠ 0 ∧
    characterEigenvalue CycleCouplingAudit.fairCoin evenSupports (frequencyCharacter 4 2)=1 := by
  have hi : Function.Injective evenSupports := by decide +kernel
  have hz : (2 : ZMod 4) ≠ 0 := by decide +kernel
  have hp : ∀ b : Bool, frequencyCharacter 4 2 (evenSupports b)=1 := by
    intro b
    cases b with
    | false => simp [evenSupports]
    | true =>
      rw [frequency_character_apply]
      change ZMod.stdAddChar ((2 : ZMod 4)*2)=1
      rw [show (2 : ZMod 4)*2=0 by decide +kernel]
      exact (ZMod.stdAddChar (N := 4)).map_zero_eq_one
  refine ⟨hi,hz,?_⟩
  unfold characterEigenvalue
  simp_rw [hp,mul_one]
  exact complex_weight_sum _

end WalkCharacterAudit
