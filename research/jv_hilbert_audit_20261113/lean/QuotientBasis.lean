import DualBasis
import Mathlib.LinearAlgebra.Basis.Basic
import Mathlib.RingTheory.Length

namespace JVHilbertAudit
open MvPolynomial
noncomputable section
def x : A := JVMacaulayAudit.qCoordinate 0
def y : A := JVMacaulayAudit.qCoordinate 1
def h : A := JVMacaulayAudit.qCoordinate 2

theorem basisVector_coordinates :
    basisVector=![1,x,y,h,x*h,y*h,h^2,h^3] := by
  ext i
  fin_cases i
  all_goals simp [basisVector,basisPoly,basisExponent,exponent_monomial,
    quotientMap,x,y,h,JVMacaulayAudit.qCoordinate]

local macro "char_ring" : tactic => `(tactic| (
  ring_nf
  all_goals reduce_mod_char!
  all_goals (try ring_nf)
  all_goals reduce_mod_char!))

theorem quotient_quadratic_relations :
    x^2=5*x*h+37*y*h+23*h^2 ∧
    x*y=59*x*h+67*y*h+72*h^2 ∧
    y^2=82*x*h+98*y*h+62*h^2 := by
  have : CharP A 101 := inferInstance
  have eqs : JVMacaulayAudit.f0 x y h=0 ∧ JVMacaulayAudit.f1 x y h=0 ∧
      JVMacaulayAudit.f2 x y h=0 := JVMacaulayAudit.actual_quotient_equations
  have ex : JVMacaulayAudit.gx x y h=0 := by
    rw [(JVMacaulayAudit.quadratic_certificates x y h).2.2,eqs.2.1,eqs.2.2]
    ring
  have exy : JVMacaulayAudit.gxy x y h=0 := by
    rw [(JVMacaulayAudit.quadratic_certificates x y h).2.1,eqs.2.1,eqs.2.2]
    ring
  have ey : JVMacaulayAudit.gy x y h=0 := by
    rw [(JVMacaulayAudit.quadratic_certificates x y h).1,eqs.1,eqs.2.1,eqs.2.2]
    ring
  unfold JVMacaulayAudit.gx at ex
  unfold JVMacaulayAudit.gxy at exy
  unfold JVMacaulayAudit.gy at ey
  exact ⟨by linear_combination (norm:=char_ring) ex,
    by linear_combination (norm:=char_ring) exy,
    by linear_combination (norm:=char_ring) ey⟩

theorem quotient_cubic_relations : JVMacaulayAudit.CubicNormal x y h :=
  JVMacaulayAudit.actual_cubic_normal_forms x y h
    JVMacaulayAudit.actual_quotient_equations.1
    JVMacaulayAudit.actual_quotient_equations.2.1
    JVMacaulayAudit.actual_quotient_equations.2.2

theorem quotient_quartic_relations : x*h^3=0 ∧ y*h^3=0 ∧ h^4=0 := by
  have van := JVMacaulayAudit.all_degree_four_vanish x y h
    JVMacaulayAudit.actual_quotient_equations.1
    JVMacaulayAudit.actual_quotient_equations.2.1
    JVMacaulayAudit.actual_quotient_equations.2.2
  exact ⟨by simpa using van 1 0 3 (by decide),
    by simpa using van 0 1 3 (by decide),by simpa using van 0 0 4 (by decide)⟩

def combination (v : Fin 8 → F) : A := ∑ i, algebraMap F A (v i)*basisVector i

theorem combination_eq_smul (v : Fin 8 → F) :
    combination v=∑ i, v i • basisVector i := by
  apply Finset.sum_congr rfl
  intro i _
  exact (Algebra.smul_def (v i) (basisVector i)).symm

/-- Multiplication by each actual coordinate in the proposed quotient basis. -/
def multiplicationCoefficients : Fin 3 → Fin 8 → Fin 8 → F :=
  ![![![0,1,0,0,0,0,0,0],![0,0,0,0,5,37,23,0],![0,0,0,0,59,67,72,0],
      ![0,0,0,0,1,0,0,0],![0,0,0,0,0,0,0,93],![0,0,0,0,0,0,0,77],
      ![0,0,0,0,0,0,0,35],![0,0,0,0,0,0,0,0]],
    ![![0,0,1,0,0,0,0,0],![0,0,0,0,59,67,72,0],![0,0,0,0,82,98,62,0],
      ![0,0,0,0,0,1,0,0],![0,0,0,0,0,0,0,77],![0,0,0,0,0,0,0,47],
      ![0,0,0,0,0,0,0,19],![0,0,0,0,0,0,0,0]],
    ![![0,0,0,1,0,0,0,0],![0,0,0,0,1,0,0,0],![0,0,0,0,0,1,0,0],
      ![0,0,0,0,0,0,1,0],![0,0,0,0,0,0,0,35],![0,0,0,0,0,0,0,19],
      ![0,0,0,0,0,0,0,1],![0,0,0,0,0,0,0,0]]]

theorem multiplication_x (j : Fin 8) :
    x*basisVector j=combination (multiplicationCoefficients 0 j) := by
  have : CharP A 101 := inferInstance
  have qr := quotient_quadratic_relations
  have cr := quotient_cubic_relations
  have vr := quotient_quartic_relations
  fin_cases j
  all_goals norm_num [combination,multiplicationCoefficients,basisVector_coordinates,
    Fin.sum_univ_succ,map_ofNat]
  all_goals solve
    | ring
    | linear_combination (norm:=char_ring) qr.1
    | linear_combination (norm:=char_ring) qr.2.1
    | linear_combination (norm:=char_ring) cr.xxh
    | linear_combination (norm:=char_ring) cr.xyh
    | linear_combination (norm:=char_ring) cr.xhh
    | linear_combination (norm:=char_ring) vr.1

theorem multiplication_y (j : Fin 8) :
    y*basisVector j=combination (multiplicationCoefficients 1 j) := by
  have : CharP A 101 := inferInstance
  have qr := quotient_quadratic_relations
  have cr := quotient_cubic_relations
  have vr := quotient_quartic_relations
  fin_cases j
  all_goals norm_num [combination,multiplicationCoefficients,basisVector_coordinates,
    Fin.sum_univ_succ,map_ofNat]
  all_goals solve
    | ring
    | linear_combination (norm:=char_ring) qr.2.1
    | linear_combination (norm:=char_ring) qr.2.2
    | linear_combination (norm:=char_ring) cr.xyh
    | linear_combination (norm:=char_ring) cr.yyh
    | linear_combination (norm:=char_ring) cr.yhh
    | linear_combination (norm:=char_ring) vr.2.1

theorem multiplication_h (j : Fin 8) :
    h*basisVector j=combination (multiplicationCoefficients 2 j) := by
  have : CharP A 101 := inferInstance
  have cr := quotient_cubic_relations
  have vr := quotient_quartic_relations
  fin_cases j
  all_goals norm_num [combination,multiplicationCoefficients,basisVector_coordinates,
    Fin.sum_univ_succ,map_ofNat]
  all_goals solve
    | ring
    | linear_combination (norm:=char_ring) cr.xhh
    | linear_combination (norm:=char_ring) cr.yhh
    | linear_combination (norm:=char_ring) vr.2.2

theorem actual_coordinate_multiplication (s : Fin 3) (j : Fin 8) :
    quotientMap (X s)*basisVector j=combination (multiplicationCoefficients s j) := by
  fin_cases s
  · exact multiplication_x j
  · exact multiplication_y j
  · exact multiplication_h j

def basisSpan : Submodule F A := Submodule.span F (Set.range basisVector)

theorem basisVector_mem (i : Fin 8) : basisVector i∈basisSpan :=
  Submodule.subset_span ⟨i,rfl⟩

theorem combination_mem (v : Fin 8 → F) : combination v∈basisSpan := by
  rw [combination_eq_smul]
  exact basisSpan.sum_mem fun i _ => basisSpan.smul_mem _ (basisVector_mem i)

theorem basisSpan_closed_coordinate (s : Fin 3) {z : A} (hz : z∈basisSpan) :
    quotientMap (X s)*z∈basisSpan := by
  induction hz using Submodule.span_induction with
  | mem z hz =>
    obtain ⟨j,rfl⟩ := hz
    rw [actual_coordinate_multiplication]
    exact combination_mem _
  | zero => simp
  | add z w _ _ hz hw => simpa only [mul_add] using basisSpan.add_mem hz hw
  | smul c z _ hz => simpa only [mul_smul_comm] using basisSpan.smul_mem c hz

theorem quotientMap_mem_basisSpan (p : Poly) : quotientMap p∈basisSpan := by
  have h1 : (1:A)∈basisSpan := by
    simpa [basisVector_coordinates] using basisVector_mem 0
  induction p using MvPolynomial.induction_on with
  | C a =>
    change algebraMap F A a∈basisSpan
    rw [Algebra.algebraMap_eq_smul_one]
    exact basisSpan.smul_mem a h1
  | add p q hp hq => simpa only [map_add] using basisSpan.add_mem hp hq
  | mul_X p s hp =>
    rw [map_mul,mul_comm]
    exact basisSpan_closed_coordinate s hp

theorem basisSpan_eq_top : basisSpan=⊤ := by
  apply top_unique
  intro z _
  obtain ⟨p,rfl⟩ := Ideal.Quotient.mkₐ_surjective F JVMacaulayAudit.sourceIdeal z
  exact quotientMap_mem_basisSpan p

def actualBasis : Module.Basis (Fin 8) F A := Module.Basis.mk basisVector_linearIndependent
  (by change ⊤ ≤ basisSpan; rw [basisSpan_eq_top])

instance : Module.Finite F A := Module.Finite.of_basis actualBasis

theorem actual_coordinate_algebra_dimension : Module.finrank F A=8 := by
  rw [Module.finrank_eq_card_basis actualBasis]
  rfl

theorem actual_scalar_module_length : Module.length F A=8 := by
  rw [Module.length_eq_finrank,actual_coordinate_algebra_dimension]
  rfl

theorem actual_geometric_zero_iff {L : Type*} [Field L] [CharP L 101] (u v w : L) :
    (JVMacaulayAudit.f0 u v w=0 ∧ JVMacaulayAudit.f1 u v w=0 ∧
      JVMacaulayAudit.f2 u v w=0) ↔ u=0 ∧ v=0 ∧ w=0 := by
  constructor
  · rintro ⟨h0,h1,h2⟩
    have van := JVMacaulayAudit.all_degree_four_vanish u v w h0 h1 h2
    have hu : u^4=0 := by simpa using van 4 0 0 (by decide)
    have hv : v^4=0 := by simpa using van 0 4 0 (by decide)
    have hw : w^4=0 := by simpa using van 0 0 4 (by decide)
    exact ⟨eq_zero_of_pow_eq_zero hu,eq_zero_of_pow_eq_zero hv,eq_zero_of_pow_eq_zero hw⟩
  · rintro ⟨rfl,rfl,rfl⟩
    simp [JVMacaulayAudit.f0,JVMacaulayAudit.f1,JVMacaulayAudit.f2]

theorem literal_jv_exact_algebra_certificate :
    Module.finrank F A=8 ∧ Module.length F A=8 ∧
    (X 2^3 : Poly)∉JVMacaulayAudit.sourceIdeal ∧
    (∀ d : ℕ, (∀ i j k : ℕ, d ≤ i+j+k →
      (X 0^i*X 1^j*X 2^k : Poly)∈JVMacaulayAudit.sourceIdeal) ↔ 4 ≤ d) ∧
    ∀ (L : Type) [Field L] [CharP L 101] (u v w : L),
      (JVMacaulayAudit.f0 u v w=0 ∧ JVMacaulayAudit.f1 u v w=0 ∧
        JVMacaulayAudit.f2 u v w=0) ↔ u=0 ∧ v=0 ∧ w=0 := by
  refine ⟨actual_coordinate_algebra_dimension,actual_scalar_module_length,
    literal_jv_sharp_monomial_cutoff.1,literal_jv_sharp_monomial_cutoff.2,?_⟩
  intro L _ _ u v w
  exact actual_geometric_zero_iff u v w

#print axioms literal_jv_exact_algebra_certificate

end
end JVHilbertAudit
