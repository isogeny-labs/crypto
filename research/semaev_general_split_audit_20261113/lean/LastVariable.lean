import NaturalDegree

namespace SemaevGeneralSplitAudit
open Polynomial SemaevSymmetryAudit
noncomputable section
variable {K S : Type*} [Field K] [CommRing S]

/-- Literal partial evaluation of the actual source with its free variable last. -/
def sourceLastKernel (W : WeierstrassCurve.Affine K) (n : ℕ) (f : K →+* S)
    (xs : Fin (n+2) → S) : Polynomial S :=
  MvPolynomial.eval₂ (C.comp f) (Fin.snoc (fun i => C (xs i)) X) (canonicalMv W n)

theorem sourceKernel_as_mv (W : WeierstrassCurve.Affine K) (n : ℕ) (f : K →+* S)
    (xs : Fin (n+2) → S) :
    sourceKernel W n f xs=
      MvPolynomial.eval₂ (C.comp f) (Fin.cases X (fun i => C (xs i))) (canonicalMv W n) := by
  have h : (mapRingHom (towerEvaluate f (n+2) xs)).comp (towerEquiv K (n+3)).toRingHom=
      MvPolynomial.eval₂Hom (C.comp f) (Fin.cases X (fun i => C (xs i))) := by
    apply MvPolynomial.ringHom_ext
    · intro a
      change (towerEquiv K (n+3) (MvPolynomial.C a)).map (towerEvaluate f (n+2) xs)=_
      rw [towerEquiv_C,towerScalar_succ,map_C,towerEvaluate_scalar]
      simp only [MvPolynomial.eval₂Hom_C,RingHom.comp_apply]
    · intro i
      refine Fin.cases ?_ (fun j => ?_) i
      · change (towerEquiv K (n+3) (MvPolynomial.X 0)).map (towerEvaluate f (n+2) xs)=_
        rw [towerEquiv_X,towerVariable_zero,map_X]
        simp only [MvPolynomial.eval₂Hom_X',Fin.cases_zero]
      · change (towerEquiv K (n+3) (MvPolynomial.X j.succ)).map (towerEvaluate f (n+2) xs)=_
        rw [towerEquiv_X,towerVariable_succ,map_C,towerEvaluate_variable]
        simp only [MvPolynomial.eval₂Hom_X',Fin.cases_succ]
  have hh := congrArg (fun h : MvPolynomial (Fin (n+3)) K →+* Polynomial S =>
    h (canonicalMv W n)) h
  change (towerEquiv K (n+3) (canonicalMv W n)).map (towerEvaluate f (n+2) xs)=
    MvPolynomial.eval₂ (C.comp f) (Fin.cases X (fun i => C (xs i))) (canonicalMv W n) at hh
  rw [canonicalMv,RingEquiv.apply_symm_apply] at hh
  exact hh

theorem sourceLastKernel_eq (W : WeierstrassCurve.Affine K) (n : ℕ) (f : K →+* S)
    (xs : Fin (n+2) → S) : sourceLastKernel W n f xs=sourceKernel W n f xs := by
  have hp : (List.ofFn (Fin.cases (X : Polynomial S) (fun i => C (xs i)))).Perm
      (List.ofFn (Fin.snoc (fun i => C (xs i)) (X : Polynomial S))) := by
    have hc : List.ofFn (Fin.cases (X : Polynomial S) (fun i => C (xs i)))=
        X::List.ofFn (fun i => C (xs i)) := by
      simp only [List.ofFn_succ,Fin.cases_zero,Fin.cases_succ]
    have hs : List.ofFn (Fin.snoc (fun i => C (xs i)) (X : Polynomial S))=
        List.ofFn (fun i => C (xs i))++[ X ] := by
      rw [List.ofFn_succ']
      simp only [Fin.snoc_castSucc,Fin.snoc_last,List.concat_eq_append]
    rw [hc,hs]
    exact List.perm_append_comm (l₁:=[ X ]) (l₂:=List.ofFn (fun i => C (xs i)))
  have h := sourceValue_ofFn_perm W n (C.comp f)
    (Fin.cases X (fun i => C (xs i))) (Fin.snoc (fun i => C (xs i)) X) hp
  rw [sourceValue_as_mv,sourceValue_as_mv] at h
  exact h.symm.trans (sourceKernel_as_mv W n f xs).symm

def leftLastFactor (W : WeierstrassCurve.Affine K) (m n : ℕ) :
    Polynomial (MvPolynomial (Fin ((m+2)+(n+2))) K) :=
  sourceLastKernel W m MvPolynomial.C (leftVariables K m n)

def rightLastFactor (W : WeierstrassCurve.Affine K) (m n : ℕ) :
    Polynomial (MvPolynomial (Fin ((m+2)+(n+2))) K) :=
  sourceLastKernel W n MvPolynomial.C (rightVariables K m n)

theorem leftLastFactor_eq (W : WeierstrassCurve.Affine K) (m n : ℕ) :
    leftLastFactor W m n=leftFactor W m n := sourceLastKernel_eq ..

theorem rightLastFactor_eq (W : WeierstrassCurve.Affine K) (m n : ℕ) :
    rightLastFactor W m n=rightFactor W m n := sourceLastKernel_eq ..

/-- Original source ordering and actual natural dimensions, for every split. -/
theorem source_order_natural_split (W : WeierstrassCurve.Affine K) (m n : ℕ) :
    Polynomial.resultant (leftLastFactor W m n) (rightLastFactor W m n)
      (leftLastFactor W m n).natDegree (rightLastFactor W m n).natDegree=
      arityPolynomial W ((m+2)+(n+2)) (blockArityProof m n) := by
  rw [leftLastFactor_eq,rightLastFactor_eq,universal_natural_split]

#print axioms source_order_natural_split
end
end SemaevGeneralSplitAudit
