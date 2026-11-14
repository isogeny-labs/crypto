module
public import NormPencilFunctional
public section
namespace NormBranchRadical
open Polynomial
variable {K : Type*} [Field K]

/-- Universal algebraic identity underlying the branch projection. -/
theorem projectionIdentity {R : Type*} [CommRing R] (A B f Q T F s : R) :
    A*(Q-s*F)-f*(B*T)=(A*Q-f*(B*T))-F*(s*A) := by ring

/-- The actual quotient moment annihilates every multiple of its modulus. -/
theorem momentMultiple (F V R : Polynomial K) (hf : F.Monic) :
    NormPencilFunctional.moment F V (F*R)=0 := by
  rw [NormPencilFunctional.momentApply]
  rw [(modByMonic_eq_zero_iff_dvd hf).mpr ⟨R*V,by ring⟩,coeff_zero]

/-- Subtracting the explicit modulus direction enforces the target branch. -/
theorem projectedBranch (F Q T : Polynomial K) (q y : K) (hq : F.eval q≠0) :
    (Q-C ((Q.eval q-y*T.eval q)/F.eval q)*F).eval q-y*T.eval q=0 := by
  simp only [eval_sub,eval_mul,eval_C]
  rw [div_mul_cancel₀ _ hq]
  ring

/-- The projection preserves the actual quotient pairing, without dividing by two. -/
theorem projectedPairing (F V A B f Q T : Polynomial K) (s : K)
    (hf : F.Monic) :
    NormPencilFunctional.moment F V (A*(Q-C s*F)-f*(B*T))=
      NormPencilFunctional.moment F V (A*Q-f*(B*T)) := by
  rw [projectionIdentity,map_sub,momentMultiple F V (C s*A) hf,sub_zero]

/-- Restricting tests to the actual branch introduces no additional radical vectors. -/
theorem restrictedTestsDetectFull (F H V f A B : Polynomial K) (q y : K)
    (hf : F.Monic) (hq : F.eval q≠0) :
    (∀ Q T : Polynomial K, Q.natDegree≤F.natDegree → T.natDegree≤H.natDegree →
      Q.eval q-y*T.eval q=0 → NormPencilFunctional.moment F V (A*Q-f*(B*T))=0) ↔
    (∀ Q T : Polynomial K, Q.natDegree≤F.natDegree → T.natDegree≤H.natDegree →
      NormPencilFunctional.moment F V (A*Q-f*(B*T))=0) := by
  constructor
  · intro h Q T hQ hT
    let s := (Q.eval q-y*T.eval q)/F.eval q
    have hdegree : (Q-C s*F).natDegree≤F.natDegree :=
      (natDegree_sub_le _ _).trans (max_le hQ (natDegree_C_mul_le s F))
    have hb : (Q-C s*F).eval q-y*T.eval q=0 := projectedBranch F Q T q y hq
    have hz := h (Q-C s*F) T hdegree hT hb
    rwa [projectedPairing F V A B f Q T s hf] at hz
  · intro h Q T hQ hT _
    exact h Q T hQ hT

/-- Two branch equations give the witness scalar line under explicit nonzero guards. -/
theorem branchScalarLine (u v y alpha beta c d : K)
    (hu : u≠0) (ha : alpha≠0) (hw : alpha*u-y*beta*v=0)
    (hb : c*u-y*d*v=0) : ∃ t : K, c=t*alpha ∧ d=t*beta := by
  have hcross : alpha*d=beta*c := by
    apply (mul_right_cancel₀ hu)
    linear_combination d*hw-beta*hb
  refine ⟨c/alpha,by rw [div_mul_cancel₀ _ ha],?_⟩
  have heq : d=(c*beta)/alpha := (eq_div_iff ha).mpr (by linear_combination hcross)
  simpa only [div_mul_eq_mul_div] using heq

/-- On the actual target branch, the polynomial radical is exactly the witness line.
The witness and Bezout data are supplied; no search procedure is asserted. -/
theorem restrictedRadicalWitnessLine (F H f A B U V : Polynomial K)
    (q y alpha beta : K) (hf : F.Monic) (hh : H.Monic)
    (hhp : 0<H.natDegree) (hgap : H.natDegree+1<F.natDegree)
    (hbez : U*F+V*(f*H)=1) (hFq : F.eval q≠0) (halpha : alpha≠0)
    (hW : (C alpha*F).eval q-y*(C beta*H).eval q=0)
    (hA : A.eval q-y*B.eval q=0)
    (hadeg : A.natDegree≤F.natDegree) (hbdeg : B.natDegree≤H.natDegree) :
    (∀ Q T : Polynomial K, Q.natDegree≤F.natDegree → T.natDegree≤H.natDegree →
      Q.eval q-y*T.eval q=0 → NormPencilFunctional.moment F V (A*Q-f*(B*T))=0) ↔
    ∃ t : K, A=C (t*alpha)*F ∧ B=C (t*beta)*H := by
  rw [restrictedTestsDetectFull F H V f A B q y hf hFq]
  simp_rw [NormPencilFunctional.momentApply]
  rw [NormPairingRadical.normBlockRadical F H f A B U V hf hh hhp hgap hbez hadeg hbdeg]
  constructor
  · rintro ⟨c,d,hc,hd⟩
    have hb : c*F.eval q-y*d*H.eval q=0 := by
      simpa only [hc,hd,eval_mul,eval_C,mul_assoc] using hA
    have hw : alpha*F.eval q-y*beta*H.eval q=0 := by
      simpa only [eval_mul,eval_C,mul_assoc] using hW
    obtain ⟨t,hct,hdt⟩ := branchScalarLine (F.eval q) (H.eval q) y alpha beta c d hFq halpha hw hb
    exact ⟨t,by rw [hc,hct],by rw [hd,hdt]⟩
  · rintro ⟨t,htA,htB⟩
    exact ⟨t*alpha,t*beta,htA,htB⟩

/-- The chosen witness direction is nonzero, by actual evaluation at the branch. -/
theorem witnessNonzero (F : Polynomial K) (q alpha : K)
    (hFq : F.eval q≠0) (halpha : alpha≠0) : C alpha*F≠0 := by
  intro hz
  have h := congrArg (fun P : Polynomial K => P.eval q) hz
  simp only [eval_mul,eval_C,eval_zero] at h
  exact (mul_ne_zero halpha hFq) h

/-- A recovered scalar witness line has the actual scaled norm, not only zero pairings. -/
theorem normOfWitnessLine (F H f G A B : Polynomial K) (alpha beta c t : K)
    (hnorm : (C alpha*F)^2-f*(C beta*H)^2=C c*G)
    (hA : A=C (t*alpha)*F) (hB : B=C (t*beta)*H) :
    A^2-f*B^2=C (t^2*c)*G := by
  calc
    A^2-f*B^2=C (t^2)*((C alpha*F)^2-f*(C beta*H)^2) := by
      rw [hA,hB]
      simp only [map_mul,map_pow]
      ring
    _=C (t^2*c)*G := by rw [hnorm,map_mul]; ring
end NormBranchRadical
#print axioms NormBranchRadical.projectionIdentity
#print axioms NormBranchRadical.momentMultiple
#print axioms NormBranchRadical.projectedBranch
#print axioms NormBranchRadical.projectedPairing
#print axioms NormBranchRadical.restrictedTestsDetectFull
#print axioms NormBranchRadical.branchScalarLine
#print axioms NormBranchRadical.restrictedRadicalWitnessLine
#print axioms NormBranchRadical.witnessNonzero
#print axioms NormBranchRadical.normOfWitnessLine
