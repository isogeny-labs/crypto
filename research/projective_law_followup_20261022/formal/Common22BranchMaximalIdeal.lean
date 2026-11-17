import Common22BranchStalk

/-!
# The actual affine branch-point maximal ideal

Polynomial induction shows that evaluation at `(a,0)` has kernel generated
by `X-a` and `Y`.  The quotient-ring kernel therefore has the corresponding
two generators.  This result does not assert that the branch stalk is a DVR.
-/

set_option autoImplicit false
noncomputable section

namespace Common22BranchMaximalIdeal

open MvPolynomial Common22SeparateChartMap Common22BranchStalk

variable {K : Type*} [Field K]

def sourceIdeal (a : K) : Ideal (Poly K) :=
  Ideal.span {(X 0 : Poly K) - C a, X 1}

theorem subtract_preEval_mem (a : K) (p : Poly K) :
    p - C (preEval a p) ∈ sourceIdeal a := by
  induction p using MvPolynomial.induction_on with
  | C b => simp [preEval]
  | add p q hp hq =>
      have h := Ideal.add_mem (sourceIdeal a) hp hq
      convert h using 1 <;> simp [map_add] <;> ring
  | mul_X p i hp =>
      fin_cases i
      · have hval : preEval a (p * X 0) = preEval a p * a := by
          simp [preEval]
        have heq : p * X 0 - C (preEval a (p * X 0)) =
            p * (X 0 - C a) + C a * (p - C (preEval a p)) := by
          rw [hval]
          simp only [map_mul]
          ring
        have hgen : X 0 - C a ∈ sourceIdeal a :=
          Ideal.subset_span (by simp [sourceIdeal])
        have hmem : p * X 0 - C (preEval a (p * X 0)) ∈ sourceIdeal a := by
          rw [heq]
          exact Ideal.add_mem _ (Ideal.mul_mem_left _ p hgen)
            (Ideal.mul_mem_left _ (C a) hp)
        simpa using hmem
      · have hz : preEval a (p * X 1) = 0 := by simp [preEval]
        have hgen : (X 1 : Poly K) ∈ sourceIdeal a :=
          Ideal.subset_span (by simp [sourceIdeal])
        simpa [hz] using
          (Ideal.mul_mem_left (sourceIdeal a) p hgen)

def coordinateIdeal (a3 a2 a1 a0 a : K) :
    Ideal (AffineRing a3 a2 a1 a0) :=
  Ideal.span {xbar a3 a2 a1 a0 -
    Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (C a), ybar a3 a2 a1 a0}

theorem mapped_sourceIdeal (a3 a2 a1 a0 a : K) :
    Ideal.map (Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0)) (sourceIdeal a) =
      coordinateIdeal a3 a2 a1 a0 a := by
  simp only [sourceIdeal, coordinateIdeal, Ideal.map_span]
  apply congrArg Ideal.span
  ext x
  constructor
  · rintro ⟨p, hp, rfl⟩
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hp
    rcases hp with rfl | rfl
    · left
      simp [xbar]
    · right
      rfl
  · intro hx
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hx
    rcases hx with rfl | rfl
    · exact ⟨X 0 - C a, by simp, by simp [xbar]⟩
    · exact ⟨X 1, by simp, rfl⟩

theorem branchIdeal_eq_coordinateIdeal (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    branchIdeal a3 a2 a1 a0 a hroot = coordinateIdeal a3 a2 a1 a0 a := by
  apply le_antisymm
  · intro q hq
    obtain ⟨p, rfl⟩ := Ideal.Quotient.mk_surjective q
    have hp : preEval a p = 0 := by
      exact (RingHom.mem_ker).mp hq
    have hsrc : p ∈ sourceIdeal a := by
      simpa [hp] using subtract_preEval_mem a p
    rw [← mapped_sourceIdeal]
    exact Ideal.mem_map_of_mem _ hsrc
  · rw [coordinateIdeal]
    apply Ideal.span_le.mpr
    intro q hq
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hq
    rcases hq with rfl | rfl
    · simpa using xbar_sub_a_mem_branchIdeal a3 a2 a1 a0 a hroot
    · exact ybar_mem_branchIdeal a3 a2 a1 a0 a hroot

#print axioms subtract_preEval_mem
#print axioms mapped_sourceIdeal
#print axioms branchIdeal_eq_coordinateIdeal

end Common22BranchMaximalIdeal
