import PrimitiveBridge
import Mathlib.Algebra.MvPolynomial.Equiv
import Mathlib.RingTheory.MvPolynomial.Symmetric.Defs

namespace SemaevIrreducibilityAudit
noncomputable section
variable {K : Type*} [Field K]

def sourceMv (a b : K) : MvPolynomial (Fin 3) K :=
  (MvPolynomial.X 0-MvPolynomial.X 1)^2*MvPolynomial.X 2^2-
    2*((MvPolynomial.X 0+MvPolynomial.X 1)*
      (MvPolynomial.X 0*MvPolynomial.X 1+MvPolynomial.C a)+2*MvPolynomial.C b)*
      MvPolynomial.X 2+
    ((MvPolynomial.X 0*MvPolynomial.X 1-MvPolynomial.C a)^2-
      4*MvPolynomial.C b*(MvPolynomial.X 0+MvPolynomial.X 1))

def twoVarEquiv (K : Type*) [Field K] :
    MvPolynomial (Fin 2) K ≃ₐ[K] TwoVar K :=
  (MvPolynomial.finSuccEquiv K 1).trans
    (Polynomial.mapAlgEquiv (MvPolynomial.uniqueAlgEquiv K (Fin 1)))

def threeVarEquiv (K : Type*) [Field K] :
    MvPolynomial (Fin 3) K ≃ₐ[K] ThreeVar K :=
  (MvPolynomial.finSuccEquiv K 2).trans (Polynomial.mapAlgEquiv (twoVarEquiv K))

theorem twoVarEquiv_X_zero :
    twoVarEquiv K (MvPolynomial.X 0)=Polynomial.X := by
  simp [twoVarEquiv,MvPolynomial.finSuccEquiv_X_zero]

theorem twoVarEquiv_X_one :
    twoVarEquiv K (MvPolynomial.X 1)=Polynomial.C Polynomial.X := by
  change (Polynomial.mapAlgEquiv (MvPolynomial.uniqueAlgEquiv K (Fin 1)))
    ((MvPolynomial.finSuccEquiv K 1) (MvPolynomial.X (0 : Fin 1).succ))=_
  rw [MvPolynomial.finSuccEquiv_X_succ]
  simp [MvPolynomial.uniqueAlgEquiv]

theorem threeVarEquiv_C (a : K) :
    threeVarEquiv K (MvPolynomial.C a)=Polynomial.C (Polynomial.C (Polynomial.C a)) := by
  simp [threeVarEquiv,twoVarEquiv,MvPolynomial.finSuccEquiv_apply,
    MvPolynomial.uniqueAlgEquiv]

theorem threeVarEquiv_X_zero :
    threeVarEquiv K (MvPolynomial.X 0)=Polynomial.X := by
  simp [threeVarEquiv,twoVarEquiv,MvPolynomial.finSuccEquiv_apply]

theorem threeVarEquiv_X_one :
    threeVarEquiv K (MvPolynomial.X 1)=Polynomial.C Polynomial.X := by
  change (Polynomial.mapAlgEquiv (twoVarEquiv K))
    ((MvPolynomial.finSuccEquiv K 2) (MvPolynomial.X (0 : Fin 2).succ))=_
  rw [MvPolynomial.finSuccEquiv_X_succ]
  simpa using congrArg Polynomial.C (twoVarEquiv_X_zero (K:=K))

theorem threeVarEquiv_X_two :
    threeVarEquiv K (MvPolynomial.X 2)=Polynomial.C (Polynomial.C Polynomial.X) := by
  change (Polynomial.mapAlgEquiv (twoVarEquiv K))
    ((MvPolynomial.finSuccEquiv K 2) (MvPolynomial.X (1 : Fin 2).succ))=_
  rw [MvPolynomial.finSuccEquiv_X_succ]
  simpa using congrArg Polynomial.C (twoVarEquiv_X_one (K:=K))

theorem sourceMv_as_literal (a b : K) :
    threeVarEquiv K (sourceMv a b)=literalSource a b := by
  simp only [sourceMv,map_add,map_sub,map_mul,map_pow,map_ofNat,
    threeVarEquiv_C,threeVarEquiv_X_zero,threeVarEquiv_X_one,threeVarEquiv_X_two]
  simp [literalSource,literalLeading,literalMiddle,literalTrailing,
    coefficientX,coefficientY,coefficientParameter]
  norm_num only [map_ofNat]
  ring

theorem sourceMv_eval (a b x y z : K) :
    MvPolynomial.eval ![x,y,z] (sourceMv a b)=SemaevS3.S3 a b x y z := by
  simp [sourceMv,SemaevS3.S3]

theorem sourceMv_elementary_symmetric (a b : K) :
    sourceMv a b=(MvPolynomial.esymm (Fin 3) K 2)^2-
      4*MvPolynomial.esymm (Fin 3) K 1*MvPolynomial.esymm (Fin 3) K 3-
      2*MvPolynomial.C a*MvPolynomial.esymm (Fin 3) K 2-
      4*MvPolynomial.C b*MvPolynomial.esymm (Fin 3) K 1+(MvPolynomial.C a)^2 := by
  have h1 : (Finset.univ : Finset (Fin 3)).powersetCard 1={{0},{1},{2}} := by decide +kernel
  have h2 : (Finset.univ : Finset (Fin 3)).powersetCard 2={{0,1},{0,2},{1,2}} := by decide +kernel
  have h3 : (Finset.univ : Finset (Fin 3)).powersetCard 3={{0,1,2}} := by decide +kernel
  have h01_02 : ({0,1} : Finset (Fin 3))≠{0,2} := by decide +kernel
  have h01_12 : ({0,1} : Finset (Fin 3))≠{1,2} := by decide +kernel
  have h02_12 : ({0,2} : Finset (Fin 3))≠{1,2} := by decide +kernel
  simp only [MvPolynomial.esymm,h1,h2,h3]
  simp [sourceMv,h01_02,h01_12,h02_12]
  ring

theorem sourceMv_isSymmetric (a b : K) : (sourceMv a b).IsSymmetric := by
  intro e
  simp only [sourceMv_elementary_symmetric,map_add,map_sub,map_mul,map_pow,map_ofNat,
    MvPolynomial.rename_esymm,MvPolynomial.rename_C]

theorem sourceMv_irreducible (a b : K) (h2 : (2:K)≠0) :
    Irreducible (sourceMv a b) := by
  apply (MulEquiv.irreducible_iff (threeVarEquiv K)).mp
  rw [sourceMv_as_literal]
  exact actual_nested_source_irreducible a b h2

theorem sourceMv_map {L : Type*} [Field L] (f : K →+* L) (a b : K) :
    MvPolynomial.map f (sourceMv a b)=sourceMv (f a) (f b) := by
  simp [sourceMv]

/-- The actual S3 polynomial stays irreducible after every field scalar
extension; this includes the algebraic closure. -/
theorem sourceMv_irreducible_after_field_map {L : Type*} [Field L]
    (f : K →+* L) (a b : K) (h2 : (2:K)≠0) :
    Irreducible (MvPolynomial.map f (sourceMv a b)) := by
  rw [sourceMv_map]
  have hh : f (2:K)≠0 := by simpa using h2
  norm_num only [map_ofNat] at hh
  exact sourceMv_irreducible (f a) (f b) hh

private instance prime2 : Fact (Nat.Prime 2) := ⟨by decide⟩
abbrev BinaryPoly := MvPolynomial (Fin 3) (ZMod 2)
def binaryFactor : BinaryPoly :=
  (MvPolynomial.X 0+MvPolynomial.X 1)*MvPolynomial.X 2+
    MvPolynomial.X 0*MvPolynomial.X 1

theorem sourceMv_binary_square : sourceMv (0:ZMod 2) 0=binaryFactor^2 := by
  have : CharP BinaryPoly 2 := inferInstance
  unfold sourceMv binaryFactor
  simp only [map_zero]
  ring_nf
  all_goals reduce_mod_char!

theorem binaryFactor_not_isUnit : ¬ IsUnit binaryFactor := by
  intro hu
  have hv := hu.map (MvPolynomial.eval (0 : Fin 3 → ZMod 2))
  have hz : MvPolynomial.eval (0 : Fin 3 → ZMod 2) binaryFactor=0 := by
    simp [binaryFactor]
  rw [hz] at hv
  exact not_isUnit_zero hv

theorem sourceMv_binary_not_irreducible : ¬ Irreducible (sourceMv (0:ZMod 2) 0) := by
  rw [sourceMv_binary_square]
  intro hir
  have hh := hir.isUnit_or_isUnit (show binaryFactor^2=binaryFactor*binaryFactor by ring)
  exact hh.elim binaryFactor_not_isUnit binaryFactor_not_isUnit

theorem actual_S3_absolute_irreducibility_with_control :
    (∀ (K : Type) [Field K] (a b : K), (2:K)≠0 →
      Irreducible (sourceMv a b) ∧ (sourceMv a b).IsSymmetric ∧
      (∀ x y z : K, MvPolynomial.eval ![x,y,z] (sourceMv a b)=SemaevS3.S3 a b x y z) ∧
      ∀ (L : Type) [Field L] (f : K →+* L),
        Irreducible (MvPolynomial.map f (sourceMv a b))) ∧
    ¬ Irreducible (sourceMv (0:ZMod 2) 0) := by
  refine ⟨?_,sourceMv_binary_not_irreducible⟩
  intro K _ a b h2
  refine ⟨sourceMv_irreducible a b h2,sourceMv_isSymmetric a b,sourceMv_eval a b,?_⟩
  intro L _ f
  exact sourceMv_irreducible_after_field_map f a b h2

#print axioms actual_S3_absolute_irreducibility_with_control

end
end SemaevIrreducibilityAudit
