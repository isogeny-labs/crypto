import KoblitzRound13Square
import Mathlib.RingTheory.Localization.Ideal

namespace KoblitzRound14

theorem product_form_residual {R : Type*} [CommRing R]
 (h out a b A B : R) :
 h*out-A*B=(h*out-a*b)+(a-A)*b+A*(b-B) := by ring

theorem linear_product_residual {R : Type*} [CommRing R]
 (h out a b A B w : R) :
 h*(out-w)=(h*out-a*b)+(a-A)*b+A*(b-B)+(A*B-h*w) := by ring

theorem expanded_product_membership {R : Type*} [CommRing R]
 (I : Ideal R) (h out a b A B : R) (k : Nat)
 (hg : h*out-a*b ∈ I) (ha : h^k*(a-A) ∈ I) (hb : h^k*(b-B) ∈ I) :
 h^k*(h*out-A*B) ∈ I := by
 rw [product_form_residual]
 have h1 := I.mul_mem_left (h^k) hg
 have h2 := I.mul_mem_right b ha
 have h3 := I.mul_mem_left A hb
 convert I.add_mem (I.add_mem h1 h2) h3 using 1 <;> ring

theorem linear_product_membership {R : Type*} [CommRing R]
 (I : Ideal R) (h out a b A B w : R) (k : Nat)
 (hg : h*out-a*b ∈ I) (ha : h^k*(a-A) ∈ I) (hb : h^k*(b-B) ∈ I)
 (hw : A*B-h*w ∈ I) : h^(k+1)*(out-w) ∈ I := by
 have he := expanded_product_membership I h out a b A B k hg ha hb
 have hm := I.mul_mem_left (h^k) hw
 convert I.add_mem he hm using 1 <;> ring

/-- The complete saturation is the contraction of the localized ideal.
This characterizes it; it does not compute generators for our circuits. -/
theorem localized_contraction_iff {R : Type*} [CommRing R]
 (I : Ideal R) (h x : R) :
 x ∈ Ideal.comap (algebraMap R (Localization.Away h))
   (I.map (algebraMap R (Localization.Away h))) ↔ ∃ n : Nat,h^n*x ∈ I := by
 change algebraMap R (Localization.Away h) x ∈ I.map _ ↔ _
 rw [IsLocalization.algebraMap_mem_map_algebraMap_iff (Submonoid.powers h)]
 constructor
 · rintro ⟨m,hm,hx⟩
   obtain ⟨n,rfl⟩ := hm
   exact ⟨n,hx⟩
 · rintro ⟨n,hn⟩
   exact ⟨h^n,⟨n,rfl⟩,hn⟩

/-- h-regularity plus containment proves equality with full saturation. -/
theorem saturation_equality_certificate {R : Type*} [CommRing R]
 (I J : Ideal R) (h : R) (hIJ : I ≤ J)
 (hJI : ∀ x ∈ J,∃ n : Nat,h^n*x ∈ I)
 (hreg : ∀ x,h*x ∈ J → x ∈ J) :
 J=Ideal.comap (algebraMap R (Localization.Away h))
   (I.map (algebraMap R (Localization.Away h))) := by
 apply le_antisymm
 · intro x hx
   exact (localized_contraction_iff I h x).mpr (hJI x hx)
 · intro x hx
   obtain ⟨n,hn⟩ := (localized_contraction_iff I h x).mp hx
   have hp : ∀ n : Nat, h^n*x ∈ J → x ∈ J := by
    intro n
    induction n with
    | zero => simpa
    | succ n ih =>
      intro hn
      apply ih
      apply hreg
      convert hn using 1 <;> ring
   exact hp n (hIJ hn)

theorem cross_target_residual {R : Type*} [CommRing R]
 (x y u v c : R) :
 x^2*(v^2+u*v-u^3-u^2-1)-
 ((u*y+c)^2+x*u*(u*y+c)-x^2*u^3-x^2*u^2-x^2)=
 (x*v-u*y-c)*(x*v+u*y+c+x*u) := by ring

theorem char_two_cross_target {R : Type*} [CommRing R] [CharP R 2]
 (x y u v c : R) (hc : x*v=u*y+c)
 (he : v^2+u*v=u^3+u^2+1) :
 x^2*u^3+(x^2+y^2+x*y)*u^2+x*c*u+x^2+c^2=0 := by
 have h := cross_target_residual x y u v c
 have hv : x*v-u*y-c=0 := by rw [hc];ring
 rw [hv,zero_mul] at h
 have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
 have he' : v^2+u*v-u^3-u^2-1=0 := by linear_combination he
 rw [he',mul_zero] at h
 have h' : (u*y+c)^2+x*u*(u*y+c)-x^2*u^3-x^2*u^2-x^2=0 := by linear_combination -h
 have hs : (u*y+c)^2=u^2*y^2+c^2 := by rw [CharTwo.add_sq];ring
 rw [hs] at h'
 linear_combination -h' + h2*(u^2*y^2+x*u^2*y+x*u*c+c^2)

theorem canonical_polarized_dimensions :
 (2*3)^5=7776 ∧ 7776-1=7775 ∧ 7776*19^2=2807136 ∧
 120*7776=933120 := by norm_num

theorem five_remaining_degree_controls :
 5*352=1760 ∧ 5*376=1880 ∧ 5*552=2760 ∧ 5*274=1370 ∧ 5*436=2180 := by norm_num

def kernelMinus : Matrix (Fin 2) (Fin 2) (ZMod 19) := !![0,17;1,17]
def kernelPlus : Matrix (Fin 2) (Fin 2) (ZMod 19) := !![0,17;1,2]
def cartierMinus : Matrix (Fin 2) (Fin 2) (ZMod 19) := !![17,18;2,0]
def cartierPlus : Matrix (Fin 2) (Fin 2) (ZMod 19) := !![2,18;2,0]
theorem actual_cartier_minus : kernelMinus.transpose*cartierMinus=2 • (1 : Matrix (Fin 2) (Fin 2) (ZMod 19)) := by decide +kernel
theorem actual_cartier_plus : kernelPlus.transpose*cartierPlus=2 • (1 : Matrix (Fin 2) (Fin 2) (ZMod 19)) := by decide +kernel

theorem scalar_deck_identity {R : Type*} [CommRing R] (a b c d e : R) :
 (5*a-(a+b+c+d+e))+(5*b-(a+b+c+d+e))+(5*c-(a+b+c+d+e))+
 (5*d-(a+b+c+d+e))+(5*e-(a+b+c+d+e))=0 := by ring

end KoblitzRound14
#print axioms KoblitzRound14.product_form_residual
#print axioms KoblitzRound14.linear_product_residual
#print axioms KoblitzRound14.expanded_product_membership
#print axioms KoblitzRound14.linear_product_membership
#print axioms KoblitzRound14.localized_contraction_iff
#print axioms KoblitzRound14.saturation_equality_certificate
#print axioms KoblitzRound14.cross_target_residual
#print axioms KoblitzRound14.char_two_cross_target
#print axioms KoblitzRound14.canonical_polarized_dimensions
#print axioms KoblitzRound14.five_remaining_degree_controls
#print axioms KoblitzRound14.actual_cartier_minus
#print axioms KoblitzRound14.actual_cartier_plus
#print axioms KoblitzRound14.scalar_deck_identity
