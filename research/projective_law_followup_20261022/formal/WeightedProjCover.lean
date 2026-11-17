import WeightedRingGraded
import Mathlib.AlgebraicGeometry.ProjectiveSpectrum.Basic

set_option autoImplicit false
noncomputable section
namespace WeightedProjCover
open MvPolynomial Common22WeightedProjHead Common22WeightedProjQuotient WeightedRingComponent WeightedRingGraded
open AlgebraicGeometry
variable {K : Type*} [Field K]

def gen (a3 a2 a1 a0 : K) (i : Fin 3) : WeightedRing a3 a2 a1 a0 :=
  Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (X i)

theorem gen_mem (a3 a2 a1 a0 : K) (i : Fin 3) :
    gen a3 a2 a1 a0 i ∈ degreeSubmodule a3 a2 a1 a0 (weights i) := by
  change component a3 a2 a1 a0 (weights i) (gen a3 a2 a1 a0 i) =
    gen a3 a2 a1 a0 i
  simpa only [gen, ite_true] using
    component_homogeneous a3 a2 a1 a0 (weights i) (weights i) (X i)
      (isWeightedHomogeneous_X (R := K) weights i)

theorem gen_adjoin_top (a3 a2 a1 a0 : K) :
    Algebra.adjoin (degreeSubmodule a3 a2 a1 a0 0)
      (Set.range (gen a3 a2 a1 a0)) = ⊤ := by
  let S := Algebra.adjoin (degreeSubmodule a3 a2 a1 a0 0)
    (Set.range (gen a3 a2 a1 a0))
  apply top_unique
  intro q hq
  clear hq
  obtain ⟨p, rfl⟩ := Ideal.Quotient.mk_surjective q
  change Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) p ∈ S
  induction p using MvPolynomial.induction_on with
  | C c =>
      have hC : Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (C c) ∈
          degreeSubmodule a3 a2 a1 a0 0 := by
        change component a3 a2 a1 a0 0
          (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (C c)) = _
        simpa only [ite_true] using
          component_homogeneous a3 a2 a1 a0 0 0 (C c)
            (isWeightedHomogeneous_C weights c)
      exact S.algebraMap_mem ⟨_, hC⟩
  | add p q hp hq =>
      simpa only [map_add] using S.add_mem hp hq
  | mul_X p i hp =>
      simpa only [map_mul, gen] using
        S.mul_mem hp (Algebra.subset_adjoin ⟨i, rfl⟩)

theorem three_basic_opens_cover (a3 a2 a1 a0 : K) :
    (⨆ i : Fin 3, Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (gen a3 a2 a1 a0 i)) = ⊤ := by
  exact Proj.iSup_basicOpen_eq_top' (degreeSubmodule a3 a2 a1 a0)
    (gen a3 a2 a1 a0) (fun i => ⟨weights i, gen_mem a3 a2 a1 a0 i⟩)
    (gen_adjoin_top a3 a2 a1 a0)

theorem y_sq_eq_z_mul (a3 a2 a1 a0 : K) :
    (gen a3 a2 a1 a0 1) ^ 2 =
      gen a3 a2 a1 a0 2 *
        ((gen a3 a2 a1 a0 0) ^ 5 +
          algebraMap K (WeightedRing a3 a2 a1 a0) a3 *
            (gen a3 a2 a1 a0 0) ^ 3 * (gen a3 a2 a1 a0 2) ^ 2 +
          algebraMap K (WeightedRing a3 a2 a1 a0) a2 *
            (gen a3 a2 a1 a0 0) ^ 2 * (gen a3 a2 a1 a0 2) ^ 3 +
          algebraMap K (WeightedRing a3 a2 a1 a0) a1 *
            gen a3 a2 a1 a0 0 * (gen a3 a2 a1 a0 2) ^ 4 +
          algebraMap K (WeightedRing a3 a2 a1 a0) a0 *
            (gen a3 a2 a1 a0 2) ^ 5) := by
  have hzero : Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0)
      (equation a3 a2 a1 a0) = 0 := by
    exact Ideal.Quotient.eq_zero_iff_mem.mpr
      (Ideal.subset_span (by simp))
  simp only [equation, map_sub, map_add, map_mul, map_pow] at hzero
  have heq := sub_eq_zero.mp hzero
  have hC (a : K) : Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (C a) =
      algebraMap K (WeightedRing a3 a2 a1 a0) a := by
    rw [← MvPolynomial.algebraMap_eq]
    exact Ideal.Quotient.mk_algebraMap K (relationIdeal a3 a2 a1 a0) a
  calc
    _ = (gen a3 a2 a1 a0 0) ^ 5 * gen a3 a2 a1 a0 2 +
        algebraMap K (WeightedRing a3 a2 a1 a0) a3 *
          ((gen a3 a2 a1 a0 0) ^ 3 * (gen a3 a2 a1 a0 2) ^ 3) +
        algebraMap K (WeightedRing a3 a2 a1 a0) a2 *
          ((gen a3 a2 a1 a0 0) ^ 2 * (gen a3 a2 a1 a0 2) ^ 4) +
        algebraMap K (WeightedRing a3 a2 a1 a0) a1 *
          (gen a3 a2 a1 a0 0 * (gen a3 a2 a1 a0 2) ^ 5) +
        algebraMap K (WeightedRing a3 a2 a1 a0) a0 *
          (gen a3 a2 a1 a0 2) ^ 6 := by simpa [gen, hC] using heq
    _ = _ := by ring

theorem y_open_le_z_open (a3 a2 a1 a0 : K) :
    Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (gen a3 a2 a1 a0 1) ≤
      Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (gen a3 a2 a1 a0 2) := by
  intro p hp
  rw [Proj.mem_basicOpen] at hp ⊢
  intro hz
  have hy2 : (gen a3 a2 a1 a0 1) ^ 2 ∈ p.asHomogeneousIdeal.toIdeal := by
    rw [y_sq_eq_z_mul]
    exact p.asHomogeneousIdeal.toIdeal.mul_mem_right _ hz
  exact hp (p.isPrime.mem_of_pow_mem 2 hy2)

theorem two_basic_opens_cover (a3 a2 a1 a0 : K) :
    Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (gen a3 a2 a1 a0 0) ⊔
      Proj.basicOpen (degreeSubmodule a3 a2 a1 a0) (gen a3 a2 a1 a0 2) = ⊤ := by
  have hthree := three_basic_opens_cover a3 a2 a1 a0
  apply top_unique
  rw [← hthree]
  apply iSup_le
  intro i
  fin_cases i
  · exact le_sup_left
  · exact (y_open_le_z_open a3 a2 a1 a0).trans le_sup_right
  · exact le_sup_right

theorem x_open_affine (a3 a2 a1 a0 : K) :
    IsAffineOpen (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (gen a3 a2 a1 a0 0)) := by
  exact Proj.isAffineOpen_basicOpen (degreeSubmodule a3 a2 a1 a0)
    (gen a3 a2 a1 a0 0) (gen_mem a3 a2 a1 a0 0) (by simp [weights])

theorem z_open_affine (a3 a2 a1 a0 : K) :
    IsAffineOpen (Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (gen a3 a2 a1 a0 2)) := by
  exact Proj.isAffineOpen_basicOpen (degreeSubmodule a3 a2 a1 a0)
    (gen a3 a2 a1 a0 2) (gen_mem a3 a2 a1 a0 2) (by simp [weights])

#print axioms three_basic_opens_cover
#print axioms y_sq_eq_z_mul
#print axioms y_open_le_z_open
#print axioms two_basic_opens_cover
#print axioms x_open_affine
#print axioms z_open_affine

end WeightedProjCover
