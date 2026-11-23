import ListPermutation

namespace SemaevGeneralSplitAudit
open Polynomial ResultantInterchangeAudit SemaevReassociationAudit SemaevSymmetryAudit
open RecursiveSemaevDegreeAudit S4DegreeAudit
noncomputable section
variable {R : Type*} [CommRing R]

theorem pairPoly_swap (b2 b4 b6 b8 x y : R) :
    pairPoly b2 b4 b6 b8 x y=pairPoly b2 b4 b6 b8 y x := by
  unfold pairPoly qaRing qbRing qcRing
  congr 1 <;> ring

theorem s3Bi_specialize (b2 b4 b6 b8 z t : R) :
    (s3Bi b2 b4 b6 b8 z).map (evalRingHom t)=pairPoly b2 b4 b6 b8 z t := by
  simp [s3Bi,pairPoly,quad,qaRing,qbRing,qcRing,map_ofNat]

theorem lift_specialize_one (p : Polynomial R) (t : R) :
    (lift p).map (evalRingHom t)=p := by
  rw [lift,Polynomial.map_map]
  have h : (evalRingHom t).comp C=RingHom.id R := by ext a; simp
  rw [h,Polynomial.map_id]

theorem cubicConvolution_eval (D : ℕ) (p : Polynomial R) (b2 b4 b6 b8 z t : R) :
    (cubicConvolution D p b2 b4 b6 b8 z).eval t=
      Polynomial.resultant p (pairPoly b2 b4 b6 b8 z t) D 2 := by
  unfold cubicConvolution retainedY
  change (evalRingHom t) (Polynomial.resultant (lift p) (s3Bi b2 b4 b6 b8 z) D 2)=_
  rw [← Polynomial.resultant_map_map,lift_specialize_one,s3Bi_specialize]

variable {K S : Type*} [Field K] [CommRing S]

theorem towerCanonical_zero_source (W : WeierstrassCurve.Affine K) :
    (towerCanonical W 0 : Polynomial (Polynomial (Polynomial K)))=
      quad (pairA (K:=K)) (pairB W) (pairC W) := by
  unfold towerCanonical
  change (basePolynomial W).map (RingHom.id (InputRing K 0))=_
  rw [Polynomial.map_id]
  rfl

theorem sourceKernel_zero (W : WeierstrassCurve.Affine K) (f : K →+* S)
    (xs : Fin 2 → S) :
    sourceKernel W 0 f xs=pairPoly (f W.b₂) (f W.b₄) (f W.b₆) (f W.b₈) (xs 1) (xs 0) := by
  let α : Tower K 2 →+* S := towerEvaluate f 2 xs
  have hs (a : K) : α (liftPair a)=f a := towerEvaluate_scalar f 2 xs a
  have hx : α (pairX (K:=K))=xs 1 := by
    exact towerEvaluate_variable f 2 xs 1
  have hy : α (pairY (K:=K))=xs 0 := by
    exact towerEvaluate_variable f 2 xs 0
  unfold sourceKernel
  rw [towerCanonical_zero_source]
  change (quad (pairA (K:=K)) (pairB W) (pairC W)).map α=_
  rw [quad_map]
  simp only [pairA,pairB,pairC,map_add,map_sub,map_mul,map_pow,map_neg,map_ofNat,hs,hx,hy]
  rfl

#print axioms sourceKernel_zero
end
end SemaevGeneralSplitAudit
