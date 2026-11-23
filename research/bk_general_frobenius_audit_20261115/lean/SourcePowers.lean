import NineteenCubic

namespace BKGeneralFrobeniusAudit.SourceNineteen
noncomputable section
open FrobeniusPointDescentAudit Matrix Module
private instance : Fact (Nat.Prime 19) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩

theorem point_cube_sign (P : (curve.baseChange Closure).Point) (ht : (3 : ℕ) • P = 0) :
    (pointFrobenius curve)^[3] P = P ∨ (pointFrobenius curve)^[3] P = -P := by
  cases P with
  | zero => exact Or.inl rfl
  | some x y hN =>
    have hx := psi_root_frobenius x
      ((ThreeTorsionRankAudit.General.three_point_iff _ hN).mp ht)
    apply WeierstrassCurve.Affine.Point.xRep_eq_xRep_iff.mp
    change ![((x^19)^19)^19, (1 : Closure)] = ![x, 1]
    have hh : ((x^19)^19)^19 = x := by
      norm_num only [← pow_mul] at hx ⊢
      exact hx
    rw [hh]

theorem linear_cube_sign (P : Torsion (curve.baseChange Closure) 3) :
    ((linearFrobenius curve 3)^3) P = P ∨ ((linearFrobenius curve 3)^3) P = -P := by
  rcases point_cube_sign P.val P.property with h | h
  · left
    apply Subtype.ext
    exact (linear_power_val curve 3 3 P).trans h
  · right
    apply Subtype.ext
    exact (linear_power_val curve 3 3 P).trans h

/-- Finite F3 matrix rigidity; an actual nonzero fixed vector excludes -I. -/
theorem sign_matrix_rigidity : ∀ A : Matrix (Fin 2) (Fin 2) (ZMod 3),
    (∃ v : Fin 2 → ZMod 3, v ≠ 0 ∧ A *ᵥ v = v) →
    (∀ v : Fin 2 → ZMod 3, A *ᵥ v = v ∨ A *ᵥ v = -v) → A = 1 := by
  decide +kernel

def sourceBasis : Basis (Fin 2) (ZMod 3) (Torsion (curve.baseChange Closure) 3) :=
  BKFrobeniusBridgeAudit.actualGeometricBasis (L := Closure) curve curve_nonsingular

def cubeMatrix : Matrix (Fin 2) (Fin 2) (ZMod 3) :=
  LinearMap.toMatrix sourceBasis sourceBasis ((linearFrobenius curve 3)^3)

theorem cubeMatrix_coordinates (P : Torsion (curve.baseChange Closure) 3) :
    cubeMatrix *ᵥ sourceBasis.repr P = sourceBasis.repr (((linearFrobenius curve 3)^3) P) :=
  LinearMap.toMatrix_mulVec_repr sourceBasis sourceBasis _ P

theorem cubeMatrix_identity : cubeMatrix = 1 := by
  apply sign_matrix_rigidity
  · obtain ⟨P,hP,hfix⟩ := actual_prime_fixed_vector (L := Closure) curve 3 (by decide)
      (by rw [rational_cardinality]; decide)
    have hf3 : ((linearFrobenius curve 3)^3) P = P := by
      change linearFrobenius curve 3 (linearFrobenius curve 3 (linearFrobenius curve 3 P)) = P
      rw [hfix,hfix,hfix]
    refine ⟨sourceBasis.equivFun P, ?_, ?_⟩
    · intro hz
      apply hP
      exact sourceBasis.equivFun.injective (hz.trans (map_zero sourceBasis.equivFun).symm)
    · change cubeMatrix *ᵥ sourceBasis.repr P = sourceBasis.repr P
      rw [cubeMatrix_coordinates, hf3]
  · intro v
    let P := sourceBasis.equivFun.symm v
    have hv : (sourceBasis.repr P : Fin 2 → ZMod 3) = v :=
      sourceBasis.equivFun.apply_symm_apply v
    rw [← hv, cubeMatrix_coordinates]
    rcases linear_cube_sign P with h | h
    · left
      rw [h]
    · right
      rw [h, map_neg]
      rfl

theorem source_frobenius_cube : (linearFrobenius (L := Closure) curve 3)^3 = 1 := by
  apply (LinearMap.toMatrix sourceBasis sourceBasis).injective
  rw [LinearMap.toMatrix_one]
  exact cubeMatrix_identity

theorem source_frobenius_order : orderOf (linearFrobenius (L := Closure) curve 3) = 3 :=
  orderOf_eq_prime source_frobenius_cube source_frobenius_not_identity

theorem source_power_iff (k : ℕ) :
    (linearFrobenius (L := Closure) curve 3)^k = 1 ↔ 3 ∣ k := by
  simpa only [source_frobenius_order] using
    (orderOf_dvd_iff_pow_eq_one (x := linearFrobenius (L := Closure) curve 3) (n := k)).symm

theorem source_all_extensions {K : Type*} [Field K] [Fintype K] [Algebra (ZMod 19) K]
    [Algebra K Closure] [IsScalarTower (ZMod 19) K Closure] [DecidableEq K]
    (k : ℕ) (hc : Fintype.card K = 19^k) :
    (∀ P : (curve.baseChange Closure).Point, (3 : ℕ) • P = 0 →
      P ∈ Set.range (WeierstrassCurve.Affine.Point.map (W' := curve)
        (Algebra.ofId K Closure))) ↔ 3 ∣ k := by
  rw [actual_extension_iff_power_identity curve 3 k (by simpa only [ZMod.card] using hc)]
  exact source_power_iff k

#print axioms source_frobenius_cube
#print axioms source_frobenius_order
#print axioms source_all_extensions
end
end BKGeneralFrobeniusAudit.SourceNineteen
