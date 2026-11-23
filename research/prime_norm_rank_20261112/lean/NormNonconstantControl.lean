module
public import NormMomentMatrix
public import Mathlib.Tactic.ComputeDegree
public section
namespace NormNonconstantControl
open Polynomial Matrix

/-- Product of two line functions gives a nonconstant lower norm factor. -/
theorem twoLineNorm {R : Type*} [CommRing R] (f L M : Polynomial R) :
    (f+L*M)^2-f*(L+M)^2=(f-L^2)*(f-M^2) := by ring

local instance : Fact (Nat.Prime 17) := ⟨by decide⟩
noncomputable def curve : Polynomial (ZMod 17) := X^3+C 2*X+C 2
noncomputable def line1 : Polynomial (ZMod 17) := C 2*X+C 8
noncomputable def line2 : Polynomial (ZMod 17) := C 4*X+C 6
noncomputable def normA : Polynomial (ZMod 17) := X^3+C 8*X^2+C 12*X+C 16
noncomputable def normB : Polynomial (ZMod 17) := C 6*X+C 14
noncomputable def lowerMonic : Polynomial (ZMod 17) := X+C 8
noncomputable def target : Polynomial (ZMod 17) :=
  (X-C 5)*(X-C 6)*(X-C 10)*X*(X-C 3)*(X-C 13)
noncomputable def bezoutU : Polynomial (ZMod 17) := C 16*X^3+C 11*X^2+C 6*X+C 12
noncomputable def inverseV : Polynomial (ZMod 17) := X^2+C 6*X+C 4

theorem construction : normA=curve+line1*line2 ∧ normB=line1+line2 := by
  constructor
  all_goals simp only [normA,normB,curve,line1,line2,map_ofNat]
  all_goals ring_nf
  all_goals reduce_mod_char

theorem lineFactors :
    curve-line1^2=(X-C 5)*(X-C 6)*(X-C 10) ∧
    curve-line2^2=X*(X-C 3)*(X-C 13) := by
  constructor
  all_goals simp only [curve,line1,line2,map_ofNat]
  all_goals ring_nf
  all_goals reduce_mod_char
  all_goals ring_nf
  all_goals reduce_mod_char

theorem actualNorm : normA^2-curve*normB^2=target := by
  obtain ⟨ha,hb⟩ := construction
  obtain ⟨hl,hm⟩ := lineFactors
  rw [ha,hb,twoLineNorm,hl,hm]
  simp only [target]
  ring

theorem lowerScale : normB=C 6*lowerMonic := by
  simp only [normB,lowerMonic,map_ofNat]
  ring_nf
  reduce_mod_char

theorem bezoutIdentity : bezoutU*normA+inverseV*(curve*lowerMonic)=1 := by
  simp only [bezoutU,normA,inverseV,curve,lowerMonic,map_ofNat]
  ring_nf
  reduce_mod_char

theorem targetBranch : normA.eval 0-6*normB.eval 0=0 := by
  simp only [normA,normB,eval_add,eval_mul,eval_pow,eval_X,eval_C]
  decide

theorem branchNonzero : normA.eval 0≠0 := by
  simp only [normA,eval_add,eval_mul,eval_pow,eval_X,eval_C]
  decide

theorem normAMonic : normA.Monic := by unfold normA; monicity <;> norm_num
theorem lowerMonicMonic : lowerMonic.Monic := by unfold lowerMonic; monicity
theorem targetMonic : target.Monic := by unfold target; monicity <;> norm_num
theorem normADegree : normA.natDegree=3 := by unfold normA; compute_degree <;> norm_num
theorem lowerDegree : lowerMonic.natDegree=1 := by unfold lowerMonic; compute_degree; norm_num
theorem curveDegree : curve.natDegree=3 := by unfold curve; compute_degree <;> norm_num
theorem targetDegree : target.natDegree=6 := by unfold target; compute_degree <;> norm_num
theorem normBDegree : normB.natDegree=1 := by unfold normB; compute_degree; decide

theorem nodesDistinct : ([0,3,5,6,10,13] : List (ZMod 17)).Nodup := by decide

def supportPoints : Fin 6 → ZMod 17 × ZMod 17 :=
  ![(5,1),(6,3),(10,11),(0,6),(3,1),(13,7)]

theorem supportPointsOnCurve : ∀ i : Fin 6,
    (supportPoints i).2^2=curve.eval (supportPoints i).1 := by
  intro i
  fin_cases i <;> simp only [curve,eval_add,eval_mul,eval_pow,eval_X,eval_C] <;> decide

theorem actualScaledNorm : (C 1*normA)^2-curve*(C 6*lowerMonic)^2=C 1*target := by
  simpa only [map_one,one_mul,← lowerScale] using actualNorm

theorem targetMomentZero : NormPencilFunctional.moment normA inverseV target=0 := by
  exact NormPencilFunctional.annihilatesTarget normA lowerMonic curve target
    bezoutU inverseV 1 6 1 normAMonic lowerMonicMonic
    (by rw [lowerDegree]; decide) (by rw [normADegree,lowerDegree]; decide)
    bezoutIdentity (by decide) actualScaledNorm

theorem coordinateDimension : (normA.natDegree+1)+(lowerMonic.natDegree+1)=6 := by
  norm_num [normADegree,lowerDegree]

/-- Actual full six-dimensional chart; no numeric matrix rank is assumed. -/
theorem actualCanonicalPencil :
    NormMomentMatrix.momentMatrix (NormPencilFunctional.moment normA inverseV) curve
        (NormMomentMatrix.upperFamily normA.natDegree lowerMonic.natDegree)
        (NormMomentMatrix.lowerFamily normA.natDegree lowerMonic.natDegree)=
      ∑ j ∈ Finset.range target.natDegree,
        NormPencilFunctional.moment normA inverseV (X^j) •
          NormMomentMatrix.coefficientMatrix target curve
            (NormMomentMatrix.upperFamily normA.natDegree lowerMonic.natDegree)
            (NormMomentMatrix.lowerFamily normA.natDegree lowerMonic.natDegree) j := by
  exact NormMomentMatrix.canonicalMatrixPencil normA lowerMonic curve target
    bezoutU inverseV 1 6 1 normAMonic lowerMonicMonic targetMonic
    (by norm_num [lowerDegree]) (by norm_num [normADegree,lowerDegree])
    (by norm_num [targetDegree,normADegree]) (by norm_num [curveDegree]) bezoutIdentity (by decide)
    actualScaledNorm
end NormNonconstantControl
#print axioms NormNonconstantControl.twoLineNorm
#print axioms NormNonconstantControl.construction
#print axioms NormNonconstantControl.lineFactors
#print axioms NormNonconstantControl.actualNorm
#print axioms NormNonconstantControl.lowerScale
#print axioms NormNonconstantControl.bezoutIdentity
#print axioms NormNonconstantControl.targetBranch
#print axioms NormNonconstantControl.branchNonzero
#print axioms NormNonconstantControl.targetMomentZero
#print axioms NormNonconstantControl.actualCanonicalPencil
#print axioms NormNonconstantControl.coordinateDimension
#print axioms NormNonconstantControl.normBDegree
#print axioms NormNonconstantControl.nodesDistinct
#print axioms NormNonconstantControl.supportPointsOnCurve
