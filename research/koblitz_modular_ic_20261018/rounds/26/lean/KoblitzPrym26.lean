import Mathlib.Algebra.CharP.Basic
import Mathlib.Data.ZMod.Basic
import Mathlib.Algebra.BigOperators.Group.Finset.Basic
import Mathlib.Tactic.NormNum
import Mathlib.Tactic.LinearCombination
import Mathlib.Tactic.FieldSimp
import Mathlib.Tactic.Ring

/-! Exact finite character facts and conditional Prym-group algebra.
The actual modular/newform and Picard bridges are written, not axioms here. -/
namespace KoblitzPrym26

theorem crt_delta : 34%11=1 ∧ 34%7=6 ∧ (34*34)%77=1 := by decide

theorem diamond_two_fifth : (2:ZMod 11)^5 = -1 := by decide

theorem shared_rational_diamond_fifth : (6:ZMod 11)^5 = -1 := by decide

theorem primitive_sigma (k:ℕ) (hk:k=1 ∨ k=3 ∨ k=7 ∨ k=9) :
    ((2:ZMod 11)^k)^5 = -1 := by
  rcases hk with rfl|rfl|rfl|rfl <;> decide

def diamondNorm (k d:ℕ) : ZMod 11 :=
  ∑ j ∈ Finset.range d, (2:ZMod 11)^(k*(j*(10/d)))

theorem nontrivial_diamond_norm (k d:ℕ)
    (hk:k=1 ∨ k=3 ∨ k=7 ∨ k=9) (hd:d=2 ∨ d=5 ∨ d=10) :
    diamondNorm k d=0 := by
  rcases hk with rfl|rfl|rfl|rfl <;>
    rcases hd with rfl|rfl|rfl <;>
      decide

theorem trivial_diamond_norm (k:ℕ) : diamondNorm k 1=1 := by
  simp [diamondNorm]

section Groups
variable {G:Type*} [AddCommGroup G]

theorem norm_zero_of_minus (s:G→G) (v:G) (hv:s v= -v) : v+s v=0 := by
  rw [hv]; exact add_neg_cancel v

theorem difference_double (s:G→G) (v:G) (hv:s v= -v) : v-s v=2 • v := by
  simp [hv, two_nsmul]

theorem six_recovers_difference (s:G→G) (v:G) (hv:s v= -v) (h11:11 • v=0) :
    6 • (v-s v)=v := by
  rw [difference_double s v hv, smul_smul]
  change 12 • v=v
  have h12 : 12 • v=11 • v+v := by simpa using add_nsmul v 11 1
  rw [h12,h11,zero_add]

theorem difference_detects_nonzero (s:G→G) (v:G) (hv:s v= -v)
    (h11:11 • v=0) (hne:v≠0) : v-s v≠0 := by
  intro h
  have hh:=six_recovers_difference s v hv h11
  rw [h,smul_zero] at hh
  exact hne hh.symm

theorem difference_is_minus (s:G→+G) (hs:Function.Involutive s) (v:G) :
    s (v-s v)= -(v-s v) := by
  rw [map_sub,hs v]; exact (neg_sub v (s v)).symm

theorem difference_norm_zero (s:G→+G) (hs:Function.Involutive s) (v:G) :
    (v-s v)+s (v-s v)=0 := norm_zero_of_minus s _ (difference_is_minus s hs v)

theorem difference_equivariant (s F:G→+G) (hc:∀v,F (s v)=s (F v)) (v:G) :
    F (v-s v)=F v-s (F v) := by rw [map_sub,hc v]
end Groups

section Functions
variable {R:Type*} [CommRing R] [CharP R 2]

theorem quadratic_function_norm (b n p q:R) :
    (p+q*n)*(p+q*(n+b))=p^2+b*p*q+q^2*(n^2+b*n) := by
  have h2:(2:R)=0:=CharP.cast_eq_zero R 2
  linear_combination p*q*n*h2
end Functions

section Fields
variable {K:Type*} [Field K]

theorem normalized_trace_one (n b:K) (hb:b≠0) : (n+b)/b=n/b+1 := by
  field_simp

theorem normalized_AS_relation (n b:K) (hb:b≠0) :
    (n/b)^2+n/b=(n^2+b*n)/b^2 := by field_simp
end Fields
end KoblitzPrym26
