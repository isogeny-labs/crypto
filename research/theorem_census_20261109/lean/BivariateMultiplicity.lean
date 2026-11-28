import Mathlib.RingTheory.AdjoinRoot
import Mathlib.Algebra.MvPolynomial.CommRing
import Mathlib.Tactic

/-! A faithful two-variable multiplicity control. No solver implementation is
part of the kernel statement, and this is not the general Diem Bezout theorem. -/
namespace BivariateMultiplicityAudit
noncomputable section
open MvPolynomial
variable (K : Type*) [Field K] (c : K)

abbrev P := MvPolynomial (Fin 2) K
def equations : Set (P K) := {X 0 * X 1, X 0 + X 1 - C c}
def equationsIdeal : Ideal (P K) := Ideal.span (equations K c)
abbrev A := P K ⧸ equationsIdeal K c
def quadratic : Polynomial K := Polynomial.X^2 - Polynomial.C c * Polynomial.X
abbrev Q := AdjoinRoot (quadratic K c)

theorem root_relation :
    (AdjoinRoot.root (quadratic K c))^2 =
      algebraMap K (Q K c) c * AdjoinRoot.root (quadratic K c) := by
  have h := AdjoinRoot.mk_self (f := quadratic K c)
  change (AdjoinRoot.mk (quadratic K c))
    (Polynomial.X^2 - Polynomial.C c * Polynomial.X) = 0 at h
  rw [map_sub, map_pow, map_mul, AdjoinRoot.mk_X, AdjoinRoot.mk_C] at h
  exact sub_eq_zero.mp h

def evaluate : P K →ₐ[K] Q K c :=
  MvPolynomial.aeval (fun i : Fin 2 => if i = 0 then
    algebraMap K (Q K c) c - AdjoinRoot.root (quadratic K c)
    else AdjoinRoot.root (quadratic K c))

theorem evaluate_kills_ideal (f : P K) (hf : f ∈ equationsIdeal K c) :
    evaluate K c f = 0 := by
  have hsub : equationsIdeal K c ≤ RingHom.ker (evaluate K c).toRingHom := by
    rw [equationsIdeal, Ideal.span_le]
    intro g hg
    change evaluate K c g = 0
    simp only [equations, Set.mem_insert_iff, Set.mem_singleton_iff] at hg
    rcases hg with rfl | rfl
    · simp [evaluate]
      have hr := root_relation K c
      simp only [AdjoinRoot.algebraMap_eq] at hr
      linear_combination -hr
    · simp [evaluate]
  exact hsub hf

def forward : A K c →ₐ[K] Q K c :=
  Ideal.Quotient.liftₐ (equationsIdeal K c) (evaluate K c) (evaluate_kills_ideal K c)

def coordinate (i : Fin 2) : A K c := Ideal.Quotient.mk (equationsIdeal K c) (X i)

theorem quotient_product : coordinate K c 0 * coordinate K c 1 = 0 := by
  rw [coordinate, coordinate, ← map_mul, Ideal.Quotient.eq_zero_iff_mem]
  exact Ideal.subset_span (by simp [equations])

theorem quotient_sum : coordinate K c 0 + coordinate K c 1 = algebraMap K (A K c) c := by
  apply sub_eq_zero.mp
  change Ideal.Quotient.mk (equationsIdeal K c) (X 0 + X 1 - C c) = 0
  rw [Ideal.Quotient.eq_zero_iff_mem]
  exact Ideal.subset_span (by simp [equations])

theorem quotient_root_relation : (coordinate K c 1)^2 =
    algebraMap K (A K c) c * coordinate K c 1 := by
  linear_combination -(quotient_product K c) +
    coordinate K c 1 * (quotient_sum K c)

def backward : Q K c →ₐ[K] A K c :=
  AdjoinRoot.liftAlgHom (quadratic K c) (Algebra.ofId K (A K c))
    (coordinate K c 1) (by
      simpa [quadratic, sub_eq_zero] using quotient_root_relation K c)

@[simp] theorem forward_coordinate (i : Fin 2) :
    forward K c (coordinate K c i) = if i = 0 then
      algebraMap K (Q K c) c - AdjoinRoot.root (quadratic K c)
    else AdjoinRoot.root (quadratic K c) := by
  change evaluate K c (X i) = _
  simp [evaluate]

@[simp] theorem backward_root :
    backward K c (AdjoinRoot.root (quadratic K c)) = coordinate K c 1 := by
  simp [backward]

def quotientEquiv : A K c ≃ₐ[K] Q K c :=
  AlgEquiv.ofAlgHom (forward K c) (backward K c)
    (by apply AdjoinRoot.algHom_ext; simp)
    (by
      apply Ideal.Quotient.algHom_ext K
      apply MvPolynomial.algHom_ext
      intro i
      change backward K c (forward K c (coordinate K c i)) = coordinate K c i
      fin_cases i
      · rw [forward_coordinate]
        change backward K c (algebraMap K (Q K c) c -
          AdjoinRoot.root (quadratic K c)) = coordinate K c 0
        rw [map_sub, AlgHom.commutes, backward_root]
        linear_combination -(quotient_sum K c)
      · simp)

theorem quotient_dimension_two : Module.finrank K (A K c) = 2 := by
  rw [(quotientEquiv K c).toLinearEquiv.finrank_eq]
  change Module.finrank K (Polynomial K ⧸ Ideal.span {quadratic K c}) = 2
  rw [finrank_quotient_span_eq_natDegree]
  have hp : quadratic K c = Polynomial.X * (Polynomial.X - Polynomial.C c) := by
    unfold quadratic
    ring
  rw [hp, Polynomial.natDegree_mul (by simp) (Polynomial.X_sub_C_ne_zero c)]
  simp

theorem triangular_generators_same_ideal :
    equationsIdeal K c = Ideal.span
      ({X 0 + X 1 - C c, X 1^2 - C c * X 1} : Set (P K)) := by
  apply le_antisymm
  · apply Ideal.span_le.mpr
    intro f hf
    simp only [equations, Set.mem_insert_iff, Set.mem_singleton_iff] at hf
    rcases hf with rfl | rfl
    · have hlin : X 0 + X 1 - C c ∈ Ideal.span
          ({X 0 + X 1 - C c, X 1^2 - C c * X 1} : Set (P K)) :=
        Ideal.subset_span (by simp)
      have hquad : X 1^2 - C c * X 1 ∈ Ideal.span
          ({X 0 + X 1 - C c, X 1^2 - C c * X 1} : Set (P K)) :=
        Ideal.subset_span (by simp)
      have hi : (X 0 * X 1 : P K) =
          X 1 * (X 0 + X 1 - C c) - (X 1^2 - C c * X 1) := by ring
      rw [hi]
      exact Ideal.sub_mem _ (Ideal.mul_mem_left _ _ hlin) hquad
    · exact Ideal.subset_span (by simp)
  · apply Ideal.span_le.mpr
    intro f hf
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hf
    rcases hf with rfl | rfl
    · exact Ideal.subset_span (by simp [equations])
    · have hlin : X 0 + X 1 - C c ∈ equationsIdeal K c :=
        Ideal.subset_span (by simp [equations])
      have hprod : X 0 * X 1 ∈ equationsIdeal K c :=
        Ideal.subset_span (by simp [equations])
      have hi : (X 1^2 - C c * X 1 : P K) =
          X 1 * (X 0 + X 1 - C c) - X 0 * X 1 := by ring
      rw [hi]
      exact Ideal.sub_mem _ (Ideal.mul_mem_left _ _ hlin) hprod

theorem solutions (x y : K) :
    (x*y=0 ∧ x+y=c) ↔ ((x=0 ∧ y=c) ∨ (x=c ∧ y=0)) := by
  constructor
  · rintro ⟨hm,hs⟩
    rcases mul_eq_zero.mp hm with hx | hy
    · exact Or.inl ⟨hx, by simpa [hx] using hs⟩
    · exact Or.inr ⟨by simpa [hy] using hs, hy⟩
  · rintro (⟨rfl,rfl⟩ | ⟨rfl,rfl⟩) <;> simp

theorem double_point_unique_solution (x y : K) :
    (x*y=0 ∧ x+y=0) ↔ (x=0 ∧ y=0) := by
  simpa using solutions K 0 x y

theorem split_solutions_distinct (hc : c ≠ 0) :
    ((0,c) : K × K) ≠ (c,0) := by
  intro h
  exact hc (congrArg Prod.fst h).symm

private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩

/-- Closed refutation of identifying the number of distinct solutions with
the quotient-algebra dimension, for this actual two-variable ideal. -/
theorem support_is_not_multiplicity :
    Module.finrank (ZMod 5) (A (ZMod 5) 0) = 2 ∧
    Fintype.card {z : ZMod 5 × ZMod 5 // z.1*z.2=0 ∧ z.1+z.2=0} = 1 := by
  exact ⟨quotient_dimension_two (ZMod 5) 0, by decide⟩

end
end BivariateMultiplicityAudit
