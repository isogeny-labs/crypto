import DoublingCertificate
import SurjectiveImports

namespace EllipticDoublingSurjectivityAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine EllipticDoublingDegreeAudit
variable {K : Type*} [Field K]

def doublingFiber (W : WeierstrassCurve.Affine K) (a : K) : K[X] :=
  doublingNumerator W-C a*doublingDenominator W

theorem doubling_fiber_degree (W : WeierstrassCurve.Affine K) (a : K) :
    (doublingFiber W a).natDegree=4 := by
  unfold doublingFiber doublingNumerator doublingDenominator
  compute_degree!

theorem actual_half_coordinates [DecidableEq K] [IsAlgClosed K]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (a : K) :
    ∃ u v : K,∃ _h : W.Nonsingular u v,
      v≠W.negY u v ∧ W.addX u u (W.slope u u v v)=a := by
  have hn : doublingFiber W a≠0 := ne_zero_of_natDegree_gt
    (show 0<(doublingFiber W a).natDegree by rw [doubling_fiber_degree];norm_num)
  obtain ⟨u,hu⟩:=IsAlgClosed.exists_root (doublingFiber W a) (by
    rw [degree_eq_natDegree hn,doubling_fiber_degree];norm_num)
  obtain ⟨v,hv⟩:=actual_fiber_exists W u
  have h : W.Nonsingular u v := (equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp hv
  have hr : (doublingNumerator W).eval u-a*(doublingDenominator W).eval u=0 := by
    simpa only [IsRoot.def,doublingFiber,eval_sub,eval_mul,eval_C] using hu
  have hden : (doublingDenominator W).eval u≠0 := by
    intro hd
    have hnum : (doublingNumerator W).eval u=0 := by simpa only [hd,mul_zero,sub_zero] using hr
    have hd' : 4*u^3+W.b₂*u^2+2*W.b₄*u+W.b₆=0 := by
      simpa only [doublingDenominator,eval_add,eval_mul,eval_pow,eval_X,eval_C] using hd
    have hn' : u^4-W.b₄*u^2-2*W.b₆*u-W.b₈=0 := by
      simpa only [doublingNumerator,eval_sub,eval_mul,eval_pow,eval_X,eval_C] using hnum
    exact (den_duplication_ne_zero_or_num_duplication_ne_zero h).elim (fun h=>h hd') (fun h=>h hn')
  have hden' : 4*u^3+W.b₂*u^2+2*W.b₄*u+W.b₆≠0 := by
    simpa only [doublingDenominator,eval_add,eval_mul,eval_pow,eval_X,eval_C] using hden
  have hnv : v≠W.negY u v := fun he=>hden' ((den_duplication_eq_zero_iff hv).mpr he)
  refine ⟨u,v,h,hnv,?_⟩
  rw [addX_self_of_Y_ne hv hnv]
  have he : (doublingNumerator W).eval u/(doublingDenominator W).eval u=a :=
    (div_eq_iff hden).mpr (sub_eq_zero.mp hr)
  simpa only [doublingNumerator,doublingDenominator,eval_sub,eval_add,eval_mul,eval_pow,eval_X,eval_C] using he

#print axioms doubling_fiber_degree
#print axioms actual_half_coordinates
end
end EllipticDoublingSurjectivityAudit
