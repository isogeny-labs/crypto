import SameUMixed
set_option autoImplicit false
namespace SameUMixed
open MixedCancellation
variable {K : Type*} [Field K]
noncomputable def sourceK (a b : D2 K) : K :=
 (a.s*((b.z*b.z)*b.z))*b.w - (b.s*((a.z*a.z)*a.z))*a.w
noncomputable def sourcePoint (a b : D2 K) : WeightedAdd11.Input K :=
 let k := sourceK a b
 let k2 := k*k
 let k3 := k2*k
 -- Native t1 crosses with z2, t2 with z1.
 let t1c := (a.t*((b.z*b.z)*(b.z*b.z)*b.z))*b.w
 let t2c := (b.t*((a.z*a.z)*(a.z*a.z)*a.z))*a.w
 let x := (t2c-t1c)*k
 ⟨x,((a.s*x+a.t*((b.z*b.z)*k2))*k3)*((b.z*b.z)*b.z),
 k*(a.z*b.z),a.w⟩
theorem source_k_scale (a b : D2 K) (haz : a.z≠0) (haw : a.w≠0)
 (hbz : b.z≠0) (hbw : b.w≠0) :
 sourceK a b = (a.z*b.z)^3*(a.w*b.w)*(ns a-ns b) := by
 dsimp [sourceK,ns]
 field_simp <;> ring

theorem source_point_decode (a b : D2 K) (haz : a.z≠0) (haw : a.w≠0)
 (hbz : b.z≠0) (hbw : b.w≠0) (hk : ns a-ns b≠0) :
 (sourcePoint a b).z≠0 ∧ (sourcePoint a b).w≠0 ∧
 WeightedAdd11.nx (sourcePoint a b)=(nt b-nt a)/(ns a-ns b) ∧
 WeightedAdd11.ny (sourcePoint a b)=ns a*((nt b-nt a)/(ns a-ns b))+nt a := by
 have hkr : sourceK a b ≠ 0 := by
  rw [source_k_scale a b haz haw hbz hbw]
  exact mul_ne_zero (mul_ne_zero (pow_ne_zero 3 (mul_ne_zero haz hbz)) (mul_ne_zero haw hbw)) hk
 have hkn : a.s*b.z^3*b.w-b.s*a.z^3*a.w ≠ 0 := by
  convert hkr using 1 <;> dsimp [sourceK] <;> ring
 have hkn2 : a.s*b.z^3*b.w-a.z^3*a.w*b.s ≠ 0 := by
  convert hkn using 1 <;> ring
 refine ⟨mul_ne_zero hkr (mul_ne_zero haz hbz),haw,?_,?_⟩
 · dsimp [sourcePoint,WeightedAdd11.nx,ns,nt]
   simp only [sourceK]
   field_simp [hkn2] <;> ring
 · dsimp [sourcePoint,WeightedAdd11.ny,ns,nt]
   simp only [sourceK]
   field_simp [hkn2] <;> ring
theorem source_point_valid (f : Polynomial K) (a b : D2 K)
 (haz : a.z≠0) (haw : a.w≠0) (hbz : b.z≠0) (hbw : b.w≠0)
 (ha : validTwo f a) (hb : validTwo f b)
 (hq : nq a=nq b) (hr : nr a=nr b)
 (hk : ns a-ns b≠0) (hn : ¬(ns a= -ns b ∧ nt a= -nt b)) :
 (sourcePoint a b).z≠0 ∧ (sourcePoint a b).w≠0 ∧ validPoint f (sourcePoint a b) := by
 have hb' : support (nq a) (nr a) ∣ f-(ordinate (ns b) (nt b))^2 := by
  rw [hq,hr]
  exact hb
 have hv := (extracted_point_valid f (nq a) (nr a) (ns a) (nt a) (ns b) (nt b) ha hb' hk hn).2
 obtain ⟨hz,hw,hx,hy⟩ := source_point_decode a b haz haw hbz hbw hk
 refine ⟨hz,hw,?_⟩
 unfold validPoint
 rw [hx,hy]
 exact hv
end SameUMixed
