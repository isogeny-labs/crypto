import FiniteFieldHandbook

/- Research consumers can use a checked reference directly, with Lean checking
   the hypotheses at the call site. These examples exercise the import interface. -/
example (K : Type*) [Field K] [Fintype K] (a : K) (ha : a ≠ 0) :
    a ^ (Fintype.card K - 1) = 1 :=
  FiniteFieldHandbook.fermat K a ha

example (K L : Type*) [Field K] [Field L] [Finite L] [Algebra K L] :
    Function.Surjective (Algebra.norm K (S := L)) :=
  FiniteFieldHandbook.norm_surjective K L

example (K : Type*) [Field K] [Fintype K] (f : K → K) :
    ∃ g : Polynomial K, g.degree < Fintype.card K ∧ ∀ a, g.eval a = f a :=
  FiniteFieldHandbook.polynomial_function_bounded K f
