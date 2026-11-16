import KoblitzRound12
import Mathlib.RingTheory.Norm.Transitivity

namespace KoblitzRound13

theorem quadratic_norm_residual {R : Type*} [CommRing R] (a b x y d : R) :
 (a+b*y)*(a+b*(-x-y))-(a^2-a*b*x-b^2*d) = -b^2*(y^2+x*y-d) := by ring

theorem quadratic_norm_zero {F : Type*} [Field F] (a b x y d : F)
 (hy : y^2+x*y-d=0) (hn : a^2-a*b*x-b^2*d=0) :
 a+b*y=0 ∨ a+b*(-x-y)=0 := by
 have h := quadratic_norm_residual a b x y d
 rw [hy,hn] at h
 exact mul_eq_zero.mp (by simpa using h)

theorem conjugate_root_factorization {R : Type*} [CommRing R] (v y x d : R) :
 (v-y)*(v+y+x)=(v^2+x*v-d)-(y^2+x*y-d) := by ring

theorem same_x_two_branches {F : Type*} [Field F] (v y x d : F)
 (hv : v^2+x*v-d=0) (hy : y^2+x*y-d=0) : v=y ∨ v=-x-y := by
 have h : (v-y)*(v+y+x)=0 := by rw [conjugate_root_factorization,hv,hy];ring
 rcases mul_eq_zero.mp h with h|h
 · exact Or.inl (sub_eq_zero.mp h)
 · right; apply sub_eq_zero.mp; linear_combination h

theorem recover_linear_y {F : Type*} [Field F] (a b y : F)
 (hb : b≠0) (h : a+b*y=0) : y=-a/b := by
 apply (eq_div_iff hb).mpr
 linear_combination h

theorem norm_tower {R S T : Type*} [CommRing R] [CommRing S] [CommRing T]
 [Algebra R S] [Algebra R T] [Algebra S T] [IsScalarTower R S T]
 [Module.Free R S] [Module.Free S T] [Module.Finite R S] [Module.Finite S T]
 (a : T) : Algebra.norm R (Algebra.norm S a)=Algebra.norm R a := by
 exact Algebra.norm_norm (S:=S)

theorem determinant_multiplicative {R : Type*} [CommRing R]
 (a b : Matrix (Fin 11) (Fin 11) R) : (a*b).det=a.det*b.det := by
 exact Matrix.det_mul a b

theorem inverse_cancellation_saturation {R : Type*} [CommRing R]
 (h out a b z : R) : h*(out*z-b*h)=(h*out-a*b)*z+b*(a*z-h^2) := by ring

theorem shared_factor_saturation {R : Type*} [CommRing R]
 (h out other a b c : R) :
 h*(out*c-other*b)=(h*out-a*b)*c-(h*other-a*c)*b := by ring

theorem associative_saturation_residual {R : Type*} [CommRing R]
 (h u v m n a b c d e f : R) :
 h*(h*u-m*c)+c*(h*m-a*b)-h*(h*v-n*f)-f*(h*n-d*e)
 =h^2*(u-v)-a*b*c+d*e*f := by ring

theorem associative_saturation {R : Type*} [CommRing R]
 (h u v m n a b c d e f : R) (hp : a*b*c=d*e*f) :
 h^2*(u-v)=h*(h*u-m*c)+c*(h*m-a*b)-h*(h*v-n*f)-f*(h*n-d*e) := by
 rw [associative_saturation_residual,hp];ring

theorem two_product_saturation {R : Type*} [CommRing R]
 (h u v w z a b c d : R) :
 h^2*(u*v-w*z)=(h*u-a*b)*(h*v)+a*b*(h*v-c*d)
 -(h*w-a*c)*(h*z)-a*c*(h*z-b*d) := by ring

theorem reciprocal_chain_saturation {R : Type*} [CommRing R]
 (h a z w : R) : h^2*(w-a)=a*(z*w-h^2)-w*(a*z-h^2) := by ring

theorem power_series_constant_uniqueness {R : Type*} [CommRing R]
 (f g : PowerSeries R) (c : R) (h : f-g=PowerSeries.C c)
 (h0 : PowerSeries.constantCoeff (f-g)=0) : f=g := by
 rw [h] at h0
 have hc : c=0 := by simpa using h0
 apply sub_eq_zero.mp
 simpa [hc] using h

theorem sum_one_unique {ι : Type*} [Fintype ι] (w : ι → ℕ)
 (h : ∑ i,w i=1) : ∃ i,w i=1 ∧ ∀ j,j≠i → w j=0 := by
 classical
 have hi : ∃ i,0<w i := by
  by_contra hn
  push Not at hn
  have hz : ∑ i,w i=0 := Finset.sum_eq_zero (fun i _ => by have := hn i;omega)
  omega
 obtain ⟨i,hi⟩ := hi
 have hile : w i≤∑ j,w j := Finset.single_le_sum (fun j _ => Nat.zero_le (w j)) (Finset.mem_univ i)
 have hwi : w i=1 := by omega
 refine ⟨i,hwi,?_⟩
 intro j hji
 have hem : i∈(Finset.univ.erase j : Finset ι) := by simp [Ne.symm hji]
 have hle : w i≤∑ k∈Finset.univ.erase j,w k :=
  Finset.single_le_sum (fun k _ => Nat.zero_le (w k)) hem
 have hs := Finset.sum_erase_add Finset.univ w (Finset.mem_univ j)
 omega

theorem simple_norm_fiber {ι : Type*} [Fintype ι] (f v : ι → ℕ)
 (hf : ∀ i,0<f i) (h : ∑ i,f i*v i=1) :
 ∃ i,f i=1 ∧ v i=1 ∧ ∀ j,j≠i → v j=0 := by
 obtain ⟨i,hi,hrest⟩ := sum_one_unique (fun i=>f i*v i) h
 have hfi := Nat.eq_one_of_mul_eq_one_right hi
 have hvi := Nat.eq_one_of_mul_eq_one_left hi
 refine ⟨i,hfi,hvi,?_⟩
 intro j hj
 have hz := hrest j hj
 rcases Nat.mul_eq_zero.mp hz with hz|hz
 · have := hf j;omega
 · exact hz

theorem five_deck_augmentation_zero {A : Type*} [AddCommGroup A] (v : Fin 5 → A) :
 (∑ i,((5:ℕ) • v i-(∑ j,v j)))=0 := by
 rw [Finset.sum_sub_distrib,←Finset.smul_sum]
 simp

theorem nonzero_five_translation_transitive (a b c : ZMod 5) (hc : c≠0) :
 ∃ k : ZMod 5,a+k*c=b := by
 let : Fact (Nat.Prime 5) := ⟨by decide⟩
 refine ⟨(b-a)/c,?_⟩
 field_simp [hc]
 ring

theorem five_fiber_degrees : 2*250=500 ∧ 4*145=580 ∧
 5*100=500 ∧ 5*116=580 := by norm_num

theorem pushed_partitions : 84+320+100=504 ∧ 412+116=528 := by norm_num

end KoblitzRound13

#print axioms KoblitzRound13.quadratic_norm_residual
#print axioms KoblitzRound13.quadratic_norm_zero
#print axioms KoblitzRound13.conjugate_root_factorization
#print axioms KoblitzRound13.same_x_two_branches
#print axioms KoblitzRound13.recover_linear_y
#print axioms KoblitzRound13.norm_tower
#print axioms KoblitzRound13.determinant_multiplicative
#print axioms KoblitzRound13.inverse_cancellation_saturation
#print axioms KoblitzRound13.shared_factor_saturation
#print axioms KoblitzRound13.associative_saturation_residual
#print axioms KoblitzRound13.associative_saturation
#print axioms KoblitzRound13.two_product_saturation
#print axioms KoblitzRound13.reciprocal_chain_saturation
#print axioms KoblitzRound13.power_series_constant_uniqueness
#print axioms KoblitzRound13.sum_one_unique
#print axioms KoblitzRound13.simple_norm_fiber
#print axioms KoblitzRound13.five_deck_augmentation_zero
#print axioms KoblitzRound13.nonzero_five_translation_transitive
#print axioms KoblitzRound13.five_fiber_degrees
#print axioms KoblitzRound13.pushed_partitions
