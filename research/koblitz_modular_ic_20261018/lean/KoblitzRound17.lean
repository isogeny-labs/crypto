import KoblitzRound17Integral
import KoblitzRound17Survivors
import Mathlib.RepresentationTheory.Invariants
namespace KoblitzRound17

theorem smith_kernel_iff {A B:Type*} [AddGroup A] [AddGroup B]
 (u:B ≃+ B) (v:A ≃+ A) (b d:A →+ B)
 (h:∀x,u (b (v x))=d x) (x:A) : b (v x)=0 ↔ d x=0 := by
 rw [←h x]
 exact (u.map_eq_zero_iff).symm

theorem smith_kernel_parameterization {A B:Type*} [AddGroup A] [AddGroup B]
 (u:B ≃+ B) (v:A ≃+ A) (b d:A →+ B)
 (h:∀x,u (b (v x))=d x) :
 ∃e:{x:A // d x=0} ≃ {x:A // b x=0},∀x,(e x).val=v x.val := by
 let e : {x:A // d x=0} ≃ {x:A // b x=0} := {
  toFun:=fun x => ⟨v x.val,(smith_kernel_iff u v b d h x.val).2 x.property⟩
  invFun:=fun y => ⟨v.symm y.val,by simpa [←h,v.apply_symm_apply,y.property]⟩
  left_inv:=fun x => Subtype.ext (v.symm_apply_apply x.val)
  right_inv:=fun y => Subtype.ext (v.apply_symm_apply y.val) }
 exact ⟨e,fun _=>rfl⟩

theorem odd_kernel_card_nonzero {R:Type*} [CommRing R] [CharP R 2] :
 (99225:R)=1 := by
 simpa using (CharP.natCast_eq_natCast_mod R 2 99225)

theorem odd_kernel_averaging_projection
 {R G V:Type*} [Field R] [CharP R 2] [Group G] [Fintype G]
 [AddCommGroup V] [Module R V] (rho:Representation R G V)
 (hcard:Fintype.card G=99225) :
 ∃p:V →ₗ[R] V,(∀x,p (p x)=p x) ∧ (∀x,p x=x ↔ x∈rho.invariants) := by
 have hn:(Fintype.card G:R)≠0 := by rw [hcard]; change (99225:R)≠0; rw [odd_kernel_card_nonzero (R:=R)]; exact one_ne_zero
 letI : Invertible (Fintype.card G:R) := invertibleOfNonzero hn
 refine ⟨rho.averageMap,?_,?_⟩
 · intro x;exact rho.averageMap_id _ (rho.averageMap_invariant x)
 · intro x;constructor
   · intro h;rw [←h];exact rho.averageMap_invariant x
   · exact rho.averageMap_id x

theorem actual_two_stage_dimensions :
 99225*87723=8704314675 ∧ 361*243=87723 ∧ 99225*361=35820225 := by norm_num

theorem T_translation_x_residual {R:Type*} [CommRing R] (x y a:R) :
 (y+1)^2+(y+1)*x+(x+a)*x^2-x=
 (y^2+x*y+x^3+a*x^2+1)+2*y := by ring

theorem T_translation_x {R:Type*} [Field R] [CharP R 2]
 (x y a:R) (hx:x≠0) (hc:y^2+x*y+x^3+a*x^2+1=0) :
 ((y+1)/x)^2+(y+1)/x+x+a=1/x := by
 have h2:(2:R)=0 := CharP.cast_eq_zero R 2
 field_simp
 linear_combination hc+y*h2

end KoblitzRound17
#print axioms KoblitzRound17.smith_kernel_iff
#print axioms KoblitzRound17.smith_kernel_parameterization
#print axioms KoblitzRound17.odd_kernel_card_nonzero
#print axioms KoblitzRound17.odd_kernel_averaging_projection
#print axioms KoblitzRound17.actual_two_stage_dimensions
#print axioms KoblitzRound17.T_translation_x_residual
#print axioms KoblitzRound17.T_translation_x
