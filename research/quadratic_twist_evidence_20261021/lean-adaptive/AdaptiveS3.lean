import ReducedS3
import S3DegreeFixture

/-!
Certificates for the adaptive row solver on the actual F43 S3 inputs.
The operational trace and row selection are independently checked in native code.
-/
namespace AdaptiveS3
open MvPolynomial ReducedS3 S3System

/-- A derived row whose explicit witness is a combination of the three inputs
does not change their generated ideal when inserted into the row pool. -/
theorem adjoining_checked_row {R : Type*} [CommRing R]
    (f g h a b c : R) :
    Ideal.span ({f,g,h,a*f+b*g+c*h} : Set R) =
      Ideal.span ({f,g,h} : Set R) := by
  apply le_antisymm
  · apply Ideal.span_le.mpr
    intro q hq
    simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hq
    rcases hq with rfl | rfl | rfl | rfl
    · exact Ideal.subset_span (by simp)
    · exact Ideal.subset_span (by simp)
    · exact Ideal.subset_span (by simp)
    · exact Ideal.add_mem _ (Ideal.add_mem _
        (Ideal.mul_mem_left _ _ (Ideal.subset_span (by simp)))
        (Ideal.mul_mem_left _ _ (Ideal.subset_span (by simp))))
        (Ideal.mul_mem_left _ _ (Ideal.subset_span (by simp)))
  · apply Ideal.span_le.mpr
    intro q hq
    simp only [Set.mem_insert_iff,Set.mem_singleton_iff] at hq
    rcases hq with rfl | rfl | rfl
    · exact Ideal.subset_span (by simp)
    · exact Ideal.subset_span (by simp)
    · exact Ideal.subset_span (by simp)

namespace F43
noncomputable section
set_option maxHeartbeats 3000000
abbrev k := S3System.F43.k
abbrev Poly := MvPolynomial (Fin 2) k
abbrev x : Poly := ReducedS3.x
abbrev y : Poly := ReducedS3.y
abbrev hm : Poly := ReducedS3.F43.hm
abbrev ht : Poly := ReducedS3.F43.ht
abbrev gx : Poly := S3DegreeFixture.gx
abbrev gy : Poly := S3DegreeFixture.gy

theorem main_membership_x : gx = x^2+C 26*x+C 9 := by
  simp only [gx,S3DegreeFixture.gx,S3DegreeFixture.term_product,map_one,map_ofNat]
  ring
theorem main_membership_y : gy = y^2+C 26*y+C 9 := by
  simp only [gy,S3DegreeFixture.gy,S3DegreeFixture.term_product,map_one,map_ofNat]
  ring

def mainDegree2Rows : Fin 3 → Poly := ![hm,gx,gy]

private theorem hm_terms : hm =
    S3DegreeFixture.term 1 1 41 + S3DegreeFixture.term 1 0 39 +
    S3DegreeFixture.term 0 1 39 + S3DegreeFixture.term 0 0 27 := by
  simp only [S3DegreeFixture.term_product,hm,ReducedS3.F43.hm,map_ofNat]
  ring

/-- The coefficient functional separates 1 from the D=2 input space.
All three F43 inputs are quadratic, so no positive-degree multiples occur at D=2. -/
def mainDual : Poly →ₗ[k] k :=
  lcoeff k (S3System.exponent 0 0) +
  34 • lcoeff k (S3System.exponent 2 0) +
  34 • lcoeff k (S3System.exponent 0 2) +
  35 • lcoeff k (S3System.exponent 1 1)

theorem mainDual_one : mainDual 1 = 1 := by
  norm_num [mainDual,lcoeff,S3System.exponent,coeff_one,Finsupp.ext_iff,Fin.forall_fin_two]

theorem mainDual_rows : ∀ i : Fin 3, mainDual (mainDegree2Rows i) = 0 := by
  intro i
  fin_cases i <;>
    norm_num [mainDual,mainDegree2Rows,hm_terms,gx,gy,
      S3DegreeFixture.gx,S3DegreeFixture.gy,S3DegreeFixture.term,
      S3DegreeFixture.monomial_times_C,lcoeff,coeff_monomial,coeff_C,coeff_one,
      S3System.exponent,Finsupp.ext_iff,Fin.forall_fin_two,mul_add,add_mul] <;>
    decide +kernel

theorem main_no_degree2_unit :
    (1 : Poly) ∉ Submodule.span k (Set.range mainDegree2Rows) := by
  have hs : Submodule.span k (Set.range mainDegree2Rows) ≤ LinearMap.ker mainDual := by
    apply Submodule.span_le.mpr
    rintro _ ⟨i,rfl⟩
    exact mainDual_rows i
  intro h
  have hz : mainDual 1=0 := hs h
  rw [mainDual_one] at hz
  exact one_ne_zero hz

def mainW0 : Poly := 37+16*x+16*y
def mainW1 : Poly := 21+32*y
def mainW2 : Poly := 21+32*x

/-- The D=3 row space contains 1, with an explicit original-input witness. -/
theorem main_degree3_unit : mainW0*hm+mainW1*gx+mainW2*gy=1 := by
  simp only [mainW0,mainW1,mainW2,hm,ReducedS3.F43.hm,
    main_membership_x,main_membership_y,map_ofNat]
  linear_combination
    (32+63*x+63*y+15*x^2+15*y^2+103*x*y+16*x^2*y+16*x*y^2)*
      ReducedS3.F43.characteristic

/-- Integer polynomial behind the F43 unit. Native IDC1 checks this same
identity separately; the Lean characteristic theorem supplies the field step. -/
theorem cleared_main_degree3_unit {R : Type*} [CommRing R] (x y : R) :
    (37+16*x+16*y)*(41*x*y+39*x+39*y+27) +
    (21+32*y)*(x^2+26*x+9) +
    (21+32*x)*(y^2+26*y+9)-1 =
    43*(32+63*x+63*y+15*x^2+15*y^2+103*x*y+16*x^2*y+16*x*y^2) := by ring

def twistGx : Poly := x^2+C 11*x+C 40
def twistGy : Poly := y^2+C 11*y+C 40

theorem twist_membership_x : twistGx=(x-C (15:k))*(x-C (17:k)) := by
  simp only [twistGx,map_ofNat]
  linear_combination (x-5)*ReducedS3.F43.characteristic
theorem twist_membership_y : twistGy=(y-C (15:k))*(y-C (17:k)) := by
  simp only [twistGy,map_ofNat]
  linear_combination (y-5)*ReducedS3.F43.characteristic

/-- The actual finite-field input has exactly three affine factor-base roots.
This rules out the fourth grid point (17,17) by the reduced S3 constraint. -/
theorem twist_exact_roots (u v : k) :
    (u^2+11*u+40=0 ∧ v^2+11*v+40=0 ∧
      17*u*v+3*u+3*v+41=0) ↔
    (u=15 ∧ v=15) ∨ (u=15 ∧ v=17) ∨ (u=17 ∧ v=15) := by
  have hc : (43:k)=0 := CharP.cast_eq_zero k 43
  have hx : u^2+11*u+40=(u-15)*(u-17) := by
    linear_combination (u-5)*hc
  have hy : v^2+11*v+40=(v-15)*(v-17) := by
    linear_combination (v-5)*hc
  have ht' : 17*u*v+3*u+3*v+41=17*(u-15)*(v-15) := by
    linear_combination (6*u+6*v-88)*hc
  constructor
  · rintro ⟨hxu,hvy,huv⟩
    rw [hx] at hxu
    rw [hy] at hvy
    rw [ht'] at huv
    rcases mul_eq_zero.mp hxu with hu | hu <;>
      rcases mul_eq_zero.mp hvy with hv | hv
    · exact Or.inl ⟨sub_eq_zero.mp hu,sub_eq_zero.mp hv⟩
    · exact Or.inr (Or.inl ⟨sub_eq_zero.mp hu,sub_eq_zero.mp hv⟩)
    · exact Or.inr (Or.inr ⟨sub_eq_zero.mp hu,sub_eq_zero.mp hv⟩)
    · have u17 := sub_eq_zero.mp hu
      have v17 := sub_eq_zero.mp hv
      subst u
      subst v
      exfalso
      norm_num at huv
      exact (show (68:k) ≠ 0 by decide) huv
  · rintro (⟨rfl,rfl⟩ | ⟨rfl,rfl⟩ | ⟨rfl,rfl⟩) <;> decide +kernel

/-- For this actual twist input, the common quadratic vanishing space of
the three factor-base roots is exactly the degree-two input row space. -/
theorem twist_quadratic_vanishing_space (a b c d e f : k)
    (h0 : 225*a+225*b+225*c+15*d+15*e+f=0)
    (h1 : 225*a+255*b+289*c+15*d+17*e+f=0)
    (h2 : 289*a+255*b+225*c+17*d+15*e+f=0) :
    C a*x^2+C b*x*y+C c*y^2+C d*x+C e*y+C f =
      C a*twistGx+C (38*b)*ht+C c*twistGy := by
  have hc : (43:k)=0 := CharP.cast_eq_zero k 43
  have hd : d=11*a+28*b := by
    linear_combination 22*h2-22*h0-(d+33*a+16*b)*hc
  have he : e=11*c+28*b := by
    linear_combination 22*h1-22*h0-(e+33*c+16*b)*hc
  have hf : f=40*a+10*b+40*c := by
    linear_combination h0-15*hd-15*he-(10*a+25*b+10*c)*hc
  rw [hd,he,hf]
  simp only [twistGx,twistGy,ht,ReducedS3.F43.ht,map_ofNat,map_add,map_mul]
  linear_combination (-15*C b*x*y-2*C b*x-2*C b*y-36*C b)*
    ReducedS3.F43.characteristic

#print axioms mainDual_one
#print axioms mainDual_rows
#print axioms main_no_degree2_unit
#print axioms main_degree3_unit
#print axioms cleared_main_degree3_unit
#print axioms twist_membership_x
#print axioms twist_membership_y
#print axioms twist_exact_roots
#print axioms twist_quadratic_vanishing_space
end
end F43
#print axioms adjoining_checked_row
end AdaptiveS3
