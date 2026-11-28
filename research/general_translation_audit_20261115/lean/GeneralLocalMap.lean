module
public import GeneralExtension

@[expose] public section
namespace GeneralTranslationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit MillerFinitePointAudit MillerBasechangeAudit
open FunctionDivisorFinitenessAudit FinitePointEvaluationAudit
variable {K L : Type*} [Field K] [Field L] [Algebra K L]

local instance pointMonoid (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Equation b c) : Monoid (PointLocalRing W b c h) :=
  (inferInstance : CommRing (PointLocalRing W b c h)).toMonoid

/-- Actual coordinate evaluation commutes with coefficient-field extension. -/
theorem point_eval_basechange (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Nonsingular b c) (a : W.CoordinateRing) :
    pointEval (extendedCurve (L := L) W) (algebraMap K L b) (algebraMap K L c)
      (base_nonsingular W h).1 (CoordinateRing.map W (algebraMap K L) a)=
      algebraMap K L (pointEval W b c h.1 a) := by
  have hy : pointEval W b c h.1 (mk W X)=c := by
    simpa only [YClass,map_sub,map_zero,sub_zero,eval_zero] using
      point_eval_YClass W h.1 (0 : K[X])
  have hy' : pointEval (extendedCurve (L := L) W) (algebraMap K L b) (algebraMap K L c)
      (base_nonsingular W h).1 (mk (extendedCurve (L := L) W) X)=algebraMap K L c := by
    simpa only [YClass,map_sub,map_zero,sub_zero,eval_zero] using
      point_eval_YClass (extendedCurve (L := L) W) (base_nonsingular W h).1 (0 : L[X])
  obtain ⟨p,q,rfl⟩ := exists_smul_basis_eq a
  rw [map_add,CoordinateRing.map_smul,CoordinateRing.map_smul,map_one,
    CoordinateRing.map_mk,Polynomial.map_X]
  simp only [smul,map_add,map_mul,mul_one,point_eval_polynomial,hy,hy',Polynomial.eval_map_apply]

def coordinateLocalBasechange (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Nonsingular b c) : W.CoordinateRing →+*
      PointLocalRing (extendedCurve (L := L) W) (algebraMap K L b) (algebraMap K L c)
        (base_nonsingular W h).1 :=
  (algebraMap (extendedCurve (L := L) W).CoordinateRing
    (PointLocalRing (extendedCurve (L := L) W) (algebraMap K L b) (algebraMap K L c)
      (base_nonsingular W h).1)).comp (CoordinateRing.map W (algebraMap K L))

theorem local_basechange_denominator (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Nonsingular b c) (s : (evaluationIdeal W b c h.1).primeCompl) :
    IsUnit (coordinateLocalBasechange (L := L) W h s) := by
  apply (local_point_unit_iff (extendedCurve (L := L) W) (base_nonsingular W h).1 _).mpr
  change localPointEval (extendedCurve (L := L) W) (base_nonsingular W h).1
    (algebraMap (extendedCurve (L := L) W).CoordinateRing _
      (CoordinateRing.map W (algebraMap K L) s))≠0
  rw [local_point_map,point_eval_basechange W h]
  intro hz
  have hs : pointEval W b c h.1 s≠0 := s.property
  exact hs ((algebraMap K L).injective (hz.trans (map_zero _).symm))

/-- Actual map between actual finite-point localizations, not a supplied local-ring certificate. -/
def pointLocalBasechange (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Nonsingular b c) : PointLocalRing W b c h.1 →+*
      PointLocalRing (extendedCurve (L := L) W) (algebraMap K L b) (algebraMap K L c)
        (base_nonsingular W h).1 := IsLocalization.lift (local_basechange_denominator W h)

theorem point_local_basechange_map (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Nonsingular b c) (a : W.CoordinateRing) :
    pointLocalBasechange (L := L) W h (algebraMap W.CoordinateRing (PointLocalRing W b c h.1) a)=
      coordinateLocalBasechange (L := L) W h a := IsLocalization.lift_eq _ a

theorem point_local_basechange_field (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Nonsingular b c) (a : PointLocalRing W b c h.1) :
    pointToField (extendedCurve (L := L) W) (base_nonsingular W h).1
      (pointLocalBasechange (L := L) W h a)=
      fieldBaseChange (L := L) W (pointToField W h.1 a) := by
  have he : (pointToField (extendedCurve (L := L) W) (base_nonsingular W h).1).comp
      (pointLocalBasechange (L := L) W h)=
      (fieldBaseChange (L := L) W).comp (pointToField W h.1) := by
    apply IsLocalization.ringHom_ext (evaluationIdeal W b c h.1).primeCompl
    apply RingHom.ext
    intro d
    change pointToField (extendedCurve (L := L) W) (base_nonsingular W h).1
      (pointLocalBasechange (L := L) W h
        (algebraMap W.CoordinateRing (PointLocalRing W b c h.1) d))=
      fieldBaseChange (L := L) W
        (pointToField W h.1 (algebraMap W.CoordinateRing (PointLocalRing W b c h.1) d))
    rw [point_local_basechange_map,point_to_field_map]
    change pointToField (extendedCurve (L := L) W) (base_nonsingular W h).1
      (algebraMap (extendedCurve (L := L) W).CoordinateRing _
        (CoordinateRing.map W (algebraMap K L) d))=fieldBaseChange (L := L) W
          (algebraMap W.CoordinateRing W.FunctionField d)
    rw [point_to_field_map,field_base_map]
  exact RingHom.congr_fun he a

theorem point_local_basechange_residue (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Nonsingular b c) (a : PointLocalRing W b c h.1) :
    localPointEval (extendedCurve (L := L) W) (base_nonsingular W h).1
      (pointLocalBasechange (L := L) W h a)=algebraMap K L (localPointEval W h.1 a) := by
  have he : (localPointEval (extendedCurve (L := L) W) (base_nonsingular W h).1).comp
      (pointLocalBasechange (L := L) W h)=
      (algebraMap K L).comp (localPointEval W h.1) := by
    apply IsLocalization.ringHom_ext (evaluationIdeal W b c h.1).primeCompl
    apply RingHom.ext
    intro d
    change localPointEval (extendedCurve (L := L) W) (base_nonsingular W h).1
      (pointLocalBasechange (L := L) W h
        (algebraMap W.CoordinateRing (PointLocalRing W b c h.1) d))=
      algebraMap K L (localPointEval W h.1
        (algebraMap W.CoordinateRing (PointLocalRing W b c h.1) d))
    rw [point_local_basechange_map,local_point_map]
    change localPointEval (extendedCurve (L := L) W) (base_nonsingular W h).1
      (algebraMap (extendedCurve (L := L) W).CoordinateRing _
        (CoordinateRing.map W (algebraMap K L) d))=algebraMap K L (pointEval W b c h.1 d)
    rw [local_point_map,point_eval_basechange W h]
  exact RingHom.congr_fun he a

#print axioms point_eval_basechange
#print axioms point_local_basechange_field
#print axioms point_local_basechange_residue
end
end GeneralTranslationAudit
