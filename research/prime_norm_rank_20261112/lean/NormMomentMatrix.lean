module
public import NormPencilFunctional
public import Mathlib.RingTheory.Polynomial.DegreeLT
import all NormLift
import all NormDualPencil
public section
namespace NormMomentMatrix
open Polynomial Matrix
variable {K I : Type*} [Field K]

noncomputable def entryPolynomial (f : Polynomial K) (A B : I → Polynomial K)
    (i j : I) : Polynomial K := A i*A j-f*(B i*B j)

noncomputable def momentMatrix (mu : Polynomial K →ₗ[K] K)
    (f : Polynomial K) (A B : I → Polynomial K) : Matrix I I K :=
  fun i j => mu (entryPolynomial f A B i j)

noncomputable def residualFunctional (G : Polynomial K) (j : ℕ) :
    Polynomial K →ₗ[K] K where
  toFun P := P.coeff j-P.coeff G.natDegree*G.coeff j
  map_add' P Q := by simp only [coeff_add]; ring
  map_smul' c P := by simp only [coeff_smul,RingHom.id_apply,smul_eq_mul]; ring

noncomputable def coefficientMatrix (G f : Polynomial K)
    (A B : I → Polynomial K) (j : ℕ) : Matrix I I K :=
  momentMatrix (residualFunctional G j) f A B

/-- Symmetry follows from the actual entry polynomials. -/
theorem momentSymmetric (mu : Polynomial K →ₗ[K] K)
    (f : Polynomial K) (A B : I → Polynomial K) :
    (momentMatrix mu f A B).IsSymm := by
  ext i j
  change mu (A j*A i-f*(B j*B i))=mu (A i*A j-f*(B i*B j))
  congr 1
  ring

/-- The quadratic expression of the actual moment matrix is the functional of the norm. -/
theorem quadraticMoment [Fintype I] (mu : Polynomial K →ₗ[K] K)
    (f : Polynomial K) (A B : I → Polynomial K) (w : I → K) :
    NormDualPencil.quadratic (momentMatrix mu f A B) w=
      mu (NormLift.norm f A B w) := by
  rw [← NormLift.outerNorm]
  simp only [NormDualPencil.quadratic,momentMatrix,entryPolynomial,
    NormLift.lift,LinearMap.coe_mk,AddHom.coe_mk,map_sum,map_smul,smul_eq_mul]
  apply Finset.sum_congr rfl
  intro i _
  apply Finset.sum_congr rfl
  intro j _
  ring

/-- These are the actual leading-eliminated coefficient quadrics, not an assumed form map. -/
theorem quadraticCoefficient [Fintype I] (G f : Polynomial K) (A B : I → Polynomial K)
    (w : I → K) (j : ℕ) :
    NormDualPencil.quadratic (coefficientMatrix G f A B j) w=
      (NormLift.norm f A B w).coeff j-
        (NormLift.norm f A B w).coeff G.natDegree*G.coeff j := by
  rw [coefficientMatrix,quadraticMoment]
  simp [residualFunctional]

/-- Functional coefficient membership is exactly membership in the actual matrix pencil. -/
theorem matrixPencil (mu : Polynomial K →ₗ[K] K) (G f : Polynomial K)
    (A B : I → Polynomial K) (hg : G.Monic) (hzero : mu G=0)
    (hdeg : ∀ i j, (entryPolynomial f A B i j).natDegree≤G.natDegree) :
    momentMatrix mu f A B=
      ∑ j ∈ Finset.range G.natDegree, mu (X^j) • coefficientMatrix G f A B j := by
  ext i j
  simp only [Matrix.sum_apply,Matrix.smul_apply,coefficientMatrix,momentMatrix,
    residualFunctional,smul_eq_mul,LinearMap.coe_mk,AddHom.coe_mk]
  rw [NormPencilFunctional.coefficientPencil mu G (entryPolynomial f A B i j)
    hg (hdeg i j) hzero]
  apply Finset.sum_congr rfl
  intro k _
  ring

/-- Actual basis-family pole-degree bounds imply every matrix-entry degree bound. -/
theorem entryDegreeBound (f : Polynomial K) (A B : I → Polynomial K) (d : ℕ)
    (ha : ∀ i, 2*(A i).natDegree≤d)
    (hb : ∀ i, f.natDegree+2*(B i).natDegree≤d) (i j : I) :
    (entryPolynomial f A B i j).natDegree≤d := by
  have haa : (A i*A j).natDegree≤d := by
    have hmul : (A i*A j).natDegree≤(A i).natDegree+(A j).natDegree := natDegree_mul_le
    have hi := ha i
    have hj := ha j
    omega
  have hbb : (f*(B i*B j)).natDegree≤d := by
    have hmul : (f*(B i*B j)).natDegree≤f.natDegree+(B i*B j).natDegree := natDegree_mul_le
    have hpair : (B i*B j).natDegree≤(B i).natDegree+(B j).natDegree := natDegree_mul_le
    have hi := hb i
    have hj := hb j
    omega
  exact (natDegree_sub_le _ _).trans (max_le_iff.mpr ⟨haa,hbb⟩)

/-- The witness-derived moment matrix belongs to the actual coefficient pencil. -/
theorem actualMatrixPencil (F H f G U V : Polynomial K) (A B : I → Polynomial K)
    (alpha beta c : K) (hf : F.Monic) (hh : H.Monic) (hg : G.Monic)
    (hhp : 0<H.natDegree) (hgap : H.natDegree+1<F.natDegree)
    (hbez : U*F+V*(f*H)=1) (hc : c≠0)
    (hnorm : (C alpha*F)^2-f*(C beta*H)^2=C c*G)
    (ha : ∀ i, 2*(A i).natDegree≤G.natDegree)
    (hb : ∀ i, f.natDegree+2*(B i).natDegree≤G.natDegree) :
    momentMatrix (NormPencilFunctional.moment F V) f A B=
      ∑ j ∈ Finset.range G.natDegree,
        NormPencilFunctional.moment F V (X^j) • coefficientMatrix G f A B j := by
  exact matrixPencil _ G f A B hg
    (NormPencilFunctional.annihilatesTarget F H f G U V alpha beta c
      hf hh hhp hgap hbez hc hnorm) (entryDegreeBound f A B G.natDegree ha hb)

noncomputable def upperFamily (a b : ℕ) (i : Fin ((a+1)+(b+1))) : Polynomial K :=
  ((Polynomial.degreeLT.basisProd K (a+1) (b+1) i).1 : Polynomial K)
noncomputable def lowerFamily (a b : ℕ) (i : Fin ((a+1)+(b+1))) : Polynomial K :=
  ((Polynomial.degreeLT.basisProd K (a+1) (b+1) i).2 : Polynomial K)

theorem upperFamilyDegree (a b : ℕ) (i : Fin ((a+1)+(b+1))) :
    (upperFamily (K:=K) a b i).natDegree≤a := by
  have hd : (upperFamily (K:=K) a b i).degree<(a+1 : ℕ) :=
    mem_degreeLT.mp (Polynomial.degreeLT.basisProd K (a+1) (b+1) i).1.property
  by_cases hz : upperFamily (K:=K) a b i=0
  · simp [hz]
  · exact Nat.le_of_lt_succ ((natDegree_lt_iff_degree_lt hz).mpr hd)

theorem lowerFamilyDegree (a b : ℕ) (i : Fin ((a+1)+(b+1))) :
    (lowerFamily (K:=K) a b i).natDegree≤b := by
  have hd : (lowerFamily (K:=K) a b i).degree<(b+1 : ℕ) :=
    mem_degreeLT.mp (Polynomial.degreeLT.basisProd K (a+1) (b+1) i).2.property
  by_cases hz : lowerFamily (K:=K) a b i=0
  · simp [hz]
  · exact Nat.le_of_lt_succ ((natDegree_lt_iff_degree_lt hz).mpr hd)

/-- The defined coefficient families represent every admissible bounded polynomial pair. -/
theorem familiesComplete (a b : ℕ) (P Q : Polynomial K)
    (hp : P.natDegree≤a) (hq : Q.natDegree≤b) :
    ∃ w : Fin ((a+1)+(b+1)) → K,
      NormLift.assemble (upperFamily a b) w=P ∧
      NormLift.assemble (lowerFamily a b) w=Q := by
  have hP : P ∈ Polynomial.degreeLT K (a+1) :=
    mem_degreeLT.mpr ((degree_le_of_natDegree_le hp).trans_lt (by exact_mod_cast Nat.lt_succ_self a))
  have hQ : Q ∈ Polynomial.degreeLT K (b+1) :=
    mem_degreeLT.mpr ((degree_le_of_natDegree_le hq).trans_lt (by exact_mod_cast Nat.lt_succ_self b))
  let x : Polynomial.degreeLT K (a+1) × Polynomial.degreeLT K (b+1) :=
    (⟨P,hP⟩,⟨Q,hQ⟩)
  let basis := Polynomial.degreeLT.basisProd K (a+1) (b+1)
  have hs := basis.sum_repr x
  refine ⟨fun i => basis.repr x i, ?_, ?_⟩
  · have h := congrArg (fun z : Polynomial.degreeLT K (a+1) ×
        Polynomial.degreeLT K (b+1) => (z.1 : Polynomial K)) hs
    simpa only [Prod.fst_sum,Prod.smul_fst,Submodule.coe_sum,Submodule.coe_smul,
      NormLift.assemble,upperFamily,basis,x] using h
  · have h := congrArg (fun z : Polynomial.degreeLT K (a+1) ×
        Polynomial.degreeLT K (b+1) => (z.2 : Polynomial K)) hs
    simpa only [Prod.snd_sum,Prod.smul_snd,Submodule.coe_sum,Submodule.coe_smul,
      NormLift.assemble,lowerFamily,basis,x] using h

/-- The full even-degree monomial chart satisfies the actual matrix-pencil identity. -/
theorem canonicalMatrixPencil (F H f G U V : Polynomial K) (alpha beta c : K)
    (hf : F.Monic) (hh : H.Monic) (hg : G.Monic) (hhp : 0<H.natDegree)
    (hchart : F.natDegree=H.natDegree+2) (hG : G.natDegree=2*F.natDegree)
    (hcurve : f.natDegree≤3) (hbez : U*F+V*(f*H)=1) (hc : c≠0)
    (hnorm : (C alpha*F)^2-f*(C beta*H)^2=C c*G) :
    momentMatrix (NormPencilFunctional.moment F V) f
        (upperFamily F.natDegree H.natDegree) (lowerFamily F.natDegree H.natDegree)=
      ∑ j ∈ Finset.range G.natDegree,
        NormPencilFunctional.moment F V (X^j) •
          coefficientMatrix G f (upperFamily F.natDegree H.natDegree)
            (lowerFamily F.natDegree H.natDegree) j := by
  apply actualMatrixPencil F H f G U V _ _ alpha beta c hf hh hg hhp
    (by omega) hbez hc hnorm
  · intro i
    have h := upperFamilyDegree (K:=K) F.natDegree H.natDegree i
    omega
  · intro i
    have h := lowerFamilyDegree (K:=K) F.natDegree H.natDegree i
    omega
end NormMomentMatrix
#print axioms NormMomentMatrix.momentSymmetric
#print axioms NormMomentMatrix.quadraticMoment
#print axioms NormMomentMatrix.quadraticCoefficient
#print axioms NormMomentMatrix.matrixPencil
#print axioms NormMomentMatrix.entryDegreeBound
#print axioms NormMomentMatrix.actualMatrixPencil
#print axioms NormMomentMatrix.upperFamilyDegree
#print axioms NormMomentMatrix.lowerFamilyDegree
#print axioms NormMomentMatrix.familiesComplete
#print axioms NormMomentMatrix.canonicalMatrixPencil
