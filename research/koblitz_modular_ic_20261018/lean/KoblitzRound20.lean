import KoblitzRound20Retraction
import Mathlib.Algebra.CharP.Lemmas
namespace KoblitzRound20
open scoped BigOperators

theorem lift_step_cancel {R:Type*} [CommRing R] (d p e:R) :
 d*(1+p*e)-e*(p*d)=d := by ring

theorem rectangular_lift_step {R:Type*} [CommRing R]
 (T U:Matrix (Fin 8) (Fin 40) R) (M:Matrix (Fin 40) (Fin 8) R)
 (E:Matrix (Fin 8) (Fin 8) R) (d p:R)
 (hT:T*M=1+p • E) (hU:U*M=(p*d) • 1) :
 (d • T-E*U)*M=d • 1 := by
 rw [Matrix.sub_mul,Matrix.smul_mul,Matrix.mul_assoc,hT,hU]
 simp [smul_add,Matrix.mul_smul,smul_smul,mul_comm d p,Matrix.mul_one]

def evalCM {R:Type*} [CommRing R] (w:R) : (ℤ×ℤ)→+R where
 toFun z := (z.1:R)+(z.2:R)*w
 map_zero' := by simp
 map_add' := by intro x y;simp only [Prod.fst_add,Prod.snd_add,Int.cast_add];ring

theorem evalCM_mul {R:Type*} [CommRing R] (w:R)
 (hw:w^2+w+2=0) (x y:ℤ×ℤ) :
 evalCM w (KoblitzRound19OrbitCover.cmul x y)=evalCM w x*evalCM w y := by
 change ((x.1*y.1-2*x.2*y.2:ℤ):R)+((x.1*y.2+x.2*y.1-x.2*y.2:ℤ):R)*w=
  ((x.1:R)+(x.2:R)*w)*((y.1:R)+(y.2:R)*w)
 push_cast
 calc
  _=((x.1:R)+(x.2:R)*w)*((y.1:R)+(y.2:R)*w)-
    (x.2:R)*(y.2:R)*(w^2+w+2) := by ring
  _=_ := by rw [hw];ring

def evalRows {R : Type*} [CommRing R] (w : R) : Matrix (Fin 20) (Fin 4) R :=
 fun i j => evalCM w (KoblitzRound20Retraction.row i j)

def evalRecovery {R : Type*} [CommRing R] (w : R) : Matrix (Fin 4) (Fin 20) R :=
 fun i j => evalCM w (KoblitzRound20Retraction.recovery i j)

theorem actual_recovery_matrix {R : Type*} [CommRing R] (w : R)
 (hw : w^2+w+2=0) : evalRecovery w * evalRows w=1 := by
 ext i j
 change (∑ k:Fin 20,evalCM w (KoblitzRound20Retraction.recovery i k)*
   evalCM w (KoblitzRound20Retraction.row k j))=(if i=j then 1 else 0)
 calc
  _=evalCM w (∑ k:Fin 20,KoblitzRound19OrbitCover.cmul
      (KoblitzRound20Retraction.recovery i k) (KoblitzRound20Retraction.row k j)) := by
   rw [map_sum]
   apply Finset.sum_congr rfl
   intro k _
   exact (evalCM_mul w hw _ _).symm
  _=_ := by
   rw [KoblitzRound20Retraction.actual_CM_recovery]
   by_cases hij:i=j <;> simp [hij,evalCM]

theorem actual_coefficient_kernel_trivial {R : Type*} [CommRing R]
 (w : R) (hw : w^2+w+2=0) (v : Fin 4→R)
 (hv : (evalRows w).mulVec v=0) : v=0 := by
 have hz : (evalRecovery w * evalRows w).mulVec v=0 := by
  rw [← Matrix.mulVec_mulVec,hv,Matrix.mulVec_zero]
 rw [actual_recovery_matrix w hw,Matrix.one_mulVec] at hz
 exact hz

theorem power19_factorization {R:Type*} [CommRing R] (u v:R) :
 (u+v)^19=(u+v)^16*(u+v)^2*(u+v) := by ring

theorem boundary_top19 {R:Type*} [CommRing R] [CharP R 2] (u v:R) :
 (u+v)^19=v^19+u*v^18+u^2*v^17+u^3*v^16+
   u^16*v^3+u^17*v^2+u^18*v+u^19 := by
 have h2:(u+v)^2=u^2+v^2 := add_pow_char u v 2
 have h16:(u+v)^16=u^16+v^16 := by simpa using add_pow_char_pow u v 2 4
 calc
  _=(u+v)^16*(u+v)^2*(u+v) := by ring
  _=_ := by rw [h16,h2];ring

theorem boundary_equal_from_power {K:Type*} [Field K] [CharP K 2]
 (u v:K) (hz:(u+v)^19=0) : u=v := by
 have he:u+v=0 := (pow_eq_zero_iff (by decide:19≠0)).mp hz
 have h2:(2:K)=0 := CharP.cast_eq_zero K 2
 have hv:v+v=0 := by rw [← two_mul,h2,zero_mul]
 calc
  u=u+(v+v) := by rw [hv,add_zero]
  _=(u+v)+v := by ring
  _=v := by rw [he,zero_add]

theorem reduced_boundary_linear {K:Type*} [Field K] [CharP K 2]
 (h u v tail:K) (hh:h=0) (he:(u+v)^19+h*tail=0) : u=v := by
 apply boundary_equal_from_power u v
 simpa [hh] using he

theorem symmetric_pic0_two_torsion {A:Type*} [AddCommGroup A]
 (d:A) (hd:-d=d) : 2 • d=0 := by
 have hs:d+d=0 := by
  calc
   d+d=(-d)+d := congrArg (fun z:A=>z+d) hd.symm
   _=0 := neg_add_cancel d
 simpa [two_nsmul] using hs

theorem coordinate_ratio_clearing {R:Type*} [CommRing R] (x y q:R) :
 (x*q)*y=x*(y*q) := by ring

theorem infinity_coordinate_ratio {K:Type*} [Field K]
 (x y:K) (k:Nat) (hx:x≠0) (hy:y≠0) : x^(k+1)/(y*x^k)=x/y := by
 field_simp [hx,hy]
 simp [pow_succ]

theorem orbit_map_translation {A B:Type*} [AddCommGroup A] [AddCommGroup B]
 (f:A→+B) (p t:A) (ht:f t=0) : f (p+t)=f p := by
 rw [map_add,ht,add_zero]


theorem projector_idempotent {R:Type*} [CommRing R]
 (M:Matrix (Fin 20) (Fin 4) R) (L:Matrix (Fin 4) (Fin 20) R)
 (h:L*M=1) : (M*L)*(M*L)=M*L := by
 calc
  _=M*((L*M)*L) := by simp only [Matrix.mul_assoc]
  _=M*L := by rw [h,Matrix.one_mul]

theorem actual_evaluated_projector_idempotent {R:Type*} [CommRing R]
 (w:R) (hw:w^2+w+2=0) :
 let M:Matrix (Fin 20) (Fin 4) R := evalRows w
 let L:Matrix (Fin 4) (Fin 20) R := evalRecovery w
 (M*L)*(M*L)=M*L := by
 exact projector_idempotent _ _ (actual_recovery_matrix w hw)

theorem fixed_projector_iff_image {A B:Type*} (f:A→B) (g:B→A)
 (h : ∀ a, g (f a) = a) (b : B) : f (g b) = b ↔ ∃ a, f a = b := by
 constructor
 · intro hb;exact ⟨g b,hb⟩
 · rintro ⟨a,rfl⟩
   rw [h]

end KoblitzRound20
#print axioms KoblitzRound20.lift_step_cancel
#print axioms KoblitzRound20.rectangular_lift_step
#print axioms KoblitzRound20.evalCM_mul
#print axioms KoblitzRound20.actual_recovery_matrix
#print axioms KoblitzRound20.actual_coefficient_kernel_trivial
#print axioms KoblitzRound20.boundary_top19
#print axioms KoblitzRound20.boundary_equal_from_power
#print axioms KoblitzRound20.reduced_boundary_linear
#print axioms KoblitzRound20.symmetric_pic0_two_torsion
#print axioms KoblitzRound20.coordinate_ratio_clearing
#print axioms KoblitzRound20.infinity_coordinate_ratio
#print axioms KoblitzRound20.orbit_map_translation
#print axioms KoblitzRound20.projector_idempotent
#print axioms KoblitzRound20.actual_evaluated_projector_idempotent
#print axioms KoblitzRound20.fixed_projector_iff_image
#print axioms KoblitzRound20.power19_factorization
