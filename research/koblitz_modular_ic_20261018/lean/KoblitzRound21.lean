import Mathlib.Algebra.CharP.Lemmas
import Mathlib.Tactic.FinCases
import Mathlib.Tactic.NormNum
import Mathlib.Tactic.LinearCombination
import Mathlib.Tactic.Ring
import Mathlib.Tactic.Abel
import Mathlib.RingTheory.RootsOfUnity.PrimitiveRoots
namespace KoblitzRound21
open scoped BigOperators

def doubling (i:Fin 18) : Fin 18 :=
 ![0,1,3,7,15,12,6,13,8,17,16,14,10,2,5,11,4,9] i

theorem doubling_bijective : Function.Bijective doubling := by decide +kernel

theorem doubling_mod19 : ∀ i:Fin 18, 2^i.val%19=(doubling i).val+1 := by decide +kernel

def trace18 {K:Type*} [CommRing K] (z:K) : K := ∑ i:Fin 18,z^(2^i.val)

theorem trace18_add {K:Type*} [CommRing K] [CharP K 2] (x y:K) :
 trace18 (x+y)=trace18 x+trace18 y := by
 simp only [trace18,add_pow_char_pow x y 2,Finset.sum_add_distrib]

def traceHom {K:Type*} [CommRing K] [CharP K 2] : K→+K where
 toFun := trace18
 map_zero' := by simp [trace18]
 map_add' := trace18_add

theorem trace18_root_reindex {K:Type*} [CommRing K] (z:K) (hz:z^19=1) :
 trace18 z=∑i:Fin 18,z^(i.val+1) := by
 let e:Fin 18≃Fin 18 := Equiv.ofBijective doubling doubling_bijective
 calc
  _=∑i:Fin 18,z^((doubling i).val+1) := by
   apply Finset.sum_congr rfl
   intro i _
   rw [pow_eq_pow_mod (2^i.val) hz,doubling_mod19]
  _=_ := Equiv.sum_comp e (fun i:Fin 18=>z^(i.val+1))

theorem geometric19 {K:Type*} [CommRing K] (z:K) :
 (z-1)*(1+∑i:Fin 18,z^(i.val+1))=z^19-1 := by
 simp only [Fin.sum_univ_succ]
 norm_num
 ring

theorem trace18_nontrivial_root {K:Type*} [Field K] [CharP K 2]
 (z:K) (hz:z^19=1) (hne:z≠1) : trace18 z=1 := by
 have hprod:(z-1)*(1+trace18 z)=0 := by
  rw [trace18_root_reindex z hz,geometric19,hz,sub_self]
 have hs:1+trace18 z=0 := (mul_eq_zero.mp hprod).resolve_left (sub_ne_zero.mpr hne)
 have htwo:(2:K)=0 := CharP.cast_eq_zero K 2
 linear_combination hs-htwo

theorem trace18_one {K:Type*} [CommRing K] [CharP K 2] : trace18 (1:K)=0 := by
 have htwo:(2:K)=0 := CharP.cast_eq_zero K 2
 simp only [trace18,one_pow,Finset.sum_const,Finset.card_univ,Fintype.card_fin,nsmul_eq_mul,mul_one]
 linear_combination 9*htwo

theorem missing_among_five (k:Fin 5→Fin 19) : ∃j:Fin 19,∀i:Fin 5,k i≠j := by
 classical
 by_contra hn
 have hs:Function.Surjective k := by
  intro j
  by_contra h
  apply hn
  refine ⟨j,?_⟩
  intro i he
  exact h ⟨i,he⟩
 have hc:=Fintype.card_le_of_surjective k hs
 norm_num at hc

theorem five_primitive_powers_ne_zero {K:Type*} [Field K] [CharP K 2]
 (z:K) (hz:IsPrimitiveRoot z 19) (k:Fin 5→Fin 19) :
 (∑i:Fin 5,z^(k i).val)≠0 := by
 classical
 obtain ⟨j,hj⟩:=missing_among_five k
 have hden:z^j.val≠0 := pow_ne_zero _ (hz.isUnit (by decide)).ne_zero
 have ht (i:Fin 5) : trace18 (z^(k i).val/z^j.val)=1 := by
  apply trace18_nontrivial_root
  · rw [div_pow,←pow_mul,←pow_mul, mul_comm (k i).val 19,mul_comm j.val 19,pow_mul,pow_mul,hz.pow_eq_one]
    simp
  · intro he
    have heq:z^(k i).val=z^j.val := (div_eq_one_iff_eq hden).mp he
    exact hj i (Fin.ext (hz.pow_inj (k i).isLt j.isLt heq))
 intro hs
 have htSum:trace18 (∑i:Fin 5,z^(k i).val/z^j.val)=1 := by
  change traceHom (∑i:Fin 5,z^(k i).val/z^j.val)=1
  rw [map_sum]
  change (∑i:Fin 5,trace18 (z^(k i).val/z^j.val))=1
  simp_rw [ht]
  simp only [Finset.sum_const,Finset.card_univ,Fintype.card_fin,nsmul_eq_mul,mul_one]
  have htwo:(2:K)=0 := CharP.cast_eq_zero K 2
  linear_combination 2*htwo
 have he:(∑i:Fin 5,z^(k i).val/z^j.val)=0 := by
  simp_rw [div_eq_mul_inv]
  rw [←Finset.sum_mul,hs,zero_mul]
 rw [he] at htSum
 have ht0:trace18 (0:K)=0 := by simp [trace18]
 rw [ht0] at htSum
 exact zero_ne_one htSum

theorem five_roots_ne_zero {K:Type*} [Field K] [CharP K 2]
 (z:K) (hz:IsPrimitiveRoot z 19) (v:Fin 5→K) (hv:∀i,(v i)^19=1) :
 (∑i:Fin 5,v i)≠0 := by
 classical
 choose k hk he using fun i=>hz.eq_pow_of_pow_eq_one (hv i)
 let indices:Fin 5→Fin 19 := fun i=>⟨k i,hk i⟩
 have h: v=fun i=>z^(indices i).val := by funext i;exact (he i).symm
 rw [h]
 exact five_primitive_powers_ne_zero z hz indices

theorem ordinary_chart_factorization {R:Type*} [CommRing R] (t z a:R) :
 z+t*z-t^3-a*t^2*z-z^3=z*(1+t-a*t^2-z^2)-t^3 := by ring

theorem trace_zero_from_actual_descent {A:Type*} [AddCommGroup A]
 (sigma:A→+A) (p:Fin 4→A)
 (h0:p 0= -sigma (p 3))
 (h1:p 1=sigma (p 0)-sigma (p 3))
 (h2:p 2=sigma (p 1)-sigma (p 3))
 (h3:p 3=sigma (p 2)-sigma (p 3)) :
 (∑i:Fin 5,(sigma^[i.val]) (p 0))=0 := by
 have hneg:-sigma (p 3)=p 0 := h0.symm
 have hp1:p 1=sigma (p 0)+p 0 := by rw [h1,sub_eq_add_neg,hneg]
 have hp2:p 2=sigma (sigma (p 0))+sigma (p 0)+p 0 := by
  rw [h2,hp1,map_add,sub_eq_add_neg,hneg]
 have hp3:p 3=sigma (sigma (sigma (p 0)))+sigma (sigma (p 0))+sigma (p 0)+p 0 := by
  rw [h3,hp2,map_add,map_add,sub_eq_add_neg,hneg]
 have hz:sigma (p 3)+p 0=0 := by rw [h0];abel
 rw [hp3,map_add,map_add,map_add] at hz
 simpa [Fin.sum_univ_succ,Function.iterate_succ_apply,add_comm,add_left_comm,add_assoc] using hz

end KoblitzRound21
#print axioms KoblitzRound21.doubling_bijective
#print axioms KoblitzRound21.doubling_mod19
#print axioms KoblitzRound21.trace18_add
#print axioms KoblitzRound21.trace18_root_reindex
#print axioms KoblitzRound21.geometric19
#print axioms KoblitzRound21.trace18_nontrivial_root
#print axioms KoblitzRound21.trace18_one
#print axioms KoblitzRound21.missing_among_five
#print axioms KoblitzRound21.five_primitive_powers_ne_zero
#print axioms KoblitzRound21.five_roots_ne_zero

#print axioms KoblitzRound21.ordinary_chart_factorization
#print axioms KoblitzRound21.trace_zero_from_actual_descent
