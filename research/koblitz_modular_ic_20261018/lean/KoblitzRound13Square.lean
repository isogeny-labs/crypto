import KoblitzRound13
import Mathlib.Algebra.CharP.Two

namespace KoblitzRound13Square

theorem weighted_square_sum {R ι : Type*} [CommRing R] [CharP R 2]
 (s : Finset ι) (c v : ι → R) :
 (∑ i ∈ s,c i*v i)^2=∑ i ∈ s,(c i)^2*(v i)^2 := by
 rw [CharTwo.sum_sq]
 simp only [mul_pow]

theorem linear_square_saturation {R ι : Type*} [CommRing R] [CharP R 2]
 (s : Finset ι) (h out a c0 : R) (c v w : ι → R) :
 h*(out-(∑ i ∈ s,(c i)^2*w i)-c0^2*h)=
 (h*out-a^2)-(∑ i ∈ s,(c i)^2*(h*w i-(v i)^2))+
 (a-((∑ i ∈ s,c i*v i)+c0*h))^2 := by
 have hs : (∑ i ∈ s,(c i)^2*(h*w i-(v i)^2))=
   h*(∑ i ∈ s,(c i)^2*w i)-(∑ i ∈ s,(c i)^2*(v i)^2) := by
  simp_rw [mul_sub,Finset.sum_sub_distrib]
  congr 1
  rw [Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro i hi
  ring
 rw [hs,sub_sq,CharTwo.add_sq,weighted_square_sum]
 have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
 simp only [h2,zero_mul,sub_zero,mul_pow]
 ring_nf
 simp [h2,CharTwo.sub_eq_add]

theorem membership_square_depth {R : Type*} [CommRing R] (I : Ideal R)
 (h g e : R) (k : Nat) (hr : h*g-e^2 ∈ I) (he : h^k*e ∈ I) :
 h^(2*k+1)*g ∈ I := by
 have hs : h^(2*k)*e^2 ∈ I := by
  have := I.mul_mem_left (h^k*e) he
  convert this using 1 <;> ring
 have hm := I.mul_mem_left (h^(2*k)) hr
 have := I.add_mem hm hs
 convert this using 1 <;> ring

end KoblitzRound13Square
#print axioms KoblitzRound13Square.weighted_square_sum
#print axioms KoblitzRound13Square.linear_square_saturation
#print axioms KoblitzRound13Square.membership_square_depth
