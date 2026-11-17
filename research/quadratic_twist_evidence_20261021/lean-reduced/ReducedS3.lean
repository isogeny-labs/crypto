import S3System

/-! Exact preprocessing of the actual S3 system. No runtime claim. -/
namespace ReducedS3
open MvPolynomial

def A {R : Type*} [CommRing R] (a z s : R) := s^2-4*s*z-2*z^2-2*a
def B {R : Type*} [CommRing R] (a b z s t : R) := -s*t+2*z*t+s*z^2-2*a*z-4*b
def C0 {R : Type*} [CommRing R] (a b z t : R) := t^2-2*t*z^2+a^2-4*b*z
def reduced {R : Type*} [CommRing R] (a b z s t x y : R) :=
  A a z s*x*y+B a b z s t*(x+y)+C0 a b z t

theorem division_identity {R : Type*} [CommRing R] (a b z s t x y : R) :
    S3System.s3 a b x y z =
      (y^2-2*z*y+z^2)*(x^2-s*x+t) +
      ((s-2*z)*x+z^2-t)*(y^2-s*y+t) + reduced a b z s t x y := by
  unfold S3System.s3 reduced A B C0
  ring

theorem membership_system_iff {R : Type*} [CommRing R] (a b z s t x y : R)
    (hx : x^2-s*x+t=0) (hy : y^2-s*y+t=0) :
    S3System.s3 a b x y z=0 ↔ reduced a b z s t x y=0 := by
  rw [division_identity,hx,hy]
  simp

theorem generated_ideals_equal {R : Type*} [CommRing R] (a b z s t x y : R) :
    Ideal.span ({S3System.s3 a b x y z,x^2-s*x+t,y^2-s*y+t} : Set R) =
      Ideal.span ({reduced a b z s t x y,x^2-s*x+t,y^2-s*y+t} : Set R) := by
  let f := S3System.s3 a b x y z
  let h := reduced a b z s t x y
  let gx := x^2-s*x+t
  let gy := y^2-s*y+t
  have ident : f=(y^2-2*z*y+z^2)*gx+((s-2*z)*x+z^2-t)*gy+h := division_identity a b z s t x y
  apply le_antisymm <;> apply Ideal.span_le.mpr <;> intro q hq
  · simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hq
    rcases hq with rfl|rfl|rfl
    · change f ∈ Ideal.span ({h,gx,gy} : Set R)
      rw [ident]
      exact Ideal.add_mem _ (Ideal.add_mem _
        (Ideal.mul_mem_left _ _ (Ideal.subset_span (by simp [gx])))
        (Ideal.mul_mem_left _ _ (Ideal.subset_span (by simp [gy]))))
        (Ideal.subset_span (by simp [h]))
    · exact Ideal.subset_span (by simp)
    · exact Ideal.subset_span (by simp)
  · simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hq
    rcases hq with rfl|rfl|rfl
    · have hi : h=f-((y^2-2*z*y+z^2)*gx+((s-2*z)*x+z^2-t)*gy) := by rw [ident];ring
      change h ∈ Ideal.span ({f,gx,gy} : Set R)
      rw [hi]
      exact Ideal.sub_mem _ (Ideal.subset_span (by simp [f])) (Ideal.add_mem _
        (Ideal.mul_mem_left _ _ (Ideal.subset_span (by simp [gx])))
        (Ideal.mul_mem_left _ _ (Ideal.subset_span (by simp [gy]))))
    · exact Ideal.subset_span (by simp)
    · exact Ideal.subset_span (by simp)

section Homogeneous
variable {k : Type*} [Field k]
noncomputable abbrev x : MvPolynomial (Fin 2) k := X 0
noncomputable abbrev y : MvPolynomial (Fin 2) k := X 1

theorem reduced_degree2_component (a b z s t : k) :
    homogeneousComponent 2 (reduced (C a) (C b) (C z) (C s) (C t) x y) =
      C (A a z s)*(x*y) := by
  have h2 : (C (A a z s)*(x*y) : MvPolynomial (Fin 2) k).IsHomogeneous 2 :=
    ((isHomogeneous_X k (0 : Fin 2)).mul (isHomogeneous_X k (1 : Fin 2))).C_mul _
  have h1 : (C (B a b z s t)*(x+y) : MvPolynomial (Fin 2) k).IsHomogeneous 1 :=
    ((isHomogeneous_X k (0 : Fin 2)).add (isHomogeneous_X k (1 : Fin 2))).C_mul _
  have h0 : (C (C0 a b z t) : MvPolynomial (Fin 2) k).IsHomogeneous 0 := isHomogeneous_C _ _
  have eq : reduced (C a) (C b) (C z) (C s) (C t) x y =
    C (A a z s)*(x*y)+C (B a b z s t)*(x+y)+C (C0 a b z t) := by
    simp only [reduced,A,B,C0,map_add,map_sub,map_mul,map_pow,map_neg,map_ofNat]
    ring
  rw [eq]
  simp only [map_add,homogeneousComponent_of_mem h2,homogeneousComponent_of_mem h1,homogeneousComponent_of_mem h0]
  norm_num

noncomputable def headIdeal : Ideal (MvPolynomial (Fin 2) k) :=
  Ideal.span ((fun e => monomial e (1 : k)) ''
    {S3System.exponent 2 0,S3System.exponent 0 2,S3System.exponent 1 1})

theorem head_generators : headIdeal (k:=k) =
    Ideal.span ({x*y,x^2,y^2} : Set (MvPolynomial (Fin 2) k)) := by
  have hm (i j : ℕ) : monomial (S3System.exponent i j) (1 : k) =
      (x^i*y^j : MvPolynomial (Fin 2) k) := by
    simp only [S3System.exponent,x,y,X_pow_eq_monomial,monomial_mul_monomial,one_mul]
  unfold headIdeal
  simp only [Set.image_insert_eq,Set.image_singleton,hm,pow_zero,mul_one,one_mul,pow_one]
  congr 1
  ext f
  simp only [Set.mem_insert_iff,Set.mem_singleton_iff]
  tauto

theorem normalized_head (c : k) (hc : c≠0) :
    Ideal.span ({C c*(x*y),x^2,y^2} : Set (MvPolynomial (Fin 2) k)) = headIdeal := by
  rw [head_generators]
  apply le_antisymm <;> apply Ideal.span_le.mpr <;> intro f hf
  · simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hf
    rcases hf with rfl|rfl|rfl
    · exact Ideal.mul_mem_left _ _ (Ideal.subset_span (by simp))
    · exact Ideal.subset_span (by simp)
    · exact Ideal.subset_span (by simp)
  · simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hf
    rcases hf with rfl|rfl|rfl
    · have hm := Ideal.mul_mem_left
        (Ideal.span ({C c*(x*y),x^2,y^2} : Set (MvPolynomial (Fin 2) k))) (C c⁻¹)
        (show C c*(x*y) ∈ Ideal.span ({C c*(x*y),x^2,y^2} : Set (MvPolynomial (Fin 2) k)) from Ideal.subset_span (by simp))
      have heq : C c⁻¹*(C c*(x*y)) = (x*y : MvPolynomial (Fin 2) k) := by
        rw [←mul_assoc,←map_mul,inv_mul_cancel₀ hc,map_one,one_mul]
      rw [heq] at hm
      exact hm
    · exact Ideal.subset_span (by simp)
    · exact Ideal.subset_span (by simp)

theorem head_monomial_member (i j : ℕ) :
    monomial (S3System.exponent i j) (1 : k) ∈ headIdeal ↔
      2 ≤ i ∨ 2 ≤ j ∨ (1 ≤ i ∧ 1 ≤ j) := by
  classical
  rw [headIdeal,mem_ideal_span_monomial_image]
  simp [support_monomial,S3System.exponent_le_iff]

theorem head_cutoff_exact :
    (∀ i j, 2 ≤ i+j → monomial (S3System.exponent i j) (1 : k) ∈ headIdeal) ∧
    (∃ i j, i+j=1 ∧ monomial (S3System.exponent i j) (1 : k) ∉ headIdeal) := by
  constructor
  · intro i j h
    rw [head_monomial_member]
    omega
  · refine ⟨1,0,rfl,?_⟩
    rw [head_monomial_member]
    omega

theorem bilinear_head (u v w : k) :
    homogeneousComponent 2 (C u*(x*y)+C v*(x+y)+C w) = C u*(x*y) := by
  have h2 : (C u*(x*y) : MvPolynomial (Fin 2) k).IsHomogeneous 2 :=
    ((isHomogeneous_X k (0 : Fin 2)).mul (isHomogeneous_X k (1 : Fin 2))).C_mul _
  have h1 : (C v*(x+y) : MvPolynomial (Fin 2) k).IsHomogeneous 1 :=
    ((isHomogeneous_X k (0 : Fin 2)).add (isHomogeneous_X k (1 : Fin 2))).C_mul _
  have h0 : (C w : MvPolynomial (Fin 2) k).IsHomogeneous 0 := isHomogeneous_C _ _
  simp only [map_add,homogeneousComponent_of_mem h2,homogeneousComponent_of_mem h1,homogeneousComponent_of_mem h0]
  norm_num
end Homogeneous

namespace F43
abbrev k := S3System.F43.k
abbrev Poly := MvPolynomial (Fin 2) k
noncomputable def hm : Poly := C 41*x*y+C 39*(x+y)+C 27
noncomputable def ht : Poly := C 17*x*y+C 3*(x+y)+C 41

theorem characteristic : (43 : Poly)=0 := CharP.cast_eq_zero Poly 43

theorem main_remainder :
    reduced (1 : Poly) 0 31 17 9 x y=hm := by
  simp only [reduced,A,B,C0,hm,map_ofNat]
  linear_combination (-88*x*y+387*x+387*y-401)*characteristic

theorem twist_remainder :
    reduced (4 : Poly) 0 24 32 40 x y=ht := by
  simp only [reduced,A,B,C0,ht,map_ofNat]
  linear_combination (-75*x*y+439*x+439*y-1035)*characteristic

theorem twist_remainder_factor : ht=C (17 : k)*(x-C 15)*(y-C 15) := by
  simp only [ht,map_ofNat]
  linear_combination (6*x+6*y-88)*characteristic

theorem both_head_ideals :
    Ideal.span ({C (41 : k)*(x*y),x^2,y^2} : Set Poly) = headIdeal ∧
    Ideal.span ({C (17 : k)*(x*y),x^2,y^2} : Set Poly) = headIdeal := by
  exact ⟨normalized_head 41 (by decide),normalized_head 17 (by decide)⟩

theorem actual_head_components :
    homogeneousComponent 2 hm=C (41 : k)*(x*y) ∧
    homogeneousComponent 2 ht=C (17 : k)*(x*y) := by
  constructor
  · simpa only [hm,mul_assoc] using bilinear_head (41 : k) 39 27
  · simpa only [ht,mul_assoc] using bilinear_head (17 : k) 3 41

#print axioms main_remainder
#print axioms twist_remainder
#print axioms twist_remainder_factor
#print axioms both_head_ideals
#print axioms actual_head_components
end F43

#print axioms division_identity
#print axioms membership_system_iff
#print axioms generated_ideals_equal
#print axioms reduced_degree2_component
#print axioms head_generators
#print axioms normalized_head
#print axioms head_monomial_member
#print axioms head_cutoff_exact
#print axioms bilinear_head
end ReducedS3
