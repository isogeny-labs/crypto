import BinaryPrimitive
import ActualPolynomial

namespace BinarySemaevIrreducibilityAudit
open SemaevIrreducibilityAudit
noncomputable section
variable {K : Type*} [Field K]

def ordinaryMv (b : K) : MvPolynomial (Fin 3) K :=
  (MvPolynomial.X 0*MvPolynomial.X 1+MvPolynomial.X 0*MvPolynomial.X 2+
    MvPolynomial.X 1*MvPolynomial.X 2)^2+
    MvPolynomial.X 0*MvPolynomial.X 1*MvPolynomial.X 2+MvPolynomial.C b

theorem ordinaryMv_eval (b x y z : K) :
    MvPolynomial.eval ![x,y,z] (ordinaryMv b)=BinaryS3Audit.binaryS3 b x y z := by
  simp [ordinaryMv,BinaryS3Audit.binaryS3]

theorem ordinaryMv_map {L : Type*} [Field L] (f : K →+* L) (b : K) :
    MvPolynomial.map f (ordinaryMv b)=ordinaryMv (f b) := by simp [ordinaryMv]

theorem ordinaryMv_elementary (b : K) : ordinaryMv b=
    (MvPolynomial.esymm (Fin 3) K 2)^2+MvPolynomial.esymm (Fin 3) K 3+
      MvPolynomial.C b := by
  have h2 : (Finset.univ : Finset (Fin 3)).powersetCard 2={{0,1},{0,2},{1,2}} := by decide +kernel
  have h3 : (Finset.univ : Finset (Fin 3)).powersetCard 3={{0,1,2}} := by decide +kernel
  have h01_02 : ({0,1} : Finset (Fin 3))≠{0,2} := by decide +kernel
  have h01_12 : ({0,1} : Finset (Fin 3))≠{1,2} := by decide +kernel
  have h02_12 : ({0,2} : Finset (Fin 3))≠{1,2} := by decide +kernel
  simp only [MvPolynomial.esymm,h2,h3]
  simp [ordinaryMv,h01_02,h01_12,h02_12]
  ring

theorem ordinaryMv_symmetric (b : K) : (ordinaryMv b).IsSymmetric := by
  intro e
  simp only [ordinaryMv_elementary,map_add,map_pow,MvPolynomial.rename_esymm,
    MvPolynomial.rename_C]

variable [CharP K 2]

theorem ordinaryMv_as_literal (b : K) : threeVarEquiv K (ordinaryMv b)=binaryLiteral b := by
  simp only [ordinaryMv,map_add,map_mul,map_pow,threeVarEquiv_C,
    threeVarEquiv_X_zero,threeVarEquiv_X_one,threeVarEquiv_X_two]
  simp [binaryLiteral,binaryLiteralLeading,binaryLiteralMiddle,binaryLiteralTrailing,
    coefficientX,coefficientY,coefficientParameter]
  ring_nf
  all_goals simp only [CharTwo.two_eq_zero,mul_zero,add_zero]

theorem ordinaryMv_irreducible (b : K) : Irreducible (ordinaryMv b) := by
  apply (MulEquiv.irreducible_iff (threeVarEquiv K)).mp
  rw [ordinaryMv_as_literal]
  exact binaryLiteral_irreducible b

theorem ordinaryMv_irreducible_after_field_map {L : Type*} [Field L]
    (f : K →+* L) (b : K) : Irreducible (MvPolynomial.map f (ordinaryMv b)) := by
  have : CharP L 2 := (f.charP_iff_charP 2).mp inferInstance
  rw [ordinaryMv_map]
  exact ordinaryMv_irreducible (f b)

theorem actual_ordinary_binary_absolute_irreducibility_with_control :
    (∀ (K : Type) [Field K] [CharP K 2] (b : K),
      Irreducible (ordinaryMv b) ∧ (ordinaryMv b).IsSymmetric ∧
      (∀ x y z : K, MvPolynomial.eval ![x,y,z] (ordinaryMv b)=BinaryS3Audit.binaryS3 b x y z) ∧
      ∀ (L : Type) [Field L] (f : K →+* L),
        Irreducible (MvPolynomial.map f (ordinaryMv b))) ∧
    ¬ Irreducible (sourceMv (0:ZMod 2) 0) := by
  refine ⟨?_,sourceMv_binary_not_irreducible⟩
  intro K _ _ b
  exact ⟨ordinaryMv_irreducible b,ordinaryMv_symmetric b,ordinaryMv_eval b,
    fun _ _ f => ordinaryMv_irreducible_after_field_map f b⟩

#print axioms actual_ordinary_binary_absolute_irreducibility_with_control
end
end BinarySemaevIrreducibilityAudit
