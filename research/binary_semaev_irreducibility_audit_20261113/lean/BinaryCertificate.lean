import BinaryActualPolynomial

namespace BinarySemaevIrreducibilityAudit
open SemaevIrreducibilityAudit
noncomputable section

theorem ordinary_not_short : ordinaryMv (0:ZMod 2)≠sourceMv (0:ZMod 2) 0 := by
  intro h
  have he := congrArg (MvPolynomial.eval (![1,1,1] : Fin 3 → ZMod 2)) h
  norm_num [ordinaryMv,sourceMv] at he
  exact (by decide +kernel : (4:ZMod 2)≠ -3) he

/-- Literal polynomial, arbitrary scalar extension, actual curve/group semantics,
and a direct negative control distinguishing the two characteristic-two models. -/
theorem actual_binary_S3_certificate :
    (∀ (K : Type) [Field K] [CharP K 2] (b : K),
      Irreducible (ordinaryMv b) ∧ (ordinaryMv b).IsSymmetric ∧
      (∀ x y z : K, MvPolynomial.eval ![x,y,z] (ordinaryMv b)=BinaryS3Audit.binaryS3 b x y z) ∧
      ∀ (L : Type) [Field L] (f : K →+* L),
        Irreducible (MvPolynomial.map f (ordinaryMv b))) ∧
    (∀ (K : Type) [Field K] [CharP K 2] [IsAlgClosed K] [DecidableEq K]
      (W : WeierstrassCurve.Affine K), W.Δ≠0 → W.a₁=1 → W.a₃=0 → W.a₄=0 →
      ∀ x y z : K, MvPolynomial.eval ![x,y,z] (ordinaryMv W.a₆)=0 ↔
        ∃ P Q R : W.Point, SemaevS3.pointX P=some x ∧ SemaevS3.pointX Q=some y ∧
          SemaevS3.pointX R=some z ∧ P+Q+R=0) ∧
    sourceMv (0:ZMod 2) 0=binaryFactor^2 ∧ ¬ IsUnit binaryFactor ∧
    ¬ Irreducible (sourceMv (0:ZMod 2) 0) ∧
    ordinaryMv (0:ZMod 2)≠sourceMv (0:ZMod 2) 0 := by
  refine ⟨actual_ordinary_binary_absolute_irreducibility_with_control.1,?_,
    sourceMv_binary_square,binaryFactor_not_isUnit,sourceMv_binary_not_irreducible,
    ordinary_not_short⟩
  intro K _ _ _ _ W hΔ ha1 ha3 ha4 x y z
  rw [ordinaryMv_eval]
  exact BinaryS3Audit.binaryS3_zero_iff_exists_points W hΔ ha1 ha3 ha4 x y z

#print axioms actual_binary_S3_certificate
end
end BinarySemaevIrreducibilityAudit
