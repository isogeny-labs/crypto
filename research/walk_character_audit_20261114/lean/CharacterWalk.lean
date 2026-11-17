module

public import ComplexExpectation

@[expose] public section

namespace WalkCharacterAudit

open WeightedCycleCouplingAudit

noncomputable def characterEigenvalue {I G : Type*} [Fintype I] [AddGroup G]
    (q : PMF I) (a : I → G) (ψ : AddChar G ℂ) : ℂ := ∑ i, (weight q i : ℂ)*ψ (a i)

theorem actual_character_walk {I G : Type*} [Fintype I] [Fintype G] [AddCommGroup G]
    (q : PMF I) (a : I → G) (ψ : AddChar G ℂ) (t : ℕ) (x : G) :
    complexExpectation (kernelLaw q (fun i y => y+a i) t x) ψ =
      (characterEigenvalue q a ψ)^t*ψ x := by
  induction t generalizing x with
  | zero => simp only [kernelLaw,complex_expectation_pure,pow_zero,one_mul]
  | succ t ih =>
    rw [kernelLaw,complex_expectation_bind]
    simp only [ih,ψ.map_add_eq_mul]
    calc
      _ = ∑ i, ((weight q i : ℂ)*ψ (a i))*((characterEigenvalue q a ψ)^t*ψ x) := by
        apply Finset.sum_congr rfl
        intro i _
        ring
      _ = (∑ i, (weight q i : ℂ)*ψ (a i))*((characterEigenvalue q a ψ)^t*ψ x) :=
        (Finset.sum_mul _ _ _).symm
      _ = _ := by rw [characterEigenvalue,pow_succ]; ring

theorem trivial_character_eigenvalue {I G : Type*} [Fintype I] [AddGroup G]
    (q : PMF I) (a : I → G) : characterEigenvalue q a (0 : AddChar G ℂ)=1 := by
  simp [characterEigenvalue,complex_weight_sum]

noncomputable def frequencyCharacter (p : ℕ) [NeZero p] (j : ZMod p) : AddChar (ZMod p) ℂ :=
  (ZMod.stdAddChar (N := p)).compAddMonoidHom (.mulRight j)

theorem frequency_character_apply {p : ℕ} [NeZero p] (j x : ZMod p) :
    frequencyCharacter p j x=ZMod.stdAddChar (x*j) := rfl

theorem frequency_character_injective {p : ℕ} [Fact p.Prime] (j : ZMod p) (hj : j ≠ 0) :
    Function.Injective (frequencyCharacter p j) := by
  intro x y h
  have hc := ZMod.injective_stdAddChar h
  change x*j=y*j at hc
  have hz : (x-y)*j=0 := by rw [sub_mul,hc,sub_self]
  exact sub_eq_zero.mp ((mul_eq_zero.mp hz).resolve_right hj)

theorem zero_frequency_eigenvalue {I : Type*} [Fintype I] {p : ℕ} [NeZero p]
    (q : PMF I) (a : I → ZMod p) : characterEigenvalue q a (frequencyCharacter p 0)=1 := by
  simp [characterEigenvalue,frequency_character_apply,complex_weight_sum]

end WalkCharacterAudit
