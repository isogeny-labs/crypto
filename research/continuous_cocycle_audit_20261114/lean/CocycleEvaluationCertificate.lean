import GaloisCocycleControl

namespace ContinuousCocycleAudit
noncomputable section

theorem integers_not_torsion (n : ℕ) (hn : 0<n) : n • (1:ℤ)≠0 := by
  simp only [nsmul_eq_mul,mul_one]
  omega

theorem noncompact_control :
    ∃ c : Multiplicative ℤ →ₜ* Multiplicative ℤ,c (Multiplicative.ofAdd (1:ℤ))≠1 :=
  ⟨ContinuousMonoidHom.id _,by decide⟩

/-- Refute the literal broad evaluation-isomorphism formula using actual
Krull Gal(F2bar/F2), exact square Frobenius, discrete trivial Z, and every
continuous 1-cocycle. No homemade H1 replaces canonical cohomology. -/
theorem continuous_cocycle_evaluation_certificate :
    CompactSpace ActualGalois ∧ DiscreteTopology ℤ ∧
    (∀ x : AlgebraicClosure (ZMod 2),galoisAlgEquiv actualFrobenius x=x^2) ∧
    (∀ σ z,trivialIntegerAction σ z=z) ∧
    Continuous (fun p : ActualGalois × ℤ => trivialIntegerAction p.1 p.2) ∧
    (∀ (c : C(ActualGalois,ℤ)),
      (∀ σ τ,c (σ*τ)=c σ+trivialIntegerAction σ (c τ)) → ∀ σ,c σ=0) ∧
    (∀ c : IntegerCocycle,c=1) ∧
    (∀ n : ℕ,0<n → n • (1:ℤ)≠0) ∧
    missingIntegerClass≠1 ∧
    (∀ σ : ActualGalois,¬Function.Surjective (evaluateCocycle σ)) ∧
    (¬Function.Surjective (evaluateCocycle actualFrobenius)) ∧
    (¬∃ (H : Type) (π : IntegerCocycle → H) (e : H ≃ IntegerCoinvariants),
      Function.Surjective π ∧ ∀ c,e (π c)=evaluateCocycle actualFrobenius c) ∧
    (∃ c : Multiplicative ℤ →ₜ* Multiplicative ℤ,c (Multiplicative.ofAdd (1:ℤ))≠1) := by
  refine ⟨inferInstance,inferInstance,actual_frobenius_square,?_,
    trivial_integer_action_continuous,every_continuous_integer_cocycle_zero,
    integer_cocycle_trivial,integers_not_torsion,missing_integer_class_ne_one,
    evaluation_not_surjective,evaluation_not_surjective actualFrobenius,
    no_evaluation_quotient_equiv actualFrobenius,noncompact_control⟩
  intro σ z
  rfl

#print axioms integers_not_torsion
#print axioms noncompact_control
#print axioms continuous_cocycle_evaluation_certificate
end
end ContinuousCocycleAudit
