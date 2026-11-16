import DomainControl

namespace CoordinateLengthAudit
noncomputable section

/-- Refute support=dimension universally, using the actual bivariate ideal. -/
theorem not_support_equals_dimension :
    ¬ (∀ (K L σ : Type) [Field K] [Field L] [Algebra K L]
      (I : Ideal (MvPolynomial σ K)) [Module.Finite K (CoordinateAlgebra I)],
      Nat.card (Solution I L) = Module.finrank K (CoordinateAlgebra I)) := by
  intro h
  have he := h (ZMod 3) (ZMod 3) (Fin 2) (biIdeal (ZMod 3) 2)
  rw [bi_support_card 2 (by omega),bi_dimension] at he
  omega

/-- Actual affine coordinate-algebra accounting, unbounded nonreduced family,
and precise finite/target-domain controls; not a projective Bezout theorem. -/
theorem actual_coordinate_length_certificate :
    (∀ (K L σ : Type) [Field K] [Field L] [Algebra K L]
      (I : Ideal (MvPolynomial σ K)),
      Nonempty (Solution I L ≃ (CoordinateAlgebra I →ₐ[K] L))) ∧
    (∀ (K L σ : Type) [Field K] [Field L] [Algebra K L]
      (I : Ideal (MvPolynomial σ K)) [Module.Finite K (CoordinateAlgebra I)],
      Finite (Solution I L) ∧
      Nat.card (Solution I L) ≤ Module.finrank K (CoordinateAlgebra I) ∧
      Module.length K (CoordinateAlgebra I) = Module.finrank K (CoordinateAlgebra I) ∧
      (Nat.card (Solution I L) : ℕ∞) ≤ Module.length K (CoordinateAlgebra I)) ∧
    (∀ (K : Type) [Field K] (m : ℕ),
      Nonempty (BiAlgebra K m ≃ₐ[K] Thick K m) ∧
      Module.finrank K (BiAlgebra K m) = m ∧ Module.length K (BiAlgebra K m) = m ∧
      (biCoord K m 0)^m=0 ∧ biCoord K m 1=biCoord K m 0) ∧
    (∀ (K L : Type) [Field K] [Field L] [Algebra K L] (m : ℕ), 0 < m →
      Nat.card (Solution (biIdeal K m) L)=1 ∧ (biCoord K m 0)^(m-1)≠0 ∧
      (1<m → ¬ IsReduced (BiAlgebra K m))) ∧
    (∀ (K L : Type) [Field K] [Field L] [Algebra K L],
      Nat.card (Solution (biIdeal K 0) L)=0 ∧ Module.finrank K (BiAlgebra K 0)=0 ∧
      Module.length K (BiAlgebra K 0)=0 ∧ Nat.card (Solution (biIdeal K 1) L)=1 ∧
      Module.finrank K (BiAlgebra K 1)=1 ∧ IsReduced (BiAlgebra K 1)) ∧
    (∀ (K L : Type) [Field K] [Field L] [Algebra K L] (b : ℕ),
      ∃ m : ℕ, 1<m ∧ b < Module.finrank K (BiAlgebra K m) ∧
        Nat.card (Solution (biIdeal K m) L)=1 ∧
        ¬ (Nat.card (Solution (biIdeal K m) L)=Module.finrank K (BiAlgebra K m))) ∧
    (¬ (∀ (K L σ : Type) [Field K] [Field L] [Algebra K L]
      (I : Ideal (MvPolynomial σ K)) [Module.Finite K (CoordinateAlgebra I)],
      Nat.card (Solution I L)=Module.finrank K (CoordinateAlgebra I))) ∧
    (Module.finrank (ZMod 3) DualThree=2 ∧
      3 ≤ Nat.card (DualThree →ₐ[ZMod 3] DualThree) ∧
      ¬ (Nat.card (DualThree →ₐ[ZMod 3] DualThree) ≤ Module.finrank (ZMod 3) DualThree) ∧
      ¬ IsReduced DualThree) := by
  refine ⟨?_,?_,?_,?_,?_,?_,not_support_equals_dimension,target_domain_counterexample⟩
  · intro K L σ _ _ _ I
    exact ⟨solutionEquivHom I⟩
  · intro K L σ _ _ _ I _
    exact ⟨solutions_finite I,support_le_dimension I,Module.length_eq_finrank _ _,
      support_le_scalar_length I⟩
  · intro K _ m
    exact ⟨⟨biEquiv K m⟩,bi_dimension K m,bi_scalar_length K m,bi_coord_power K m,bi_coord_equal K m⟩
  · intro K L _ _ _ m hm
    exact ⟨bi_support_card m hm,bi_previous_power_ne_zero K m hm,bi_not_reduced K m⟩
  · intro K L _ _ _
    exact ⟨bi_zero_control,bi_dimension K 0,bi_scalar_length K 0,
      bi_support_card 1 (by omega),bi_dimension K 1,bi_one_reduced K⟩
  · intro K L _ _ _ b
    obtain ⟨m,hm,hb,hc⟩ := bi_arbitrary_gap (K := K) (L := L) b
    refine ⟨m,hm,hb,hc,?_⟩
    rw [hc,bi_dimension]
    omega

#print axioms actual_coordinate_length_certificate
end
end CoordinateLengthAudit
