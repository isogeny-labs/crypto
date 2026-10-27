import Mathlib.LinearAlgebra.Pi
import Mathlib.Algebra.BigOperators.Fin
import Mathlib.Data.Fin.VecNotation
import Mathlib.Algebra.CharP.Defs
import Mathlib.Tactic.FinCases
import Mathlib.Tactic.NormNum
import Mathlib.Tactic.LinearCombination
import Mathlib.Tactic.Abel
namespace KoblitzRound21Equivariant
open scoped BigOperators
abbrev S (K:Type*) := Fin 4→K
abbrev T (K:Type*) := Fin 4→Fin 5→K
variable {K:Type*} [CommRing K]

def rotate (v:S K) : S K := ![-(v 0+v 1+v 2+v 3),v 0,v 1,v 2]
def cycle (v:T K) : T K := fun i j=>v i (j+1)
def sourceNorm (v:S K) : S K := ∑k:Fin 5,(rotate^[k.val]) v
def targetNorm (v:T K) : T K := ∑k:Fin 5,(cycle^[k.val]) v

theorem source_norm_zero (v:S K) : sourceNorm v=0 := by
 funext i
 fin_cases i <;> simp [sourceNorm,rotate,Fin.sum_univ_succ,Function.iterate_succ_apply] <;> abel

theorem actual_fixed_source [CharP K 5] : rotate (fun _=>(1:K))=(fun _=>1) := by
 have hfive:(5:K)=0 := CharP.cast_eq_zero K 5
 funext i
 fin_cases i <;> norm_num [rotate] <;> linear_combination -hfive

theorem target_norm_coordinates (v:T K) : targetNorm v=fun i _=>∑j:Fin 5,v i j := by
 funext i j
 fin_cases j <;> simp [targetNorm,cycle,Fin.sum_univ_succ,Function.iterate_succ_apply] <;> abel

theorem fixed_target_is_norm (v:T K) (hv:cycle v=v) : ∃w:T K,targetNorm w=v := by
 have hcol (i:Fin 4) (j:Fin 5) : v i (j+1)=v i j := congrFun (congrFun hv i) j
 have hall (i:Fin 4) (j:Fin 5) : v i j=v i 0 := by
  have h0:=hcol i 0
  have h1:=hcol i 1
  have h2:=hcol i 2
  have h3:=hcol i 3
  fin_cases j <;> simp_all
 refine ⟨fun i j=>if j=0 then v i 0 else 0,?_⟩
 rw [target_norm_coordinates]
 funext i j
 simp [Fin.sum_univ_succ,hall]

theorem intertwine_iterates (g:T K→ₗ[K]S K)
 (hg:∀w,g (cycle w)=rotate (g w)) (n:Nat) (w:T K) :
 g ((cycle^[n]) w)=(rotate^[n]) (g w) := by
 induction n with
 | zero=>rfl
 | succ n ih=>
  rw [Function.iterate_succ_apply',Function.iterate_succ_apply',hg,ih]

theorem equivariant_scalar_recovery_zero [CharP K 5]
 (f:S K→ₗ[K]T K) (g:T K→ₗ[K]S K)
 (hf:∀v,f (rotate v)=cycle (f v)) (hg:∀w,g (cycle w)=rotate (g w))
 (d:K) (hgf:∀v,g (f v)=d • v) : d=0 := by
 let v:S K:=fun _=>1
 have hv:rotate v=v := actual_fixed_source
 have hfix:cycle (f v)=f v := by rw [←hf,hv]
 obtain ⟨w,hw⟩:=fixed_target_is_norm (f v) hfix
 have hz:g (f v)=0 := by
  rw [←hw]
  change g (∑k:Fin 5,(cycle^[k.val]) w)=0
  rw [map_sum]
  simp_rw [intertwine_iterates g hg]
  exact source_norm_zero (g w)
 rw [hgf] at hz
 have he:=congrFun hz 0
 simpa [v] using he

theorem no_equivariant_retraction [CharP K 5] [Nontrivial K]
 (f:S K→ₗ[K]T K) (g:T K→ₗ[K]S K)
 (hf:∀v,f (rotate v)=cycle (f v)) (hg:∀w,g (cycle w)=rotate (g w)) :
 ¬(∀v,g (f v)=v) := by
 intro h
 have he:=equivariant_scalar_recovery_zero f g hf hg 1 (by simpa using h)
 exact one_ne_zero he
end KoblitzRound21Equivariant
#print axioms KoblitzRound21Equivariant.source_norm_zero
#print axioms KoblitzRound21Equivariant.actual_fixed_source
#print axioms KoblitzRound21Equivariant.target_norm_coordinates
#print axioms KoblitzRound21Equivariant.fixed_target_is_norm
#print axioms KoblitzRound21Equivariant.intertwine_iterates
#print axioms KoblitzRound21Equivariant.equivariant_scalar_recovery_zero
#print axioms KoblitzRound21Equivariant.no_equivariant_retraction
