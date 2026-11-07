import KoblitzRound21Equivariant
import Mathlib.Algebra.Group.Hom.Defs
namespace KoblitzRound22
open scoped BigOperators
open KoblitzRound21Equivariant
variable {K : Type*} [CommRing K]

def e0 : S K := ![1,0,0,0]
def coordinates (v : S K) : S K :=
  ![v 0+v 1+v 2+v 3,v 1+v 2+v 3,v 2+v 3,v 3]
def action (a v : S K) : S K := ∑ k : Fin 4, a k • (rotate^[k.val]) v
def embed (v : S K) : Fin 5 → K := ![v 3,v 2,v 1,v 0,-(v 0+v 1+v 2+v 3)]
def project (v : Fin 5 → K) : S K := ![v 3,v 2,v 1,v 0]

theorem project_embed (v : S K) : project (embed v)=v := by
  funext i; fin_cases i <;> rfl

theorem embed_norm_zero (v : S K) : ∑ i : Fin 5, embed v i=0 := by
  simp [embed,Fin.sum_univ_succ]; ring

theorem embed_project (v : Fin 5 → K) (h : ∑ i : Fin 5,v i=0) :
    embed (project v)=v := by
  funext i
  fin_cases i <;> simp [embed,project]
  simp [Fin.sum_univ_succ] at h
  linear_combination -h

theorem augmentation_equivariant (v : S K) :
    embed (rotate v)=fun i=>embed v (i+1) := by
  funext i
  fin_cases i <;> simp [embed,rotate] <;> ring

theorem cyclic_expansion (v : S K) : action (coordinates v) e0=v := by
  funext i
  fin_cases i <;>
    simp [action,coordinates,e0,rotate,Fin.sum_univ_succ,Function.iterate_succ_apply] <;> ring

theorem coefficient_reconstruction (a : S K) : coordinates (action a e0)=a := by
  funext i
  fin_cases i <;>
    simp [action,coordinates,e0,rotate,Fin.sum_univ_succ,Function.iterate_succ_apply] <;> ring

theorem action_cyclic_symmetry (v w : S K) :
    action (coordinates v) w=action (coordinates w) v := by
  funext i
  fin_cases i <;>
    simp [action,coordinates,rotate,Fin.sum_univ_succ,Function.iterate_succ_apply] <;> ring

theorem commute_iterates (f : S K →ₗ[K] S K)
    (hf : ∀ v,f (rotate v)=rotate (f v)) (n : Nat) (v : S K) :
    f ((rotate^[n]) v)=(rotate^[n]) (f v) := by
  induction n with
  | zero => rfl
  | succ n ih =>
    rw [Function.iterate_succ_apply',Function.iterate_succ_apply',hf,ih]

/-- Every commuting linear map is in the four-term polynomial span,
    with no bound on the coefficients. -/
theorem complete_commutant_parameterization (f : S K →ₗ[K] S K)
    (hf : ∀ v,f (rotate v)=rotate (f v)) (v : S K) :
    f v=action (coordinates (f e0)) v := by
  calc
    f v=f (action (coordinates v) e0) := by rw [cyclic_expansion]
    _=action (coordinates v) (f e0) := by
      simp only [action,map_sum,map_smul,commute_iterates f hf]
    _=action (coordinates (f e0)) v := action_cyclic_symmetry v (f e0)

/-- A right inverse gives every solution of the full integer system. -/
theorem complete_solution_parameterization
    {X Y : Type*} [AddCommGroup X] [Module K X] [AddCommGroup Y] [Module K Y]
    (H : X →ₗ[K] Y) (W : Y →ₗ[K] X) (h : ∀ y,H (W y)=y) (t : Y) (x : X) :
    H x=t ↔ ∃ z,x=W t+(z-W (H z)) := by
  constructor
  · intro hx
    refine ⟨x,?_⟩
    rw [hx]
    abel
  · rintro ⟨z,rfl⟩
    simp [h]

theorem normalized_torsion_recovery
    {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (f : A →+ B) (g : B →+ A) (d u v r : ℤ)
    (hg : ∀ x,g (f x)=d • x) (hbezout : u*d+v*r=1)
    (x : A) (hx : r • x=0) : u • g (f x)=x := by
  have h:=congrArg (fun n : ℤ=>n • x) hbezout
  simpa [add_zsmul,mul_zsmul,hx,hg] using h

theorem injective_on_torsion
    {A B : Type*} [AddCommGroup A] [AddCommGroup B]
    (f : A →+ B) (g : B →+ A) (d u v r : ℤ)
    (hg : ∀ x,g (f x)=d • x) (hbezout : u*d+v*r=1)
    (x y : A) (hx : r • x=0) (hy : r • y=0) (heq : f x=f y) : x=y := by
  calc
    x=u • g (f x) := (normalized_torsion_recovery f g d u v r hg hbezout x hx).symm
    _=u • g (f y) := by rw [heq]
    _=y := normalized_torsion_recovery f g d u v r hg hbezout y hy

end KoblitzRound22
#print axioms KoblitzRound22.project_embed
#print axioms KoblitzRound22.embed_norm_zero
#print axioms KoblitzRound22.embed_project
#print axioms KoblitzRound22.augmentation_equivariant
#print axioms KoblitzRound22.cyclic_expansion
#print axioms KoblitzRound22.coefficient_reconstruction
#print axioms KoblitzRound22.action_cyclic_symmetry
#print axioms KoblitzRound22.commute_iterates
#print axioms KoblitzRound22.complete_commutant_parameterization
#print axioms KoblitzRound22.complete_solution_parameterization
#print axioms KoblitzRound22.normalized_torsion_recovery
#print axioms KoblitzRound22.injective_on_torsion
