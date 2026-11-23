module
public import InfiniteControl

@[expose] public section
namespace ProcyclicGaloisCocycleAudit
noncomputable section

/-- Actual finite-field Galois generation, finite continuous cocycles and their
standard class quotient. This does not assume or claim the comparison to the
separate Mathlib continuous homogeneous-complex cohomology object. -/
theorem procyclic_galois_cocycle_certificate :
    ∀ (K : Type) [Field K] [Fintype K],
      Nonempty (Gamma K ≃ₜ* Field.absoluteGaloisGroup K) ∧
      (∀ a : Omega K, asAlgEquiv K (frobenius K) a = a ^ Fintype.card K) ∧
      DenseRange (fun n : ℤ => frobenius K ^ n) ∧
      (∀ (H : Type) [Group H] [Finite H] [TopologicalSpace H] [DiscreteTopology H],
        Function.Bijective (fun f : Gamma K →ₜ* H => f (frobenius K))) ∧
      (∀ (M : Type) [CommGroup M] [Finite M] [TopologicalSpace M] [DiscreteTopology M]
        [TopologicalSpace (MulAut M)] [DiscreteTopology (MulAut M)]
        (ρ : Gamma K →ₜ* MulAut M),
        Function.Bijective (cocycleEvaluation K M ρ) ∧
        Function.Bijective (classEvaluationHom K M ρ) ∧
        Nonempty (CocycleClasses K M ρ ≃* Coinvariants K M ρ) ∧
        (∀ c : Cocycles K M ρ,
          classEvaluationEquiv K M ρ (QuotientGroup.mk c) =
            QuotientGroup.mk (c.val (frobenius K)))) ∧
      missingIntegerClass K ≠ 1 ∧
      (∀ c : IntegerClasses K, integerEvaluation K c = 1) ∧
      ¬ Function.Surjective (integerEvaluation K) := by
  intro K _ _
  refine ⟨⟨absoluteGaloisEquiv K⟩, frobenius_apply K, frobenius_powers_dense K,
    ?_, ?_, missing_integer_class_ne_one K, integer_class_evaluation_one K,
    finite_guard_needed K⟩
  · intro H _ _ _ _
    exact finite_hom_evaluation_bijective H
  · intro M _ _ _ _ _ _ ρ
    exact ⟨cocycle_evaluation_bijective K M ρ, class_evaluation_bijective K M ρ,
      ⟨classEvaluationEquiv K M ρ⟩, classEvaluation_mk K M ρ⟩

#print axioms procyclic_galois_cocycle_certificate
end
end ProcyclicGaloisCocycleAudit
