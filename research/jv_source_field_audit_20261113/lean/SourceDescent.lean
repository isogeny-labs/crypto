import ActualCurve
import ActualIdeal

namespace JVSourceFieldAudit
open MvPolynomial
noncomputable section

abbrev Poly := MvPolynomial (Fin 2) F
abbrev ExtPoly := MvPolynomial (Fin 2) K
def f0 : Poly := JVMacaulayAudit.f0 (X 0) (X 1) 1
def f1 : Poly := JVMacaulayAudit.f1 (X 0) (X 1) 1
def f2 : Poly := JVMacaulayAudit.f2 (X 0) (X 1) 1

def symmetric : ExtPoly :=
  C (targetX^2)*X 0^2+C (-2*targetX)*X 0*X 1+X 1^2+
    C (-2*a*targetX-4*b)*X 0+C (-4*targetX^2-2*a)*X 1+C (a^2-4*b*targetX)

local macro "alpha_ring" : tactic => `(tactic| (
  have : CharP K 101 := inferInstance
  try ring_nf
  all_goals (try simp only [alpha_cube,alpha_four,alpha_five,alpha_six])
  all_goals (try ring_nf)
  all_goals reduce_mod_char!
  all_goals (try ring_nf)
  all_goals reduce_mod_char!
  all_goals ring))

theorem source_coefficient_identities :
    targetX^2=100+29*alpha+59*alpha^2 ∧
    -2*targetX=32+34*alpha+27*alpha^2 ∧
    -2*a*targetX-4*b=55+71*alpha+31*alpha^2 ∧
    -4*targetX^2-2*a=17+83*alpha+48*alpha^2 ∧
    a^2-4*b*targetX=81+16*alpha+32*alpha^2 := by
  refine ⟨?_,?_,?_,?_,?_⟩
  all_goals simp only [targetX,a,b]
  all_goals alpha_ring

theorem source_polynomial_identity (x y : K) :
    SemaevS3.S3 a b x y targetX=MvPolynomial.eval ![x+y,x*y] symmetric := by
  simp [SemaevS3.S3,symmetric]
  ring

theorem coefficient_decomposition :
    symmetric=MvPolynomial.map (algebraMap F K) f0+
      C alpha*MvPolynomial.map (algebraMap F K) f1+
      C (alpha^2)*MvPolynomial.map (algebraMap F K) f2 := by
  unfold symmetric
  rw [source_coefficient_identities.2.2.2.2,source_coefficient_identities.2.2.2.1,
    source_coefficient_identities.2.2.1,source_coefficient_identities.2.1,
    source_coefficient_identities.1]
  simp [f0,f1,f2,JVMacaulayAudit.f0,JVMacaulayAudit.f1,JVMacaulayAudit.f2]
  norm_num only [map_ofNat]
  ring

def descend (i : ℕ) (g : ExtPoly) : Poly :=
  .ofCoeff (Finsupp.mapRange (coordinate i) (coordinate_zero i) (AddMonoidAlgebra.coeff g))

theorem coeff_descend (i : ℕ) (m : Fin 2 →₀ ℕ) (g : ExtPoly) :
    (descend i g).coeff m=coordinate i (g.coeff m) := rfl

theorem coefficient_as_representation (m : Fin 2 →₀ ℕ) :
    symmetric.coeff m=representation (f0.coeff m) (f1.coeff m) (f2.coeff m) := by
  rw [coefficient_decomposition]
  simp only [AddMonoidAlgebra.coeff_add,Finsupp.add_apply,coeff_C_mul,coeff_map,representation]
  ring

theorem actual_descent : descend 0 symmetric=f0 ∧ descend 1 symmetric=f1 ∧
    descend 2 symmetric=f2 := by
  refine ⟨?_,?_,?_⟩
  all_goals ext m
  all_goals rw [coeff_descend,coefficient_as_representation]
  all_goals simp [coordinate_representation]

theorem evaluated_descent (u v : F) :
    MvPolynomial.eval ![algebraMap F K u,algebraMap F K v] symmetric=
      representation (JVMacaulayAudit.f0 u v 1) (JVMacaulayAudit.f1 u v 1)
        (JVMacaulayAudit.f2 u v 1) := by
  rw [coefficient_decomposition]
  simp [f0,f1,f2,JVMacaulayAudit.f0,JVMacaulayAudit.f1,JVMacaulayAudit.f2,
    representation]
  norm_num only [map_ofNat]
  ring

theorem source_symmetric_has_no_base_solution (u v : F) :
    MvPolynomial.eval ![algebraMap F K u,algebraMap F K v] symmetric≠0 := by
  rw [evaluated_descent]
  intro hz
  exact JVMacaulayAudit.actual_affine_no_geometric_solution u v
    ((representation_zero_iff _ _ _).mp hz)

theorem source_S3_has_no_base_solution (u v : F) :
    SemaevS3.S3 a b (algebraMap F K u) (algebraMap F K v) targetX≠0 := by
  rw [source_polynomial_identity]
  simpa only [map_add,map_mul] using source_symmetric_has_no_base_solution (u+v) (u*v)

end
end JVSourceFieldAudit
