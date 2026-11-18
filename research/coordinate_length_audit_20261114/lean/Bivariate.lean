import Thickening

namespace CoordinateLengthAudit
noncomputable section
open MvPolynomial

variable (K : Type*) [Field K]

abbrev BiPoly := MvPolynomial (Fin 2) K

def biIdeal (m : ℕ) : Ideal (BiPoly K) :=
  Ideal.span ({X 0^m,X 1-X 0} : Set (BiPoly K))

abbrev BiAlgebra (m : ℕ) := CoordinateAlgebra (biIdeal K m)

def biCoord (m : ℕ) (i : Fin 2) : BiAlgebra K m :=
  Ideal.Quotient.mk (biIdeal K m) (X i)

theorem bi_coord_power (m : ℕ) : (biCoord K m 0)^m = 0 := by
  rw [biCoord,← map_pow,Ideal.Quotient.eq_zero_iff_mem]
  exact Ideal.subset_span (by simp)

theorem bi_coord_equal (m : ℕ) : biCoord K m 1 = biCoord K m 0 := by
  apply sub_eq_zero.mp
  change Ideal.Quotient.mk (biIdeal K m) (X 1-X 0) = 0
  rw [Ideal.Quotient.eq_zero_iff_mem]
  exact Ideal.subset_span (by simp)

def biEvaluate (m : ℕ) : BiPoly K →ₐ[K] Thick K m :=
  aeval (fun _ => thickRoot K m)

theorem bi_evaluate_kills_ideal (m : ℕ) (p : BiPoly K) (hp : p ∈ biIdeal K m) :
    biEvaluate K m p = 0 := by
  have hsub : biIdeal K m ≤ RingHom.ker (biEvaluate K m).toRingHom := by
    rw [biIdeal,Ideal.span_le]
    intro p hp
    change biEvaluate K m p=0
    simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hp
    rcases hp with rfl | rfl
    · simpa [biEvaluate] using thick_root_power K m
    · simp [biEvaluate]
  exact hsub hp

def biForward (m : ℕ) : BiAlgebra K m →ₐ[K] Thick K m :=
  Ideal.Quotient.liftₐ (biIdeal K m) (biEvaluate K m) (bi_evaluate_kills_ideal K m)

def biBackward (m : ℕ) : Thick K m →ₐ[K] BiAlgebra K m :=
  AdjoinRoot.liftAlgHom (Polynomial.X^m : Polynomial K) (Algebra.ofId K (BiAlgebra K m))
    (biCoord K m 0) (by simpa using bi_coord_power K m)

@[simp] theorem bi_forward_coordinate (m : ℕ) (i : Fin 2) :
    biForward K m (biCoord K m i) = thickRoot K m := by
  change biEvaluate K m (X i) = _
  simp [biEvaluate]

@[simp] theorem bi_backward_root (m : ℕ) :
    biBackward K m (thickRoot K m) = biCoord K m 0 := by
  simp [biBackward,thickRoot]

def biEquiv (m : ℕ) : BiAlgebra K m ≃ₐ[K] Thick K m :=
  AlgEquiv.ofAlgHom (biForward K m) (biBackward K m)
    (by
      apply AdjoinRoot.algHom_ext
      change biForward K m (biBackward K m (thickRoot K m)) = thickRoot K m
      rw [bi_backward_root,bi_forward_coordinate])
    (by
      apply Ideal.Quotient.algHom_ext K
      apply MvPolynomial.algHom_ext
      intro i
      change biBackward K m (biForward K m (biCoord K m i)) = biCoord K m i
      rw [bi_forward_coordinate,bi_backward_root]
      fin_cases i
      · rfl
      · exact (bi_coord_equal K m).symm)

instance biFinite (m : ℕ) : Module.Finite K (BiAlgebra K m) :=
  Module.Finite.of_basis
    (((AdjoinRoot.powerBasis' (Polynomial.monic_X.pow m :
      (Polynomial.X^m : Polynomial K).Monic)).basis).map (biEquiv K m).symm.toLinearEquiv)

theorem bi_dimension (m : ℕ) : Module.finrank K (BiAlgebra K m) = m := by
  rw [(biEquiv K m).toLinearEquiv.finrank_eq,thick_dimension]

theorem bi_scalar_length (m : ℕ) : Module.length K (BiAlgebra K m) = m := by
  rw [Module.length_eq_finrank,bi_dimension]

theorem bi_one_reduced : IsReduced (BiAlgebra K 1) :=
  isReduced_of_injective ((biEquiv K 1).trans thickOneEquiv).toRingHom
    ((biEquiv K 1).trans thickOneEquiv).injective

theorem bi_not_reduced (m : ℕ) (hm : 1 < m) : ¬ IsReduced (BiAlgebra K m) := by
  intro h
  let : IsReduced (BiAlgebra K m) := h
  have ht : IsReduced (Thick K m) :=
    isReduced_of_injective (biEquiv K m).symm.toRingHom (biEquiv K m).symm.injective
  exact thick_not_reduced m hm ht

theorem bi_previous_power_ne_zero (m : ℕ) (hm : 0 < m) :
    (biCoord K m 0)^(m-1) ≠ 0 := by
  intro h
  apply thick_previous_power_ne_zero K m hm
  have he := congrArg (biForward K m) h
  simpa only [map_pow,map_zero,bi_forward_coordinate] using he

variable {K} {L : Type*} [Field L] [Algebra K L]

/-- This identifies the exact ideal's solutions, not just its displayed generators. -/
theorem bi_solution_iff (m : ℕ) (x : Fin 2 → L) :
    (∀ p, p ∈ biIdeal K m → aeval x p = 0) ↔ x 0^m=0 ∧ x 1=x 0 := by
  constructor
  · intro h
    have h0 := h (X 0^m) (Ideal.subset_span (by simp))
    have h1 := h (X 1-X 0) (Ideal.subset_span (by simp))
    exact ⟨by simpa using h0,sub_eq_zero.mp (by simpa using h1)⟩
  · rintro ⟨h0,h1⟩
    have hsub : biIdeal K m ≤ RingHom.ker (aeval x).toRingHom := by
      rw [biIdeal,Ideal.span_le]
      intro p hp
      change aeval x p=0
      simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hp
      rcases hp with rfl | rfl
      · simpa using h0
      · simp [h1]
    exact fun p hp => hsub hp

theorem bi_solution_origin (m : ℕ) (x : Solution (biIdeal K m) L) : x.1=0 := by
  obtain ⟨h0,h1⟩ := (bi_solution_iff m x.1).mp x.2
  have hx0 : x.1 0=0 := eq_zero_of_pow_eq_zero h0
  funext i
  fin_cases i
  · exact hx0
  · exact h1.trans hx0

def biOrigin (m : ℕ) (hm : 0 < m) : Solution (biIdeal K m) L :=
  ⟨0,(bi_solution_iff m 0).mpr ⟨by simp [Nat.ne_of_gt hm],rfl⟩⟩

theorem bi_support_card (m : ℕ) (hm : 0 < m) :
    Nat.card (Solution (biIdeal K m) L) = 1 := by
  let : Unique (Solution (biIdeal K m) L) :=
    ⟨⟨biOrigin m hm⟩,fun x => Subtype.ext (bi_solution_origin m x)⟩
  exact Nat.card_unique

theorem bi_zero_control : Nat.card (Solution (biIdeal K 0) L) = 0 := by
  let : IsEmpty (Solution (biIdeal K 0) L) := ⟨fun x => by
    have h := ((bi_solution_iff 0 x.1).mp x.2).1
    simp at h⟩
  simp

theorem bi_arbitrary_gap (b : ℕ) :
    ∃ m : ℕ, 1 < m ∧ b < Module.finrank K (BiAlgebra K m) ∧
      Nat.card (Solution (biIdeal K m) L)=1 := by
  exact ⟨b+2,by omega,by rw [bi_dimension]; omega,bi_support_card (b+2) (by omega)⟩

end
end CoordinateLengthAudit
